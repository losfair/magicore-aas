fn main() {
  println!("cargo:rustc-link-search=native=./primitives");
  println!("cargo:rustc-link-lib=static=magicore_primitives");
}
