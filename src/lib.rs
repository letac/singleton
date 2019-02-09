pub struct Singleton<T> {
    value: T,
}

impl<T> Default for Singleton<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            value: Default::default(),
        }
    }
}

/// Conversion
impl<T> Singleton<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

/// Conversion
impl<T> From<T> for Singleton<T> {
    fn from(value: T) -> Self {
        Self { value }
    }
}

/// Semi regular
impl<T> Clone for Singleton<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}

/// Semi regular
impl<T> Drop for Singleton<T> {
    fn drop(&mut self) {}
}

/// Regular
impl<T> PartialEq for Singleton<T>
where
    T: PartialEq,
{
    fn eq(&self, x: &Self) -> bool {
        self.value.eq(&x.value)
    }
}

/// Regular
impl<T> Eq for Singleton<T> where T: Eq {}

/// Totally-ordered
impl<T> PartialOrd for Singleton<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, x: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&x.value)
    }
}

/// Totally-ordered
impl<T> Ord for Singleton<T>
where
    T: Ord,
{
    fn cmp(&self, x: &Self) -> std::cmp::Ordering {
        self.value.cmp(&x.value)
    }
}
