use regex::Regex;
use std::collections::HashMap;

/// Type of SPARQL query
#[derive(Debug, Clone, PartialEq)]
pub enum QueryType {
    Select,
    Construct,
    Ask,
    Describe,
}

/// Parsed SPARQL query structure containing all components extracted from the query
#[derive(Debug, Clone)]
pub struct ParsedSparqlQuery {
    /// Type of query (SELECT, CONSTRUCT, ASK, DESCRIBE)
    pub query_type: QueryType,
    /// Prefix mappings
    pub prefixes: HashMap<String, String>,
    /// SELECT clause (variables or *)
    pub select_clause: String,
    /// FROM clauses (default graphs)
    pub from_clauses: Vec<String>,
    /// FROM NAMED clauses (named graphs)
    pub from_named_clauses: Vec<String>,
    /// WHERE clause
    pub where_clause: String,
    /// ORDER BY clause
    pub order_by: Option<String>,
    /// LIMIT value
    pub limit: Option<u64>,
    /// OFFSET value
    pub offset: Option<u64>,
    /// DISTINCT flag
    pub distinct: bool,
    /// REDUCED flag
    pub reduced: bool,
    /// Original query
    pub original_query: String,
}

/// Parser for SPARQL queries
pub struct SparqlParser {
    prefix: Regex,
    select: Regex,
    construct: Regex,
    ask: Regex,
    describe: Regex,
    from: Regex,
    from_named: Regex,
    order_by: Regex,
    limit: Regex,
    offset: Regex,
}

