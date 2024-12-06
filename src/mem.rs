//! Memory management.
use super::Beap;

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

    /// Consumes the `Beap<T>` and returns the underlying vector `Vec<T>`
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
