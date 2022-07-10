use jni::objects::JClass;
use jni::sys::{jintArray, jsize};
use jni::JNIEnv;
use pyo3::{PyResult, Python};

pub fn pyo3_demo() -> PyResult<Vec<i64>> {
    Python::with_gil(|py| {
        let py_result = py
            .eval("[i * 10 for i in range(5)]", None, None)
            .map_err(|e| {
                e.print_and_set_sys_last_vars(py);
            })
            .unwrap();
        let rust_result: Vec<i64> = py_result.extract().unwrap();
        Ok(rust_result)
    })
}

#[cfg(test)]
mod tests {
    use super::pyo3_demo;

    #[test]
    #[allow(unused_must_use)]
    fn basic_test() {
        let result = pyo3_demo();
        assert_eq!(result.unwrap(), vec![0, 10, 20, 30, 40])
    }
}

#[no_mangle]
#[allow(unused)]
pub extern "system" fn Java_com_theoremlp_pex4j_SimpleJNIDemo_genRange(
    env: JNIEnv,
    class: JClass,
) -> jintArray {
    let result = pyo3_demo().expect("Failed pyo3 call");
    let output = env
        .new_int_array(result.len() as jsize)
        .expect("Failed to allocate jintArray");
    output
}
