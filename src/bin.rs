//! Naming this file `bin.rs` instead of the conventional `main.rs` helps when updating from
//! upstream. The old `main.rs` has been renamed to `lib.rs` and Git recognises this. If we named
//! this file `main.rs` then Git would think that `main.rs` had been significantly change and so
//! produce more conflicts.

fn main() {
    geckodriver_librs::bin_main();
}
