use pretty_version::pretty_version;

fn main() {
    println!("{}", pretty_version(env!("CARGO_PKG_VERSION")));
}
