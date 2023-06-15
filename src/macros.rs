#[macro_export]
macro_rules! hash {
    ($hashable_object:expr) => {{
        let mut hasher = DefaultHasher::new();
        $hashable_object.hash(&mut hasher);
        hasher.finish()
    }};
}

#[macro_export]
macro_rules! quote {
    ($opening:literal, $expr:expr, $closing:literal) => {{
        format!("{}{}{}", $opening, $expr, $closing)
    }};
}
