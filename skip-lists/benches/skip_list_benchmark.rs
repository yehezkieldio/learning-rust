// cargo bench

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use skip_lists::SkipList;

fn benchmark_skip_list(c: &mut Criterion) {
    c.bench_function("append 1000 items", |b| {
        b.iter(|| {
            let mut list = SkipList {
                head: None,
                tails: vec![None; 32],
                max_level: 32,
                length: 0,
            };

            for i in 0..1000 {
                list.append(i, format!("command {}", i));
            }
        })
    });

    c.bench_function("find 1000 items", |b| {
        let mut list = SkipList {
            head: None,
            tails: vec![None; 32],
            max_level: 32,
            length: 0,
        };

        for i in 0..1000 {
            list.append(i, format!("command {}", i));
        }

        b.iter(|| {
            for i in 0..1000 {
                black_box(list.find(i));
            }
        })
    });
}

criterion_group!(benches, benchmark_skip_list);
criterion_main!(benches);
