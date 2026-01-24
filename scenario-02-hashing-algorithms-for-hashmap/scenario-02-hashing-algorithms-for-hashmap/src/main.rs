mod ahash_examples;
mod foldhash_examples;
mod fxhash_examples;
mod nohash_examples;
mod siphash_examples;
mod xxhash_examples;

use ahash_examples::run_all as ahash_run_all;
use foldhash_examples::run_all as foldhash_run_all;
use fxhash_examples::run_all as fxhash_run_all;
use nohash_examples::run_all as nohash_run_all;
use rustc_version_runtime;
use siphash_examples::run_all as siphash_run_all;
use xxhash_examples::run_all as xxhash_run_all;
fn main() {
    println!("Hashing Algorithms for HashMap - Demo");
    println!("Compiled with: {:?}", rustc_version_runtime::version());
    println!();

    // siphash_run_all();
    // fxhash_run_all();
    // ahash_run_all();
    // foldhash_run_all();
    // xxhash_run_all();
    nohash_run_all();
}
