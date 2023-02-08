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

use std::fmt;
use std::iter::FusedIterator;
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
        self.beap.data.get(0).unwrap()
    }
}

impl<T: Ord> DerefMut for PeekMut<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        debug_assert!(!self.beap.is_empty());
        self.sift = true;
        self.beap.data.get_mut(0).unwrap()
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
        let value = this.beap.remove_from_pos(this.pos).unwrap();
        this.sift = false;
        value
    }
}

impl<T: Ord> Beap<T> {
    /// Returns a mutable reference to the greatest item in the beap, or
    /// `None` if it is empty.
    ///
    /// Note: If the `PeekMut` value is leaked, the beap may be in an
    /// inconsistent state.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let mut beap = Beap::new();
    /// assert!(beap.peek_mut().is_none());
    ///
    /// beap.push(1);
    /// beap.push(5);
    /// beap.push(2);
    /// {
    ///     let mut val = beap.peek_mut().unwrap();
    ///     *val = 0;
    /// }
    /// assert_eq!(beap.peek(), Some(&2));
    /// ```
    ///
    /// # Time complexity
    ///
    /// If the item is modified then the worst case time complexity is *O*(sqrt(*2n*)),
    /// otherwise it's *O*(1).
    pub fn peek_mut(&mut self) -> Option<PeekMut<'_, T>> {
        if self.is_empty() {
            None
        } else {
            Some(PeekMut {
                beap: self,
                sift: false,
            })
        }
    }

    /// Pushes an item onto the beap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let mut beap = Beap::new();
    /// beap.push(3);
    /// beap.push(5);
    /// beap.push(1);
    ///
    /// assert_eq!(beap.len(), 3);
    /// assert_eq!(beap.peek(), Some(&5));
    /// ```
    ///
    /// # Time complexity
    ///
    /// *O*(sqrt(*2n*))
    pub fn push(&mut self, item: T) {
        if let Some((_, end)) = self.span(self.height) {
            if self.data.len() > end {
                self.height += 1;
            }
        } else {
            self.height = 1;
        }

        self.data.push(item);
        self.siftup(self.data.len() - 1, self.height);
    }

    /// Removes the greatest item from the beap and returns it, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let mut beap = Beap::from(vec![1, 3]);
    ///
    /// assert_eq!(beap.pop(), Some(3));
    /// assert_eq!(beap.pop(), Some(1));
    /// assert_eq!(beap.pop(), None);
    /// ```
    ///
    /// # Time complexity
    ///
    /// The worst case cost of `pop` on a beap containing *n* elements is *O*(sqrt(*2n*)).
    pub fn pop(&mut self) -> Option<T> {
        self.data.pop().map(|mut item| {
            if !self.is_empty() {
                let (start, _) = self.span(self.height).unwrap();
                if start == self.data.len() {
                    self.height -= 1;
                }
                std::mem::swap(&mut item, &mut self.data[0]);
                self.siftdown(0, 1);
            } else {
                self.height = 0;
            }
            item
        })
    }

    /// Effective equivalent to a sequential `push()` and `pop()` calls.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let mut beap = Beap::new();
    /// assert_eq!(beap.pushpop(5), 5);
    /// assert!(beap.is_empty());
    ///
    /// beap.push(10);
    /// assert_eq!(beap.pushpop(20), 20);
    /// assert_eq!(beap.peek(), Some(&10));
    ///
    /// assert_eq!(beap.pushpop(5), 10);
    /// assert_eq!(beap.peek(), Some(&5));
    /// ```
    ///
    /// # Time complexity
    ///
    /// If the beap is empty or the element being added
    /// is larger (or equal) than the current top of the heap,
    /// then the time complexity will be *O*(1), otherwise *O*(sqrt(*2n*)).
    /// And unlike the sequential call of `push()` and `pop()`, the resizing never happens.
    pub fn pushpop(&mut self, mut item: T) -> T {
        if !self.is_empty() && self.data[0] > item {
            std::mem::swap(&mut item, &mut self.data[0]);
            self.siftdown(0, 1);
        }
        item
    }

    /// Returns true if the beap contains a value.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let beap = Beap::from([1, 5, 3, 7]);
    ///
    /// assert!(beap.contains(&1));
    /// assert!(beap.contains(&5));
    /// assert!(!beap.contains(&0));
    /// ```
    ///
    /// # Time complexity
    ///
    /// *O*(sqrt(*2n*))
    pub fn contains(&self, val: &T) -> bool {
        self.index(val).is_some()
    }

    /// Removes a value from the beap. Returns whether the value was present in the beap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let mut beap = Beap::from([1, 5, 3]);
    ///
    /// assert!(beap.remove(&3));
    /// assert!(!beap.remove(&3));
    /// assert_eq!(beap.len(), 2);
    /// ```
    ///
    /// # Time complexity
    ///
    /// *O*(sqrt(*2n*))
    pub fn remove(&mut self, val: &T) -> bool {
        match self.index(val) {
            Some(idx) => {
                self.remove_from_pos(idx);
                true
            }
            None => false,
        }
    }

    /// Replaces the first found element with the value ```old``` with the
    /// value ```new```, returns ```true``` if the element ```old``` was found.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let mut beap = Beap::new();
    /// beap.push(5);
    /// beap.push(10);
    ///
    /// assert!(beap.replace(&10, 100));
    /// assert!(!beap.replace(&1, 200));
    ///
    /// assert_eq!(beap.into_sorted_vec(), vec![5, 100]);
    /// ```
    ///
    /// # Time complexity
    ///
    /// *O*(sqrt(*2n*)).
    pub fn replace(&mut self, old: &T, new: T) -> bool {
        let idx = self.index(old);
        match idx {
            Some(pos) => {
                self.data[pos] = new;
                self.repair(pos);
                true
            }
            None => false,
        }
    }

    /// Returns the smallest item in the beap, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let mut beap = Beap::new();
    /// assert_eq!(beap.tail(), None);
    ///
    /// beap.push(9);
    /// beap.push(3);
    /// beap.push(6);
    /// assert_eq!(beap.tail(), Some(&3));
    /// ```
    ///
    /// # Time complexity
    ///
    /// *O*(sqrt(*2n*))
    pub fn tail(&self) -> Option<&T> {
        match self.span(self.height) {
            None => None,
            Some((start, end)) => {
                if self.height == 1 {
                    self.data.get(0)
                } else {
                    let empty = end + 1 - self.len();
                    self.data.get(
                        ((start - empty)..=(end - empty))
                            .min_by_key(|&i| &self.data[i])
                            .unwrap(),
                    )
                }
            }
        }
    }

    /// Returns a mutable reference to the smallest item in the beap, or
    /// `None` if it is empty.
    ///
    /// Note: If the `TailMut` value is leaked, the beap may be in an
    /// inconsistent state.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let mut beap = Beap::new();
    /// assert!(beap.tail_mut().is_none());
    ///
    /// beap.push(1);
    /// beap.push(5);
    /// beap.push(2);
    /// {
    ///     let mut val = beap.tail_mut().unwrap();
    ///     *val = 10;
    /// }
    /// assert_eq!(beap.tail(), Some(&2));
    /// ```
    ///
    /// # Time complexity
    ///
    /// *O*(sqrt(*2n*)),
    pub fn tail_mut(&mut self) -> Option<TailMut<'_, T>> {
        if let Some((start, end)) = self.span(self.height) {
            let empty = end + 1 - self.len();
            let idx = ((start - empty)..=(end - empty))
                .min_by_key(|&i| &self.data[i])
                .unwrap();
            Some(TailMut {
                beap: self,
                sift: false,
                pos: idx,
            })
        } else {
            None
        }
    }

    /// Removes the smallest item from the beap and returns it, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let mut beap = Beap::from(vec![1, 3]);
    ///
    /// assert_eq!(beap.pop_tail(), Some(1));
    /// assert_eq!(beap.pop_tail(), Some(3));
    /// assert_eq!(beap.pop_tail(), None);
    /// ```
    ///
    /// # Time complexity
    ///
    /// *O*(sqrt(*2n*)).
    pub fn pop_tail(&mut self) -> Option<T> {
        if let Some((start, end)) = self.span(self.height) {
            let empty = end + 1 - self.len();
            let idx = ((start - empty)..=(end - empty))
                .min_by_key(|&i| &self.data[i])
                .unwrap();
            self.remove_from_pos(idx)
        } else {
            None
        }
    }

    /// Consumes the `Beap` and returns a vector in sorted
    /// (ascending) order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    ///
    /// let mut beap = Beap::from(vec![1, 2, 4, 5, 7]);
    /// beap.push(6);
    /// beap.push(3);
    ///
    /// let vec = beap.into_sorted_vec();
    /// assert_eq!(vec, [1, 2, 3, 4, 5, 6, 7]);
    /// ```
    ///
    /// # Time complexity
    ///
    /// *O*(*nlog(n)*)
    ///
    /// Inside, `Vec::sort_unstable` is used.
    pub fn into_sorted_vec(mut self) -> Vec<T> {
        self.data.sort_unstable();
        self.data
    }

    /// Changing the current element with its least priority parent until the beap property is restored
    fn siftup(&mut self, mut pos: usize, mut block: usize) {
        let (mut start, _) = self.span(block).unwrap();

        while block > 1 {
            // Position of the element in the block.
            let pos_in_block = pos - start;

            // The first and last index of the elements of the previous block.
            let (prev_start, prev_end) = self.span(block - 1).unwrap();

            let parent;
            if pos_in_block > 0 {
                let left_parent = prev_start + pos_in_block - 1;
                let right_parent = prev_start + pos_in_block;

                if pos_in_block == block - 1 {
                    parent = prev_end; // The `pos` element does not have a right parent.
                } else if self.data[right_parent] < self.data[left_parent] {
                    // The priority of the right parent is less than the left one
                    parent = right_parent;
                } else {
                    parent = left_parent;
                }
            } else {
                parent = prev_start; // The `pos` element does not have a left parent.
            }

            if self.data[parent] >= self.data[pos] {
                break; // The beap property is met.
            }

            self.data.swap(pos, parent);
            pos = parent;
            start = prev_start;
            block -= 1;
        }
    }

    /// Sift down in time O(sqrt(2N)).
    /// Swap the element with its largest child until the heap property is restored.
    fn siftdown(&mut self, mut pos: usize, mut block: usize) {
        let (mut start, _) = self.span(block).unwrap();
        while block < self.height {
            let (next_start, _) = self.span(block + 1).unwrap();
            let level_pos = pos - start;

            // We will find the highest priority descendant.
            let mut child = next_start + level_pos;
            if child >= self.data.len() {
                break; // The `pos` element has no descendants.
            }

            if child + 1 < self.data.len() && self.data[child + 1] > self.data[child] {
                child += 1;
            }

            if self.data[pos] >= self.data[child] {
                break; // The beap property is met.
            }

            self.data.swap(pos, child);
            block += 1;
            start = next_start;
            pos = child;
        }
    }

    /// Restore the beap property (after changing the `pos` element).
    fn repair(&mut self, pos: usize) {
        if pos == 0 {
            self.siftdown(pos, 1);
        } else {
            let b = ((2 * (pos + 1)) as f64).sqrt().round() as usize;
            self.siftup(pos, b);
            self.siftdown(pos, b);
        }
    }

    /// Given the val value, find the index of an element with such a value
    /// or return None if such an element does not exist.
    /// Time complexity: O(sqrt(2n)).
    ///
    /// Let there be Beap        9
    ///                        8   7
    ///                      6   5   4
    ///                    3   2   1   0
    ///
    /// Consider it as the upper left corner of the matrix:
    /// 9 7 4 0
    /// 8 5 1
    /// 6 2
    /// 3
    ///
    /// Let's start the search from the upper-right corner
    /// (the last element of the inner vector).
    ///
    /// 1) If the priority of the desired element is greater than that
    /// of the element in the current position, then move to the left along the line.
    ///
    /// 2) If the priority of the desired element is less than that of the element
    /// in the current position, then move it down the column,
    ///
    /// 3) and if there is no element at the bottom, then move down and to the left
    /// (= left on the last layer of the heap).
    ///
    /// 4) As soon as we find an element with equal val priority, we return its index,
    /// and if we find ourselves in the left in the lower corner and the value in it
    /// is not equal to val, so the desired element does not exist and it's time to return None.
    fn index(&self, val: &T) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        let mut block = self.height;
        let (left_low, mut right_up) = self.span(self.height).unwrap();

        if right_up >= self.len() {
            block -= 1;
            right_up = self.span(block).unwrap().1;
        }

        let mut pos = right_up;
        while pos != left_low {
            if self.data[pos] == *val {
                return Some(pos);
            }

            let (start, _) = self.span(block).unwrap();
            let block_pos = pos - start;

            if block > 1 && block_pos > 0 && *val > self.data[pos] {
                // Case 1: go to the left
                let (prev_start, _) = self.span(block - 1).unwrap();
                pos = prev_start + block_pos - 1;
                block -= 1;
            } else if *val < self.data[pos] && block < self.height {
                let (next_start, _) = self.span(block + 1).unwrap();
                if next_start + block_pos >= self.len() {
                    pos -= 1; // Case 3: Go left and down (diagonally).
                } else {
                    // Case 2: Go down.
                    pos = next_start + block_pos;
                    block += 1;
                }
            } else if block_pos > 0 {
                pos -= 1; // Case 3: Go left and down (diagonally).
            } else {
                return None; // Element not found.
            }
        }

        if *val == self.data[left_low] {
            Some(left_low)
        } else {
            None
        }
    }

    // Removing an item in the specified position.
    fn remove_from_pos(&mut self, pos: usize) -> Option<T> {
        self.data.pop().map(|mut item| {
            if !self.is_empty() {
                let (start, _) = self.span(self.height).unwrap();
                if start == self.data.len() {
                    self.height -= 1;
                }

                if pos != self.len() {
                    std::mem::swap(&mut item, &mut self.data[pos]);
                    self.repair(pos);
                }
            } else {
                self.height = 0;
            }
            item
        })
    }

    /// Moves all the elements of `other` into `self`, leaving `other` empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    ///
    /// let v = vec![-10, 1, 2, 3, 3];
    /// let mut a = Beap::from(v);
    ///
    /// let v = vec![-20, 5, 43];
    /// let mut b = Beap::from(v);
    ///
    /// a.append(&mut b);
    ///
    /// assert_eq!(a.into_sorted_vec(), [-20, -10, 1, 2, 3, 3, 5, 43]);
    /// assert!(b.is_empty());
    /// ```
    ///
    /// # Time complexity
    ///
    /// Operation can be done in *O*(n*log(n)),
    /// where *n* = self.len() + other.len().
    pub fn append(&mut self, other: &mut Self) {
        other.height = 0;
        self.data.append(&mut other.data);
        self.data.sort_unstable_by(|x, y| y.cmp(x));
    }

    /// Moves all the elements of `other` into `self`, leaving `other` empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    ///
    /// let mut beap = Beap::from([-10, 1, 2, 3, 3]);
    ///
    /// let mut v = vec![-20, 5, 43];
    /// beap.append_vec(&mut v);
    ///
    /// assert_eq!(beap.into_sorted_vec(), [-20, -10, 1, 2, 3, 3, 5, 43]);
    /// assert!(v.is_empty());
    /// ```
    ///
    /// # Time complexity
    ///
    /// Operation can be done in *O*(n*log(n)),
    /// where *n* = self.len() + other.len().
    pub fn append_vec(&mut self, other: &mut Vec<T>) {
        self.data.append(other);
        self.data.sort_unstable_by(|x, y| y.cmp(x));
    }
}

