use beap::Beap;
use criterion::measurement::WallTime;
use criterion::BenchmarkGroup;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};
use std::collections::{BTreeSet, BinaryHeap};

const SEED: u64 = 1830123;

trait PriorityQueue: Clone {
    type Item;

    fn new() -> Self;

    fn push(&mut self, x: Self::Item);

    fn pop(&mut self) -> Option<Self::Item>;

    fn peek(&self) -> Option<&Self::Item>;

    fn tail(&self) -> Option<&Self::Item>;

    fn contains(&self, val: &Self::Item) -> bool;

    fn len(&self) -> usize;

    fn describe() -> String;
}

impl<T: Ord + Clone> PriorityQueue for Beap<T> {
    type Item = T;

    fn new() -> Self {
        Beap::new()
    }

    fn push(&mut self, x: Self::Item) {
        self.push(x)
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.pop()
    }

    fn peek(&self) -> Option<&Self::Item> {
        self.peek()
    }

    fn tail(&self) -> Option<&Self::Item> {
        self.tail()
    }

    fn contains(&self, val: &Self::Item) -> bool {
        self.contains(val)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn describe() -> String {
        "Beap".to_string()
    }
}

impl<T: Ord + Clone> PriorityQueue for BinaryHeap<T> {
    type Item = T;

    fn new() -> Self {
        BinaryHeap::new()
    }

    fn push(&mut self, x: Self::Item) {
        self.push(x)
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.pop()
    }

    fn peek(&self) -> Option<&Self::Item> {
        self.peek()
    }

    fn tail(&self) -> Option<&Self::Item> {
        self.iter().min()
    }

    fn contains(&self, val: &Self::Item) -> bool {
        self.iter().any(|x| x == val)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn describe() -> String {
        "BinaryHeap".to_string()
    }
}

impl<T: Ord + Clone> PriorityQueue for BTreeSet<T> {
    type Item = T;

    fn new() -> Self {
        BTreeSet::new()
    }

    fn push(&mut self, x: Self::Item) {
        self.insert(x);
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.pop_last()
    }

    fn peek(&self) -> Option<&Self::Item> {
        self.last()
    }

    fn tail(&self) -> Option<&Self::Item> {
        self.first()
    }

    fn contains(&self, val: &Self::Item) -> bool {
        self.contains(val)
    }
    fn len(&self) -> usize {
        self.len()
    }

