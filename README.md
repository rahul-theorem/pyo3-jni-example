# pyo3-jni-example

A simple example of how JNI, the Rust `jni` crate & `pyo3` can be used
to execute Python functions from a Java runtime environment.

## Dev Environment
* This project uses Java 11 & Python 3.9.12 (arm64 build) with `LD_LIBRARY_PATH` set
to `$PYTHON_HOME/lib` (this is required when linking `pyo3`).
* IntelliJ Rust Plugin
* This project uses both Gradle & Cargo to build Java & Rust source code, respectively.
  This project has the Gradle IDEA plugin enabled, so initial IDE setup is as simple as
 `./gradlew idea; open pyo3-jni-example.ipr`.
* After opening the project in IntelliJ for the first time, opening any `.rs` file in
  the source tree will prompt you to attach the Cargo project. After
  the Cargo project is attached, both Java & Rust dev environments will
  work in IntelliJ.

## Testing
To run unit tests for the Java source tree, run `./gradlew test`. To run Rust tests,
run `cargo test` within the `/pyo3-jni` directory.

## Running
To run the Java example in `SimpleJNIDemo`, simply run `./gradlew run`.
