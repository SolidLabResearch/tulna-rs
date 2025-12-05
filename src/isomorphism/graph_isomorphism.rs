//! # Graph Isomorphism Module
//!
//! This module implements efficient RDF graph isomorphism checking using a hash-based
//! grounding algorithm. The implementation determines whether two RDF graphs are
//! structurally equivalent, treating blank nodes and variables as interchangeable
//! based on their structural context.
//!
//! ## Algorithm Overview
//!
//! The hash-based grounding algorithm solves the graph isomorphism problem by iteratively
//! identifying and "grounding" blank nodes (or variables treated as blank nodes) based on
//! their structural signatures. This approach achieves **O(n log n)** average-case complexity
//! instead of the **O(n!)** complexity of brute-force bijection search.
//!
//! ### Key Concepts
//!
//! 1. **Grounding**: A blank node is "grounded" when it can be uniquely identified by its
//!    structural context. Grounded nodes can be safely matched between graphs.
//!
//! 2. **Hash Signatures**: Each blank node is assigned a hash signature based on the
//!    structure of triples it participates in. Nodes with unique signatures can be grounded.
//!
//! 3. **Iterative Refinement**: The algorithm iteratively refines hash signatures, using
//!    already-grounded nodes to help ground additional nodes.
//!
//! 4. **Recursive Search**: When multiple nodes share the same signature (ambiguous cases),
//!    the algorithm speculatively grounds one node and recurses.
//!
//! ## Algorithm Steps
//!
//! The algorithm proceeds as follows:
//!
//! 1. **Normalize graphs**: Convert variables to blank nodes for uniform processing.
//!
//! 2. **Separate non-blank triples**: Extract and compare triples without blank nodes.
//!    These must match exactly between isomorphic graphs.
//!
//! 3. **Initialize blank node sets**: Identify all blank nodes in both graphs.
//!
//! 4. **Iterative hashing**:
//!    - For each blank node, compute a hash signature based on:
//!      - The predicates and objects/subjects it appears with
//!      - The hash signatures of already-grounded nodes it connects to
//!    - Sort these signatures to create a canonical hash
//!    - Mark nodes with unique hashes as grounded
//!    - Repeat until no new nodes can be grounded
//!
//! 5. **Build bijection**: Match blank nodes from graph A to graph B based on their hashes.
//!
//! 6. **Handle ambiguity**: If nodes remain ungrounded (same hash), speculatively assign
//!    matching pairs and recurse to verify the assignment leads to a valid bijection.
//!
//! ## Performance Characteristics
//!
//! - **Best case**: O(n) when all blank nodes can be uniquely grounded immediately
//! - **Average case**: O(n log n) with iterative refinement
//! - **Worst case**: O(n!) for completely symmetric graphs (rare in practice)
//!
//! ### Example Performance
//!
//! For a graph with 10 blank nodes:
//! - **Brute force**: 10! = 3,628,800 potential bijections to check
//! - **Hash-based**: ~20-30 hash iterations in typical cases
//!
//! ## Hash Function
//!
//! The implementation uses **MurmurHash3** (128-bit, reduced to 64-bit) for:
//! - Deterministic hashing across platforms
//! - Low collision probability
//! - Fast computation
//!
//! ## Algorithm Sources
//!
//! This implementation is based on:
//! - [RDF isomorphism in RDF.rb](http://blog.datagraph.org/2010/03/rdf-isomorphism)
//! - [Jeremy Carroll's work on RDF graph equivalence](http://www.hpl.hp.com/techreports/2001/HPL-2001-293.pdf)
//! - [rdf-isomorphic.js](https://github.com/rubensworks/rdf-isomorphic.js/) by @rubensworks
//!
//! ## Implementation Details
//!
//! ### Signature Generation
//!
//! For each blank node, we generate a signature from all triples it appears in:
//!
//! ```text
//! For blank node _:b1 in triple: _:b1 <knows> _:b2
//! Signature component: "@self|<knows>|<hash_of_b2_or_@blank>"
//! ```
//!
//! The signature uses:
//! - `@self` for the node being hashed
//! - `@blank` for ungrounded blank nodes
//! - Hash values for grounded blank nodes
//! - Literal values for IRIs and literals
//!
//! ### Grounding Process
//!
//! Nodes are grounded when:
//! 1. All nodes they connect to are already grounded AND
//! 2. Their computed hash is unique in the graph
//!
//! Example progression:
//! ```text
//! Iteration 1: Ground nodes connected only to literals/IRIs
//! Iteration 2: Ground nodes connected to iteration-1 nodes
//! Iteration 3: Continue until no new groundings
//! ```
//!
//! ### Bijection Verification
//!
//! A valid bijection must:
//! - Map every blank node in graph A to exactly one in graph B
//! - Preserve all structural relationships
//! - Result in identical normalized graphs when applied
//!
//! ## Examples
//!
//! ### Basic Usage
//!
//! ```
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
//! let result = GraphIsomorphism::are_isomorphic(&graph1, &graph2).unwrap();
//! assert!(result); // Graphs are isomorphic despite different variable names
//! ```
//!
//! ### Complex Graph
//!
//! ```
//! use tulna_rs::graph::{GraphIsomorphism, Triple, TripleNode};
//!
//! // Graph with multiple interconnected blank nodes
//! let graph1 = vec![
//!     Triple {
//!         subject: TripleNode::Variable("person".to_string()),
//!         predicate: TripleNode::IRI("http://xmlns.com/foaf/0.1/name".to_string()),
//!         object: TripleNode::Literal("Alice".to_string()),
//!     },
//!     Triple {
//!         subject: TripleNode::Variable("person".to_string()),
//!         predicate: TripleNode::IRI("http://xmlns.com/foaf/0.1/knows".to_string()),
//!         object: TripleNode::Variable("friend".to_string()),
//!     },
//! ];
//!
//! // Same structure, different variable names
//! let graph2 = vec![
//!     Triple {
//!         subject: TripleNode::Variable("x".to_string()),
//!         predicate: TripleNode::IRI("http://xmlns.com/foaf/0.1/name".to_string()),
//!         object: TripleNode::Literal("Alice".to_string()),
//!     },
//!     Triple {
//!         subject: TripleNode::Variable("x".to_string()),
//!         predicate: TripleNode::IRI("http://xmlns.com/foaf/0.1/knows".to_string()),
//!         object: TripleNode::Variable("y".to_string()),
//!     },
//! ];
//!
//! assert!(GraphIsomorphism::are_isomorphic(&graph1, &graph2).unwrap());
//! ```

