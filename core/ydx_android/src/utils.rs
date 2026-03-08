use jni::JNIEnv;
use jni::objects::JString;

/// Extension trait for JNIEnv to simplify string conversion
pub trait JNIEnvExt<'local> {
    fn to_string(&mut self, jstring: &JString<'local>) -> String;
}

impl<'local> JNIEnvExt<'local> for JNIEnv<'local> {
    fn to_string(&mut self, jstring: &JString<'local>) -> String {
        self.get_string(jstring)
            .expect("Failed to get Java string")
            .into()
    }
}
