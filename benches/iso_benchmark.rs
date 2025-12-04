use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
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

fn generate_star_graph(size: u64) -> (Vec<Triple>, Vec<Triple>) {
    let mut graph1 = Vec::with_capacity(size as usize);
    let mut graph2 = Vec::with_capacity(size as usize);

    for i in 0..size {
        graph1.push(make_triple(
            "?root",
            "http://hasChild",
            &format!("?child{}", i),
        ));
        graph2.push(make_triple("?r", "http://hasChild", &format!("?c{}", i)));
    }
    (graph1, graph2)
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

fn bench_star_graph_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("star_graph_scaling");

    // Define the sizes to benchmark
    let sizes = [10, 100, 1_000, 10_000]; // 100k might be too slow for CI/quick tests, can be added if needed

    for size in sizes.iter() {
        group.throughput(Throughput::Elements(*size));
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let (graph1, graph2) = generate_star_graph(size);
            b.iter(|| GraphIsomorphism::are_isomorphic(black_box(&graph1), black_box(&graph2)));
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_simple_isomorphism,
    bench_regular_graph_verification,
    bench_star_graph_scaling
);
criterion_main!(benches);
