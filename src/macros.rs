#[macro_export]
macro_rules! hash {
    ($hashable_object:expr) => {{
        let mut hasher = DefaultHasher::new();
        $hashable_object.hash(&mut hasher);
        hasher.finish()
    }};
}

#[macro_export]
macro_rules! throw {
    ($err:expr) => {
        return Err(Box::new($err))
    };
}
