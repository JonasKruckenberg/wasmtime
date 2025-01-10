#[cfg(feature = "std")]
pub use rustc_hash::{FxHashMap, FxHashSet};

#[cfg(not(feature = "std"))]
pub type FxHashMap<K, V> =
crate::HashMap<K, V, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>;

#[cfg(not(feature = "std"))]
pub type FxHashSet<V> = hashbrown::HashSet<V, core::hash::BuildHasherDefault<rustc_hash::FxHasher>>;