impl<T> Beap<T> {
    /// Creates an empty `Beap` as a max-beap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let mut beap = Beap::new();
    /// assert!(beap.is_empty());
    ///
    /// beap.push(4);
    /// assert_eq!(beap.len(), 1);
    /// ```
    #[must_use]
    pub fn new() -> Beap<T> {
        Beap {
            data: vec![],
            height: 0,
        }
    }

    /// Returns an iterator visiting all values in the underlying vector, in
    /// arbitrary order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let beap = Beap::from(vec![1, 2, 3, 4]);
    ///
    /// // Print 1, 2, 3, 4 in arbitrary order
    /// for x in beap.iter() {
    ///     println!("{}", x);
    /// }
    ///
    /// assert_eq!(beap.into_sorted_vec(), vec![1, 2, 3, 4]);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            iter: self.data.iter(),
        }
    }

    /// Creates an empty `Beap` with a specific capacity.
    /// This preallocates enough memory for `capacity` elements,
    /// so that the `Beap` does not have to be reallocated
    /// until it contains at least that many values.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let mut beap = Beap::with_capacity(10);
    /// beap.push(4);
    /// ```
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Beap<T> {
        Beap {
            data: Vec::with_capacity(capacity),
            height: 0,
        }
    }

    /// Returns the greatest item in the beap, or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let mut beap = Beap::new();
    /// assert_eq!(beap.peek(), None);
    ///
    /// beap.push(1);
    /// beap.push(5);
    /// beap.push(2);
    /// assert_eq!(beap.peek(), Some(&5));
    /// ```
    ///
    /// # Time complexity
    ///
    /// Cost is *O*(1) in the worst case.
    #[must_use]
    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }

    /// Returns the number of elements the beap can hold without reallocating.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let mut beap = Beap::with_capacity(100);
    /// assert!(beap.capacity() >= 100);
    /// beap.push(4);
    /// ```
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    /// Reserves the minimum capacity for exactly `additional` more elements to be inserted in the
    /// given `Beap`. Does nothing if the capacity is already sufficient.
    ///
    /// Note that the allocator may give the collection more space than it requests. Therefore
    /// capacity can not be relied upon to be precisely minimal. Prefer [`reserve`] if future
    /// insertions are expected.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let mut beap = Beap::new();
    /// beap.reserve_exact(100);
    /// assert!(beap.capacity() >= 100);
    /// beap.push(4);
    /// ```
    ///
    /// [`reserve`]: Beap::reserve
    pub fn reserve_exact(&mut self, additional: usize) {
        self.data.reserve_exact(additional);
    }

    /// Reserves capacity for at least `additional` more elements to be inserted in the
    /// `Beap`. The collection may reserve more space to avoid frequent reallocations.
    ///
    /// # Panics
    ///
    /// Panics if the new capacity overflows `usize`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let mut beap = Beap::new();
    /// beap.reserve(100);
    /// assert!(beap.capacity() >= 100);
    /// beap.push(4);
    /// ```
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(additional);
    }

    /// Discards as much additional capacity as possible.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let mut beap: Beap<i32> = Beap::with_capacity(100);
    ///
    /// assert!(beap.capacity() >= 100);
    /// beap.shrink_to_fit();
    /// assert!(beap.capacity() == 0);
    /// ```
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    /// Discards capacity with a lower bound.
    ///
    /// The capacity will remain at least as large as both the length
    /// and the supplied value.
    ///
    /// If the current capacity is less than the lower limit, this is a no-op.
    ///
    /// # Examples
    ///
    /// ```
    /// use beap::Beap;
    /// let mut beap: Beap<i32> = Beap::with_capacity(100);
    ///
    /// assert!(beap.capacity() >= 100);
    /// beap.shrink_to(10);
    /// assert!(beap.capacity() >= 10);
    /// ```
    #[inline]
    pub fn shrink_to(&mut self, min_capacity: usize) {
        self.data.shrink_to(min_capacity);
    }

    /// Consumes the `Beap<T>` and returns the underlying vector Vec<T>
    /// in arbitrary order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let beap = Beap::from(vec![1, 2, 3, 4, 5, 6, 7]);
    /// let vec = beap.into_vec();
    ///
    /// // Will print in some order
    /// for x in vec {
    ///     println!("{}", x);
    /// }
    /// ```
    #[must_use = "`self` will be dropped if the result is not used"]
    pub fn into_vec(self) -> Vec<T> {
        self.data
    }

    /// Returns the length of the beap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let beap = Beap::from(vec![1, 3]);
    ///
    /// assert_eq!(beap.len(), 2);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Checks if the beap is empty.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let mut beap = Beap::new();
    ///
    /// assert!(beap.is_empty());
    ///
    /// beap.push(3);
    /// beap.push(5);
    /// beap.push(1);
    ///
    /// assert!(!beap.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Clears the bi-parental heap, returning an iterator over the removed elements
    /// in arbitrary order. If the iterator is dropped before being fully
    /// consumed, it drops the remaining elements in arbitrary order.
    ///
    /// The returned iterator keeps a mutable borrow on the beap to optimize
    /// its implementation.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let mut beap = Beap::from([1, 3, 5]);
    ///
    /// assert!(!beap.is_empty());
    ///
    /// for x in beap.drain() {
    ///     println!("{}", x);
    /// }
    ///
    /// assert!(beap.is_empty());
    /// ```
    pub fn drain(&mut self) -> Drain<'_, T> {
        self.height = 0;
        Drain {
            iter: self.data.drain(..),
        }
    }

    /// Drops all items from the beap.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let mut beap = Beap::from([1, 3, 5]);
    ///
    /// assert!(!beap.is_empty());
    ///
    /// beap.clear();
    ///
    /// assert!(beap.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.drain();
    }

    /// Start and end indexes of block b.
    /// Returns `None` if the block is empty.
    fn span(&self, b: usize) -> Option<(usize, usize)> {
        if b == 0 {
            None
        } else {
            Some((b * (b - 1) / 2, b * (b + 1) / 2 - 1))
        }
    }
}

