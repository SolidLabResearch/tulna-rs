//! Graph Isomorphism Example
//!
//! This example demonstrates how to use the GraphIsomorphism API directly
//! for checking RDF graph isomorphism without needing to work with queries.

use tulna_rs::graph::{GraphIsomorphism, Triple, TripleNode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Graph Isomorphism Examples ===\n");

    // Example 1: Simple isomorphic graphs with different variable names
    example_1_variable_renaming()?;

    // Example 2: Non-isomorphic graphs
    example_2_non_isomorphic()?;

    // Example 3: Complex graph with multiple triples
    example_3_complex_graph()?;

    // Example 4: Graph with blank nodes
    example_4_blank_nodes()?;

    // Example 5: Symmetric structure
    example_5_symmetric_structure()?;

    // Example 6: Graph with literals
    example_6_with_literals()?;

    Ok(())
}

fn example_1_variable_renaming() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 1: Variable Renaming");
    println!("Checking if graphs are isomorphic despite different variable names\n");

    let graph1 = vec![Triple {
        subject: TripleNode::Variable("x".to_string()),
        predicate: TripleNode::IRI("http://example.org/knows".to_string()),
        object: TripleNode::Variable("y".to_string()),
    }];

    let graph2 = vec![Triple {
        subject: TripleNode::Variable("person".to_string()),
        predicate: TripleNode::IRI("http://example.org/knows".to_string()),
        object: TripleNode::Variable("friend".to_string()),
    }];

    let result = GraphIsomorphism::are_isomorphic(&graph1, &graph2)?;
    println!("Graph 1: ?x <knows> ?y");
    println!("Graph 2: ?person <knows> ?friend");
    println!("Are isomorphic: {}\n", result);

    Ok(())
}

fn example_2_non_isomorphic() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 2: Non-Isomorphic Graphs");
    println!("Checking graphs with different predicates\n");

    let graph1 = vec![Triple {
        subject: TripleNode::Variable("x".to_string()),
        predicate: TripleNode::IRI("http://example.org/knows".to_string()),
        object: TripleNode::Variable("y".to_string()),
    }];

    let graph2 = vec![Triple {
        subject: TripleNode::Variable("a".to_string()),
        predicate: TripleNode::IRI("http://example.org/likes".to_string()),
        object: TripleNode::Variable("b".to_string()),
    }];

    let result = GraphIsomorphism::are_isomorphic(&graph1, &graph2)?;
    println!("Graph 1: ?x <knows> ?y");
    println!("Graph 2: ?a <likes> ?b");
    println!("Are isomorphic: {}\n", result);

    Ok(())
}

fn example_3_complex_graph() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 3: Complex Graph with Multiple Triples");
    println!("Checking a graph describing a person with multiple properties\n");

    let graph1 = vec![
        Triple {
            subject: TripleNode::Variable("person".to_string()),
            predicate: TripleNode::IRI("http://xmlns.com/foaf/0.1/name".to_string()),
            object: TripleNode::Literal("Alice".to_string()),
        },
        Triple {
            subject: TripleNode::Variable("person".to_string()),
            predicate: TripleNode::IRI("http://xmlns.com/foaf/0.1/age".to_string()),
            object: TripleNode::Literal("30".to_string()),
        },
        Triple {
            subject: TripleNode::Variable("person".to_string()),
            predicate: TripleNode::IRI("http://xmlns.com/foaf/0.1/knows".to_string()),
            object: TripleNode::Variable("friend".to_string()),
        },
    ];

    let graph2 = vec![
        Triple {
            subject: TripleNode::Variable("x".to_string()),
            predicate: TripleNode::IRI("http://xmlns.com/foaf/0.1/name".to_string()),
            object: TripleNode::Literal("Alice".to_string()),
        },
        Triple {
            subject: TripleNode::Variable("x".to_string()),
            predicate: TripleNode::IRI("http://xmlns.com/foaf/0.1/age".to_string()),
            object: TripleNode::Literal("30".to_string()),
        },
        Triple {
            subject: TripleNode::Variable("x".to_string()),
            predicate: TripleNode::IRI("http://xmlns.com/foaf/0.1/knows".to_string()),
            object: TripleNode::Variable("y".to_string()),
        },
    ];

    let result = GraphIsomorphism::are_isomorphic(&graph1, &graph2)?;
    println!("Graph 1: ?person has name 'Alice', age '30', knows ?friend");
    println!("Graph 2: ?x has name 'Alice', age '30', knows ?y");
    println!("Are isomorphic: {}\n", result);

    Ok(())
}

