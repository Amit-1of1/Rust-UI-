//! Launches the showcase on whichever renderer the Cargo feature picks (README.md).

fn main() {
    #[cfg(feature = "native")]
    {
        use dioxus::native::{Config, LogicalSize, WindowAttributes};
        // Blitz opens 800x600 by default; give the desktop layout room. Phones ignore this.
        let window = WindowAttributes::default()
            .with_title("Atoll Showcase")
            .with_inner_size(LogicalSize::new(1100.0, 780.0));
        // `new` is marked deprecated when only `native` is on (dioxus 0.7.10 leaves it out of
        // the attribute's list), but it picks the native renderer.
        #[allow(deprecated)]
        dioxus::LaunchBuilder::new()
            .with_cfg(Config::new().with_window_attributes(window))
            .launch(showcase::App);
    }
    #[cfg(not(feature = "native"))]
    dioxus::launch(showcase::App);
}
