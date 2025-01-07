# Bi-parental heap

* [Documentation](https://docs.rs/beap/)
* [Crate](https://crates.io/crates/beap)

## Description

A priority queue implemented with a bi-parental heap.

Beap (bi-parental heap) is an
[implict data structure](https://en.wikipedia.org/wiki/Implicit_data_structure)
which allows efficient insertion and searching of elements, requiring low (*O*(1)) overhead.

Insertion and popping the largest element have *O*(sqrt(*2n*)) time complexity.
Checking the largest element is *O*(1). Converting a vector to a beap
can be done by using sorting, and has *O*(nlog(*n*)) time complexity.
Despite the insertion and popping operations that are slower compared to the classical binary heap,
the bi-parental heap has an important advantage:
searching and removing an arbitrary element, as well as finding the minimum,
have the asymptotics *O*(sqrt(*2n*),) while the binary heap has *O*(*n*).

This create presents an implementation of the bi-parental heap - `Beap`,
which has an identical interface with [`BinaryHeap`](https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html) from `std::collections`,
and at the same time it has several new useful methods.

# Read about bi-parental heap:
* [Wikipedia](https://en.wikipedia.org/wiki/Beap)

## Usage

As a library

```rust
use beap::Beap;

// Type inference lets us omit an explicit type signature (which
// would be `Beap<i32>` in this example).
let mut beap = Beap::new();

// We can use peek to look at the next item in the beap. In this case,
// there's no items in there yet so we get None.
assert_eq!(beap.peek(), None);

// Let's add some scores...
beap.push(1);
beap.push(5);
beap.push(2);

// Now peek shows the most important item in the beap.
assert_eq!(beap.peek(), Some(&5));

// We can check the length of a beap.
assert_eq!(beap.len(), 3);

// We can iterate over the items in the beap, although they are returned in
// a random order.
for x in beap.iter() {
    println!("{}", x);
}

// If we instead pop these scores, they should come back in order.
assert_eq!(beap.pop(), Some(5));
assert_eq!(beap.pop(), Some(2));
assert_eq!(beap.pop(), Some(1));
assert_eq!(beap.pop(), None);

// We can clear the beap of any remaining items.
beap.clear();

// The beap should now be empty.
assert!(beap.is_empty())
```

A `Beap` with a known list of items can be initialized from an array:

```rust
use beap::Beap;
let beap = Beap::from([1, 5, 2]);
```

## Min-heap

Either `core::cmp::Reverse` or a custom `Ord` implementation can be used to
make `Beap` a min-heap. This makes `beap.pop()` return the smallest
value instead of the greatest one.

```rust
use beap::Beap;
use std::cmp::Reverse;

let mut beap = Beap::new();

// Wrap values in `Reverse`
beap.push(Reverse(1));
beap.push(Reverse(5));
beap.push(Reverse(2));

// If we pop these scores now, they should come back in the reverse order.
assert_eq!(beap.pop(), Some(Reverse(1)));
assert_eq!(beap.pop(), Some(Reverse(2)));
assert_eq!(beap.pop(), Some(Reverse(5)));
assert_eq!(beap.pop(), None);
```

## Sorting

```rust
use beap::Beap;

let beap = Beap::from([5, 3, 1, 7]);
assert_eq!(beap.into_sorted_vec(), vec![1, 3, 5, 7]);
```

## Benchmarks
The charts below shows the results of `Beap<i64>` vs `BinaryHeap<i64>` vs `BTreeSet<i64>` benches.

5 scenarios were tested:
1. Sequential `push` calls
2. Sequential `push` + `peek` calls
3. Sequential `push` + `tail` (search for min) calls
4. Call `contains` for each value in the collection
5. Sequential `pop` calls

each with `100`, `1000` and `10000` elements.

![100 items](assets/100_items.ppm)
![1000 items](assets/1000_items.ppm)
![10000 items](assets/10000_items.ppm)

To summarize, in some usage scenarios, `Beap` may be preferable, 
but most often it is worth choosing `BinaryHeap` or `BTreeSet` depending on the task.

#
If you have any comments or suggestions, or you suddenly found an error, please start a new issue or pool request.
