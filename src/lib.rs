//! A priority queue implemented with a bi-parental heap.
//!
//! Beap (bi-parental heap) is an
//! [implict data structure](https://en.wikipedia.org/wiki/Implicit_data_structure)
//! which allows efficient insertion and searching of elements, requiring low (*O*(1)) overhead.
//!
//! Insertion and popping the largest element have *O*(sqrt(*2n*)) time complexity.
//! Checking the largest element is *O*(1). Converting a vector to a beap
//! can be done by using sorting, and has *O*(*n* * log(*n*)) time complexity.
//! Despite the insertion and popping operations that are slower compared to the classical binary heap,
//! the bi-parental heap has an important advantage:
//! searching and removing an arbitrary element, as well as finding the minimum,
//! have the asymptotics *O*(sqrt(*2n*),) while the binary heap has *O*(*n*).
//!
//! This create presents an implementation of the bi-parental heap - `Beap`,
//! which has an identical interface with [`BinaryHeap`] from `std::collections`,
//! and at the same time it has several new useful methods.
//!
//! # Read about bi-parental heap:
//! * [Wikipedia](https://en.wikipedia.org/wiki/Beap)
//!
//! [`BinaryHeap`]: std::collections::BinaryHeap
//!

mod core;
pub mod iter;
mod mem;

pub use iter::{Drain, IntoIter, Iter};
use std::fmt;
use std::ops::{Deref, DerefMut};

/// A priority queue implemented with a bi-parental heap (beap).
///
/// This will be a max-heap.
///
/// # Examples
///
/// ```
/// use beap::Beap;
///
/// // Type inference lets us omit an explicit type signature (which
/// // would be `Beap<i32>` in this example).
/// let mut beap = Beap::new();
///
/// // We can use peek to look at the next item in the beap. In this case,
/// // there's no items in there yet so we get None.
/// assert_eq!(beap.peek(), None);
///
/// // Let's add some scores...
/// beap.push(1);
/// beap.push(5);
/// beap.push(2);
///
/// // Now peek shows the most important item in the beap.
/// assert_eq!(beap.peek(), Some(&5));
///
/// // We can check the length of a beap.
/// assert_eq!(beap.len(), 3);
///
/// // We can iterate over the items in the beap, although they are returned in
/// // a random order.
/// for x in beap.iter() {
///     println!("{}", x);
/// }
///
/// // If we instead pop these scores, they should come back in order.
/// assert_eq!(beap.pop(), Some(5));
/// assert_eq!(beap.pop(), Some(2));
/// assert_eq!(beap.pop(), Some(1));
/// assert_eq!(beap.pop(), None);
///
/// // We can clear the beap of any remaining items.
/// beap.clear();
///
/// // The beap should now be empty.
/// assert!(beap.is_empty())
/// ```
///
/// A `Beap` with a known list of items can be initialized from an array:
///
/// ```
/// use beap::Beap;
///
/// let beap = Beap::from([1, 5, 2]);
/// ```
///
/// ## Min-heap
///
/// Either [`core::cmp::Reverse`] or a custom [`Ord`] implementation can be used to
/// make `Beap` a min-heap. This makes `beap.pop()` return the smallest
/// value instead of the greatest one.
///
/// ```
/// use beap::Beap;
/// use std::cmp::Reverse;
///
/// let mut beap = Beap::new();
///
/// // Wrap values in `Reverse`
/// beap.push(Reverse(1));
/// beap.push(Reverse(5));
/// beap.push(Reverse(2));
///
/// // If we pop these scores now, they should come back in the reverse order.
/// assert_eq!(beap.pop(), Some(Reverse(1)));
/// assert_eq!(beap.pop(), Some(Reverse(2)));
/// assert_eq!(beap.pop(), Some(Reverse(5)));
/// assert_eq!(beap.pop(), None);
/// ```
///
/// ## Sorting
///
/// ```
/// use beap::Beap;
///
/// let beap = Beap::from([5, 3, 1, 7]);
/// assert_eq!(beap.into_sorted_vec(), vec![1, 3, 5, 7]);
/// ```
pub struct Beap<T> {
    data: Vec<T>,
    height: usize,
}