use crate::isomorphism::core::{Triple, TripleNode};
use crate::TulnaError;
use std::collections::{HashMap, HashSet};
use std::io::Cursor;

/// Graph isomorphism checker for RDF graphs using hash-based grounding algorithm.
///
/// This struct provides static methods for checking graph isomorphism. See the module-level
/// documentation for detailed information about the algorithm implementation.
pub struct GraphIsomorphism;

impl GraphIsomorphism {
    /// Check if two RDF graphs are isomorphic.
    ///
    /// This is the main public API for graph isomorphism checking. It uses the hash-based
    /// grounding algorithm to efficiently determine if two graphs are structurally identical,
    /// treating variables and blank nodes as equivalent based on their structural position.
    ///
    /// # Arguments
    ///
    /// * `graph1` - First RDF graph as a slice of triples
    /// * `graph2` - Second RDF graph as a slice of triples
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - Graphs are isomorphic
    /// * `Ok(false)` - Graphs are not isomorphic
    /// * `Err(_)` - An error occurred during processing
    ///
    /// # Examples
    ///
    /// ```
    /// use tulna_rs::graph::{GraphIsomorphism, Triple, TripleNode};
    ///
    /// let graph1 = vec![
    ///     Triple {
    ///         subject: TripleNode::IRI("http://example.org/alice".to_string()),
    ///         predicate: TripleNode::IRI("http://example.org/name".to_string()),
    ///         object: TripleNode::Literal("Alice".to_string()),
    ///     }
    /// ];
    ///
    /// let graph2 = vec![
    ///     Triple {
    ///         subject: TripleNode::IRI("http://example.org/alice".to_string()),
    ///         predicate: TripleNode::IRI("http://example.org/name".to_string()),
    ///         object: TripleNode::Literal("Alice".to_string()),
    ///     }
    /// ];
    ///
    /// assert!(GraphIsomorphism::are_isomorphic(&graph1, &graph2).unwrap());
    /// ```
    pub fn are_isomorphic(graph1: &[Triple], graph2: &[Triple]) -> Result<bool, TulnaError> {
        Self::check_bgp_isomorphism(graph1, graph2)
    }

    /// Check if two BGPs are isomorphic using hash-based grounding algorithm.
    /// This converts variables to blank nodes and checks for graph isomorphism.
    ///
    /// This method is used internally and by the query isomorphism API.
    pub fn check_bgp_isomorphism(bgp1: &[Triple], bgp2: &[Triple]) -> Result<bool, TulnaError> {
        if bgp1.len() != bgp2.len() {
            return Ok(false);
        }

        // Convert to normalized string representations
        let graph1 = Self::normalize_bgp(bgp1);
        let graph2 = Self::normalize_bgp(bgp2);

        // Check if graphs are isomorphic using hash-based algorithm
        Ok(Self::is_isomorphic(&graph1, &graph2))
    }

    /// Normalize a BGP by converting it to a canonical form
    /// Variables are replaced with blank node identifiers
    fn normalize_bgp(bgp: &[Triple]) -> Vec<NormalizedTriple> {
        let mut var_map: HashMap<String, String> = HashMap::new();
        let mut counter = 0;

        bgp.iter()
            .map(|triple| {
                let subject = Self::normalize_node(&triple.subject, &mut var_map, &mut counter);
                let predicate = Self::normalize_node(&triple.predicate, &mut var_map, &mut counter);
                let object = Self::normalize_node(&triple.object, &mut var_map, &mut counter);

                NormalizedTriple {
                    subject,
                    predicate,
                    object,
                }
            })
            .collect()
    }

    /// Normalize a node, converting variables to blank nodes with consistent IDs
    fn normalize_node(
        node: &TripleNode,
        var_map: &mut HashMap<String, String>,
        counter: &mut u32,
    ) -> String {
        match node {
            TripleNode::IRI(iri) => format!("<{}>", iri),
            TripleNode::Variable(var) => {
                // Map each variable to a unique blank node ID
                if !var_map.contains_key(var) {
                    var_map.insert(var.clone(), format!("_:b{}", counter));
                    *counter += 1;
                }
                var_map.get(var).unwrap().clone()
            }
            TripleNode::Literal(lit) => format!("\"{}\"", lit),
            TripleNode::BlankNode(id) => format!("_:{}", id),
        }
    }

