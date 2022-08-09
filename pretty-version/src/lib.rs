//! pretty_version()! returns a version string (type &str)
//! that looks like `0.9.4.234-07f1468-dirty`.
//!
//! Parts:
//!
//!     0.9.4 ..... version from Cargo.toml
//!     234 ....... Jenkins BUILD_NUMBER or 0 if unset
//!     07f1468 ... abbreviated git commit hash
//!     -dirty .... added only if there are uncommited changes
//!
//! Usage:
//!
//!     use pretty_version::pretty_version;
//!     fn main() {
//!         println!("{}", pretty_version!());
//!     }

/// Returns a version string like `0.9.4.234-07f1468-dirty` built
/// from the crate version, Jenkins build number and the git shorthash.
#[macro_export]
macro_rules! pretty_version{
	() => {
		format!("{}.{}-{}",
			env!("CARGO_PKG_VERSION"),
			option_env!("BUILD_NUMBER").unwrap_or("0"),
			// empty "--match" disables all tags and forces output of shorthash
			pretty_version::git_describe!("--always", "--match=", "--dirty")).as_str()
    };
}

#[doc(hidden)]
pub use git_version::git_describe;