impl SparqlParser {
    /// Creates a new SparqlParser instance
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(SparqlParser {
            prefix: Regex::new(r"(?i)PREFIX\s+([^\s]+):\s*<([^>]+)>")?,
            select: Regex::new(r"(?i)SELECT\s+(DISTINCT\s+|REDUCED\s+)?(.+?)(?:WHERE|FROM|\{)")?,
            construct: Regex::new(r"(?i)CONSTRUCT\s*\{")?,
            ask: Regex::new(r"(?i)ASK\s*\{")?,
            describe: Regex::new(r"(?i)DESCRIBE\s+(.+?)(?:WHERE|FROM|\{)")?,
            from: Regex::new(r"(?i)FROM\s+(?!NAMED)(<[^>]+>|\S+)")?,
            from_named: Regex::new(r"(?i)FROM\s+NAMED\s+(<[^>]+>|\S+)")?,
            order_by: Regex::new(r"(?i)ORDER\s+BY\s+(.+?)(?:LIMIT|OFFSET|$)")?,
            limit: Regex::new(r"(?i)LIMIT\s+(\d+)")?,
            offset: Regex::new(r"(?i)OFFSET\s+(\d+)")?,
        })
    }

    /// Parses a SPARQL query string
    pub fn parse(&self, query: &str) -> Result<ParsedSparqlQuery, Box<dyn std::error::Error>> {
        let mut parsed = ParsedSparqlQuery {
            query_type: QueryType::Select,
            prefixes: HashMap::new(),
            select_clause: String::new(),
            from_clauses: Vec::new(),
            from_named_clauses: Vec::new(),
            where_clause: String::new(),
            order_by: None,
            limit: None,
            offset: None,
            distinct: false,
            reduced: false,
            original_query: query.to_string(),
        };

        let lines: Vec<&str> = query.lines().collect();
        let mut in_where_clause = false;
        let mut where_lines: Vec<&str> = Vec::new();
        let mut brace_count = 0;

        // Determine query type
        parsed.query_type = self.determine_query_type(query)?;

        for line in &lines {
            let trimmed_line = line.trim();

            if trimmed_line.is_empty() {
                if in_where_clause {
                    where_lines.push(line);
                }
                continue;
            }

            // Extract prefixes
            if trimmed_line.to_uppercase().starts_with("PREFIX") {
                if let Some(captures) = self.prefix.captures(trimmed_line) {
                    let prefix = captures.get(1).unwrap().as_str().to_string();
                    let namespace = captures.get(2).unwrap().as_str().to_string();
                    parsed.prefixes.insert(prefix, namespace);
                }
            }
            // Extract SELECT clause
            else if trimmed_line.to_uppercase().starts_with("SELECT") {
                if let Some(captures) = self.select.captures(trimmed_line) {
                    let modifier = captures.get(1).map(|m| m.as_str().trim()).unwrap_or("");
                    let vars = captures.get(2).unwrap().as_str().trim();

                    if modifier.to_uppercase().contains("DISTINCT") {
                        parsed.distinct = true;
                    }
                    if modifier.to_uppercase().contains("REDUCED") {
                        parsed.reduced = true;
                    }

                    parsed.select_clause = vars.to_string();
                }
            }
            // Extract FROM clauses
            else if trimmed_line.to_uppercase().starts_with("FROM NAMED") {
                if let Some(captures) = self.from_named.captures(trimmed_line) {
                    let graph = captures.get(1).unwrap().as_str();
                    parsed
                        .from_named_clauses
                        .push(self.unwrap_iri(graph, &parsed.prefixes));
                }
            } else if trimmed_line.to_uppercase().starts_with("FROM") {
                if let Some(captures) = self.from.captures(trimmed_line) {
                    let graph = captures.get(1).unwrap().as_str();
                    parsed
                        .from_clauses
                        .push(self.unwrap_iri(graph, &parsed.prefixes));
                }
            }
            // Track WHERE clause
            else if trimmed_line.to_uppercase().starts_with("WHERE")
                || trimmed_line.starts_with('{')
            {
                in_where_clause = true;
                where_lines.push(line);
                brace_count += trimmed_line.matches('{').count();
                brace_count -= trimmed_line.matches('}').count();
            } else if in_where_clause {
                brace_count += trimmed_line.matches('{').count();
                brace_count -= trimmed_line.matches('}').count();
                where_lines.push(line);

                // Stop collecting WHERE clause when braces are balanced
                if brace_count == 0 {
                    in_where_clause = false;
                }
            }
        }

        parsed.where_clause = where_lines.join("\n");

        // Extract ORDER BY
        if let Some(captures) = self.order_by.captures(query) {
            parsed.order_by = Some(captures.get(1).unwrap().as_str().trim().to_string());
        }

        // Extract LIMIT
        if let Some(captures) = self.limit.captures(query) {
            parsed.limit = Some(captures.get(1).unwrap().as_str().parse()?);
        }

        // Extract OFFSET
        if let Some(captures) = self.offset.captures(query) {
            parsed.offset = Some(captures.get(1).unwrap().as_str().parse()?);
        }

        Ok(parsed)
    }

    /// Determines the type of SPARQL query
    fn determine_query_type(&self, query: &str) -> Result<QueryType, Box<dyn std::error::Error>> {
        let upper_query = query.to_uppercase();

        if self.select.is_match(query) {
            Ok(QueryType::Select)
        } else if self.construct.is_match(query) {
            Ok(QueryType::Construct)
        } else if self.ask.is_match(query) {
            Ok(QueryType::Ask)
        } else if self.describe.is_match(query) {
            Ok(QueryType::Describe)
        } else if upper_query.contains("SELECT") {
            Ok(QueryType::Select)
        } else {
            Err("Unable to determine query type".into())
        }
    }

    /// Unwraps a prefixed IRI to its full form
    fn unwrap_iri(&self, prefixed_iri: &str, prefix_mapper: &HashMap<String, String>) -> String {
        let trimmed = prefixed_iri.trim();

        // Already a full IRI
        if trimmed.starts_with('<') && trimmed.ends_with('>') {
            return trimmed[1..trimmed.len() - 1].to_string();
        }

        // Handle prefixed form
        if let Some(colon_pos) = trimmed.find(':') {
            let prefix = &trimmed[..colon_pos];
            let local_part = &trimmed[colon_pos + 1..];
            if let Some(namespace) = prefix_mapper.get(prefix) {
                return format!("{}{}", namespace, local_part);
            }
        }

        trimmed.to_string()
    }

    /// Wraps an IRI with prefix if available
    pub fn wrap_iri(&self, iri: &str, prefixes: &HashMap<String, String>) -> String {
        for (prefix, namespace) in prefixes {
            if iri.starts_with(namespace) {
                let local_part = &iri[namespace.len()..];
                return format!("{}:{}", prefix, local_part);
            }
        }
        format!("<{}>", iri)
    }

    /// Extracts GRAPH patterns from WHERE clause
    pub fn extract_graph_patterns(&self, where_clause: &str) -> Vec<String> {
        let graph_regex = Regex::new(r"(?i)GRAPH\s+(<[^>]+>|\S+)").unwrap();
        graph_regex
            .captures_iter(where_clause)
            .map(|cap| cap.get(1).unwrap().as_str().to_string())
            .collect()
    }

    /// Extracts variables from SELECT clause
    pub fn extract_variables(&self, select_clause: &str) -> Vec<String> {
        if select_clause.trim() == "*" {
            return vec!["*".to_string()];
        }

        let var_regex = Regex::new(r"\?(\w+)").unwrap();
        var_regex
            .captures_iter(select_clause)
            .map(|cap| format!("?{}", cap.get(1).unwrap().as_str()))
            .collect()
    }
}