    /// Check if two normalized graphs are isomorphic using hash-based grounding
    fn is_isomorphic(graph_a: &[NormalizedTriple], graph_b: &[NormalizedTriple]) -> bool {
        if graph_a.len() != graph_b.len() {
            return false;
        }

        // Get bijection using hash-based algorithm
        Self::get_bijection(graph_a, graph_b).is_some()
    }

    /// Calculate a bijection from graph A blank nodes to graph B blank nodes.
    ///
    /// This is the entry point for the hash-based grounding algorithm. It performs initial
    /// validation by comparing non-blank-node triples, then delegates to the recursive
    /// bijection finder.
    ///
    /// # Algorithm Steps
    ///
    /// 1. **Extract and compare non-blank triples**: Triples without blank nodes must match
    ///    exactly between isomorphic graphs. This is an early-exit optimization.
    ///
    /// 2. **Separate blank-containing triples**: Extract triples that contain at least one
    ///    blank node for structural analysis.
    ///
    /// 3. **Identify blank nodes**: Get the set of all blank node identifiers from each graph.
    ///
    /// 4. **Delegate to recursive finder**: Call `get_bijection_inner` with empty initial
    ///    grounding to begin the iterative hash-based matching process.
    ///
    /// # Arguments
    ///
    /// * `graph_a` - First normalized graph
    /// * `graph_b` - Second normalized graph
    ///
    /// # Returns
    ///
    /// * `Some(bijection)` - A mapping from graph A blank nodes to graph B blank nodes if graphs are isomorphic
    /// * `None` - If graphs are not isomorphic
    fn get_bijection(
        graph_a: &[NormalizedTriple],
        graph_b: &[NormalizedTriple],
    ) -> Option<HashMap<String, String>> {
        // Check if all non-blank-node-containing quads in the two graphs are equal
        let non_blank_a = Self::get_quads_without_blank_nodes(graph_a);
        let non_blank_b = Self::get_quads_without_blank_nodes(graph_b);

        let index_a = Self::index_graph(&non_blank_a);
        let index_b = Self::index_graph(&non_blank_b);

        if index_a.len() != index_b.len() {
            return None;
        }

        for key in index_a.keys() {
            if !index_b.contains_key(key) {
                return None;
            }
        }

        // Pre-process data for iteration
        let blank_quads_a = Self::uniq_graph(&Self::get_quads_with_blank_nodes(graph_a));
        let blank_quads_b = Self::uniq_graph(&Self::get_quads_with_blank_nodes(graph_b));
        let blank_nodes_a = Self::get_graph_blank_nodes(graph_a);
        let blank_nodes_b = Self::get_graph_blank_nodes(graph_b);

        if blank_nodes_a.len() != blank_nodes_b.len() {
            return None;
        }

        Self::get_bijection_inner(
            &blank_quads_a,
            &blank_quads_b,
            &blank_nodes_a,
            &blank_nodes_b,
            &HashMap::new(),
            &HashMap::new(),
        )
    }

