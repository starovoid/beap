pub struct Beap<T> {
    data: Vec<T>,
    pub height: usize,
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
                let (start, _) = self.span(self.height).unwrap();
                if start == self.data.len() {
                    self.height -= 1;
                }
                std::mem::swap(&mut item, &mut self.data[0]);
                self.siftdown(0, 1);
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
        if self.len() != 0 && self.data[0] > item {
            std::mem::swap(&mut item, &mut self.data[0]);
            self.siftdown(0, 1);
        }
        item
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

    /// Start and end indexes of block b.
    /// Returns `None` if the beap is empty.
    fn span(&self, b: usize) -> Option<(usize, usize)> {
        if b == 0 {
            None
        } else {
            Some((b * (b - 1) / 2, b * (b + 1) / 2 - 1))
        }
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

#[cfg(test)]
mod tests;
