use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;
use ydx_core::{ffi_response, service::client::HttpClient};

use crate::utils::JNIEnvExt;

mod utils;

/// Init http client
#[unsafe(no_mangle)]
pub extern "C" fn Java_com_keygenqt_ydx_YdxAndroid_requestInit<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    base_url: JString<'local>,
    uuid: JString<'local>,
) {
    HttpClient::init(&env.to_string(&base_url), &env.to_string(&uuid));
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
    let result = ffi_response!(
        HttpClient::instance().signin(env.to_string(&email), env.to_string(&password))
    );
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
    let result = ffi_response!(
        HttpClient::instance().signup(env.to_string(&email), env.to_string(&password))
    );
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
