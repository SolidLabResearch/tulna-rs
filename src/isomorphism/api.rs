use crate::isomorphism::core::{IsomorphismQuery, QueryIsomorphism, Triple};
use crate::TulnaError;

/// Public API for checking query isomorphism
///
/// This struct exposes high-level methods for comparing queries across different languages
/// (SPARQL, RSP-QL, JanusQL). It serves as the main entry point for library users.
pub struct QueryIsomorphismAPI;

impl QueryIsomorphismAPI {
    /// Check if two queries are isomorphic
    ///
    /// This method automatically detects the query language of both inputs, parses them,
    /// and checks for semantic equivalence. It supports:
    /// - Variable renaming (canonicalization)
    /// - Blank node isomorphism
    /// - Stream/Window parameter comparison (for streaming queries)
    ///
    /// # Arguments
    ///
    /// * `query1` - First query string
    /// * `query2` - Second query string
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - Queries are isomorphic
    /// * `Ok(false)` - Queries are not isomorphic
    /// * `Err(_)` - Error parsing or processing queries
    pub fn is_isomorphic(query1: &str, query2: &str) -> Result<bool, TulnaError> {
        QueryIsomorphism::is_isomorphic(query1, query2)
    }

    /// Detect the language of a query
    ///
    /// # Arguments
    ///
    /// * `query` - Query string
    ///
    /// # Returns
    ///
    /// Detected `QueryLanguage` (SPARQL, RSPQL, or JanusQL)
    pub fn detect_query_language(query: &str) -> crate::isomorphism::core::QueryLanguage {
        QueryIsomorphism::detect_query_type(query)
    }

    /// Extract the Basic Graph Pattern (BGP) from a query
    ///
    /// This is useful for debugging or for applications that only need to analyze
    /// the graph pattern part of a query.
    ///
    /// # Arguments
    ///
    /// * `query` - Query string
    ///
    /// # Returns
    ///
    /// Vector of `Triple` objects representing the BGP
    pub fn extract_bgp(query: &str) -> Result<Vec<Triple>, TulnaError> {
        QueryIsomorphism::generate_bgp_quads_from_query(query)
    }

    /// Parse a query into its structured representation
    ///
    /// # Arguments
    ///
    /// * `query` - Query string
    ///
    /// # Returns
    ///
    /// `IsomorphismQuery` structure containing parsed components
    pub fn parse_query(query: &str) -> Result<IsomorphismQuery, TulnaError> {
        QueryIsomorphism::parse_query(query)
    }

    /// Compare two queries and return detailed comparison results
    ///
    /// This provides granular information about why two queries might or might not
    /// be isomorphic (e.g., same language, same BGP structure, etc.).
    ///
    /// # Arguments
    ///
    /// * `query1` - First query string
    /// * `query2` - Second query string
    ///
    /// # Returns
    ///
    /// `QueryComparisonResult` with detailed flags
    pub fn compare_queries(
        query1: &str,
        query2: &str,
    ) -> Result<QueryComparisonResult, TulnaError> {
        let lang1 = QueryIsomorphism::detect_query_type(query1);
        let lang2 = QueryIsomorphism::detect_query_type(query2);

        let same_language = lang1 == lang2;

        let q1_parsed = QueryIsomorphism::parse_query(query1)?;
        let q2_parsed = QueryIsomorphism::parse_query(query2)?;

        let bgp1 = &q1_parsed.bgp;
        let bgp2 = &q2_parsed.bgp;

        let same_bgp_size = bgp1.len() == bgp2.len();

        let bgp_isomorphic = crate::isomorphism::graph_isomorphism::GraphIsomorphism::check_bgp_isomorphism(bgp1, bgp2)?;

        let is_isomorphic = QueryIsomorphism::is_isomorphic(query1, query2)?;

        Ok(QueryComparisonResult {
            is_isomorphic,
            same_language,
            same_bgp_size,
            bgp_isomorphic,
        })
    }

    /// Check if stream parameters match between two queries
    ///
    /// # Arguments
    ///
    /// * `query1` - First query string
    /// * `query2` - Second query string
    ///
    /// # Returns
    ///
    /// Boolean indicating if stream parameters (window type, range, slide, etc.) match
    pub fn check_stream_parameters(query1: &str, query2: &str) -> Result<bool, TulnaError> {
        let q1 = QueryIsomorphism::parse_query(query1)?;
        let q2 = QueryIsomorphism::parse_query(query2)?;

        let params_match = q1.stream_name == q2.stream_name
            && q1.window_name == q2.window_name
            && q1.width == q2.width
            && q1.slide == q2.slide
            && q1.offset == q2.offset
            && q1.start == q2.start
            && q1.end == q2.end;

        Ok(params_match)
    }

    /// Check if window names match between two queries
    ///
    /// # Arguments
    ///
    /// * `query1` - First query string
    /// * `query2` - Second query string
    ///
    /// # Returns
    ///
    /// Boolean indicating if window names match
    pub fn check_window_names(query1: &str, query2: &str) -> Result<bool, TulnaError> {
        let q1 = QueryIsomorphism::parse_query(query1)?;
        let q2 = QueryIsomorphism::parse_query(query2)?;
        Ok(q1.window_name == q2.window_name)
    }
}

/// Detailed comparison result
#[derive(Debug, Clone)]
pub struct QueryComparisonResult {
    pub is_isomorphic: bool,
    pub same_language: bool,
    pub same_bgp_size: bool,
    pub bgp_isomorphic: bool,
}

impl QueryComparisonResult {
    pub fn summary(&self) -> String {
        format!(
            "Isomorphic: {}, Same Language: {}, Same BGP Size: {}, BGP Isomorphic: {}",
            self.is_isomorphic, self.same_language, self.same_bgp_size, self.bgp_isomorphic
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_language() {
        let sparql = "SELECT * WHERE { ?s ?p ?o }";
        assert_eq!(
            QueryIsomorphismAPI::detect_query_language(sparql),
            crate::isomorphism::core::QueryLanguage::SPARQL
        );
    }

    #[test]
    fn test_extract_bgp() {
        let query = "SELECT * WHERE { ?s <http://p> ?o }";
        let bgp = QueryIsomorphismAPI::extract_bgp(query).unwrap();
        assert_eq!(bgp.len(), 1);
    }

    #[test]
    fn test_sparql_isomorphism() {
        let q1 = "SELECT ?s ?p ?o WHERE { ?s ?p ?o }";
        let q2 = "SELECT ?x ?y ?z WHERE { ?x ?y ?z }";
        assert!(QueryIsomorphismAPI::is_isomorphic(q1, q2).unwrap());
    }

    #[test]
    fn test_compare_queries() {
        let q1 = "SELECT ?s ?p ?o WHERE { ?s ?p ?o }";
        let q2 = "SELECT ?x ?y ?z WHERE { ?x ?y ?z }";
        let result = QueryIsomorphismAPI::compare_queries(q1, q2).unwrap();
        assert!(result.is_isomorphic);
        assert!(result.same_language);
        assert!(result.same_bgp_size);
        assert!(result.bgp_isomorphic);
    }
}