#[cfg(target_os = "macos")]
fn main() {
  println!("cargo:rustc-link-lib=framework=JavaScriptCore");
}
