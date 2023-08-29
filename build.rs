fn main() {
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    {
        use cc::Build;
        use std::path::Path;
        const TARGET_MACOS: &str = "macos";
        const TARGET_IOS: &str = "ios";

        // check cross-compile target. Only build lladdr.o when actually targeting macOS.
        let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
        if [TARGET_MACOS, TARGET_IOS].contains(&target_os.as_str()) {
            let path = Path::new("src")
                .join("target")
                .join("apple")
                .join("ffi")
                .join("lladdr.c");

            Build::new().file(path).compile("ffi");
        }
    }
}
