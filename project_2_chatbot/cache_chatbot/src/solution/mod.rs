pub mod v5;

pub mod slow_cache;
pub mod fast_cache;

#[cfg(feature = "slow_cache")]
pub use slow_cache::Cache;

#[cfg(not(feature = "slow_cache"))]
pub use fast_cache::Cache;