    /// Inner recursive bijection finder using iterative hash-based grounding.
    ///
    /// This is the core of the isomorphism algorithm. It iteratively refines hash signatures
    /// for blank nodes, grounding nodes that can be uniquely identified, and building a
    /// bijection between the two graphs. When ambiguity remains (multiple nodes share the
    /// same hash), it speculatively assigns matching pairs and recurses.
    ///
    /// # Algorithm Flow
    ///
    /// 1. **Hash all blank nodes** using structural signatures based on their triple patterns
    ///    and already-grounded neighbors (via `hash_terms`).
    ///
    /// 2. **Validate grounded hashes** match between graphs. If different nodes are grounded,
    ///    graphs cannot be isomorphic.
    ///
    /// 3. **Build bijection** by matching nodes with identical ungrounded hashes.
    ///
    /// 4. **Check completeness**:
    ///    - If all blank nodes are in the bijection → Success, return bijection
    ///    - If some nodes remain unmapped → Recursion needed
    ///
    /// 5. **Recursive speculation**: For ungrounded nodes with matching hashes, speculatively
    ///    assign them the same hash value (ground them together) and recurse. This explores
    ///    possible bijections until a valid one is found or all possibilities are exhausted.
    ///
    /// # Arguments
    ///
    /// * `blank_quads_a` - Triples containing blank nodes from graph A
    /// * `blank_quads_b` - Triples containing blank nodes from graph B
    /// * `blank_nodes_a` - Set of blank node identifiers in graph A
    /// * `blank_nodes_b` - Set of blank node identifiers in graph B
    /// * `grounded_hashes_a` - Already-grounded blank nodes and their hash values for graph A
    /// * `grounded_hashes_b` - Already-grounded blank nodes and their hash values for graph B
    ///
    /// # Returns
    ///
    /// * `Some(bijection)` - Valid mapping from graph A to graph B blank nodes
    /// * `None` - No valid bijection exists with current groundings
    fn get_bijection_inner(
        blank_quads_a: &[NormalizedTriple],
        blank_quads_b: &[NormalizedTriple],
        blank_nodes_a: &[String],
        blank_nodes_b: &[String],
        grounded_hashes_a: &HashMap<String, u64>,
        grounded_hashes_b: &HashMap<String, u64>,
    ) -> Option<HashMap<String, String>> {
        // Hash every term based on the signature of the quads it appears in
        let (hashes_a, ungrounded_hashes_a) =
            Self::hash_terms(blank_quads_a, blank_nodes_a, grounded_hashes_a);
        let (hashes_b, ungrounded_hashes_b) =
            Self::hash_terms(blank_quads_b, blank_nodes_b, grounded_hashes_b);

        // Break quickly if graphs contain different grounded nodes
        if hashes_a.len() != hashes_b.len() {
            println!(
                "DEBUG: Different grounded count: {} vs {}",
                hashes_a.len(),
                hashes_b.len()
            );
            return None;
        }

        for hash_value in hashes_a.values() {
            if !Self::hash_contains_value(&hashes_b, *hash_value) {
                println!("DEBUG: Hash mismatch in grounded");
                return None;
            }
        }

        // Map blank nodes from graph A to graph B using created hashes
        // Only map grounded nodes here; leave ambiguous nodes for speculation phase
        let mut bijection: HashMap<String, String> = HashMap::new();
        let mut used_b_nodes: HashSet<String> = HashSet::new();

        for node_a in blank_nodes_a {
            // Only map if this node is grounded (uniquely identifiable)
            if let Some(&hash_a) = hashes_a.get(node_a) {
                for node_b in blank_nodes_b {
                    if used_b_nodes.contains(node_b) {
                        continue;
                    }
                    // Match against grounded nodes in graph B
                    if let Some(&hash_b) = hashes_b.get(node_b) {
                        if hash_a == hash_b {
                            bijection.insert(node_a.clone(), node_b.clone());
                            used_b_nodes.insert(node_b.clone());
                            break;
                        }
                    }
                }
            }
        }

        // Check if all nodes are in the bijection
        let mut bijection_keys: Vec<String> = bijection.keys().cloned().collect();
        bijection_keys.sort();
        let mut blank_nodes_a_sorted: Vec<String> = blank_nodes_a.to_vec();
        blank_nodes_a_sorted.sort();

        let mut bijection_values: Vec<String> = bijection.values().cloned().collect();
        bijection_values.sort();
        let mut blank_nodes_b_sorted: Vec<String> = blank_nodes_b.to_vec();
        blank_nodes_b_sorted.sort();

        if bijection_keys != blank_nodes_a_sorted || bijection_values != blank_nodes_b_sorted {
            // Speculatively mark pairs with matching ungrounded hashes as bijected and recurse
            for node_a in blank_nodes_a {
                // Only replace ungrounded node hashes
                if hashes_a.contains_key(node_a) {
                    continue;
                }

                for node_b in blank_nodes_b {
                    // Only replace ungrounded node hashes
                    if hashes_b.contains_key(node_b) {
                        continue;
                    }

                    if let (Some(&hash_a), Some(&hash_b)) = (
                        ungrounded_hashes_a.get(node_a),
                        ungrounded_hashes_b.get(node_b),
                    ) {
                        if hash_a == hash_b {
                            println!("DEBUG: Speculating {} -> {}", node_a, node_b);
                            let new_hash = Self::hash_string(node_a);
                            let mut new_grounded_a = grounded_hashes_a.clone();
                            new_grounded_a.insert(node_a.clone(), new_hash);
                            let mut new_grounded_b = grounded_hashes_b.clone();
                            new_grounded_b.insert(node_b.clone(), new_hash);

                            if let Some(result) = Self::get_bijection_inner(
                                blank_quads_a,
                                blank_quads_b,
                                blank_nodes_a,
                                blank_nodes_b,
                                &new_grounded_a,
                                &new_grounded_b,
                            ) {
                                return Some(result);
                            }
                        }
                    }
                }
            }
            println!("DEBUG: Recursion failed");
            return None;
        }

        // Verify the bijection preserves graph structure (edges) before returning
        if Self::verify_bijection(blank_quads_a, blank_quads_b, &bijection) {
            Some(bijection)
        } else {
            println!("DEBUG: Verification failed");
            None
        }
    }

    /// Verify that applying the bijection to graph A yields graph B.
    fn verify_bijection(
        graph_a: &[NormalizedTriple],
        graph_b: &[NormalizedTriple],
        bijection: &HashMap<String, String>,
    ) -> bool {
        if graph_a.len() != graph_b.len() {
            return false;
        }

        let index_b = Self::index_graph(graph_b);

        for quad in graph_a {
            let s = bijection.get(&quad.subject).unwrap_or(&quad.subject);
            let p = bijection.get(&quad.predicate).unwrap_or(&quad.predicate);
            let o = bijection.get(&quad.object).unwrap_or(&quad.object);

            let key = format!("{}|{}|{}", s, p, o);
            if !index_b.contains_key(&key) {
                return false;
            }
        }
        true
    }