impl<T> Default for Beap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> From<Vec<T>> for Beap<T> {
    /// Converts a `Vec<T>` into a `Beap<T>`.
    ///
    /// This conversion happens in-place, and has *O*(*n*) time complexity.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let beap = Beap::from(vec![5, 3, 2, 4, 1]);
    /// assert_eq!(beap.into_sorted_vec(), vec![1, 2, 3, 4, 5]);
    /// ```
    fn from(mut vec: Vec<T>) -> Beap<T> {
        vec.sort_unstable_by(|x, y| y.cmp(x));
        let h = ((vec.len() * 2) as f64).sqrt().round() as usize;
        Beap {
            data: vec,
            height: h,
        }
    }
}

impl<T: Ord, const N: usize> From<[T; N]> for Beap<T> {
    /// Converts a `[T, N]` into a `Beap<T>`.
    ///
    /// This conversion has *O*(*nlog(n)*) time complexity.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    ///
    /// let mut b1 = Beap::from([1, 4, 2, 3]);
    /// let mut b2: Beap<_> = [1, 4, 2, 3].into();
    /// assert_eq!(b1.into_vec(), vec![4, 3, 2, 1]);
    /// assert_eq!(b2.into_vec(), vec![4, 3, 2, 1]);
    /// ```
    fn from(arr: [T; N]) -> Self {
        Beap::from(Vec::from(arr))
    }
}

