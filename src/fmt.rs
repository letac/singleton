use super::Singleton;

impl<T> std::fmt::Debug for Singleton<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T> std::fmt::Display for Singleton<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T> std::fmt::Octal for Singleton<T>
where
    T: std::fmt::Octal,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T> std::fmt::LowerHex for Singleton<T>
where
    T: std::fmt::LowerHex,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T> std::fmt::UpperHex for Singleton<T>
where
    T: std::fmt::UpperHex,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T> std::fmt::Pointer for Singleton<T>
where
    T: std::fmt::Pointer,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T> std::fmt::Binary for Singleton<T>
where
    T: std::fmt::Binary,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T> std::fmt::LowerExp for Singleton<T>
where
    T: std::fmt::LowerExp,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T> std::fmt::UpperExp for Singleton<T>
where
    T: std::fmt::UpperExp,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.value.fmt(f)
    }
}
