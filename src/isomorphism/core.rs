use crate::parsing::janusql_parser::JanusQLParser;
use crate::parsing::rspql_parser::RSPQLParser;
use crate::parsing::sparql_parser::SparqlParser;
use regex::Regex;

/// Supported query types for isomorphism checking
#[derive(Debug, Clone, PartialEq)]
pub enum QueryLanguage {
    SPARQL,
    RSPQL,
    JanusQL,
}

/// A simple triple representation for BGP extraction
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Triple {
    pub subject: TripleNode,
    pub predicate: TripleNode,
    pub object: TripleNode,
}

/// Node types in a triple
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TripleNode {
    IRI(String),
    Variable(String),
    Literal(String),
    BlankNode(String),
}

/// Result of parsing a query for isomorphism checking
#[derive(Debug)]
pub struct IsomorphismQuery {
    pub query_language: QueryLanguage,
    pub bgp: Vec<Triple>,
    pub stream_name: Option<String>,
    pub window_name: Option<String>,
    pub width: Option<i64>,
    pub slide: Option<i64>,
    pub offset: Option<u64>,
    pub start: Option<u64>,
    pub end: Option<u64>,
}

/// Main API for checking query isomorphism
pub struct QueryIsomorphism;

impl QueryIsomorphism {
    /// Detect the query language type
    ///
    /// JanusQL is an extension of RSP-QL that adds support for historical windows.
    /// Detection priority:
    /// 1. JanusQL - if historical window keywords are present (OFFSET, START, END)
    /// 2. RSP-QL - if streaming keywords are present (REGISTER, STREAM, or window syntax)
    /// 3. SPARQL - default for standard queries
    pub fn detect_query_type(query: &str) -> QueryLanguage {
        let upper = query.to_uppercase();

        // JanusQL extends RSP-QL with historical windows
        // Check for JanusQL-specific keywords (OFFSET with sliding window, or START/END for fixed window)
        if (upper.contains("OFFSET") && upper.contains("RANGE") && upper.contains("STEP"))
            || (upper.contains("START") && upper.contains("END"))
        {
            return QueryLanguage::JanusQL;
        }

        // RSP-QL queries with REGISTER operator (can also be JanusQL if historical keywords present)
        if upper.contains("REGISTER") && upper.contains("STREAM") {
            return QueryLanguage::RSPQL;
        }

        // RSP-QL queries without REGISTER (direct window syntax)
        if upper.contains("FROM")
            && upper.contains("NAMED")
            && upper.contains("WINDOW")
            && upper.contains("ON STREAM")
        {
            return QueryLanguage::RSPQL;
        }

        // Standard SPARQL queries
        QueryLanguage::SPARQL
    }

    /// Parse a query based on its detected type
    pub fn parse_query(query: &str) -> Result<IsomorphismQuery, Box<dyn std::error::Error>> {
        let query_type = Self::detect_query_type(query);

        match query_type {
            QueryLanguage::SPARQL => Self::parse_sparql(query),
            QueryLanguage::RSPQL => Self::parse_rspql(query),
            QueryLanguage::JanusQL => Self::parse_janusql(query),
        }
    }

    /// Parse a SPARQL query
    fn parse_sparql(query: &str) -> Result<IsomorphismQuery, Box<dyn std::error::Error>> {
        let parser = SparqlParser::new()?;
        let parsed = parser.parse(query)?;
        let bgp = Self::extract_bgp_from_where(&parsed.where_clause)?;

        Ok(IsomorphismQuery {
            query_language: QueryLanguage::SPARQL,
            bgp,
            stream_name: None,
            window_name: None,
            width: None,
            slide: None,
            offset: None,
            start: None,
            end: None,
        })
    }

    /// Parse an RSPQL query
    fn parse_rspql(query: &str) -> Result<IsomorphismQuery, Box<dyn std::error::Error>> {
        let parser = RSPQLParser::new(query.to_string());
        let parsed = parser.parse();
        let bgp = Self::extract_bgp_from_where(&parsed.sparql_query)?;

        let (stream_name, window_name, width, slide) = if !parsed.s2r.is_empty() {
            let window = &parsed.s2r[0];
            (
                Some(window.stream_name.clone()),
                Some(window.window_name.clone()),
                Some(window.width),
                Some(window.slide),
            )
        } else {
            (None, None, None, None)
        };

        Ok(IsomorphismQuery {
            query_language: QueryLanguage::RSPQL,
            bgp,
            stream_name,
            window_name,
            width,
            slide,
            offset: None,
            start: None,
            end: None,
        })
    }