    /// Create hash signatures for blank nodes based on their structural context.
    ///
    /// This function implements the iterative grounding process. It computes hash signatures
    /// for each blank node based on the triples it appears in, taking into account already-
    /// grounded nodes. The process repeats until no new nodes can be grounded.
    ///
    /// # Grounding Rules
    ///
    /// A blank node is **grounded** when:
    /// 1. All other blank nodes in its connected triples are already grounded, AND
    /// 2. Its computed hash signature is unique (no other node has the same hash)
    ///
    /// # Hash Signature Computation
    ///
    /// For each blank node:
    /// 1. Find all triples containing that node
    /// 2. Generate a signature for each triple (see `quad_to_signature`)
    /// 3. Sort signatures for canonical ordering
    /// 4. Hash the concatenated signatures using MurmurHash3
    ///
    /// # Iterative Process
    ///
    /// ```text
    /// Iteration 1: Ground nodes connected only to non-blank nodes (IRIs/literals)
    /// Iteration 2: Ground nodes connected to iteration-1 grounded nodes
    /// Iteration 3: Continue until no new nodes can be uniquely identified
    /// ```
    ///
    /// # Arguments
    ///
    /// * `quads` - The triples containing blank nodes to analyze
    /// * `terms` - The blank node identifiers to compute hashes for
    /// * `grounded_hashes` - Previously grounded nodes with their assigned hash values
    ///
    /// # Returns
    ///
    /// A tuple of:
    /// * `grounded_hashes` - All nodes that have been conclusively grounded (unique hashes)
    /// * `ungrounded_hashes` - Hash values for all nodes (including grounded ones), used for matching
    fn hash_terms(
        quads: &[NormalizedTriple],
        terms: &[String],
        grounded_hashes: &HashMap<String, u64>,
    ) -> (HashMap<String, u64>, HashMap<String, u64>) {
        let mut hashes = grounded_hashes.clone();
        let mut ungrounded_hashes: HashMap<String, u64> = HashMap::new();
        let mut hash_needed = true;

        // Iteratively mark nodes as grounded
        while hash_needed {
            let initial_grounded_count = hashes.len();

            for term in terms {
                if !hashes.contains_key(term) {
                    let (grounded, hash) = Self::hash_term(term, quads, &hashes);
                    if grounded {
                        hashes.insert(term.clone(), hash);
                    }
                    ungrounded_hashes.insert(term.clone(), hash);
                }
            }

            // All terms that have a unique hash at this point can be marked as grounded
            let mut hash_to_term: HashMap<u64, Option<String>> = HashMap::new();
            for (term, &hash) in &ungrounded_hashes {
                if let Some(existing) = hash_to_term.get(&hash) {
                    if existing.is_some() {
                        hash_to_term.insert(hash, None); // Mark as non-unique
                    }
                } else {
                    hash_to_term.insert(hash, Some(term.clone()));
                }
            }

            for (hash, term_opt) in hash_to_term {
                if let Some(term) = term_opt {
                    hashes.insert(term, hash);
                }
            }

            hash_needed = initial_grounded_count != hashes.len();
        }

        (hashes, ungrounded_hashes)
    }

    /// Generate a hash signature for a single blank node.
    ///
    /// This method finds all triples containing the target blank node and creates a
    /// structural signature that captures the node's context. The signature includes
    /// information about connected predicates and objects/subjects.
    ///
    /// # Signature Components
    ///
    /// For a node appearing in: `_:b1 <predicate> <object>`
    /// - Uses "@self" for the target node position
    /// - Uses hash values for grounded connected blank nodes
    /// - Uses "@blank" for ungrounded connected blank nodes
    /// - Uses literal representations for IRIs and literals
    ///
    /// # Grounding Check
    ///
    /// The node is considered grounded if all other blank nodes in its connected
    /// triples are already grounded. This ensures the signature is stable and unique.
    ///
    /// # Arguments
    ///
    /// * `term` - The blank node identifier to hash
    /// * `quads` - All triples to search for occurrences of this node
    /// * `hashes` - Currently grounded nodes and their hash values
    ///
    /// # Returns
    ///
    /// A tuple of:
    /// * `is_grounded` - Whether this node can be considered grounded (all neighbors grounded)
    /// * `hash` - The computed hash signature for this node
    fn hash_term(
        term: &str,
        quads: &[NormalizedTriple],
        hashes: &HashMap<String, u64>,
    ) -> (bool, u64) {
        let mut quad_signatures = Vec::new();
        let mut grounded = true;

        for quad in quads {
            let terms_in_quad = [&quad.subject, &quad.predicate, &quad.object];
            if terms_in_quad.iter().any(|&t| t == term) {
                quad_signatures.push(Self::quad_to_signature(quad, hashes, term));

                for quad_term in &terms_in_quad {
                    if !Self::is_term_grounded(quad_term, hashes) && *quad_term != term {
                        grounded = false;
                    }
                }
            }
        }

        quad_signatures.sort();
        let hash = Self::hash_string(&quad_signatures.join(""));
        (grounded, hash)
    }

    /// Convert a triple to a signature string for hashing.
    ///
    /// Creates a canonical string representation of a triple from the perspective of a
    /// specific blank node. The signature uses special markers to distinguish the target
    /// node from other nodes.
    ///
    /// # Format
    ///
    /// `"<subject_sig>|<predicate_sig>|<object_sig>"`
    ///
    /// Where each position uses:
    /// - `@self` for the target blank node
    /// - Hash value (as string) for grounded blank nodes
    /// - `@blank` for ungrounded blank nodes
    /// - Literal representation for IRIs and literals
    ///
    /// # Example
    ///
    /// For triple `_:b1 <knows> _:b2` with target `_:b1`:
    /// - If `_:b2` is grounded with hash `12345`: `"@self|<knows>|12345"`
    /// - If `_:b2` is not grounded: `"@self|<knows>|@blank"`
    fn quad_to_signature(
        quad: &NormalizedTriple,
        hashes: &HashMap<String, u64>,
        term: &str,
    ) -> String {
        let s_sig = Self::term_to_signature(&quad.subject, hashes, term);
        let p_sig = Self::term_to_signature(&quad.predicate, hashes, term);
        let o_sig = Self::term_to_signature(&quad.object, hashes, term);
        format!("{}|{}|{}", s_sig, p_sig, o_sig)
    }

