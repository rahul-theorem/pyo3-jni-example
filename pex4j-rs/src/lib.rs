use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jstring};

#[no_mangle]
#[allow(unused)]
pub extern "system" fn Java_com_theoremlp_pex4j_FFIDemo_sayHello(
    env: JNIEnv,
    class: JClass,
    who: JString,
) -> jstring {
    let input: String = env.get_string(who)
        .expect("Failed to load java string")
        .into();
    let output = env.new_string(format!("Hello, {}!", input))
        .expect("Failed to create java string");
    output.into_inner()
}