    /// Parse a JanusQL query
    fn parse_janusql(query: &str) -> Result<IsomorphismQuery, Box<dyn std::error::Error>> {
        let parser = JanusQLParser::new()?;
        let parsed = parser.parse(query)?;
        let bgp = Self::extract_bgp_from_where(&parsed.where_clause)?;

        let (stream_name, window_name, width, slide, offset, start, end) =
            if !parsed.live_windows.is_empty() {
                let window = &parsed.live_windows[0];
                (
                    Some(window.stream_name.clone()),
                    Some(window.window_name.clone()),
                    Some(window.width as i64),
                    Some(window.slide as i64),
                    None,
                    None,
                    None,
                )
            } else if !parsed.historical_windows.is_empty() {
                let window = &parsed.historical_windows[0];
                (
                    Some(window.stream_name.clone()),
                    Some(window.window_name.clone()),
                    Some(window.width as i64),
                    Some(window.slide as i64),
                    window.offset,
                    window.start,
                    window.end,
                )
            } else {
                (None, None, None, None, None, None, None)
            };

        Ok(IsomorphismQuery {
            query_language: QueryLanguage::JanusQL,
            bgp,
            stream_name,
            window_name,
            width,
            slide,
            offset,
            start,
            end,
        })
    }

    /// Extract Basic Graph Pattern from WHERE clause
    fn extract_bgp_from_where(
        where_clause: &str,
    ) -> Result<Vec<Triple>, Box<dyn std::error::Error>> {
        let mut bgp = Vec::new();

        // Extract content between braces
        let content = Self::extract_inner_braces(where_clause);

        if content.is_empty() {
            return Ok(bgp);
        }

        // Parse triples using regex - simple pattern for SPO with dots
        let triple_pattern = Regex::new(
            r#"([?$]\w+|<[^>]+>|[\w:]+)\s+([?$]\w+|<[^>]+>|[\w:]+|a)\s+([?$]\w+|<[^>]+>|[\w:]+|'[^']*'|"[^"]*")\s*\."#,
        )?;

        for caps in triple_pattern.captures_iter(&content) {
            let subject = Self::parse_node(caps.get(1).unwrap().as_str());
            let predicate = if caps.get(2).unwrap().as_str() == "a" {
                TripleNode::IRI("http://www.w3.org/1999/02/22-rdf-syntax-ns#type".to_string())
            } else {
                Self::parse_node(caps.get(2).unwrap().as_str())
            };
            let object = Self::parse_node(caps.get(3).unwrap().as_str());

            bgp.push(Triple {
                subject,
                predicate,
                object,
            });
        }

        Ok(bgp)
    }

