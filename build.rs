use std::process::Command;
fn main() {

    if cfg!(target_os = "linux") {
       Command::new("make")
               .current_dir("src/c-src")
               .status()
               .expect("failed to make!");
   }
    println!(r"cargo:rustc-link-search=/home/jonas/Documents/projects/rust-opencl/src/c-src");
}