impl<T: Ord> FromIterator<T> for Beap<T> {
    /// Building Beap from iterator.
    ///
    /// This conversion has *O*(*nlog(n)*) time complexity.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    ///
    /// let mut b1 = Beap::from([1, 4, 2, 3]);
    /// let mut b2: Beap<i32> = [1, 4, 2, 3].into_iter().collect();
    /// while let Some((a, b)) = b1.pop().zip(b2.pop()) {
    ///     assert_eq!(a, b);
    /// }
    /// ```
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Beap<T> {
        Beap::from(iter.into_iter().collect::<Vec<_>>())
    }
}

impl<T: Ord> Extend<T> for Beap<T> {
    /// Extend Beap with elements from the iterator.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    ///
    /// let mut beap = Beap::new();
    /// beap.extend(vec![7, 1, 0, 4, 5, 3]);
    /// assert_eq!(beap.into_sorted_vec(), [0, 1, 3, 4, 5, 7]);
    /// ```
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for x in iter {
            self.push(x);
        }
    }
}

impl<'a, T: 'a + Ord + Copy> Extend<&'a T> for Beap<T> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().cloned());
    }
}

impl<T> IntoIterator for Beap<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Creates a consuming iterator, that is, one that moves each value out of
    /// the beap in arbitrary order. The beap cannot be used
    /// after calling this.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let beap = Beap::from(vec![1, 2, 3, 4]);
    ///
    /// // Print 1, 2, 3, 4 in arbitrary order
    /// for x in beap.into_iter() {
    ///     // x has type i32, not &i32
    ///     println!("{}", x);
    /// }
    /// ```
    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            iter: self.data.into_iter(),
        }
    }
}

