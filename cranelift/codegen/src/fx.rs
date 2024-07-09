#[cfg(feature = "std")]
pub use rustc_hash::{FxHashMap, FxHashSet};

#[cfg(not(feature = "std"))]
pub type FxHashMap<K, V> = crate::HashMap<K, V, rustc_hash::FxBuildHasher>;

#[cfg(not(feature = "std"))]
pub type FxHashSet<V> = crate::HashSet<V, rustc_hash::FxBuildHasher>;
