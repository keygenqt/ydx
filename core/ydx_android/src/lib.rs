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
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    token: JString<'local>,
) -> jstring {
    let token = env.to_string(&token);
    let result = ffi_response!(HttpClient::instance().signcheck(Some(token)));
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

/// Get user by id
/// GET
#[unsafe(no_mangle)]
pub extern "C" fn Java_com_keygenqt_ydx_YdxAndroid_requestGetUser<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    id: JString<'local>,
    token: JString<'local>,
) -> jstring {
    let id = env.to_string(&id);
    let token = env.to_string(&token);
    let result = ffi_response!(HttpClient::instance().get_user(id, Some(token)));
    env.new_string(result.as_str())
        .expect("Couldn't create Java string")
        .into_raw()
}

/// Signout user
/// DELETE
#[unsafe(no_mangle)]
pub extern "C" fn Java_com_keygenqt_ydx_YdxAndroid_requestSignout<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    token: JString<'local>,
) -> jstring {
    let token = env.to_string(&token);
    let result = ffi_response!(HttpClient::instance().signout(Some(token)));
    env.new_string(result.as_str())
        .expect("Couldn't create Java string")
        .into_raw()
}
