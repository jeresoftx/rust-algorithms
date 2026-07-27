use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use rust_algorithms::patterns::binary_search::search_insert;
use rust_algorithms::patterns::dynamic_programming::longest_increasing_subsequence;
use rust_algorithms::patterns::range_queries::FenwickTree;
use rust_algorithms::patterns::string_algorithms::find_pattern_positions;
use rust_algorithms::patterns::weighted_graphs::dijkstra_shortest_paths;
use std::hint::black_box;

fn bench_search_insert(c: &mut Criterion) {
    let values: Vec<i32> = (0..100_000).map(|value| value * 2).collect();

    c.bench_function("binary_search::search_insert", |bencher| {
        bencher.iter_batched(
            || values.clone(),
            |input| search_insert(input, black_box(73_333)),
            BatchSize::SmallInput,
        );
    });
}

fn bench_longest_increasing_subsequence(c: &mut Criterion) {
    let values: Vec<i32> = (0_i32..4_000).map(|value| value * 37 % 997).collect();

    c.bench_function(
        "dynamic_programming::longest_increasing_subsequence",
        |bencher| {
            bencher.iter_batched(
                || values.clone(),
                longest_increasing_subsequence,
                BatchSize::SmallInput,
            );
        },
    );
}

fn bench_string_matching(c: &mut Criterion) {
    let mut text = String::new();
    for _ in 0..2_000 {
        text.push_str("rustacean-algorithms-");
    }

    c.bench_function("string_algorithms::find_pattern_positions", |bencher| {
        bencher.iter(|| find_pattern_positions(black_box(&text), black_box("algo")));
    });
}

fn bench_fenwick_prefix_queries(c: &mut Criterion) {
    let values: Vec<i32> = (0..50_000).map(|value| value % 31).collect();
    let tree = FenwickTree::from_values(&values);

    c.bench_function("range_queries::fenwick_prefix_sum", |bencher| {
        bencher.iter(|| {
            let mut total = 0;
            for index in (0..values.len()).step_by(257) {
                total += tree.prefix_sum(black_box(index)).unwrap_or_default();
            }
            total
        });
    });
}

fn bench_dijkstra(c: &mut Criterion) {
    let node_count = 1_000;
    let mut edges = Vec::new();

    for node in 0..node_count - 1 {
        edges.push((node, node + 1, 1));
        if node + 10 < node_count {
            edges.push((node, node + 10, 3));
        }
    }

    c.bench_function("weighted_graphs::dijkstra_shortest_paths", |bencher| {
        bencher.iter(|| {
            dijkstra_shortest_paths(black_box(node_count), black_box(&edges), black_box(0))
        });
    });
}

criterion_group!(
    benches,
    bench_search_insert,
    bench_longest_increasing_subsequence,
    bench_string_matching,
    bench_fenwick_prefix_queries,
    bench_dijkstra
);
criterion_main!(benches);
