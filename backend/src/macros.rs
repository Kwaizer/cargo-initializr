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
    ($opening:literal, $expr:expr, $closing:literal) => {{ format!("{}{}{}", $opening, $expr, $closing) }};
}

#[macro_export]
macro_rules! push {
    ($base:expr, $new:expr) => {{
        let mut path = PathBuf::from(&$base);
        path.push($new);
        path
    }};
}

// #[macro_export]
// macro_rules! downcast {
//     ($downcast_to:ty) => {{
//         |e| {
//             let err = e.downcast::<$downcast_to>().unwrap();
//
//             *err
//         }
//     }};
// }
