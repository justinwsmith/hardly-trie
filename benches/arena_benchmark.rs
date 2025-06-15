use criterion::{black_box, criterion_group, criterion_main, Criterion};
use hardly_trie::trie::Trie;

fn benchmark_insertions(c: &mut Criterion) {
    c.bench_function("trie_insertions_1000", |b| {
        b.iter(|| {
            let mut trie: Trie<str, usize, 16> = Trie::new();
            for i in 0..1000 {
                let key = format!("key_{}", i);
                trie.insert(black_box(&key), black_box(i));
            }
            trie
        })
    });
}

fn benchmark_mixed_operations(c: &mut Criterion) {
    c.bench_function("trie_mixed_ops_1000", |b| {
        b.iter(|| {
            let mut trie: Trie<str, usize, 16> = Trie::new();

            // Insert 1000 items
            for i in 0..1000 {
                let key = format!("key_{}", i);
                trie.insert(black_box(&key), black_box(i));
            }

            // Lookup every 10th item
            for i in (0..1000).step_by(10) {
                let key = format!("key_{}", i);
                black_box(trie.get(black_box(&key)));
            }

            // Delete every 5th item
            for i in (0..1000).step_by(5) {
                let key = format!("key_{}", i);
                black_box(trie.delete(black_box(&key)));
            }

            trie
        })
    });
}

fn benchmark_memory_reuse(c: &mut Criterion) {
    c.bench_function("trie_memory_reuse", |b| {
        b.iter(|| {
            let mut trie: Trie<str, usize, 16> = Trie::new();

            // Insert and delete in cycles to test memory reuse
            for cycle in 0..10 {
                // Insert 100 items
                for i in 0..100 {
                    let key = format!("cycle_{}_key_{}", cycle, i);
                    trie.insert(black_box(&key), black_box(i));
                }

                // Delete all items from this cycle
                for i in 0..100 {
                    let key = format!("cycle_{}_key_{}", cycle, i);
                    black_box(trie.delete(black_box(&key)));
                }
            }

            trie
        })
    });
}

criterion_group!(
    benches,
    benchmark_insertions,
    benchmark_mixed_operations,
    benchmark_memory_reuse
);
criterion_main!(benches);
