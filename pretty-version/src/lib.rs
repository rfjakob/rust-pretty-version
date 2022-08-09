use git_version::git_describe;

/// Returns the a version string that looks like `0.9.4.234-07f1468-dirty`.
///
/// Parts:
///
///     0.9.4 ..... version from Cargo.toml
///     234 ....... Jenkins BUILD_NUMBER or 0 if unset
///     07f1468 ... abbreviated git commit hash
///     dirty ..... added only if there are uncommited changes
///
/// Usage:
///
///     use pretty_version::pretty_version;
///     fn main() {
///         println!("{}", pretty_version(env!("CARGO_PKG_VERSION")));
///     }
///
/// `env!("CARGO_PKG_VERSION")` has to be passed by the caller, as otherwise
/// it would expand to the version of the pretty-version crate itself.
pub fn pretty_version(cargo_pkg_version: &str) -> String {
	return format!("{}.{}-{}", 
		cargo_pkg_version,
		option_env!("BUILD_NUMBER").unwrap_or("0"),
		// empty "--match" disables all tags and forces output of shorthash
		git_describe!("--always", "--match=", "--dirty"));
}
