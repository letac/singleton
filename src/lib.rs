use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Singleton<T> {
    value: T,
}
impl<T: Debug> Singleton<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemiRegular<T> {
    value: T,
}
impl<T: Debug + Clone> SemiRegular<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Regular<T> {
    value: T,
}
impl<T: Debug + Clone + Eq> Regular<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TotallyOrdered<T> {
    value: T,
}
impl<T: Debug + Clone + Eq + Ord> TotallyOrdered<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}
