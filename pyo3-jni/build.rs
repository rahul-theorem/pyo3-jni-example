use pyo3_build_config;

// Add extension module link args since the extension-module
// feature disables linking to libpython for MacOS builds
fn main() {
    pyo3_build_config::add_extension_module_link_args();
}
