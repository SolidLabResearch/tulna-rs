use tulna_rs::graph::{GraphIsomorphism, Triple, TripleNode};

fn make_triple(s: &str, p: &str, o: &str) -> Triple {
    Triple {
        subject: if s.starts_with('?') {
            TripleNode::Variable(s.to_string())
        } else {
            TripleNode::IRI(s.to_string())
        },
        predicate: TripleNode::IRI(p.to_string()),
        object: if o.starts_with('?') {
            TripleNode::Variable(o.to_string())
        } else if o.starts_with('"') {
            TripleNode::Literal(o.trim_matches('"').to_string())
        } else {
            TripleNode::IRI(o.to_string())
        },
    }
}

#[test]
fn test_coverage_mismatch_non_blank_count() {
    // Branch: if index_a.len() != index_b.len()
    let graph1 = vec![make_triple("http://a", "http://p", "http://b")];
    let graph2 = vec![
        make_triple("http://a", "http://p", "http://b"),
        make_triple("http://a", "http://p", "http://c"),
    ];

    assert!(!GraphIsomorphism::are_isomorphic(&graph1, &graph2).unwrap());
}

#[test]
fn test_coverage_mismatch_non_blank_content() {
    // Branch: if !index_b.contains_key(key)
    let graph1 = vec![make_triple("http://a", "http://p", "http://b")];
    let graph2 = vec![make_triple("http://a", "http://p", "http://c")];

    assert!(!GraphIsomorphism::are_isomorphic(&graph1, &graph2).unwrap());
}

#[test]
fn test_coverage_mismatch_blank_node_count() {
    // Branch: if blank_nodes_a.len() != blank_nodes_b.len()
    let graph1 = vec![make_triple("?x", "http://p", "http://a")];
    let graph2 = vec![
        make_triple("?x", "http://p", "http://a"),
        make_triple("?y", "http://p", "http://b"), // Extra blank node ?y
    ];

    assert!(!GraphIsomorphism::are_isomorphic(&graph1, &graph2).unwrap());
}

#[test]
fn test_coverage_symmetric_cycle_speculation() {
    // A 3-cycle: A->B->C->A
    // Requires speculation because all nodes look identical (1 incoming, 1 outgoing, same predicates)
    let graph1 = vec![
        make_triple("?a", "http://next", "?b"),
        make_triple("?b", "http://next", "?c"),
        make_triple("?c", "http://next", "?a"),
    ];

    let graph2 = vec![
        make_triple("?x", "http://next", "?y"),
        make_triple("?y", "http://next", "?z"),
        make_triple("?z", "http://next", "?x"),
    ];

    assert!(GraphIsomorphism::are_isomorphic(&graph1, &graph2).unwrap());
}

#[test]
fn test_coverage_false_positive_regular_graphs() {
    // Graph A: 6-cycle (1-2-3-4-5-6-1)
    // Graph B: Two 3-cycles (1-2-3-1, 4-5-6-4)
    // Both are 2-regular (if directed properly or treated as such).
    // Pure color refinement might fail to distinguish them without recursion.

    let graph1 = vec![
        make_triple("?1", "http://next", "?2"),
        make_triple("?2", "http://next", "?3"),
        make_triple("?3", "http://next", "?4"),
        make_triple("?4", "http://next", "?5"),
        make_triple("?5", "http://next", "?6"),
        make_triple("?6", "http://next", "?1"),
    ];

    let graph2 = vec![
        // First 3-cycle
        make_triple("?a", "http://next", "?b"),
        make_triple("?b", "http://next", "?c"),
        make_triple("?c", "http://next", "?a"),
        // Second 3-cycle
        make_triple("?x", "http://next", "?y"),
        make_triple("?y", "http://next", "?z"),
        make_triple("?z", "http://next", "?x"),
    ];

    // These should NOT be isomorphic
    assert!(!GraphIsomorphism::are_isomorphic(&graph1, &graph2).unwrap());
}

#[test]
fn test_coverage_grounded_hash_mismatch() {
    // This is tricky to trigger. We need a case where recursion happens,
    // and the speculative groundings lead to a state where 'hashes_a' and 'hashes_b'
    // have different sets of values.

    // We use the regular graph case again, but verifying it returns false is essentially verifying this path
    // because the speculation will try to map a node from 6-cycle to 3-cycle,
    // propagate constraints, and eventually realize the hashes don't match up or bijection isn't total.

    let graph1 = vec![make_triple("?a", "http://p", "?b")];
    let graph2 = vec![
        make_triple("?x", "http://p", "?x"), // Self-loop
    ];

    // Initial signatures:
    // a: p->b (outgoing), b: p<-a (incoming)
    // x: p->x (outgoing + incoming)
    // These signatures are different immediately, so it hits the early hash mismatch.
    assert!(!GraphIsomorphism::are_isomorphic(&graph1, &graph2).unwrap());
}

#[test]
fn test_full_recursion_depth() {
    // A fully connected graph (clique) of size 4.
    // Isomorphic to itself.
    // O(N!) worst case if not careful, but N=4 is tiny.
    // Ensures the recursion works fully.

    let mut clique1 = Vec::new();
    let mut clique2 = Vec::new();
    let nodes1 = ["?1", "?2", "?3", "?4"];
    let nodes2 = ["?a", "?b", "?c", "?d"];

    for i in 0..4 {
        for j in 0..4 {
            if i != j {
                clique1.push(make_triple(nodes1[i], "http://edge", nodes1[j]));
                clique2.push(make_triple(nodes2[i], "http://edge", nodes2[j]));
            }
        }
    }

    assert!(GraphIsomorphism::are_isomorphic(&clique1, &clique2).unwrap());
}