impl<'a, T> IntoIterator for &'a Beap<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    /// Returns an iterator visiting all values in the underlying vector, in
    /// arbitrary order.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let beap = Beap::from(vec![1, 2, 3, 4]);
    ///
    /// // Print 1, 2, 3, 4 in arbitrary order
    /// for x in &beap {
    ///     // x has type &i32
    ///     println!("{}", x);
    /// }
    ///
    /// assert_eq!(beap.into_sorted_vec(), vec![1, 2, 3, 4]);
    /// ```
    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

/// An iterator over the elements of a `Beap`.
///
/// This `struct` is created by [`Beap::iter()`]. See its
/// documentation for more.
///
/// [`iter`]: Beap::iter
#[derive(Clone)]
pub struct Iter<'a, T: 'a> {
    iter: std::slice::Iter<'a, T>,
}

impl<T: fmt::Debug> fmt::Debug for Iter<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Iter").field(&self.iter.as_slice()).finish()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<&'a T> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }

    #[inline]
    fn last(self) -> Option<&'a T> {
        self.iter.last()
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a T> {
        self.iter.next_back()
    }
}

impl<T> FusedIterator for Iter<'_, T> {}

/// An owning iterator over the elements of a `Beap`.
///
/// This `struct` is created by [`Beap::into_iter()`]
/// (provided by the [`IntoIterator`] trait). See its documentation for more.
///
/// [`into_iter`]: Beap::into_iter
/// [`IntoIterator`]: core::iter::IntoIterator
#[derive(Clone)]
pub struct IntoIter<T> {
    iter: std::vec::IntoIter<T>,
}

impl<T: fmt::Debug> fmt::Debug for IntoIter<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("IntoIter")
            .field(&self.iter.as_slice())
            .finish()
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

impl<T> FusedIterator for IntoIter<T> {}

/// A draining iterator over the elements of a `Beap`.
///
/// This `struct` is created by [`Beap::drain()`]. See its
/// documentation for more.
///
/// [`drain`]: Beap::drain
#[derive(Debug)]
pub struct Drain<'a, T: 'a> {
    iter: std::vec::Drain<'a, T>,
}

impl<T> Iterator for Drain<'_, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<T> {
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<T> DoubleEndedIterator for Drain<'_, T> {
    #[inline]
    fn next_back(&mut self) -> Option<T> {
        self.iter.next_back()
    }
}

impl<T> FusedIterator for Drain<'_, T> {}

#[cfg(test)]
mod tests;
