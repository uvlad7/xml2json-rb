#[cfg_attr(feature = "mri", path = "magnus.rs")]
#[cfg_attr(feature = "jruby", path = "jni.rs")]
#[cfg(feature = "mri")]
mod implementation;

use std::fmt::Debug;
#[cfg(feature = "mri")]
use crate::implementation::args::{Args};
#[cfg(feature = "mri")]
use crate::implementation::errors::{Error, type_error, runtime_error};

#[cfg(feature = "mri")]
use magnus::{Value, define_module, function, Module, Object};

#[cfg(feature = "jruby")]
use std::os::raw::c_void;
use std::ptr::null;
#[cfg(feature = "jruby")]
use robusta_jni::jni::{JavaVM, JNIEnv, NativeMethod, objects::JClass, strings::JNIString};
#[cfg(feature = "jruby")]
use robusta_jni::jni::sys::{jfloat, jint, JNI_ERR, JNI_VERSION_1_4};
#[cfg(feature = "jruby")]
use robusta_jni::jni::objects::JString;

use xml2json_rs::{XmlBuilder, JsonBuilder, JsonConfig, XmlConfig, Declaration, Version, Encoding, Indentation};
#[macro_export]
macro_rules! set_arg {
    ($config:expr, $opts:expr, $arg:ident, $arg_type:ident) => (
        let arg_value = $opts.lookup::<$arg_type>(stringify!($arg))?;
        if let Some(value) = arg_value {
            $config.$arg(value);
        }
    );
}

#[cfg(feature = "mri")]
fn map_xml2json_err(error: xml2json_rs::X2JError) -> Error {
    runtime_error(error.details())
}

#[cfg(feature = "mri")]
fn build_xml_impl(args: Args, build_pretty: bool) -> Result<String, Error> {
    let mut xml_builder: XmlBuilder;
    if let Some(opts) = args.opts() { // yep, even if it's an empty hash
        let mut config = XmlConfig::new();
        set_arg!(config, opts, root_name, String);
        set_arg!(config, opts, attrkey, String);
        set_arg!(config, opts, charkey, String);

        let decl_version = opts.lookup_str("version")?;
        let decl_encoding = opts.lookup_str("encoding")?;
        let decl_standalone = opts.lookup::<bool>("standalone")?;
        if decl_version.is_some()
            || decl_encoding.is_some()
            || decl_standalone.is_some()
        { // something is specified
            // I didn't find a way to get defaults without copying them
            let decl_version_val = if let Some(decl_version_str) = decl_version {
                Version::try_from(decl_version_str.as_str())
                    .map_err(map_xml2json_err)?
            } else { Version::XML10 };
            let decl_encoding_val = if let Some(decl_encoding_str) = decl_encoding {
                Some(Encoding::try_from(decl_encoding_str.as_str())
                    .map_err(map_xml2json_err)?)
            } else { None };

            let decl = Declaration::new(
                decl_version_val,
                decl_encoding_val,
                decl_standalone,
            );
            config.decl(decl);
        }

        let indent = opts.lookup::<bool>("indent")?;
        if indent.unwrap_or(true) {
            let indent_char = opts.lookup::<char>("indent_char")?;
            let indent_size = opts.lookup::<usize>("indent_size")?;
            if indent_char.is_some()
                || indent_size.is_some()
            {
                let indent_char_val: u8 = if indent_char.is_some() {
                    u8::try_from(indent_char.unwrap()).map_err(|error| type_error(error.to_string()))?
                } else { b' ' };
                config.rendering(Indentation::new(
                    indent_char_val,
                    indent_size.unwrap_or(2),
                ));
            } else if indent.unwrap_or(build_pretty) {
                // because Indentation::default is not the same as not calling config.rendering at all;
                config.rendering(Indentation::default());
            }
        }
        xml_builder = config.finalize();
    } else if build_pretty {
        xml_builder = XmlConfig::new().rendering(Indentation::default()).finalize();
    } else { xml_builder = XmlBuilder::default(); }

    xml_builder.build_from_json_string(&args.str_arg).map_err(|error| {
        map_xml2json_err(error)
    })
}

#[cfg(feature = "mri")]
fn build_json_impl(args: Args, mut build_pretty: bool) -> Result<String, Error> {
    let json_builder: JsonBuilder;
    if let Some(opts) = args.opts() { // yep, even if it's an empty hash
        let mut config = JsonConfig::new();
        set_arg!(config, opts, charkey, String);
        set_arg!(config, opts, attrkey, String);
        set_arg!(config, opts, empty_tag, String);
        set_arg!(config, opts, explicit_root, bool);
        set_arg!(config, opts, trim, bool);
        set_arg!(config, opts, ignore_attrs, bool);
        set_arg!(config, opts, merge_attrs, bool);
        set_arg!(config, opts, normalize_text, bool);
        set_arg!(config, opts, lowercase_tags, bool);
        set_arg!(config, opts, explicit_array, bool);
        set_arg!(config, opts, explicit_charkey, bool);
        json_builder = config.finalize();

        let indent = opts.lookup::<bool>("indent")?;
        if let Some(indent_value) = indent {
            build_pretty = indent_value;
        }
    } else { json_builder = JsonBuilder::default(); }

    if build_pretty {
        json_builder.build_pretty_string_from_xml(&args.str_arg)
    } else {
        json_builder.build_string_from_xml(&args.str_arg)
    }.map_err(|error| {
        map_xml2json_err(error)
    })
}

