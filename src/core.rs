//! Beap logic.
use crate::PosMut;

use super::{Beap, PeekMut, TailMut};

impl<T: Ord> Beap<T> {
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
                if let Some((start, _)) = self.span(self.height) {
                    if start == self.data.len() {
                        self.height -= 1;
                    }
                    std::mem::swap(&mut item, &mut self.data[0]);
                    self.siftdown(0, 1);
                }
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
                self.remove_index(idx);
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
        self.span(self.height).and_then(|(start, end)| {
            if self.height == 1 {
                self.data.first()
            } else {
                let empty = end + 1 - self.len();
                self.data.get(
                    ((start - empty)..=(end - empty))
                        .min_by_key(|&i| &self.data[i])
                        .unwrap(),
                )
            }
        })
    }

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

    /// Returns a mutable reference to the item with given position, or
    /// `None` if the position is out of bounds.
    ///
    /// Note: If the `PosMut` value is leaked, the beap may be in an
    /// inconsistent state.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use beap::Beap;
    /// let mut beap = Beap::new();
    /// assert!(beap.get_mut(0).is_none());
    ///
    /// beap.push(1);
    /// beap.push(5);
    /// beap.push(2);
    /// beap.push(3);
    /// beap.push(0);
    /// {
    ///     let mut val = beap.get_mut(3).unwrap();
    ///     assert_eq!(*val, 1);
    ///     *val = 10;
    /// }
    /// assert_eq!(beap.peek(), Some(&10));
    /// assert!(beap.get_mut(100).is_none());
    /// ```
    ///
    /// # Time complexity
    ///
    /// *O*(sqrt(*2n*)),
    pub fn get_mut(&mut self, pos: usize) -> Option<PosMut<'_, T>> {
        if pos < self.data.len() {
            Some(PosMut {
                beap: self,
                sift: false,
                pos,
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
        self.span(self.height).and_then(|(start, end)| {
            let empty = end + 1 - self.len();
            let idx = ((start - empty)..=(end - empty))
                .min_by_key(|&i| &self.data[i])
                .unwrap();
            self.remove_index(idx)
        })
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
        let (mut start, _) = match self.span(block) {
            Some(idxs) => idxs,
            None => return,
        };

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
    pub(crate) fn siftdown(&mut self, mut pos: usize, mut block: usize) {
        let (mut start, _) = match self.span(block) {
            Some(idxs) => idxs,
            None => return,
        };

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
    pub(crate) fn repair(&mut self, pos: usize) {
        if pos == 0 {
            self.siftdown(pos, 1);
        } else {
            let b = ((2 * (pos + 1)) as f64).sqrt().round() as usize;
            self.siftup(pos, b);
            self.siftdown(pos, b);
        }
    }

    /// Find the index of an element with given value
    /// or return `None` if such element does not exist.
    ///
    /// Time complexity: *O(sqrt(2n))*.
    ///
    /// # Algorithm
    ///
    /// Let there be `Beap`
    /// ```text
    ///          9
    ///        8   7
    ///      6   5   4
    ///    3   2   1   0
    /// ```
    ///
    /// Consider it as the upper left corner of the matrix:
    /// ```text
    ///    9 7 4 0
    ///    8 5 1
    ///    6 2
    ///    3
    /// ```
    ///
    /// Let's start the search from the upper-right corner
    /// (the last element of the inner vector).
    ///
    /// 1) If the priority of the desired element is greater than that
    ///     of the element in the current position, then move to the left along the line.
    ///
    /// 2) If the priority of the desired element is less than that of the element
    ///     in the current position, then move it down the column,
    ///
    /// 3) and if there is no element at the bottom, then move down and to the left
    ///     (= left on the last layer of the heap).
    ///
    /// 4) As soon as we find an element with equal val priority, we return its index,
    ///     and if we find ourselves in the left in the lower corner and the value in it
    ///     is not equal to val, so the desired element does not exist and it's time to return None.
    ///
    /// # Example
    ///
    /// ```
    /// use beap::Beap;
    ///
    /// let b = Beap::<i32>::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    /// assert_eq!(b.index(&9), Some(0));
    /// assert_eq!(b.index(&4), Some(5));
    /// assert_eq!(b.index(&1), Some(8));
    /// assert_eq!(b.index(&999), None);
    /// ```
    pub fn index(&self, val: &T) -> Option<usize> {
        let (left_low, mut right_up) = match self.span(self.height) {
            Some(idxs) => idxs,
            None => return None, // Beap is empty.
        };

        let mut block = self.height;

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

    /// Remove an element at the specified position.
    ///
    /// If the passed index is greater than the max index of the beap, it returns `None`.
    ///
    /// # Time complexity
    ///
    /// *O*(sqrt(*2n*))
    ///
    /// # Examples
    ///
    /// ```
    /// use beap::Beap;
    ///
    /// let mut b = Beap::from([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    /// assert_eq!(b.remove_index(7), Some(2));
    /// assert_eq!(b.remove_index(0), Some(9));
    ///
    /// let idx4 = b.index(&4).unwrap();
    /// assert_eq!(b.remove_index(idx4), Some(4));
    ///
    /// assert_eq!(b.remove_index(100), None);
    ///
    /// ```
    pub fn remove_index(&mut self, pos: usize) -> Option<T> {
        if pos > self.data.len() {
            return None;
        }

        self.data.pop().map(|mut item| {
            if !self.is_empty() {
                if let Some((start, _)) = self.span(self.height) {
                    if start == self.data.len() {
                        self.height -= 1;
                    }

                    if pos != self.len() {
                        std::mem::swap(&mut item, &mut self.data[pos]);
                        self.repair(pos);
                    }
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
        self.data.first()
    }

    /// Get an item at the specified position.
    ///
    /// Returns `None` if the `pos` goes beyond the beap.
    ///
    /// # Time complexity
    ///
    /// Cost is *O*(1) in the worst case.
    ///
    /// # Examples
    ///
    /// ```
    /// use beap::Beap;
    ///
    /// let b = Beap::from([1, 3, 2, 4]);
    /// assert_eq!(b.get(0), Some(&4));
    /// assert_eq!(b.get(3), Some(&1));
    /// assert_eq!(b.get(100), None);
    /// ```
    pub fn get(&self, pos: usize) -> Option<&T> {
        self.data.get(pos)
    }

    /// Start and end indexes of block b.
    /// Returns `None` if the block is empty.
    pub(crate) fn span(&self, b: usize) -> Option<(usize, usize)> {
        if b == 0 {
            None
        } else {
            Some((b * (b - 1) / 2, b * (b + 1) / 2 - 1))
        }
    }
}
