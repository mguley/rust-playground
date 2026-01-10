use rustc_version_runtime;

fn main() {
    println!("Rust Collections Demo");
    println!("Compiled with: {:?}", rustc_version_runtime::version());
}
