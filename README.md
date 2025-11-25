# tulna-rs

A Rust library for RDF graph isomorphism and semantic query equivalence checking using an efficient hash-based grounding algorithm to detect the graph isomorphism.

## Features

- **Graph Isomorphism** - Efficient RDF graph structural comparison
- **Query Isomorphism** - Semantic equivalence checking for SPARQL, RSP-QL, and Janus-QL
- **Auto-Detection** - Automatically detect query language type
- **Stream Support** - Full support for streaming query extensions

## Installation

```toml
[dependencies]
tulna-rs = "0.1.0"
```

## Quick Start

### Graph Isomorphism

Compare RDF graphs directly:

```rust
use tulna_rs::graph::{GraphIsomorphism, Triple, TripleNode};

let graph1 = vec![
    Triple {
        subject: TripleNode::Variable("x".to_string()),
        predicate: TripleNode::IRI("http://example.org/knows".to_string()),
        object: TripleNode::Variable("y".to_string()),
    }
];

let graph2 = vec![
    Triple {
        subject: TripleNode::Variable("person".to_string()),
        predicate: TripleNode::IRI("http://example.org/knows".to_string()),
        object: TripleNode::Variable("friend".to_string()),
    }
];

let result = GraphIsomorphism::are_isomorphic(&graph1, &graph2)?;
assert!(result); // true - same structure, different variable names
```

### Query Isomorphism

Compare SPARQL/RSP-QL/JanusQL queries:

```rust
use tulna_rs::query::QueryIsomorphismAPI;

let query1 = "SELECT ?s ?p ?o WHERE { ?s ?p ?o . }";
let query2 = "SELECT ?x ?y ?z WHERE { ?x ?y ?z . }";

let result = QueryIsomorphismAPI::is_isomorphic(query1, query2)?;
assert!(result); // true - semantically equivalent
```

## Supported Query Languages

- **SPARQL 1.1** - Standard SELECT queries
- **RSP-QL** - Streaming with RANGE/STEP windows
- **JanusQL** - Historical windows with OFFSET/START/END

## Algorithm

Uses a hash-based grounding algorithm that:
1. Separates blank and non-blank nodes
2. Iteratively hashes blank nodes based on structural signatures
3. Grounds nodes with unique signatures
4. Only recurses on ambiguous cases

## Examples

Run included examples:

```bash
# Graph isomorphism examples
cargo run --example graph_isomorphism

# Query isomorphism examples
cargo run --example query_isomorphism
```

## API Overview

### Graph API

```rust
use tulna_rs::graph::{GraphIsomorphism, Triple, TripleNode};

// Check if two graphs are isomorphic
GraphIsomorphism::are_isomorphic(&graph1, &graph2)?;
```

### Query API

```rust
use tulna_rs::query::{QueryIsomorphismAPI, QueryLanguage};

// Check query isomorphism
QueryIsomorphismAPI::is_isomorphic(query1, query2)?;

// Detect query language
QueryIsomorphismAPI::detect_query_language(query);

// Extract basic graph pattern
QueryIsomorphismAPI::extract_bgp(query)?;

// Compare with details
QueryIsomorphismAPI::compare_queries(query1, query2)?;
```

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Generate documentation
cargo doc --open
```

## Use Cases

**Graph Isomorphism:**
- RDF dataset comparison and deduplication
- Testing RDF transformations
- Normalizing data with blank nodes

**Query Isomorphism:**
- Query optimization and caching
- Duplicate detection in query logs
- Query equivalence testing

## Documentation

- API documentation: `cargo doc --open`
- Examples: `examples/` directory
- Tests: `tests/` directory

## Dependencies

- `regex` - Query parsing
- `murmur3` - Hash function for the grounding algorithm

## License

Copyright by [Ghent University - imec](https://www.ugent.be/ea/idlab/en)

Released under the [MIT License](./LICENSE)

## Acknowledgments

Algorithm based on:
- [RDF isomorphism in RDF.rb](http://blog.datagraph.org/2010/03/rdf-isomorphism)
- [Jeremy Carroll's work](http://www.hpl.hp.com/techreports/2001/HPL-2001-293.pdf)
- [rdf-isomorphic.js](https://github.com/rubensworks/rdf-isomorphic.js/) by @rubensworks

## Contributing

Contributions welcome. Please ensure:
- All tests pass: `cargo test`
- Code is formatted: `cargo fmt`
- No clippy warnings: `cargo clippy`
