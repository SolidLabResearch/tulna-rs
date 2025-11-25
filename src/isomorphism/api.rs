use crate::isomorphism::core::{IsomorphismQuery, QueryIsomorphism, QueryLanguage, Triple};
use std::error::Error;

/// High-level API for query isomorphism checking
///
/// This module provides a simple interface for checking if two queries are isomorphic,
/// similar to the TypeScript implementation.
pub struct QueryIsomorphismAPI;

impl QueryIsomorphismAPI {
    /// Check if two queries are isomorphic to each other.
    ///
    /// This function:
    /// 1. Auto-detects the query language (SPARQL, RSPQL, or JanusQL)
    /// 2. Parses both queries
    /// 3. For streaming queries (RSPQL/JanusQL), checks if stream parameters match
    /// 4. Checks if the Basic Graph Patterns (BGP) are isomorphic
    ///
    /// # Arguments
    /// * `query_one` - The first query string
    /// * `query_two` - The second query string
    ///
    /// # Returns
    /// * `Ok(true)` - If the queries are isomorphic
    /// * `Ok(false)` - If the queries are not isomorphic
    /// * `Err` - If there was an error parsing the queries
    ///
    /// # Example
    /// ```
    /// use tulna_rs::isomorphism::api::QueryIsomorphismAPI;
    ///
    /// let query1 = "SELECT ?s ?p ?o WHERE { ?s ?p ?o }";
    /// let query2 = "SELECT ?x ?y ?z WHERE { ?x ?y ?z }";
    ///
    /// let is_iso = QueryIsomorphismAPI::is_isomorphic(query1, query2).unwrap();
    /// assert!(is_iso);
    /// ```
    pub fn is_isomorphic(query_one: &str, query_two: &str) -> Result<bool, Box<dyn Error>> {
        QueryIsomorphism::is_isomorphic(query_one, query_two)
    }

    /// Parse a query and return its structured representation
    ///
    /// # Arguments
    /// * `query` - The query string to parse
    ///
    /// # Returns
    /// * `Ok(IsomorphismQuery)` - The parsed query structure
    /// * `Err` - If there was an error parsing the query
    pub fn parse_query(query: &str) -> Result<IsomorphismQuery, Box<dyn Error>> {
        QueryIsomorphism::parse_query(query)
    }

    /// Detect the query language type
    ///
    /// # Arguments
    /// * `query` - The query string to analyze
    ///
    /// # Returns
    /// * `QueryLanguage` - The detected language (SPARQL, RSPQL, or JanusQL)
    pub fn detect_query_language(query: &str) -> QueryLanguage {
        QueryIsomorphism::detect_query_type(query)
    }

    /// Extract the Basic Graph Pattern from a query
    ///
    /// # Arguments
    /// * `query` - The query string to extract BGP from
    ///
    /// # Returns
    /// * `Ok(Vec<Triple>)` - The extracted BGP as a vector of triples
    /// * `Err` - If there was an error parsing the query
    pub fn extract_bgp(query: &str) -> Result<Vec<Triple>, Box<dyn Error>> {
        QueryIsomorphism::generate_bgp_quads_from_query(query)
    }

    /// Check if two RSPQL queries have matching stream parameters
    ///
    /// # Arguments
    /// * `query_one` - The first RSPQL query
    /// * `query_two` - The second RSPQL query
    ///
    /// # Returns
    /// * `Ok(true)` - If stream parameters match
    /// * `Ok(false)` - If stream parameters don't match
    /// * `Err` - If there was an error parsing the queries
    pub fn check_stream_parameters(
        query_one: &str,
        query_two: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let q1 = QueryIsomorphism::parse_query(query_one)?;
        let q2 = QueryIsomorphism::parse_query(query_two)?;

        Ok(q1.stream_name == q2.stream_name
            && q1.width == q2.width
            && q1.slide == q2.slide
            && q1.offset == q2.offset
            && q1.start == q2.start
            && q1.end == q2.end)
    }

    /// Check if two queries have matching window names
    ///
    /// # Arguments
    /// * `query_one` - The first query
    /// * `query_two` - The second query
    ///
    /// # Returns
    /// * `Ok(true)` - If window names match
    /// * `Ok(false)` - If window names don't match
    /// * `Err` - If there was an error parsing the queries
    pub fn check_window_names(query_one: &str, query_two: &str) -> Result<bool, Box<dyn Error>> {
        let q1 = QueryIsomorphism::parse_query(query_one)?;
        let q2 = QueryIsomorphism::parse_query(query_two)?;

        Ok(q1.window_name == q2.window_name)
    }

