use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use ydx_core::{ffi_response, service::client::HttpClient};

/// Init http client
#[unsafe(no_mangle)]
pub extern "C" fn Java_com_keygenqt_ydx_YdxAndroid_requestInit<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    base_url: JString<'local>,
    uuid: JString<'local>,
) {
    let base_url_str: String = env
        .get_string(&base_url)
        .expect("Couldn't get Java string")
        .into();
    let uuid_str: String = env
        .get_string(&uuid)
        .expect("Couldn't get Java string")
        .into();

    HttpClient::init(&base_url_str, &uuid_str);
}

/// Check auth user
/// GET
#[unsafe(no_mangle)]
pub extern "C" fn Java_com_keygenqt_ydx_YdxAndroid_requestSigncheck<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jstring {
    let result = ffi_response!(HttpClient::instance().signcheck());
    env.new_string(result.as_str())
        .expect("Couldn't create Java string")
        .into_raw()
}

/// Signin user
/// POST
#[unsafe(no_mangle)]
pub extern "C" fn Java_com_keygenqt_ydx_YdxAndroid_requestSignin<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    email: JString<'local>,
    password: JString<'local>,
) -> jstring {
    let email_str: String = env
        .get_string(&email)
        .expect("Couldn't get Java string")
        .into();
    let password_str: String = env
        .get_string(&password)
        .expect("Couldn't get Java string")
        .into();

    let result = ffi_response!(HttpClient::instance().signin(email_str, password_str));
    env.new_string(result.as_str())
        .expect("Couldn't create Java string")
        .into_raw()
}

/// Signup user
/// POST
#[unsafe(no_mangle)]
pub extern "C" fn Java_com_keygenqt_ydx_YdxAndroid_requestSignup<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    email: JString<'local>,
    password: JString<'local>,
) -> jstring {
    let email_str: String = env
        .get_string(&email)
        .expect("Couldn't get Java string")
        .into();
    let password_str: String = env
        .get_string(&password)
        .expect("Couldn't get Java string")
        .into();

    let result = ffi_response!(HttpClient::instance().signup(email_str, password_str));
    env.new_string(result.as_str())
        .expect("Couldn't create Java string")
        .into_raw()
}

/// Signout user
/// DELETE
#[unsafe(no_mangle)]
pub extern "C" fn Java_com_keygenqt_ydx_YdxAndroid_requestSignout<'local>(
    env: JNIEnv<'local>,
    _class: JClass<'local>,
) -> jstring {
    let result = ffi_response!(HttpClient::instance().signout());
    env.new_string(result.as_str())
        .expect("Couldn't create Java string")
        .into_raw()
}
