use crate::Beap;
use rand::{thread_rng, Rng};
use std::collections::BinaryHeap;

#[test]
fn test_push() {
    // Fixed tests
    let mut beap: Beap<i32> = Beap::new();
    assert_eq!(beap.len(), 0);
    assert_eq!(beap.peek(), None);

    beap.push(1);
    assert_eq!(beap.len(), 1);
    assert_eq!(beap.peek(), Some(&1));

    beap.push(2);
    assert_eq!(beap.len(), 2);
    assert_eq!(beap.peek(), Some(&2));

    beap.push(3);
    assert_eq!(beap.len(), 3);
    assert_eq!(beap.peek(), Some(&3));

    beap.push(0);
    assert_eq!(beap.len(), 4);
    assert_eq!(beap.peek(), Some(&3));

    beap.push(5);
    assert_eq!(beap.len(), 5);
    assert_eq!(beap.peek(), Some(&5));

    beap.push(4);
    assert_eq!(beap.len(), 6);
    assert_eq!(beap.peek(), Some(&5));

    println!("{:?}", beap.into_vec());

    // Random tests
    let mut rng = thread_rng();

    for size in 1..=100 {
        let mut elements: Vec<i64> = Vec::with_capacity(size);
        for _ in 0..size {
            elements.push(rng.gen_range(-30..=30));
        }

        let mut beap: Beap<i64> = Beap::new();
        assert_eq!(beap.len(), 0);

        let mut max_item = elements[0];
        for (i, &x) in elements.iter().enumerate() {
            max_item = max_item.max(x);
            beap.push(x);
            assert_eq!(beap.len(), i + 1);
            assert_eq!(beap.peek(), Some(&max_item));
        }

        elements.sort();
        let mut beap_vec = beap.into_vec();
        beap_vec.sort();
        assert_eq!(beap_vec, elements);
    }
}

#[test]
fn test_pop() {
    // Fixed tests
    let mut beap = Beap::<i32>::new();
    assert_eq!(beap.pop(), None);
    assert_eq!(beap.pop(), None);

    beap.push(1);
    assert_eq!(beap.pop(), Some(1));
    assert_eq!(beap.pop(), None);

    beap.push(0);
    assert_eq!(beap.pop(), Some(0));
    assert_eq!(beap.pop(), None);

    let mut beap = Beap::from([3, 5, 1, 2, 4]);
    assert_eq!(beap.pop(), Some(5));
    assert_eq!(beap.pop(), Some(4));
    assert_eq!(beap.pop(), Some(3));
    assert_eq!(beap.pop(), Some(2));
    assert_eq!(beap.pop(), Some(1));
    assert_eq!(beap.pop(), None);

    // Random tests against BinaryHeap
    let mut rng = thread_rng();

    for size in 0..=100 {
        let mut elements: Vec<i64> = Vec::with_capacity(size);
        for _ in 0..size {
            elements.push(rng.gen_range(-30..=30));
        }

        let mut binary_heap: BinaryHeap<i64> = BinaryHeap::from(elements.clone());
        let mut beap: Beap<i64> = Beap::from(elements);

        while !binary_heap.is_empty() {
            assert_eq!(beap.pop(), binary_heap.pop());
            assert_eq!(beap.len(), binary_heap.len());
        }
        assert_eq!(beap.is_empty(), binary_heap.is_empty());
    }
}

#[test]
fn test_from() {
    let b1: Beap<i32> = Beap::from(vec![]);
    assert_eq!(b1.len(), 0);

    let b2: Beap<i32> = Beap::from([]);
    assert_eq!(b2.len(), 0);

    let b1: Beap<i32> = Beap::from(vec![3, 5, 9, 7]);
    assert_eq!(b1.len(), 4);
    assert_eq!(b1.peek(), Some(&9));

    let b2: Beap<i32> = Beap::from([3, 5, 9, 7]);
    assert_eq!(b2.len(), 4);
    assert_eq!(b2.peek(), Some(&9));

    // Random tests
    let mut rng = thread_rng();

    for size in 1..=20 {
        let mut elements: Vec<i64> = Vec::with_capacity(size);
        for _ in 0..size {
            elements.push(rng.gen_range(-30..=30));
        }

        let beap: Beap<i64> = Beap::from(elements.clone());
        assert_eq!(beap.len(), size);

        elements.sort_unstable_by(|x, y| y.cmp(x));
        assert_eq!(beap.peek(), Some(&elements[0]));

        assert_eq!(beap.into_vec(), elements); // The sorted vector satisfies the beap properties.
    }
}

#[test]
fn test_into_sorted_vec() {
    let beap: Beap<i32> = Beap::from(vec![]);
    assert_eq!(beap.into_sorted_vec(), vec![]);

    let beap: Beap<i32> = Beap::from(vec![3, 5, 9, 7]);
    assert_eq!(beap.into_sorted_vec(), vec![3, 5, 7, 9]);

    // Random tests
    let mut rng = thread_rng();

    for size in 1..=50 {
        let mut elements: Vec<i64> = Vec::with_capacity(size);
        for _ in 0..size {
            elements.push(rng.gen_range(-20..=20));
        }

        let beap: Beap<i64> = Beap::from(elements.clone());
        assert_eq!(beap.len(), size);

        elements.sort_unstable();
        assert_eq!(beap.into_sorted_vec(), elements);
    }
}