    /// Compare two queries and provide detailed information about their similarity
    ///
    /// # Arguments
    /// * `query_one` - The first query
    /// * `query_two` - The second query
    ///
    /// # Returns
    /// * `Ok(QueryComparisonResult)` - Detailed comparison information
    /// * `Err` - If there was an error parsing the queries
    pub fn compare_queries(
        query_one: &str,
        query_two: &str,
    ) -> Result<QueryComparisonResult, Box<dyn Error>> {
        let q1 = QueryIsomorphism::parse_query(query_one)?;
        let q2 = QueryIsomorphism::parse_query(query_two)?;

        let same_language = q1.query_language == q2.query_language;
        let same_stream_params = q1.stream_name == q2.stream_name
            && q1.width == q2.width
            && q1.slide == q2.slide
            && q1.offset == q2.offset
            && q1.start == q2.start
            && q1.end == q2.end;
        let same_window_name = q1.window_name == q2.window_name;
        let same_bgp_size = q1.bgp.len() == q2.bgp.len();

        // Check BGP isomorphism
        let bgp_isomorphic = if same_bgp_size {
            crate::isomorphism::graph_isomorphism::GraphIsomorphism::check_bgp_isomorphism(
                &q1.bgp, &q2.bgp,
            )
            .unwrap_or(false)
        } else {
            false
        };

        let is_isomorphic = if q1.query_language != QueryLanguage::SPARQL
            || q2.query_language != QueryLanguage::SPARQL
        {
            same_language && same_stream_params && same_window_name && bgp_isomorphic
        } else {
            same_language && bgp_isomorphic
        };

        Ok(QueryComparisonResult {
            is_isomorphic,
            same_language,
            same_stream_params,
            same_window_name,
            same_bgp_size,
            bgp_isomorphic,
            query1_language: q1.query_language,
            query2_language: q2.query_language,
            query1_bgp_size: q1.bgp.len(),
            query2_bgp_size: q2.bgp.len(),
        })
    }
}

/// Detailed comparison result between two queries
#[derive(Debug, Clone)]
pub struct QueryComparisonResult {
    /// Whether the queries are isomorphic
    pub is_isomorphic: bool,
    /// Whether both queries use the same language
    pub same_language: bool,
    /// Whether stream parameters match (for RSPQL/JanusQL)
    pub same_stream_params: bool,
    /// Whether window names match
    pub same_window_name: bool,
    /// Whether BGP sizes match
    pub same_bgp_size: bool,
    /// Whether BGPs are isomorphic
    pub bgp_isomorphic: bool,
    /// Language of first query
    pub query1_language: QueryLanguage,
    /// Language of second query
    pub query2_language: QueryLanguage,
    /// BGP size of first query
    pub query1_bgp_size: usize,
    /// BGP size of second query
    pub query2_bgp_size: usize,
}

impl QueryComparisonResult {
    /// Get a human-readable summary of the comparison
    pub fn summary(&self) -> String {
        let mut lines = vec![
            format!("Isomorphic: {}", self.is_isomorphic),
            format!(
                "Same Language: {} ({:?} vs {:?})",
                self.same_language, self.query1_language, self.query2_language
            ),
            format!(
                "Same BGP Size: {} ({} vs {})",
                self.same_bgp_size, self.query1_bgp_size, self.query2_bgp_size
            ),
            format!("BGP Isomorphic: {}", self.bgp_isomorphic),
        ];

        if self.query1_language != QueryLanguage::SPARQL
            || self.query2_language != QueryLanguage::SPARQL
        {
            lines.push(format!("Same Stream Params: {}", self.same_stream_params));
            lines.push(format!("Same Window Name: {}", self.same_window_name));
        }

        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparql_isomorphism() {
        let query1 = "SELECT ?s ?p ?o WHERE { ?s ?p ?o . }";
        let query2 = "SELECT ?x ?y ?z WHERE { ?x ?y ?z . }";

        let result = QueryIsomorphismAPI::is_isomorphic(query1, query2);
        if let Err(e) = &result {
            println!("Error checking isomorphism: {}", e);
        }
        assert!(result.is_ok());
    }

    #[test]
    fn test_detect_language() {
        let sparql = "SELECT ?s ?p ?o WHERE { ?s ?p ?o }";
        assert_eq!(
            QueryIsomorphismAPI::detect_query_language(sparql),
            QueryLanguage::SPARQL
        );
    }

    #[test]
    fn test_extract_bgp() {
        let query = "SELECT ?s ?p ?o WHERE { ?s ?p ?o . }";
        let bgp = QueryIsomorphismAPI::extract_bgp(query);
        if let Err(e) = &bgp {
            println!("Error extracting BGP: {}", e);
        }
        assert!(bgp.is_ok());
    }

    #[test]
    fn test_compare_queries() {
        let query1 = "SELECT ?s ?p ?o WHERE { ?s ?p ?o . }";
        let query2 = "SELECT ?x ?y ?z WHERE { ?x ?y ?z . }";

        let result = QueryIsomorphismAPI::compare_queries(query1, query2);
        if let Err(e) = &result {
            println!("Error comparing queries: {}", e);
        }
        assert!(result.is_ok());

        let comparison = result.unwrap();
        assert!(comparison.same_language);
        assert!(comparison.same_bgp_size);
    }
}
