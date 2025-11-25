# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive documentation for graph isomorphism algorithm implementation
- Module-level documentation explaining hash-based grounding algorithm
- Detailed method documentation for all core functions
- Performance characteristics and complexity analysis
- Algorithm step-by-step explanation with examples

### Changed
- Renamed `isomorphism::isomorphism` module to `isomorphism::core` to fix clippy module_inception warning
- Moved regex compilation outside loops in RSPQLParser to improve performance and fix clippy warnings

### Fixed
- Clippy warnings: module_inception and regex_creation_in_loops
- All clippy warnings now pass with `-D warnings` flag

## [0.1.0] - 2024

### Added
- Initial release of tulna-rs
- RDF graph isomorphism checking using hash-based grounding algorithm
- Query isomorphism for SPARQL 1.1 queries
- Query isomorphism for RSP-QL queries with streaming extensions
- Query isomorphism for JanusQL queries with live and historical windows
- Auto-detection of query language types
- Basic Graph Pattern (BGP) extraction from queries
- Stream and window parameter validation
- Comprehensive test suite with 64+ tests
- Examples for graph and query isomorphism
- Full API documentation

### Features

#### Graph Isomorphism
- Efficient O(n log n) average-case hash-based algorithm
- Support for blank nodes and variables
- Handles complex graph structures with multiple interconnected nodes
- Deduplication and structural comparison

#### Query Isomorphism
- SPARQL 1.1 SELECT query comparison
- RSP-QL streaming query comparison with RANGE/STEP windows
- JanusQL temporal query comparison with OFFSET/START/END parameters
- Variable renaming detection
- BGP structural equivalence checking
- Stream and window parameter matching

#### Developer Experience
- Clean, well-documented API
- Comprehensive examples
- Extensive test coverage
- Type-safe interfaces
- Error handling with Result types

### Dependencies
- `regex` (1.12.2) - Query parsing and pattern matching
- `murmur3` (0.5) - Hash function for grounding algorithm

[Unreleased]: https://github.com/SolidLabResearch/tulna-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/SolidLabResearch/tulna-rs/releases/tag/v0.1.0