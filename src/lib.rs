//! # tulna-rs
//!
//! A Rust library for parsing and checking equivalence (isomorphism) of semantic queries
//! including SPARQL, RSP-QL, and JanusQL, as well as direct RDF graph isomorphism checking.
//!
//! ## Features
//!
//! - **Query Isomorphism**: Check if two queries are semantically equivalent
//!   - SPARQL 1.1 queries
//!   - RSP-QL queries with streaming extensions
//!   - JanusQL queries with live and historical windows
//! - **Graph Isomorphism**: Direct RDF graph isomorphism checking using hash-based grounding algorithm
//! - **Query Parsing**: Auto-detect and parse different query languages
//! - **Stream Parameters**: Validate stream and window parameters for streaming queries
//!
//! ## Quick Start
//!
//! ### Query Isomorphism
//!
//! ```rust
//! use tulna_rs::isomorphism::api::QueryIsomorphismAPI;
//!
//! let query1 = "SELECT ?s ?p ?o WHERE { ?s ?p ?o }";
//! let query2 = "SELECT ?x ?y ?z WHERE { ?x ?y ?z }";
//!
//! let is_isomorphic = QueryIsomorphismAPI::is_isomorphic(query1, query2).unwrap();
//! assert!(is_isomorphic); // true - same structure, different variables
//! ```
//!
//! ### Direct Graph Isomorphism
//!
//! ```rust
//! use tulna_rs::graph::{GraphIsomorphism, Triple, TripleNode};
//!
//! let graph1 = vec![
//!     Triple {
//!         subject: TripleNode::Variable("x".to_string()),
//!         predicate: TripleNode::IRI("http://example.org/knows".to_string()),
//!         object: TripleNode::Variable("y".to_string()),
//!     }
//! ];
//!
//! let graph2 = vec![
//!     Triple {
//!         subject: TripleNode::Variable("a".to_string()),
//!         predicate: TripleNode::IRI("http://example.org/knows".to_string()),
//!         object: TripleNode::Variable("b".to_string()),
//!     }
//! ];
//!
//! let is_isomorphic = GraphIsomorphism::are_isomorphic(&graph1, &graph2).unwrap();
//! assert!(is_isomorphic); // true
//! ```

pub mod isomorphism;
pub mod parsing;
pub mod error;

pub use error::TulnaError;

// Re-export commonly used types for graph isomorphism
pub mod graph {
    //! Direct RDF graph isomorphism checking.
    //!
    //! This module provides the hash-based grounding algorithm for efficient
    //! graph isomorphism checking, independent of query parsing.

    pub use crate::isomorphism::core::{Triple, TripleNode};
    pub use crate::isomorphism::graph_isomorphism::GraphIsomorphism;
}

// Re-export query isomorphism API
pub mod query {
    //! Query isomorphism checking for SPARQL, RSP-QL, and JanusQL.

    pub use crate::isomorphism::api::{QueryComparisonResult, QueryIsomorphismAPI};
    pub use crate::isomorphism::core::{IsomorphismQuery, QueryLanguage};
}
