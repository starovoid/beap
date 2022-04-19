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
fn test_pop_with_push() {
    // Let's make sure that push and pop do not interfere with each other's work.

    // Fixed tests
    let mut beap = Beap::new();
    beap.push(2);
    assert_eq!(beap.peek(), Some(&2));
    assert_eq!(beap.len(), 1);
    beap.push(4);
    assert_eq!(beap.peek(), Some(&4));
    assert_eq!(beap.len(), 2);
    assert_eq!(beap.pop(), Some(4));
    assert_eq!(beap.len(), 1);
    assert_eq!(beap.pop(), Some(2));
    assert_eq!(beap.len(), 0);
    assert_eq!(beap.pop(), None);
    assert_eq!(beap.len(), 0);

    // Random tests against BinaryHeap
    let mut rng = thread_rng();

    for size in 0..=100 {
        let mut elements: Vec<i64> = Vec::with_capacity(size);
        for _ in 0..size {
            elements.push(rng.gen_range(-30..=30));
        }

        let mut binary_hep: BinaryHeap<i64> = BinaryHeap::new();
        let mut beap: Beap<i64> = Beap::new();

        for x in elements {
            binary_hep.push(x);
            beap.push(x);
            if x % 2 == 0 {
                assert_eq!(beap.pop(), binary_hep.pop());
                assert_eq!(beap.len(), binary_hep.len());
            }
        }
        assert_eq!(beap.into_sorted_vec(), binary_hep.into_sorted_vec());
    }
}

#[test]
fn test_pushpop() {
    let mut beap: Beap<i64> = Beap::new();
    assert_eq!(beap.pushpop(5), 5);
    assert_eq!(beap.len(), 0);

    beap.push(3);
    assert_eq!(beap.pushpop(2), 3);
    assert_eq!(beap.peek(), Some(&2));
    assert_eq!(beap.len(), 1);

    assert_eq!(beap.pushpop(4), 4);
    assert_eq!(beap.peek(), Some(&2));
    assert_eq!(beap.len(), 1);

    // Random tests against push and pop
    let mut rng = thread_rng();

    for size in 0..=100 {
        let mut elements: Vec<i64> = Vec::with_capacity(size);
        for _ in 0..size {
            elements.push(rng.gen_range(-30..=30));
        }

        let mut beap1 = Beap::from(elements); // pushpop
        let mut beap2 = beap1.clone(); //push and pop

        for _ in 0..size * 2 {
            let item = rng.gen_range(-50..50);
            beap2.push(item);
            assert_eq!(beap1.pushpop(item), beap2.pop().unwrap());
            assert_eq!(beap1.len(), beap2.len());
            assert_eq!(beap1.peek(), beap2.peek());
        }

        assert_eq!(beap1.into_sorted_vec(), beap2.into_sorted_vec());
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

#[test]
fn test_peek() {
    let mut beap = Beap::new();
    assert_eq!(beap.peek(), None);

    beap.push(1);
    assert_eq!(beap.peek(), Some(&1));

    beap.push(5);
    assert_eq!(beap.peek(), Some(&5));

    beap.pop();
    assert_eq!(beap.peek(), Some(&1));
    beap.pop();
    assert_eq!(beap.peek(), None);
}

#[test]
fn test_clone() {
    let h1 = Beap::from(vec![7, 5, 9, 0, 2]);
    let h2 = h1.clone();
    let mut h3 = Beap::<i32>::new();
    h3.clone_from(&h1);
    let res = h1.into_vec();
    assert_eq!(h2.into_vec(), res);
    assert_eq!(h3.into_vec(), res);
}

#[test]
fn test_capacity() {
    let mut beap: Beap<i32> = Beap::new();
    assert_eq!(beap.capacity(), 0);
    beap.push(1);
    assert_eq!(beap.capacity(), 4);

    let mut beap: Beap<i32> = Beap::with_capacity(2);
    assert_eq!(beap.capacity(), 2);

    beap.push(1);
    beap.push(2);
    assert_eq!(beap.capacity(), 2);

    beap.push(3);
    assert_eq!(beap.capacity(), 4);
}

#[test]
fn test_reserve() {
    let mut beap = Beap::from([3, 4]);
    assert_eq!(beap.capacity(), 2);
    beap.reserve(100);
    assert!(beap.capacity() >= 102);
}

#[test]
fn test_reserve_exact() {
    let mut beap = Beap::from([3, 4]);
    assert_eq!(beap.capacity(), 2);
    beap.reserve_exact(100);
    assert!(beap.capacity() >= 102);
}

#[test]
fn test_shrink_to() {
    let mut beap: Beap<i32> = Beap::with_capacity(20);
    assert_eq!(beap.capacity(), 20);

    beap.shrink_to(100);
    assert_eq!(beap.capacity(), 20);

    beap.shrink_to(10);
    assert_eq!(beap.capacity(), 10);
}

#[test]
fn test_shrink_to_fit() {
    let mut beap: Beap<i32> = Beap::with_capacity(10);
    beap.shrink_to_fit();
    assert_eq!(beap.capacity(), 0);

    beap.push(1);
    beap.push(2);
    beap.push(3);
    beap.shrink_to_fit();
    assert_eq!(beap.capacity(), 3);
}

#[test]
fn test_is_empty() {
    let mut beap = Beap::new();
    assert!(beap.is_empty());
    beap.push(1);
    assert!(!beap.is_empty());
    beap.pop();
    assert!(beap.is_empty());
}
