//! Configuration, errors, ids and telemetry shared by all crates.

pub mod config;
pub mod error;
pub mod id;
pub mod telemetry;

/// Version of the lazypock workspace this crate belongs to.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    #[test]
    fn version_is_not_empty() {
        assert!(!super::version().is_empty());
    }
}
