use tulna_rs::graph::{GraphIsomorphism, Triple, TripleNode};

#[test]
fn test_repro_proptest_failure() {
    let t1 = Triple {
        subject: TripleNode::BlankNode("0".to_string()),
        predicate: TripleNode::Variable("a".to_string()),
        object: TripleNode::Variable("aa".to_string()),
    };
    let t2 = Triple {
        subject: TripleNode::Variable("a0".to_string()),
        predicate: TripleNode::Variable("ab".to_string()),
        object: TripleNode::BlankNode("1".to_string()),
    };

    let graph1 = vec![t1.clone(), t2.clone()];
    // Reversed order
    let graph2 = vec![t2.clone(), t1.clone()];

    // They should be isomorphic (identical sets of triples)
    let result = GraphIsomorphism::are_isomorphic(&graph1, &graph2);
    assert!(result.is_ok());
    assert!(result.unwrap());
}