    /// Extract content from innermost braces
    fn extract_inner_braces(text: &str) -> String {
        let mut result = String::new();
        let mut depth = 0;
        let mut start_collecting = false;

        for ch in text.chars() {
            match ch {
                '{' => {
                    depth += 1;
                    if depth == 1 {
                        start_collecting = true;
                    }
                }
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        start_collecting = false;
                    }
                }
                _ => {
                    if start_collecting && depth == 1 {
                        result.push(ch);
                    }
                }
            }
        }

        result.trim().to_string()
    }

    /// Parse a node from string representation
    fn parse_node(node_str: &str) -> TripleNode {
        let trimmed = node_str.trim();

        if trimmed.starts_with('?') || trimmed.starts_with('$') {
            TripleNode::Variable(trimmed[1..].to_string())
        } else if trimmed.starts_with('<') && trimmed.ends_with('>') {
            TripleNode::IRI(trimmed[1..trimmed.len() - 1].to_string())
        } else if trimmed.starts_with('"') || trimmed.starts_with('\'') {
            TripleNode::Literal(trimmed.trim_matches(|c| c == '"' || c == '\'').to_string())
        } else if let Some(stripped) = trimmed.strip_prefix("_:") {
            TripleNode::BlankNode(stripped.to_string())
        } else {
            // Assume it's a prefixed IRI
            TripleNode::IRI(trimmed.to_string())
        }
    }

    /// Convert BGP to normalized graph format (as Vec of string triples)
    fn bgp_to_normalized_graph(bgp: &[Triple]) -> Vec<(String, String, String)> {
        bgp.iter()
            .map(|triple| {
                let s = Self::node_to_string(&triple.subject);
                let p = Self::node_to_string(&triple.predicate);
                let o = Self::node_to_string(&triple.object);
                (s, p, o)
            })
            .collect()
    }

    /// Convert TripleNode to string, replacing variables with blank nodes
    fn node_to_string(node: &TripleNode) -> String {
        match node {
            TripleNode::IRI(iri) => format!("<{}>", iri),
            TripleNode::Variable(var) => format!("_:{}", var), // Variables become blank nodes
            TripleNode::Literal(lit) => format!("\"{}\"", lit),
            TripleNode::BlankNode(id) => format!("_:{}", id),
        }
    }

    /// Check if stream parameters are equal
    /// For historical windows, also checks offset, start, and end times
    fn check_stream_parameters_equal(q1: &IsomorphismQuery, q2: &IsomorphismQuery) -> bool {
        q1.stream_name == q2.stream_name
            && q1.width == q2.width
            && q1.slide == q2.slide
            && q1.offset == q2.offset
            && q1.start == q2.start
            && q1.end == q2.end
    }

    /// Check if window names are equal
    fn check_window_names_equal(q1: &IsomorphismQuery, q2: &IsomorphismQuery) -> bool {
        q1.window_name == q2.window_name
    }

    /// Check if two queries are isomorphic
    pub fn is_isomorphic(
        query_one: &str,
        query_two: &str,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let q1 = Self::parse_query(query_one)?;
        let q2 = Self::parse_query(query_two)?;

        // For RSPQL and JanusQL, check stream parameters first
        if q1.query_language != QueryLanguage::SPARQL || q2.query_language != QueryLanguage::SPARQL
        {
            if !Self::check_stream_parameters_equal(&q1, &q2) {
                return Ok(false);
            }
            if !Self::check_window_names_equal(&q1, &q2) {
                return Ok(false);
            }
        }

        // Check BGP isomorphism
        Ok(Self::check_bgp_isomorphism(&q1.bgp, &q2.bgp))
    }

    /// Check if two BGPs are isomorphic using hash-based graph isomorphism
    fn check_bgp_isomorphism(bgp1: &[Triple], bgp2: &[Triple]) -> bool {
        if bgp1.len() != bgp2.len() {
            return false;
        }

        // Use graph isomorphism checker for proper isomorphism checking
        match crate::isomorphism::graph_isomorphism::GraphIsomorphism::check_bgp_isomorphism(
            bgp1, bgp2,
        ) {
            Ok(result) => result,
            Err(_) => {
                // Fallback to simple comparison if graph isomorphism fails
                let graph1 = Self::bgp_to_normalized_graph(bgp1);
                let graph2 = Self::bgp_to_normalized_graph(bgp2);
                let mut g1_sorted = graph1.clone();
                let mut g2_sorted = graph2.clone();
                g1_sorted.sort();
                g2_sorted.sort();
                g1_sorted == g2_sorted
            }
        }
    }

    /// Generate BGP quads from a query string (similar to TypeScript version)
    pub fn generate_bgp_quads_from_query(
        query: &str,
    ) -> Result<Vec<Triple>, Box<dyn std::error::Error>> {
        let parsed = Self::parse_query(query)?;
        Ok(parsed.bgp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_sparql() {
        let query = "SELECT ?s ?p ?o WHERE { ?s ?p ?o }";
        assert_eq!(
            QueryIsomorphism::detect_query_type(query),
            QueryLanguage::SPARQL
        );
    }

    #[test]
    fn test_detect_rspql() {
        let query = "REGISTER STREAM <output> AS SELECT ?s ?p ?o FROM NAMED WINDOW <w> ON STREAM <s> [RANGE 10 STEP 5]";
        assert_eq!(
            QueryIsomorphism::detect_query_type(query),
            QueryLanguage::RSPQL
        );
    }

    #[test]
    fn test_parse_node_variable() {
        let node = QueryIsomorphism::parse_node("?var");
        assert!(matches!(node, TripleNode::Variable(_)));
    }

    #[test]
    fn test_parse_node_iri() {
        let node = QueryIsomorphism::parse_node("<http://example.org/resource>");
        assert!(matches!(node, TripleNode::IRI(_)));
    }

    #[test]
    fn test_bgp_extraction() {
        let where_clause = "WHERE { ?s <http://example.org/p> ?o . }";
        let bgp = QueryIsomorphism::extract_bgp_from_where(where_clause).unwrap();
        assert_eq!(bgp.len(), 1);
    }
}