    fn describe() -> String {
        "BTreeSet".to_string()
    }
}

fn bench_push(c: &mut Criterion) {
    call_push_group(c, 100);
    call_push_group(c, 1000);
    call_push_group(c, 10000);
}

fn call_push_group(c: &mut Criterion, n: i64) {
    let mut group = c.benchmark_group(format!("Push {n} i64 items"));
    group.sample_size(30);

    let mut rng = SmallRng::seed_from_u64(SEED);
    let mut items: Vec<i64> = (0..n).collect();
    items.shuffle(&mut rng);

    do_bench_push::<Beap<i64>>(&mut group, &items);
    do_bench_push::<BinaryHeap<i64>>(&mut group, &items);
    do_bench_push::<BTreeSet<i64>>(&mut group, &items);

    group.finish();
}

fn do_bench_push<Q: PriorityQueue<Item: Ord + Clone>>(
    c: &mut BenchmarkGroup<WallTime>,
    items: &[Q::Item],
) {
    c.bench_function(Q::describe(), |b| {
        b.iter(|| {
            let mut queue = Q::new();
            for i in items {
                queue.push(i.clone());
            }
            black_box(queue)
        })
    });
}

fn bench_pop(c: &mut Criterion) {
    call_pop_group(c, 100);
    call_pop_group(c, 1000);
    call_pop_group(c, 10000);
}

fn call_pop_group(c: &mut Criterion, n: i64) {
    let mut group = c.benchmark_group(format!("Pop {n} i64 items"));
    group.sample_size(30);

    let mut rng = SmallRng::seed_from_u64(SEED);
    let mut items: Vec<i64> = (0..n).collect();
    items.shuffle(&mut rng);

    do_bench_pop::<Beap<i64>>(&mut group, Beap::from(items.clone()));
    do_bench_pop::<BinaryHeap<i64>>(&mut group, BinaryHeap::from(items.clone()));
    do_bench_pop::<BTreeSet<i64>>(&mut group, BTreeSet::from_iter(items));

    group.finish();
}

fn do_bench_pop<Q: PriorityQueue<Item: Ord + Clone>>(c: &mut BenchmarkGroup<WallTime>, q: Q) {
    c.bench_function(Q::describe(), |b| {
        b.iter(|| {
            let mut queue = q.clone();
            let mut popped = Vec::with_capacity(q.len());
            while let Some(x) = queue.pop() {
                popped.push(x);
            }
            black_box((queue, popped))
        })
    });
}

fn bench_push_peek(c: &mut Criterion) {
    call_push_peek_group(c, 100);
    call_push_peek_group(c, 1000);
    call_push_peek_group(c, 10000);
}

fn call_push_peek_group(c: &mut Criterion, n: i64) {
    let mut group = c.benchmark_group(format!("Push & peek {n} i64 items"));
    group.sample_size(30);

    let mut rng = SmallRng::seed_from_u64(SEED);
    let mut items: Vec<i64> = (0..n).collect();
    items.shuffle(&mut rng);

    do_bench_push_peek::<Beap<i64>>(&mut group, &items);
    do_bench_push_peek::<BinaryHeap<i64>>(&mut group, &items);
    do_bench_push_peek::<BTreeSet<i64>>(&mut group, &items);

    group.finish();
}

fn do_bench_push_peek<Q: PriorityQueue<Item: Ord + Clone>>(
    c: &mut BenchmarkGroup<WallTime>,
    items: &[Q::Item],
) {
    c.bench_function(Q::describe(), |b| {
        b.iter(|| {
            let mut queue = Q::new();
            let mut heads = Vec::with_capacity(items.len());
            for i in items {
                queue.push(i.clone());
                heads.push(queue.peek().cloned());
            }
            black_box((queue, heads))
        })
    });
}

fn bench_contains(c: &mut Criterion) {
    call_contains_group(c, 100);
    call_contains_group(c, 1000);
    call_contains_group(c, 10000);
}

fn call_contains_group(c: &mut Criterion, n: i64) {
    let mut group = c.benchmark_group(format!("Contains {n} i64 items"));
    group.sample_size(30);

    let mut rng = SmallRng::seed_from_u64(SEED);
    let mut items: Vec<i64> = (0..n).collect();
    items.shuffle(&mut rng);

    do_bench_contains::<Beap<i64>>(&mut group, Beap::from(items.clone()), &items);
    do_bench_contains::<BinaryHeap<i64>>(&mut group, BinaryHeap::from(items.clone()), &items);
    do_bench_contains::<BTreeSet<i64>>(
        &mut group,
        BTreeSet::from_iter(items.iter().cloned()),
        &items,
    );

    group.finish();
}

fn do_bench_contains<Q: PriorityQueue<Item: Ord + Clone>>(
    c: &mut BenchmarkGroup<WallTime>,
    q: Q,
    items: &[Q::Item],
) {
    c.bench_function(Q::describe(), |b| {
        b.iter(|| {
            let mut result = Vec::with_capacity(items.len());
            for i in items {
                result.push(q.contains(i));
            }
            black_box(result)
        })
    });
}

fn bench_push_tail(c: &mut Criterion) {
    call_push_tail_group(c, 100);
    call_push_tail_group(c, 1000);
    call_push_tail_group(c, 10000);
}

fn call_push_tail_group(c: &mut Criterion, n: i64) {
    let mut group = c.benchmark_group(format!("Push & tail {n} i64 items"));
    group.sample_size(30);

    let mut rng = SmallRng::seed_from_u64(SEED);
    let mut items: Vec<i64> = (0..n).collect();
    items.shuffle(&mut rng);

    do_bench_push_tail::<Beap<i64>>(&mut group, &items);
    do_bench_push_tail::<BinaryHeap<i64>>(&mut group, &items);
    do_bench_push_tail::<BTreeSet<i64>>(&mut group, &items);

    group.finish();
}

fn do_bench_push_tail<Q: PriorityQueue<Item: Ord + Clone>>(
    c: &mut BenchmarkGroup<WallTime>,
    items: &[Q::Item],
) {
    c.bench_function(Q::describe(), |b| {
        b.iter(|| {
            let mut queue = Q::new();
            let mut tails = Vec::with_capacity(items.len());
            for i in items {
                queue.push(i.clone());
                tails.push(queue.tail().cloned());
            }
            black_box((queue, tails))
        })
    });
}

criterion_group!(
    basics,
    bench_push,
    bench_pop,
    bench_push_peek,
    bench_contains,
    bench_push_tail
);
criterion_main!(basics);