/// Structure wrapping a mutable reference to the greatest item on a `Beap`.
///
/// This `struct` is created by the [`peek_mut`] method on [`Beap`]. See
/// its documentation for more.
///
/// [`peek_mut`]: Beap::peek_mut
pub struct PeekMut<'a, T: 'a + Ord> {
    beap: &'a mut Beap<T>,
    sift: bool,
}

impl<T: Ord + fmt::Debug> fmt::Debug for PeekMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PeekMut").field(&self.beap.data[0]).finish()
    }
}

impl<T> Default for Beap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> Drop for PeekMut<'_, T> {
    fn drop(&mut self) {
        if self.sift {
            self.beap.siftdown(0, 1);
        }
    }
}

impl<T: Ord> Deref for PeekMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        debug_assert!(!self.beap.is_empty());
        self.beap.data.first().unwrap()
    }
}

impl<T: Ord> DerefMut for PeekMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        debug_assert!(!self.beap.is_empty());
        self.sift = true;
        self.beap.data.first_mut().unwrap()
    }
}

impl<'a, T: Ord> PeekMut<'a, T> {
    /// Removes the peeked value from the heap and returns it.
    pub fn pop(mut this: PeekMut<'a, T>) -> T {
        let value = this.beap.pop().unwrap();
        this.sift = false;
        value
    }
}

impl<T: Clone> Clone for Beap<T> {
    fn clone(&self) -> Self {
        Beap {
            data: self.data.clone(),
            height: self.height,
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.data.clone_from(&source.data);
        self.height.clone_from(&source.height);
    }
}

/// Structure wrapping a mutable reference to the smallest item on a `Beap`.
///
/// This `struct` is created by the [`tail_mut`] method on [`Beap`]. See
/// its documentation for more.
///
/// [`tail_mut`]: Beap::tail_mut
pub struct TailMut<'a, T: 'a + Ord> {
    beap: &'a mut Beap<T>,
    sift: bool,
    pos: usize,
}

impl<T: Ord + fmt::Debug> fmt::Debug for TailMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("TailMut")
            .field(&self.beap.data[self.pos])
            .finish()
    }
}

impl<T: Ord> Drop for TailMut<'_, T> {
    fn drop(&mut self) {
        if self.sift {
            self.beap.repair(self.pos);
        }
    }
}

impl<T: Ord> Deref for TailMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.beap.data.get(self.pos).unwrap()
    }
}

impl<T: Ord> DerefMut for TailMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.sift = true;
        self.beap.data.get_mut(self.pos).unwrap()
    }
}

impl<'a, T: Ord> TailMut<'a, T> {
    /// Removes the peeked value from the beap and returns it.
    pub fn pop(mut this: TailMut<'a, T>) -> T {
        let value = this.beap.remove_index(this.pos).unwrap();
        this.sift = false;
        value
    }
}

/// Structure wrapping a mutable reference to the item with provided index on a `Beap`.
///
/// This `struct` is created by the [`get_mut`] method on [`Beap`]. See
/// its documentation for more.
///
/// [`get_mut`]: Beap::get_mut
pub struct PosMut<'a, T: 'a + Ord> {
    beap: &'a mut Beap<T>,
    sift: bool,
    pos: usize,
}

impl<T: Ord + fmt::Debug> fmt::Debug for PosMut<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("PosMut")
            .field(&self.beap.data[self.pos])
            .finish()
    }
}

impl<T: Ord> Drop for PosMut<'_, T> {
    fn drop(&mut self) {
        if self.sift {
            self.beap.repair(self.pos);
        }
    }
}

impl<T: Ord> Deref for PosMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.beap.data.get(self.pos).unwrap()
    }
}

impl<T: Ord> DerefMut for PosMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.sift = true;
        self.beap.data.get_mut(self.pos).unwrap()
    }
}

impl<'a, T: Ord> PosMut<'a, T> {
    /// Removes the borrowed value from the beap and returns it.
    pub fn remove(mut this: PosMut<'a, T>) -> T {
        let value = this.beap.remove_index(this.pos).unwrap();
        this.sift = true;
        value
    }
}

#[cfg(test)]
mod tests;