    /// Convert a single term to its signature representation.
    ///
    /// Maps a term to a string used in signature generation, handling the special
    /// cases of the target node, grounded/ungrounded blank nodes, and literal values.
    ///
    /// # Arguments
    ///
    /// * `term` - The term to convert
    /// * `hashes` - Map of grounded blank nodes to their hash values
    /// * `target` - The blank node currently being hashed (to use "@self" marker)
    ///
    /// # Returns
    ///
    /// - `"@self"` if term equals target
    /// - Hash value as string if term is a grounded blank node
    /// - `"@blank"` if term is an ungrounded blank node
    /// - Literal representation otherwise (e.g., `"<http://example.org/iri>"`)
    fn term_to_signature(term: &str, hashes: &HashMap<String, u64>, target: &str) -> String {
        if term == target {
            "@self".to_string()
        } else if term.starts_with("_:") {
            hashes
                .get(term)
                .map(|h| h.to_string())
                .unwrap_or_else(|| "@blank".to_string())
        } else {
            term.to_string()
        }
    }

    /// Check if a term is grounded (either not a blank node, or a grounded blank node).
    ///
    /// A term is grounded if it's not a blank node, or if it's a blank node that has
    /// been assigned a unique hash value.
    ///
    /// # Arguments
    ///
    /// * `term` - The term to check
    /// * `hashes` - Map of grounded blank nodes
    ///
    /// # Returns
    ///
    /// `true` if the term is not a blank node or is a grounded blank node, `false` otherwise
    fn is_term_grounded(term: &str, hashes: &HashMap<String, u64>) -> bool {
        !term.starts_with("_:") || hashes.contains_key(term)
    }

    /// Hash a string using MurmurHash3 (128-bit, truncated to 64-bit).
    ///
    /// Uses the MurmurHash3 algorithm for fast, deterministic hashing with low
    /// collision probability. The 128-bit hash is truncated to 64 bits for simplicity.
    ///
    /// # Arguments
    ///
    /// * `data` - The string to hash
    ///
    /// # Returns
    ///
    /// A 64-bit hash value
    fn hash_string(data: &str) -> u64 {
        let mut cursor = Cursor::new(data.as_bytes());
        let hash128 = murmur3::murmur3_x64_128(&mut cursor, 0).unwrap_or(0);
        // Use the lower 64 bits of the 128-bit hash
        (hash128 & 0xFFFFFFFFFFFFFFFF) as u64
    }

    /// Check if a hash map contains a specific value.
    ///
    /// Helper function to determine if any key in the map has the given value.
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash map to search
    /// * `value` - The value to look for
    ///
    /// # Returns
    ///
    /// `true` if the value exists in the map, `false` otherwise
    fn hash_contains_value(hash: &HashMap<String, u64>, value: u64) -> bool {
        hash.values().any(|&v| v == value)
    }

    /// Filter triples to only those containing at least one blank node.
    ///
    /// Extracts all triples where the subject, predicate, or object is a blank node
    /// (identifier starts with "_:"). These triples require structural analysis for
    /// isomorphism checking.
    ///
    /// # Arguments
    ///
    /// * `graph` - The normalized graph to filter
    ///
    /// # Returns
    ///
    /// Vector of triples containing at least one blank node
    fn get_quads_with_blank_nodes(graph: &[NormalizedTriple]) -> Vec<NormalizedTriple> {
        graph
            .iter()
            .filter(|quad| {
                quad.subject.starts_with("_:")
                    || quad.predicate.starts_with("_:")
                    || quad.object.starts_with("_:")
            })
            .cloned()
            .collect()
    }

    /// Filter triples to only those without any blank nodes.
    ///
    /// Extracts all triples where none of the subject, predicate, or object positions
    /// contain blank nodes. These triples must match exactly between isomorphic graphs
    /// and serve as an early-exit optimization.
    ///
    /// # Arguments
    ///
    /// * `graph` - The normalized graph to filter
    ///
    /// # Returns
    ///
    /// Vector of triples without blank nodes
    fn get_quads_without_blank_nodes(graph: &[NormalizedTriple]) -> Vec<NormalizedTriple> {
        graph
            .iter()
            .filter(|quad| {
                !quad.subject.starts_with("_:")
                    && !quad.predicate.starts_with("_:")
                    && !quad.object.starts_with("_:")
            })
            .cloned()
            .collect()
    }

    /// Create a hash map index of triples for fast lookup.
    ///
    /// Converts each triple to a canonical string key (subject|predicate|object) and
    /// stores it in a hash map. This enables O(1) membership testing for comparing
    /// non-blank triples between graphs.
    ///
    /// # Arguments
    ///
    /// * `graph` - The normalized graph to index
    ///
    /// # Returns
    ///
    /// Hash map where keys are triple string representations and values are always `true`
    fn index_graph(graph: &[NormalizedTriple]) -> HashMap<String, bool> {
        let mut index = HashMap::new();
        for quad in graph {
            let key = format!("{}|{}|{}", quad.subject, quad.predicate, quad.object);
            index.insert(key, true);
        }
        index
    }

    /// Remove duplicate triples from a graph.
    ///
    /// Uses hash map indexing to identify and remove duplicate triples, returning
    /// only unique triples. This is necessary because the algorithm may generate
    /// duplicate entries during processing.
    ///
    /// # Arguments
    ///
    /// * `graph` - The normalized graph to deduplicate
    ///
    /// # Returns
    ///
    /// Vector of unique triples
    fn uniq_graph(graph: &[NormalizedTriple]) -> Vec<NormalizedTriple> {
        let index = Self::index_graph(graph);
        index
            .keys()
            .map(|key| {
                let parts: Vec<&str> = key.split('|').collect();
                NormalizedTriple {
                    subject: parts[0].to_string(),
                    predicate: parts[1].to_string(),
                    object: parts[2].to_string(),
                }
            })
            .collect()
    }

