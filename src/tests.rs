use crate::{Beap, PeekMut, PosMut, TailMut};
use rand::{thread_rng, Rng};
use std::cmp::Reverse;
use std::collections::binary_heap;
use std::collections::{BinaryHeap, HashSet};

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
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_push_random() {
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
    assert_eq!(beap.pop(), None);
    assert_eq!(beap.pop(), None);
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_pop_random() {
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
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_pop_with_push_random() {
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
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_push_pop_random() {
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

    // From iter
    let beap_from_vec = Beap::from(vec![3, 2, 5, 4, 1]);

    let mut beap_from_iter: Beap<i32> = [3, 2, 5, 4, 1].into_iter().collect();
    let mut temp_beap = beap_from_vec.clone();
    while let Some((a, b)) = temp_beap.pop().zip(beap_from_iter.pop()) {
        assert_eq!(a, b);
    }
    assert!(beap_from_iter.is_empty());

    let mut beap_from_iter = Beap::from_iter([3, 2, 5, 4, 1]);
    let mut temp_beap = beap_from_vec.clone();
    while let Some((a, b)) = temp_beap.pop().zip(beap_from_iter.pop()) {
        assert_eq!(a, b);
    }
    assert!(beap_from_iter.is_empty());
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_from_random() {
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
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_into_sorted_vec_random() {
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

#[test]
fn test_contains() {
    let mut beap = Beap::new();
    assert!(!beap.contains(&0));

    beap.push(0);
    assert!(beap.contains(&0));
    assert!(!beap.contains(&10));

    beap.push(1);
    beap.push(2);
    assert!(beap.contains(&1));
    assert!(beap.contains(&2));
    assert!(!beap.contains(&30));
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_contains_random() {
    // Random tests against HashSet
    let mut rng = thread_rng();

    for size in 0..=100 {
        let mut elements: Vec<i64> = Vec::with_capacity(size);
        for _ in 0..size {
            elements.push(rng.gen_range(-30..=30));
        }

        let beap = Beap::from(elements.clone());
        let set: HashSet<i64> = elements.clone().into_iter().collect();

        for _ in 0..100 {
            let x = rng.gen_range(-30..=30);
            assert_eq!(beap.contains(&x), set.contains(&x));
        }

        for x in elements {
            assert!(beap.contains(&x));
        }
    }
}

#[test]
fn test_remove() {
    let mut beap = Beap::from([1, 2, 3, 4, 5]);
    assert!(!beap.remove(&10));
    assert_eq!(beap.len(), 5);

    assert!(beap.remove(&3));
    assert_eq!(beap.len(), 4);

    assert!(beap.remove(&5));
    assert_eq!(beap.len(), 3);

    assert!(!beap.remove(&3));
    assert_eq!(beap.len(), 3);

    assert!(beap.remove(&2));
    assert_eq!(beap.len(), 2);

    assert!(beap.remove(&1));
    assert_eq!(beap.len(), 1);

    assert!(beap.remove(&4));
    assert_eq!(beap.len(), 0);

    assert!(!beap.remove(&4));
    assert_eq!(beap.len(), 0);
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_remove_random() {
    // Random tests against Vec
    let mut rng = thread_rng();

    for size in 0..=100 {
        let mut elements: Vec<i64> = Vec::with_capacity(size);
        for _ in 0..size {
            elements.push(rng.gen_range(-30..=30));
        }

        let mut beap = Beap::from(elements.clone());

        for _ in 0..100 {
            let x = rng.gen_range(-30..=30);
            let cont = elements.contains(&x);
            assert_eq!(beap.remove(&x), cont);

            if cont {
                let mut idx = 0;
                for (i, &v) in elements.iter().enumerate() {
                    if v == x {
                        idx = i;
                        break;
                    }
                }
                elements.remove(idx);
            }
        }

        for x in elements.clone() {
            assert_eq!(beap.contains(&x), elements.contains(&x));
        }
    }
}

#[test]
fn test_peek_mut() {
    let mut beap: Beap<i32> = Beap::new();
    assert!(beap.peek_mut().is_none());

    beap.push(3);
    {
        let mut top = beap.peek_mut().unwrap();
        *top = 4;
    }
    assert_eq!(beap.peek(), Some(&4));

    beap.push(1);
    beap.push(6);
    assert_eq!(beap.peek(), Some(&6));
    {
        let mut top = beap.peek_mut().unwrap();
        *top = 0;
    }
    assert_eq!(beap.peek(), Some(&4));

    {
        let top = beap.peek_mut().unwrap();
        assert_eq!(PeekMut::pop(top), 4);
    }
    assert_eq!(beap.peek(), Some(&1));
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_peek_mut_random() {
    // Random tests against BinaryHeap
    let mut rng = thread_rng();

    for size in 1..=100 {
        let mut elements: Vec<i64> = Vec::with_capacity(size);
        for _ in 0..size {
            elements.push(rng.gen_range(-30..=30));
        }

        let mut binary_heap: BinaryHeap<i64> = BinaryHeap::from(elements.clone());
        let mut beap: Beap<i64> = Beap::from(elements);

        for _ in 0..size * 2 {
            {
                let new_val: i64 = rng.gen_range(-50..=50);
                let mut binary_heap_top = binary_heap.peek_mut().unwrap();
                let mut beap_top = beap.peek_mut().unwrap();
                *binary_heap_top = new_val;
                *beap_top = new_val;
            }
            assert_eq!(beap.peek(), binary_heap.peek());
        }

        assert_eq!(
            beap.clone().into_sorted_vec(),
            binary_heap.clone().into_sorted_vec()
        );

        for _ in 0..size {
            {
                let bin_val = binary_heap.peek_mut().unwrap();
                let weak_val = beap.peek_mut().unwrap();
                assert_eq!(PeekMut::pop(weak_val), binary_heap::PeekMut::pop(bin_val));
            }
            assert_eq!(beap.peek(), binary_heap.peek());
        }
        assert!(beap.is_empty());
        assert!(binary_heap.is_empty());
    }
}

#[test]
fn test_replace() {
    let mut beap: Beap<i32> = Beap::new();
    assert!(!beap.replace(&2, 1));

    beap.push(2);
    assert!(!beap.replace(&10, 2));
    assert!(beap.replace(&2, 1));
    assert_eq!(beap.peek(), Some(&1));

    beap.push(3);
    beap.push(4);
    assert!(beap.replace(&4, 5));
    assert_eq!(beap.peek(), Some(&5));

    assert!(beap.replace(&3, 30));
    assert_eq!(beap.peek(), Some(&30));

    beap.push(5);
    assert!(beap.replace(&5, 500));

    assert_eq!(beap.into_sorted_vec(), vec![1, 5, 30, 500]);
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_replace_random() {
    // Random tests against Vec
    let mut rng = thread_rng();

    for size in 0..=100 {
        let mut elements: Vec<i64> = Vec::with_capacity(size);
        for _ in 0..size {
            elements.push(rng.gen_range(-30..=30));
        }

        let mut beap = Beap::from(elements.clone());

        for _ in 0..100 {
            let old: i64 = rng.gen_range(-30..=30);
            let new: i64 = rng.gen_range(-30..=30);

            let mut cont = false;
            for item in elements.iter_mut().take(size) {
                if *item == old {
                    *item = new;
                    cont = true;
                    break;
                }
            }

            assert_eq!(beap.replace(&old, new), cont);
        }

        elements.sort_unstable();
        assert_eq!(beap.into_sorted_vec(), elements);
    }
}

#[test]
fn test_tail() {
    let mut beap: Beap<i32> = Beap::new();
    assert_eq!(beap.tail(), None);

    beap.push(1);
    assert_eq!(beap.tail(), Some(&1));

    beap.push(2);
    assert_eq!(beap.tail(), Some(&1));

    beap.push(3);
    assert_eq!(beap.tail(), Some(&1));

    beap.push(0);
    assert_eq!(beap.tail(), Some(&0));

    beap.push(5);
    assert_eq!(beap.tail(), Some(&0));

    beap.push(0);
    assert_eq!(beap.tail(), Some(&0));

    beap.push(-1);
    assert_eq!(beap.tail(), Some(&-1));
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_tail_random() {
    // Random tests against BinaryHeap
    let mut rng = thread_rng();

    for size in 0..=100 {
        let mut bin_heap = BinaryHeap::with_capacity(size);
        let mut beap = Beap::with_capacity(size);

        for _ in 0..size {
            let x: i64 = rng.gen_range(-30..=30);
            bin_heap.push(Reverse(x));
            beap.push(x);
            assert_eq!(*beap.tail().unwrap(), bin_heap.peek().unwrap().0);
        }
    }
}

#[test]
fn test_tail_mut() {
    let mut beap: Beap<i32> = Beap::new();
    assert!(beap.tail_mut().is_none());

    beap.push(3);
    {
        let mut top = beap.tail_mut().unwrap();
        *top = 4;
    }
    assert_eq!(beap.tail(), Some(&4));

    beap.push(1);
    beap.push(6);
    assert_eq!(beap.tail(), Some(&1));
    {
        let mut tail = beap.tail_mut().unwrap();
        *tail = 0;
    }
    assert_eq!(beap.tail(), Some(&0));

    {
        let mut tail = beap.tail_mut().unwrap();
        *tail = 10;
    }
    assert_eq!(beap.tail(), Some(&4));

    println!("{:?}", beap.clone().into_vec());
    {
        let tail = beap.tail_mut().unwrap();
        println!("{:?}", tail);
        assert_eq!(TailMut::pop(tail), 4);
    }
    assert_eq!(beap.tail(), Some(&6));
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_tail_mut_random() {
    // Random tests against BinaryHeap
    let mut rng = thread_rng();

    for size in 1..=100 {
        let mut elements: Vec<i64> = Vec::with_capacity(size);
        for _ in 0..size {
            elements.push(rng.gen_range(-30..=30));
        }

        let mut bin_heap: BinaryHeap<Reverse<i64>> =
            BinaryHeap::from_iter(elements.iter().map(|&x| Reverse(x)));
        let mut beap: Beap<i64> = Beap::from(elements);

        for _ in 0..size * 2 {
            {
                let new_val: i64 = rng.gen_range(-50..=50);
                let mut bin_heap_tail = bin_heap.peek_mut().unwrap();
                let mut beap_tail = beap.tail_mut().unwrap();
                *bin_heap_tail = Reverse(new_val);
                *beap_tail = new_val;
            }
            assert_eq!(*beap.tail().unwrap(), bin_heap.peek().unwrap().0);
        }

        let bin_heap_items = bin_heap.clone().into_sorted_vec();
        let bin_heap_items: Vec<i64> = bin_heap_items.into_iter().map(|x| x.0).rev().collect();
        assert_eq!(beap.clone().into_sorted_vec(), bin_heap_items,);

        for _ in 0..size {
            {
                let bin_val = bin_heap.peek_mut().unwrap();
                let beap_val = beap.tail_mut().unwrap();
                assert_eq!(TailMut::pop(beap_val), binary_heap::PeekMut::pop(bin_val).0);
            }
            assert_eq!(beap.is_empty(), bin_heap.is_empty());
            if !beap.is_empty() {
                assert_eq!(*beap.tail().unwrap(), bin_heap.peek().unwrap().0);
            }
        }
        assert!(beap.is_empty());
        assert!(bin_heap.is_empty());
    }
}

#[test]
fn test_pop_tail() {
    // Fixed tests
    let mut beap = Beap::<i32>::new();
    assert_eq!(beap.pop_tail(), None);
    assert_eq!(beap.pop_tail(), None);

    beap.push(1);
    assert_eq!(beap.pop_tail(), Some(1));
    assert_eq!(beap.pop_tail(), None);

    beap.push(0);
    assert_eq!(beap.pop_tail(), Some(0));
    assert_eq!(beap.pop_tail(), None);

    let mut beap = Beap::from([3, 5, 1, 2, 4]);
    assert_eq!(beap.pop_tail(), Some(1));
    assert_eq!(beap.pop_tail(), Some(2));
    assert_eq!(beap.pop_tail(), Some(3));
    assert_eq!(beap.pop_tail(), Some(4));
    assert_eq!(beap.pop_tail(), Some(5));
    assert_eq!(beap.pop_tail(), None);
    assert_eq!(beap.pop_tail(), None);
    assert_eq!(beap.pop_tail(), None);

    let mut beap = Beap::from([3, 5, 1, 2, 4]);
    assert_eq!(beap.pop_tail(), Some(1));
    assert_eq!(beap.pop(), Some(5));
    assert_eq!(beap.pop_tail(), Some(2));
    assert_eq!(beap.pop(), Some(4));
    assert_eq!(beap.pop(), Some(3));
    assert_eq!(beap.pop_tail(), None);

    beap.push(0);
    assert_eq!(beap.pop_tail(), Some(0));
    assert_eq!(beap.pop(), None);
    assert_eq!(beap.pop_tail(), None);
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_pop_tail_random() {
    // Random tests against BinaryHeap
    let mut rng = thread_rng();

    for size in 0..=100 {
        let mut elements: Vec<i64> = Vec::with_capacity(size);
        for _ in 0..size {
            elements.push(rng.gen_range(-30..=30));
        }

        let mut bin_heap: BinaryHeap<Reverse<i64>> =
            BinaryHeap::from_iter(elements.iter().map(|&x| Reverse(x)));
        let mut beap: Beap<i64> = Beap::from(elements.clone());

        while !bin_heap.is_empty() {
            assert_eq!(beap.pop_tail().unwrap(), bin_heap.pop().unwrap().0);
            assert_eq!(beap.len(), bin_heap.len());
        }
        assert_eq!(beap.is_empty(), bin_heap.is_empty());
    }
}

#[test]
fn test_drain() {
    let mut beap = Beap::from([5, 3, 1, 4, 2]);
    assert!(!beap.is_empty());

    let mut content: Vec<i32> = beap.drain().collect();
    content.sort();
    assert_eq!(content, vec![1, 2, 3, 4, 5]);

    assert!(beap.is_empty());
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_drain_random() {
    // Random tests
    let mut rng = rand::thread_rng();
    for size in 0..=20 {
        let mut elements: Vec<i64> = Vec::with_capacity(size);
        for _ in 0..size {
            elements.push(rng.gen_range(-30..=30));
        }

        let mut beap = Beap::from(elements.clone());
        assert_eq!(beap.len(), size);

        let mut content: Vec<i64> = beap.drain().collect();
        assert!(beap.is_empty());

        content.sort();
        elements.sort();

        assert_eq!(content, elements);
    }
}

#[test]
fn test_clear() {
    let mut rng = rand::thread_rng();
    let mut beap = Beap::with_capacity(20);
    for _ in 0..20 {
        beap.push(rng.gen_range(-30..=30));
    }
    assert_eq!(beap.len(), 20);
    beap.clear();
    assert!(beap.is_empty());
}

#[test]
fn test_append() {
    let mut b1: Beap<i64> = Beap::new();
    let mut b2: Beap<i64> = Beap::new();
    b1.append(&mut b2);
    assert_eq!(b1.into_sorted_vec(), vec![]);
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_append_random() {
    // Random tests against BinaryHeap

    let mut rng = thread_rng();
    for size1 in 0..100 {
        let mut elements1: Vec<i64> = Vec::with_capacity(size1);
        for _ in 0..size1 {
            elements1.push(rng.gen_range(-30..=30));
        }

        let beap = Beap::from(elements1.clone());
        let bin_heap = BinaryHeap::from(elements1);

        for size2 in 0..100 {
            let mut elements2: Vec<i64> = Vec::with_capacity(size2);
            for _ in 0..size2 {
                elements2.push(rng.gen_range(-30..=30));
            }

            let mut b2 = Beap::from(elements2.clone());
            let mut bh2 = BinaryHeap::from(elements2);

            let mut b1 = beap.clone();
            let mut bh1 = bin_heap.clone();

            b1.append(&mut b2);
            bh1.append(&mut bh2);

            assert_eq!(b1.peek(), bh1.peek());
            assert_eq!(b1.len(), bh1.len());
            assert!(b2.is_empty());
            assert_eq!(b2.tail(), None);
            assert_eq!(b1.into_sorted_vec(), bh1.into_sorted_vec());
        }
    }
}

#[test]
fn append_vec() {
    let mut beap = Beap::new();
    beap.append_vec(&mut vec![]);
    assert_eq!(beap.len(), 0);

    beap.append_vec(&mut vec![3, 8, 5]);
    assert_eq!(beap.into_sorted_vec(), vec![3, 5, 8]);
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_append_vec_random() {
    let mut rng = thread_rng();
    let mut beap: Beap<i64> = Beap::new();
    let mut all_elements: Vec<i64> = Vec::with_capacity(5050);

    let mut len = 0;
    for size in 0..=100 {
        len += size;

        let mut elements: Vec<i64> = Vec::with_capacity(size);
        for _ in 0..size {
            elements.push(rng.gen_range(-30..=30));
        }

        all_elements.append(&mut elements.clone());
        all_elements.sort_unstable();

        beap.append_vec(&mut elements);

        assert_eq!(beap.len(), len);
        assert_eq!(beap.clone().into_sorted_vec(), all_elements);
        assert!(elements.is_empty());
    }

    assert_eq!(beap.into_sorted_vec(), all_elements);
}

#[test]
fn test_extend() {
    let mut beap: Beap<i64> = Beap::new();

    beap.extend([0]);
    assert_eq!(beap.len(), 1);

    beap.extend(Vec::<i64>::new());
    assert_eq!(beap.len(), 1);

    beap.extend([7, 9, 2, 1]);
    assert_eq!(beap.into_sorted_vec(), [0, 1, 2, 7, 9]);
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_extend_random() {
    // Random tests against BinaryHeap

    let mut rng = thread_rng();

    let mut beap = Beap::new();
    let mut bin_heap = BinaryHeap::new();

    for size in 0..100 {
        let mut elements: Vec<i64> = Vec::with_capacity(size);
        for _ in 0..size {
            elements.push(rng.gen_range(-30..=30));
        }

        beap.extend(elements.clone());
        bin_heap.extend(elements.clone());

        beap.extend(elements.clone().into_iter());
        bin_heap.extend(elements.into_iter());

        assert_eq!(beap.len(), bin_heap.len());
        assert_eq!(beap.peek(), bin_heap.peek());
        assert_eq!(
            beap.clone().into_sorted_vec(),
            bin_heap.clone().into_sorted_vec()
        );
    }

    assert_eq!(beap.into_sorted_vec(), bin_heap.into_sorted_vec());
}

#[test]
fn test_extend_ref() {
    let mut beap: Beap<i64> = Beap::new();

    beap.extend([&0]);
    assert_eq!(beap.len(), 1);

    beap.extend(Vec::<i64>::new());
    assert_eq!(beap.len(), 1);

    beap.extend([&7, &9, &2, &1]);
    beap.extend([&4, &3, &6, &5]);
    assert_eq!(beap.into_sorted_vec(), vec![0, 1, 2, 3, 4, 5, 6, 7, 9]);
}

#[test]
fn test_into_iter() {
    let beap: Beap<i32> = Beap::new();
    assert_eq!(beap.into_iter().next(), None);

    let beap = Beap::from(vec![3, 8, 5]);
    let mut data: Vec<i32> = beap.into_iter().collect();
    data.sort();
    assert_eq!(data, vec![3, 5, 8]);

    // Random tests
}

#[test]
#[cfg_attr(miri, ignore)]
fn test_into_iter_random() {
    let mut rng = rand::thread_rng();
    for size in 0..=100 {
        let mut elements: Vec<i64> = Vec::with_capacity(size);
        for _ in 0..size {
            elements.push(rng.gen_range(-30..=30));
        }

        let b = Beap::from(elements.clone());
        elements.sort();
        assert_eq!(b.into_sorted_vec(), elements);
    }
}

#[test]
fn test_iter() {
    let beap: Beap<i32> = Beap::new();
    assert_eq!(beap.into_iter().next(), None);

    let beap = Beap::from(vec![3, 8]);
    let iter = beap.iter();

    // Clone
    let data: Vec<&i32> = iter.clone().collect();
    assert_eq!(data, vec![&8, &3]);
    // Size hint
    assert_eq!(iter.size_hint(), (2, Some(2)));
    // Debug
    assert_eq!(format!("{:?}", iter), "Iter([8, 3])");
    // Size hint
    assert_eq!(iter.size_hint(), (2, Some(2)));
    // Last
    assert_eq!(iter.last(), Some(&3));
}

#[test]
fn test_into_iter_ref() {
    let beap: Beap<i32> = Beap::new();
    assert_eq!((&beap).into_iter().next(), None);

    let beap = Beap::from(vec![3, 8]);
    let iter = (&beap).into_iter();

    for x in &beap {
        println!("{}", x);
    }

    for x in iter {
        println!("{}", x);
    }

    let iter = (&beap).into_iter();
    // Clone
    let data: Vec<&i32> = iter.clone().collect();
    assert_eq!(data, vec![&8, &3]);
    // Size hint
    assert_eq!(iter.size_hint(), (2, Some(2)));
    // Debug
    assert_eq!(format!("{:?}", iter), "Iter([8, 3])");
    // Size hint
    assert_eq!(iter.size_hint(), (2, Some(2)));
    // Last
    assert_eq!(iter.last(), Some(&3));
}

#[test]
fn test_leak() {
    let x = Beap::from([1u32, 2u32, 3u32]);

    let data_ref: &'static mut [u32] = x.leak();
    assert_eq!(data_ref, &[3, 2, 1]);

    data_ref[0] += 1;
    assert_eq!(data_ref, &[4, 2, 1]);

    // Manually free it later.
    unsafe {
        let _b = Box::from_raw(data_ref as *mut [u32]);
    }
}

#[test]
fn test_into_boxed_slice() {
    let mut b = Beap::with_capacity(100);
    b.extend([1, 2, 3, 0]);

    let slice = b.into_boxed_slice();
    let v = slice.into_vec();

    assert_eq!(v, [3, 1, 2, 0]);
    assert_eq!(v.capacity(), 4);
}

#[test]
fn test_try_reserve() {
    let mut b = Beap::from([1, 2, 3]);
    assert!(b.try_reserve(50).is_ok());
    assert!(b.capacity() >= 53);
}

#[test]
fn test_try_reserve_exact() {
    let mut b = Beap::from([1, 2, 3]);
    assert!(b.try_reserve_exact(50).is_ok());
    assert!(b.capacity() >= 53);
}

#[test]
fn test_index() {
    let mut b = Beap::<i32>::new();
    assert_eq!(b.index(&42), None);
    b.push(42);
    assert_eq!(b.index(&42), Some(0));
    b.extend([1, 2, 3]);
    assert_eq!(b.index(&42), Some(0));
    assert_eq!(b.index(&1), Some(3));
    b.push(100);
    assert_eq!(b.index(&1), Some(3));
    assert_eq!(b.index(&2), Some(4));
    assert_eq!(b.index(&42), Some(2));
}

#[test]
fn test_remove_from_pos() {
    let mut b = Beap::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);

    assert_eq!(b.remove_index(8), Some(1));
    let idx4 = b.index(&4).unwrap();
    assert_eq!(b.remove_index(idx4), Some(4));
    assert_eq!(b.remove_index(100), None);
    assert_eq!(b.remove_index(0), Some(9));
}

#[test]
fn test_get_mut() {
    let mut beap: Beap<i32> = Beap::new();
    assert!(beap.get_mut(0).is_none());

    beap.push(3);
    {
        let mut top = beap.get_mut(0).unwrap();
        *top = 4;
    }
    assert_eq!(beap.get(0), Some(&4));

    beap.push(1);
    beap.push(6);
    assert_eq!(beap.get(1), Some(&1));
    {
        let mut x = beap.get_mut(1).unwrap();
        *x = 0;
    }
    assert_eq!(beap.get(1), Some(&0));

    {
        let mut x = beap.get_mut(1).unwrap();
        *x = 10;
    }
    assert_eq!(beap.peek(), Some(&10));
    assert_eq!(beap.get(1), Some(&6));

    println!("{:?}", beap.clone().into_vec());
    {
        let pos = beap.get_mut(1).unwrap();
        println!("{:?}", pos);
        assert_eq!(PosMut::remove(pos), 6);
    }
    assert_eq!(beap.tail(), Some(&4));
}
