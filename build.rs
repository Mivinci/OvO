#[cfg(target_os = "macos")]
fn main() {
  println!("cargo:rustc-link-search=native=build");
  println!("cargo:rustc-link-lib=static=JavaScriptCore-CXX-Bindings");
  println!("cargo:rustc-link-lib=framework=JavaScriptCore");
}
