#[macro_use]
extern crate criterion;
use criterion::*;

extern crate allocators;
use allocators::{
    // no bookkeeping
    freelist::FreeList,     // dead simple free list
    // has bookkeeping
    handlemap::HandleMap,   // free list with an explicit stack
    handlemap2::HandleMap2, // free list with implicit stack
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("alloc (freelist)", |b| {
        let mut fl: FreeList<u64> = FreeList::new();
        let mut handles = Vec::new();
        handles.reserve(100000);
        b.iter(|| {
            for i in 0..100000 {
                handles.push(fl.insert(i));
            }

            for h in &handles {
                fl.remove(*h);
            }
        });
    });

    c.bench_function("alloc (handlemap)", |b| {
        let mut hm: HandleMap<u64> = HandleMap::new();
        let mut handles = Vec::new();
        handles.reserve(100000);
        b.iter(|| {
            for i in 0..100000 {
                handles.push(hm.insert(i));
            }

            for h in &handles {
                hm.remove(*h);
            }
        });
    });

    c.bench_function("alloc (handlemap2)", |b| {
        let mut hm2: HandleMap2<u64> = HandleMap2::new();
        let mut handles = Vec::new();
        handles.reserve(100000);
        b.iter(|| {
            for i in 0..100000 {
                handles.push(hm2.insert(i));
            }

            for h in &handles {
                hm2.remove(*h);
            }
        });
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
