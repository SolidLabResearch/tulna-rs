use proptest::prelude::*;
use tulna_rs::graph::{GraphIsomorphism, Triple, TripleNode};
use std::collections::HashMap;

// Strategy to generate random TripleNodes
fn arb_triple_node() -> impl Strategy<Value = TripleNode> {
    prop_oneof![
        // IRIs
        "[a-z][a-z0-9_]*".prop_map(|s| TripleNode::IRI(format!("http://example.org/{}", s))),
        // Variables
        "[a-z][a-z0-9_]*".prop_map(|s| TripleNode::Variable(s)),
        // Literals
        "[a-zA-Z0-9 ]+".prop_map(|s| TripleNode::Literal(s)),
        // Blank Nodes
        "[a-z0-9]+".prop_map(|s| TripleNode::BlankNode(s)),
    ]
}

// Strategy to generate random Triples
fn arb_triple() -> impl Strategy<Value = Triple> {
    (arb_triple_node(), arb_triple_node(), arb_triple_node()).prop_map(|(s, p, o)| Triple {
        subject: s,
        predicate: p,
        object: o,
    })
}

// Strategy to generate random Graphs (up to 20 triples)
fn arb_graph() -> impl Strategy<Value = Vec<Triple>> {
    prop::collection::vec(arb_triple(), 0..20)
}

proptest! {
    // Property 1: Reflexivity - A graph is always isomorphic to itself
    #[test]
    fn test_isomorphism_reflexivity(graph in arb_graph()) {
        let result = GraphIsomorphism::are_isomorphic(&graph, &graph);
        prop_assert!(result.is_ok());
        prop_assert!(result.unwrap());
    }

    // Property 2: Symmetry - If A is isomorphic to B, then B is isomorphic to A
    // We test this by shuffling A to get B (which is definitely isomorphic)
    #[test]
    fn test_isomorphism_permutation(graph in arb_graph()) {
        let mut shuffled_graph = graph.clone();
        
        // Simple shuffle (deterministic for proptest replayability if needed, 
        // but here we just rely on randomness from proptest runner if we wanted true shuffle, 
        // but proptest inputs are fixed per run. We can't easily shuffle inside proptest without a seed.
        // Instead, we generate TWO graphs, check if they are isomorphic, and verify symmetry.)
        
        // Actually, let's verify A iso A_reversed
        shuffled_graph.reverse();
        
        let result1 = GraphIsomorphism::are_isomorphic(&graph, &shuffled_graph);
        prop_assert!(result1.is_ok());
        let r1 = result1.unwrap();
        prop_assert!(r1);
        
        let result2 = GraphIsomorphism::are_isomorphic(&shuffled_graph, &graph);
        prop_assert!(result2.is_ok());
        prop_assert_eq!(r1, result2.unwrap());
    }

    // Property 3: Variable Renaming
    // If we consistently rename all variables in a graph, it should still be isomorphic
    #[test]
    fn test_isomorphism_variable_renaming(graph in arb_graph()) {
        // 1. Collect all variables
        let mut variables = std::collections::HashSet::new();
        for t in &graph {
            if let TripleNode::Variable(v) = &t.subject { variables.insert(v.clone()); }
            if let TripleNode::Variable(v) = &t.predicate { variables.insert(v.clone()); }
            if let TripleNode::Variable(v) = &t.object { variables.insert(v.clone()); }
        }

        // 2. Create mapping var -> var_renamed
        let mut mapping = HashMap::new();
        for (i, v) in variables.iter().enumerate() {
            mapping.insert(v.clone(), format!("var_{}", i));
        }

        // 3. Apply mapping to create graph2
        let graph2: Vec<Triple> = graph.iter().map(|t| {
            let map_node = |n: &TripleNode| match n {
                TripleNode::Variable(v) => TripleNode::Variable(mapping.get(v).unwrap_or(v).clone()),
                other => other.clone(),
            };
            Triple {
                subject: map_node(&t.subject),
                predicate: map_node(&t.predicate),
                object: map_node(&t.object),
            }
        }).collect();

        // 4. Check isomorphism
        let result = GraphIsomorphism::are_isomorphic(&graph, &graph2);
        prop_assert!(result.is_ok());
        prop_assert!(result.unwrap());
    }
}
