#[cfg(target_os = "macos")]
fn main() {
    use cc::Build;
    use std::path::Path;

    let path = Path::new("src")
        .join("target")
        .join("macos")
        .join("helpers.c");

    Build::new().file(path).compile("helpers");
}