#[cfg(feature = "mri")]
fn build_xml(args: &[Value]) -> Result<String, Error> {
    build_xml_impl(Args::new(args)?, false)
}

#[cfg(feature = "mri")]
fn build_pretty_xml(args: &[Value]) -> Result<String, Error> {
    build_xml_impl(Args::new(args)?, true)
}

#[cfg(feature = "mri")]
fn build_json(args: &[Value]) -> Result<String, Error> {
    build_json_impl(Args::new(args)?, false)
}

#[cfg(feature = "mri")]
fn build_pretty_json(args: &[Value]) -> Result<String, Error> {
    build_json_impl(Args::new(args)?, true)
}

#[cfg(feature = "mri")]
#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("Xml2Json")?;
    let xml = module.define_module("Xml")?;
    let xml_version = xml.define_module("Version")?;
    xml_version.const_set("XML10", Version::XML10.to_string())?;
    xml_version.const_set("XML11", Version::XML11.to_string())?;
    let xml_encoding = xml.define_module("Encoding")?;
    xml_encoding.const_set("UTF8", Encoding::UTF8.to_string())?;
    xml.define_singleton_method("build", function!(build_xml, -1))?;
    xml.define_singleton_method("build_pretty", function!(build_pretty_xml, -1))?;
    let json = module.define_module("Json")?;
    json.define_singleton_method("build", function!(build_json, -1))?;
    json.define_singleton_method("build_pretty", function!(build_pretty_json, -1))?;
    Ok(())
}

#[cfg(feature = "jruby")]
extern "system" fn jni_version(env: JNIEnv, class: JClass) -> jfloat {
    println!("{}", 1.4f32);
    1.4f32
}

#[cfg(feature = "jruby")]
extern "system" fn build_xml<'local>(mut env: JNIEnv<'local>,
                                     _class: JClass<'local>,
                                     input: JString<'local>) -> JString<'local> {
    match env.get_string(input) {
        Ok(java_str) => {
            let json_s: String = java_str.into();
            let mut xml_builder: XmlBuilder = XmlBuilder::default();
            match xml_builder.build_from_json_string(&json_s) {
                Ok(res) => {
                    match env.new_string(res) {
                        Ok(conv_res) => { return conv_res; }
                        Err(err) => {
                            // No need to handle err, ClassNotFoundException will be thrown implicitly
                            let _ = env.throw_new("java/lang/RuntimeException", format!("{:?}", err));
                        }
                    }
                }
                Err(err) => {
                    // No need to handle err, ClassNotFoundException will be thrown implicitly
                    let _ = env.throw_new("java/lang/RuntimeException", err.details());
                }
            }
        }
        Err(err) => {
            // No need to handle err, ClassNotFoundException will be thrown implicitly
            let _ = env.throw_new("java/lang/RuntimeException", format!("{:?}", err));
        }
    }
    JString::from(std::ptr::null_mut())
}

#[cfg(feature = "jruby")]
/// This function is executed on loading native library by JVM.
/// It initializes the cache of method and class references.
#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn JNI_OnLoad<'local>(vm: JavaVM, _: *mut c_void) -> jint {
    let Ok(mut env) = vm.get_env() else { return JNI_ERR; };
    let Ok(clazz) = env.find_class(
        "io/github/uvlad7/xml2json/Xml"
    ) else { return JNI_ERR; };
    // mem::transmute() - https://rust-lang.github.io/unsafe-code-guidelines/layout/function-pointers.htmlg
    // Like function! macro
    let func = jni_version as unsafe extern "system" fn(env: JNIEnv, class: JClass) -> jfloat;
    // like func.as_ptr()
    let func_ptr = func as *mut c_void;
    let method = NativeMethod {
        name: JNIString::from("nativeGetJniVersion"),
        sig: JNIString::from("()F"),
        fn_ptr: func_ptr,
    };
    let build_xml_ptr = build_xml as unsafe extern "system" fn(env: JNIEnv<'local>, _class: JClass<'local>, input: JString<'local>) -> JString<'local>;
    let build_xml_method = NativeMethod {
        name: JNIString::from("buildImpl"),
        sig: JNIString::from("(Ljava/lang/String;)Ljava/lang/String;"),
        fn_ptr: build_xml_ptr as *mut c_void,
    };
    let Ok(_) = env.register_native_methods(clazz, &[method, build_xml_method]) else { return JNI_ERR; };
    JNI_VERSION_1_4
}