    /// Extract all unique blank node identifiers from a graph.
    ///
    /// Scans all triples and collects unique blank node identifiers (those starting
    /// with "_:") from subject, predicate, and object positions. Returns them in
    /// sorted order for consistent processing.
    ///
    /// # Arguments
    ///
    /// * `graph` - The normalized graph to scan
    ///
    /// # Returns
    ///
    /// Sorted vector of unique blank node identifiers
    fn get_graph_blank_nodes(graph: &[NormalizedTriple]) -> Vec<String> {
        let mut blanks = HashSet::new();
        for quad in graph {
            if quad.subject.starts_with("_:") {
                blanks.insert(quad.subject.clone());
            }
            if quad.predicate.starts_with("_:") {
                blanks.insert(quad.predicate.clone());
            }
            if quad.object.starts_with("_:") {
                blanks.insert(quad.object.clone());
            }
        }
        let mut result: Vec<String> = blanks.into_iter().collect();
        result.sort();
        result
    }
}

/// Normalized triple representation with string-based node values.
///
/// Internal representation used by the graph isomorphism algorithm. All nodes
/// (subjects, predicates, objects) are normalized to string representations:
/// - IRIs: `"<http://example.org/iri>"`
/// - Literals: `"\"literal value\""`
/// - Blank nodes: `"_:identifier"`
/// - Variables (treated as blank nodes): `"_:b0"`, `"_:b1"`, etc.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct NormalizedTriple {
    subject: String,
    predicate: String,
    object: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::isomorphism::core::{Triple, TripleNode};

    #[test]
    fn test_normalize_bgp() {
        let bgp = vec![Triple {
            subject: TripleNode::Variable("s".to_string()),
            predicate: TripleNode::IRI("http://example.org/predicate".to_string()),
            object: TripleNode::Variable("o".to_string()),
        }];

        let normalized = GraphIsomorphism::normalize_bgp(&bgp);
        assert_eq!(normalized.len(), 1);
        assert!(normalized[0].subject.starts_with("_:"));
        assert!(normalized[0].object.starts_with("_:"));
    }

    #[test]
    fn test_isomorphic_bgps() {
        let bgp1 = vec![Triple {
            subject: TripleNode::Variable("x".to_string()),
            predicate: TripleNode::IRI("http://example.org/p".to_string()),
            object: TripleNode::Variable("y".to_string()),
        }];

        let bgp2 = vec![Triple {
            subject: TripleNode::Variable("a".to_string()),
            predicate: TripleNode::IRI("http://example.org/p".to_string()),
            object: TripleNode::Variable("b".to_string()),
        }];

        let result = GraphIsomorphism::check_bgp_isomorphism(&bgp1, &bgp2);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_non_isomorphic_bgps() {
        let bgp1 = vec![Triple {
            subject: TripleNode::Variable("x".to_string()),
            predicate: TripleNode::IRI("http://example.org/p1".to_string()),
            object: TripleNode::Variable("y".to_string()),
        }];

        let bgp2 = vec![Triple {
            subject: TripleNode::Variable("a".to_string()),
            predicate: TripleNode::IRI("http://example.org/p2".to_string()),
            object: TripleNode::Variable("b".to_string()),
        }];

        let result = GraphIsomorphism::check_bgp_isomorphism(&bgp1, &bgp2);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_multiple_triples() {
        let bgp1 = vec![
            Triple {
                subject: TripleNode::Variable("x".to_string()),
                predicate: TripleNode::IRI("http://example.org/p".to_string()),
                object: TripleNode::Variable("y".to_string()),
            },
            Triple {
                subject: TripleNode::Variable("y".to_string()),
                predicate: TripleNode::IRI("http://example.org/q".to_string()),
                object: TripleNode::Literal("value".to_string()),
            },
        ];

        let bgp2 = vec![
            Triple {
                subject: TripleNode::Variable("a".to_string()),
                predicate: TripleNode::IRI("http://example.org/p".to_string()),
                object: TripleNode::Variable("b".to_string()),
            },
            Triple {
                subject: TripleNode::Variable("b".to_string()),
                predicate: TripleNode::IRI("http://example.org/q".to_string()),
                object: TripleNode::Literal("value".to_string()),
            },
        ];

        let result = GraphIsomorphism::check_bgp_isomorphism(&bgp1, &bgp2);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_hash_string() {
        let hash1 = GraphIsomorphism::hash_string("test");
        let hash2 = GraphIsomorphism::hash_string("test");
        let hash3 = GraphIsomorphism::hash_string("different");

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_get_graph_blank_nodes() {
        let graph = vec![NormalizedTriple {
            subject: "_:b0".to_string(),
            predicate: "<http://example.org/p>".to_string(),
            object: "_:b1".to_string(),
        }];

        let blanks = GraphIsomorphism::get_graph_blank_nodes(&graph);
        assert_eq!(blanks.len(), 2);
        assert!(blanks.contains(&"_:b0".to_string()));
        assert!(blanks.contains(&"_:b1".to_string()));
    }

    #[test]
    fn test_complex_isomorphism() {
        // Test a more complex case with multiple blank nodes
        let bgp1 = vec![
            Triple {
                subject: TripleNode::Variable("x".to_string()),
                predicate: TripleNode::IRI("http://example.org/p".to_string()),
                object: TripleNode::Variable("y".to_string()),
            },
            Triple {
                subject: TripleNode::Variable("x".to_string()),
                predicate: TripleNode::IRI("http://example.org/q".to_string()),
                object: TripleNode::Variable("z".to_string()),
            },
            Triple {
                subject: TripleNode::Variable("y".to_string()),
                predicate: TripleNode::IRI("http://example.org/r".to_string()),
                object: TripleNode::Literal("A".to_string()),
            },
            Triple {
                subject: TripleNode::Variable("z".to_string()),
                predicate: TripleNode::IRI("http://example.org/r".to_string()),
                object: TripleNode::Literal("B".to_string()),
            },
        ];

        let bgp2 = vec![
            Triple {
                subject: TripleNode::Variable("a".to_string()),
                predicate: TripleNode::IRI("http://example.org/p".to_string()),
                object: TripleNode::Variable("b".to_string()),
            },
            Triple {
                subject: TripleNode::Variable("a".to_string()),
                predicate: TripleNode::IRI("http://example.org/q".to_string()),
                object: TripleNode::Variable("c".to_string()),
            },
            Triple {
                subject: TripleNode::Variable("b".to_string()),
                predicate: TripleNode::IRI("http://example.org/r".to_string()),
                object: TripleNode::Literal("A".to_string()),
            },
            Triple {
                subject: TripleNode::Variable("c".to_string()),
                predicate: TripleNode::IRI("http://example.org/r".to_string()),
                object: TripleNode::Literal("B".to_string()),
            },
        ];

        let result = GraphIsomorphism::check_bgp_isomorphism(&bgp1, &bgp2);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_pathological_case_many_blank_nodes() {
        // Test a case with many blank nodes that would be slow with brute-force
        // The hash-based algorithm should handle this efficiently
        let bgp1 = vec![
            Triple {
                subject: TripleNode::Variable("v1".to_string()),
                predicate: TripleNode::IRI("http://example.org/p1".to_string()),
                object: TripleNode::Literal("A".to_string()),
            },
            Triple {
                subject: TripleNode::Variable("v2".to_string()),
                predicate: TripleNode::IRI("http://example.org/p1".to_string()),
                object: TripleNode::Literal("B".to_string()),
            },
            Triple {
                subject: TripleNode::Variable("v3".to_string()),
                predicate: TripleNode::IRI("http://example.org/p1".to_string()),
                object: TripleNode::Literal("C".to_string()),
            },
            Triple {
                subject: TripleNode::Variable("v4".to_string()),
                predicate: TripleNode::IRI("http://example.org/p1".to_string()),
                object: TripleNode::Literal("D".to_string()),
            },
            Triple {
                subject: TripleNode::Variable("v5".to_string()),
                predicate: TripleNode::IRI("http://example.org/p1".to_string()),
                object: TripleNode::Literal("E".to_string()),
            },
            Triple {
                subject: TripleNode::Variable("v6".to_string()),
                predicate: TripleNode::IRI("http://example.org/p1".to_string()),
                object: TripleNode::Literal("F".to_string()),
            },
        ];

        let bgp2 = vec![
            Triple {
                subject: TripleNode::Variable("x1".to_string()),
                predicate: TripleNode::IRI("http://example.org/p1".to_string()),
                object: TripleNode::Literal("A".to_string()),
            },
            Triple {
                subject: TripleNode::Variable("x2".to_string()),
                predicate: TripleNode::IRI("http://example.org/p1".to_string()),
                object: TripleNode::Literal("B".to_string()),
            },
            Triple {
                subject: TripleNode::Variable("x3".to_string()),
                predicate: TripleNode::IRI("http://example.org/p1".to_string()),
                object: TripleNode::Literal("C".to_string()),
            },
            Triple {
                subject: TripleNode::Variable("x4".to_string()),
                predicate: TripleNode::IRI("http://example.org/p1".to_string()),
                object: TripleNode::Literal("D".to_string()),
            },
            Triple {
                subject: TripleNode::Variable("x5".to_string()),
                predicate: TripleNode::IRI("http://example.org/p1".to_string()),
                object: TripleNode::Literal("E".to_string()),
            },
            Triple {
                subject: TripleNode::Variable("x6".to_string()),
                predicate: TripleNode::IRI("http://example.org/p1".to_string()),
                object: TripleNode::Literal("F".to_string()),
            },
        ];

        // This should complete quickly with hash-based grounding
        let result = GraphIsomorphism::check_bgp_isomorphism(&bgp1, &bgp2);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_non_isomorphic_with_many_nodes() {
        // Similar structure but different literals - should detect non-isomorphism quickly
        let bgp1 = vec![
            Triple {
                subject: TripleNode::Variable("v1".to_string()),
                predicate: TripleNode::IRI("http://example.org/p1".to_string()),
                object: TripleNode::Literal("A".to_string()),
            },
            Triple {
                subject: TripleNode::Variable("v2".to_string()),
                predicate: TripleNode::IRI("http://example.org/p1".to_string()),
                object: TripleNode::Literal("B".to_string()),
            },
        ];

        let bgp2 = vec![
            Triple {
                subject: TripleNode::Variable("x1".to_string()),
                predicate: TripleNode::IRI("http://example.org/p1".to_string()),
                object: TripleNode::Literal("X".to_string()),
            },
            Triple {
                subject: TripleNode::Variable("x2".to_string()),
                predicate: TripleNode::IRI("http://example.org/p1".to_string()),
                object: TripleNode::Literal("Y".to_string()),
            },
        ];

        let result = GraphIsomorphism::check_bgp_isomorphism(&bgp1, &bgp2);
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }
}