fn example_4_blank_nodes() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 4: Graph with Blank Nodes");
    println!("Checking graphs with explicit blank nodes\n");

    let graph1 = vec![
        Triple {
            subject: TripleNode::BlankNode("b1".to_string()),
            predicate: TripleNode::IRI("http://example.org/type".to_string()),
            object: TripleNode::IRI("http://example.org/Person".to_string()),
        },
        Triple {
            subject: TripleNode::BlankNode("b1".to_string()),
            predicate: TripleNode::IRI("http://example.org/name".to_string()),
            object: TripleNode::Literal("Bob".to_string()),
        },
    ];

    let graph2 = vec![
        Triple {
            subject: TripleNode::BlankNode("blank0".to_string()),
            predicate: TripleNode::IRI("http://example.org/type".to_string()),
            object: TripleNode::IRI("http://example.org/Person".to_string()),
        },
        Triple {
            subject: TripleNode::BlankNode("blank0".to_string()),
            predicate: TripleNode::IRI("http://example.org/name".to_string()),
            object: TripleNode::Literal("Bob".to_string()),
        },
    ];

    let result = GraphIsomorphism::are_isomorphic(&graph1, &graph2)?;
    println!("Graph 1: _:b1 is a Person named 'Bob'");
    println!("Graph 2: _:blank0 is a Person named 'Bob'");
    println!("Are isomorphic: {}\n", result);

    Ok(())
}

fn example_5_symmetric_structure() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 5: Symmetric Graph Structure");
    println!("Testing the hash-based algorithm with symmetric nodes\n");

    let graph1 = vec![
        Triple {
            subject: TripleNode::Variable("x".to_string()),
            predicate: TripleNode::IRI("http://example.org/connected".to_string()),
            object: TripleNode::Variable("y".to_string()),
        },
        Triple {
            subject: TripleNode::Variable("y".to_string()),
            predicate: TripleNode::IRI("http://example.org/connected".to_string()),
            object: TripleNode::Variable("x".to_string()),
        },
    ];

    let graph2 = vec![
        Triple {
            subject: TripleNode::Variable("a".to_string()),
            predicate: TripleNode::IRI("http://example.org/connected".to_string()),
            object: TripleNode::Variable("b".to_string()),
        },
        Triple {
            subject: TripleNode::Variable("b".to_string()),
            predicate: TripleNode::IRI("http://example.org/connected".to_string()),
            object: TripleNode::Variable("a".to_string()),
        },
    ];

    let result = GraphIsomorphism::are_isomorphic(&graph1, &graph2)?;
    println!("Graph 1: ?x <-> ?y (bidirectional connection)");
    println!("Graph 2: ?a <-> ?b (bidirectional connection)");
    println!("Are isomorphic: {}\n", result);

    Ok(())
}

fn example_6_with_literals() -> Result<(), Box<dyn std::error::Error>> {
    println!("Example 6: Distinguishing Nodes by Literals");
    println!("Hash-based grounding identifies nodes by their literal values\n");

    let graph1 = vec![
        Triple {
            subject: TripleNode::Variable("person1".to_string()),
            predicate: TripleNode::IRI("http://xmlns.com/foaf/0.1/name".to_string()),
            object: TripleNode::Literal("Alice".to_string()),
        },
        Triple {
            subject: TripleNode::Variable("person2".to_string()),
            predicate: TripleNode::IRI("http://xmlns.com/foaf/0.1/name".to_string()),
            object: TripleNode::Literal("Bob".to_string()),
        },
    ];

    let graph2 = vec![
        Triple {
            subject: TripleNode::Variable("x".to_string()),
            predicate: TripleNode::IRI("http://xmlns.com/foaf/0.1/name".to_string()),
            object: TripleNode::Literal("Alice".to_string()),
        },
        Triple {
            subject: TripleNode::Variable("y".to_string()),
            predicate: TripleNode::IRI("http://xmlns.com/foaf/0.1/name".to_string()),
            object: TripleNode::Literal("Bob".to_string()),
        },
    ];

    let result = GraphIsomorphism::are_isomorphic(&graph1, &graph2)?;
    println!("Graph 1: ?person1 named 'Alice', ?person2 named 'Bob'");
    println!("Graph 2: ?x named 'Alice', ?y named 'Bob'");
    println!(
        "Are isomorphic: {} (nodes grounded by their literal values)\n",
        result
    );

    Ok(())
}
