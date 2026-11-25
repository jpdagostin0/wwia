//! Bindings to the Java standard library.

#[jvm::class("java.lang.Object")]
pub struct JObject;

#[jvm::class("java.lang.String")]
pub struct JString;

#[jvm::class("java.lang.Class")]
impl JClass {
    #[jvm::static_method("forName")]
    pub fn for_name(name: *mut JString) -> *mut Self {}
}

#[jvm::class("java.lang.Integer", rename_all = "camelCase")]
impl JInteger {
    #[jvm::static_method]
    pub fn value_of(value: i32) -> *mut Self {}

    #[jvm::method]
    pub fn int_value(&self) -> i32 {}
}

#[jvm::class("java.lang.Boolean", rename_all = "camelCase")]
impl JBoolean {
    #[jvm::method]
    pub fn boolean_value(&self) -> bool {}
}

#[jvm::class("java.nio.file.Path")]
pub struct JPath;

#[jvm::interface("java.lang.Runnable")]
pub trait Runnable {
    fn run(&self);
}

#[jvm::interface("java.util.function.Consumer")]
pub trait Consumer {
    fn accept(&self, value: *mut JObject);
}

#[jvm::interface("java.util.function.Function")]
pub trait Function {
    fn apply(&self, value: *mut JObject) -> *mut JObject;
}

#[jvm::static_method(class = "org.rustlang.runtime.Utf8View", name = "toJavaString")]
fn utf8_to_java(s: &str) -> *mut JString {}

#[jvm::static_method(class = "org.rustlang.runtime.Utf8View", name = "fromJavaString")]
fn utf8_from_java(s: *mut JString) -> &'static str {}

#[jvm::static_method(class = "java.lang.String", name = "valueOf")]
fn string_value_of(value: *mut JObject) -> *mut JString {}

#[jvm::static_method(class = "java.util.Objects", name = "isNull")]
pub fn objects_is_null(value: *mut JObject) -> bool {}

/// Null check for JVM object references. (`<*mut T>::is_null` goes through
/// rustc_codegen_jvm's raw-pointer runtime, which expects a Rust pointer, not
/// a plain JVM reference.)
#[macro_export]
macro_rules! is_null {
    ($value:expr) => {
        $crate::jvm_std::objects_is_null($value as *mut $crate::jvm_std::JObject)
    };
}

/// Rust string -> `java.lang.String`.
pub fn jstr(s: &str) -> *mut JString {
    utf8_to_java(s)
}

/// `java.lang.String` -> Rust string.
pub fn rstr(s: *mut JString) -> String {
    if crate::is_null!(s) {
        return String::new();
    }
    utf8_from_java(s).to_owned()
}

/// `String.valueOf(object)`, i.e. `toString()` that tolerates null.
pub fn to_string(value: *mut JObject) -> String {
    rstr(string_value_of(value))
}

/// Loads a JVM class by its binary name (`Block.class` etc.).
pub fn class(name: &str) -> *mut JClass {
    JClass::for_name(jstr(name))
}