impl Default for SparqlParser {
    fn default() -> Self {
        Self::new().expect("Failed to create SparqlParser")
    }
}

impl ParsedSparqlQuery {
    /// Creates a new empty ParsedSparqlQuery
    pub fn new() -> Self {
        Self {
            query_type: QueryType::Select,
            prefixes: HashMap::new(),
            select_clause: String::new(),
            from_clauses: Vec::new(),
            from_named_clauses: Vec::new(),
            where_clause: String::new(),
            order_by: None,
            limit: None,
            offset: None,
            distinct: false,
            reduced: false,
            original_query: String::new(),
        }
    }

    /// Reconstructs the SPARQL query from parsed components
    pub fn to_query_string(&self) -> String {
        let mut lines: Vec<String> = Vec::new();

        // Add prefixes
        for (prefix, namespace) in &self.prefixes {
            lines.push(format!("PREFIX {}: <{}>", prefix, namespace));
        }

        if !self.prefixes.is_empty() {
            lines.push(String::new());
        }

        // Add query type and clause
        match self.query_type {
            QueryType::Select => {
                let mut select = "SELECT".to_string();
                if self.distinct {
                    select.push_str(" DISTINCT");
                }
                if self.reduced {
                    select.push_str(" REDUCED");
                }
                select.push_str(&format!(" {}", self.select_clause));
                lines.push(select);
            }
            QueryType::Construct => {
                lines.push("CONSTRUCT {".to_string());
            }
            QueryType::Ask => {
                lines.push("ASK".to_string());
            }
            QueryType::Describe => {
                lines.push(format!("DESCRIBE {}", self.select_clause));
            }
        }

        // Add FROM clauses
        for from in &self.from_clauses {
            lines.push(format!("FROM <{}>", from));
        }

        for from_named in &self.from_named_clauses {
            lines.push(format!("FROM NAMED <{}>", from_named));
        }

        // Add WHERE clause
        if !self.where_clause.is_empty() {
            lines.push(self.where_clause.clone());
        }

        // Add solution modifiers
        if let Some(ref order_by) = self.order_by {
            lines.push(format!("ORDER BY {}", order_by));
        }

        if let Some(limit) = self.limit {
            lines.push(format!("LIMIT {}", limit));
        }

        if let Some(offset) = self.offset {
            lines.push(format!("OFFSET {}", offset));
        }

        lines.join("\n")
    }
}

impl Default for ParsedSparqlQuery {
    fn default() -> Self {
        Self::new()
    }
}
