use jni::objects::JClass;
use jni::sys::{jint, jintArray, jsize};
use jni::JNIEnv;
use pyo3::{PyResult, Python};

pub fn pyo3_demo(max: i32) -> PyResult<Vec<i32>> {
    Python::with_gil(|py| {
        let code = format!("list(range({}))", max);
        let py_result = py
            .eval(code.as_str(), None, None)
            .map_err(|e| {
                e.print_and_set_sys_last_vars(py);
            })
            .unwrap();
        let rust_result: Vec<i32> = py_result.extract().unwrap();
        Ok(rust_result)
    })
}

#[cfg(test)]
mod tests {
    use super::pyo3_demo;

    #[test]
    #[allow(unused_must_use)]
    fn basic_test() {
        let result = pyo3_demo(5).unwrap();
        assert_eq!(result, vec![0, 1, 2, 3, 4])
    }
}

#[no_mangle]
#[allow(unused)]
pub extern "system" fn Java_com_theoremlp_demo_SimpleJNIDemo_range(
    env: JNIEnv,
    class: JClass,
    max: jint,
) -> jintArray {
    let result = pyo3_demo(max).expect("pyo3 call failed");
    let output = env
        .new_int_array(result.len() as jsize)
        .expect("Failed to allocate jintArray");
    env.set_int_array_region(output, 0, result.as_slice());
    output
}
