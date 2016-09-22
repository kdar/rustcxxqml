extern crate rustcxx_codegen;

use std::env;
use std::path::Path;

fn main() {
  let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  let src_path = Path::new(&manifest_dir).join("src");

  if cfg!(any(target_os = "windows")) {
    let qt_dir = env::var("QT_DIR")
      .unwrap_or_else(|e| panic!("{}: please set QT_DIR env var to qt's location", e));;
    println!("cargo:rustc-link-search=native={}",
             Path::new(&qt_dir).join("lib").display());
  }

  let mut cxxflags = env::var("CXXFLAGS").unwrap_or(String::from(""));
  cxxflags.push_str(&format!(" -I {}", src_path.display()));
  env::set_var("CXXFLAGS", cxxflags);

  rustcxx_codegen::build("src/main.rs");
}
