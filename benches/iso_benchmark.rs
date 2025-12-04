use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tulna_rs::graph::{GraphIsomorphism, Triple, TripleNode};

fn make_triple(s: &str, p: &str, o: &str) -> Triple {
    Triple {
        subject: if s.starts_with('?') { TripleNode::Variable(s.to_string()) } else { TripleNode::IRI(s.to_string()) },
        predicate: TripleNode::IRI(p.to_string()),
        object: if o.starts_with('?') { TripleNode::Variable(o.to_string()) } else if o.starts_with('"') { TripleNode::Literal(o.trim_matches('"').to_string()) } else { TripleNode::IRI(o.to_string()) },
    }
}

fn bench_simple_isomorphism(c: &mut Criterion) {
    let graph1 = vec![
        make_triple("?a", "http://knows", "?b"),
        make_triple("?b", "http://knows", "?c"),
    ];
    let graph2 = vec![
        make_triple("?x", "http://knows", "?y"),
        make_triple("?y", "http://knows", "?z"),
    ];

    c.bench_function("simple_chain_3", |b| {
        b.iter(|| GraphIsomorphism::are_isomorphic(black_box(&graph1), black_box(&graph2)))
    });
}

fn bench_regular_graph_verification(c: &mut Criterion) {
    // This triggers the speculation and verification logic
    let graph1 = vec![
        make_triple("?1", "http://next", "?2"),
        make_triple("?2", "http://next", "?3"),
        make_triple("?3", "http://next", "?4"),
        make_triple("?4", "http://next", "?5"),
        make_triple("?5", "http://next", "?6"),
        make_triple("?6", "http://next", "?1"),
    ];

    let graph2 = vec![
        make_triple("?a", "http://next", "?b"),
        make_triple("?b", "http://next", "?c"),
        make_triple("?c", "http://next", "?a"),
        make_triple("?x", "http://next", "?y"),
        make_triple("?y", "http://next", "?z"),
        make_triple("?z", "http://next", "?x"),
    ];

    c.bench_function("regular_graph_false_positive_check", |b| {
        b.iter(|| GraphIsomorphism::are_isomorphic(black_box(&graph1), black_box(&graph2)))
    });
}

fn bench_large_star_graph(c: &mut Criterion) {
    let mut graph1 = Vec::new();
    let mut graph2 = Vec::new();
    
    for i in 0..100 {
        graph1.push(make_triple("?root", "http://hasChild", &format!("?child{}", i)));
        graph2.push(make_triple("?r", "http://hasChild", &format!("?c{}", i)));
    }

    c.bench_function("star_graph_100", |b| {
        b.iter(|| GraphIsomorphism::are_isomorphic(black_box(&graph1), black_box(&graph2)))
    });
}

criterion_group!(benches, bench_simple_isomorphism, bench_regular_graph_verification, bench_large_star_graph);
criterion_main!(benches);
