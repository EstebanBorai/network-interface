fn main() {
    #[cfg(target_os = "macos")]
    {
        use cc::Build;
        use std::path::Path;
        const TARGET_MACOS: &str = "macos";

        // check cross-compile target. Only build lladdr.o when actually targeting macOS.
        let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
        if target_os == TARGET_MACOS {
            let path = Path::new("src")
                .join("target")
                .join(TARGET_MACOS)
                .join("ffi")
                .join("lladdr.c");

            Build::new().file(path).compile("ffi");
        }
    }
}
