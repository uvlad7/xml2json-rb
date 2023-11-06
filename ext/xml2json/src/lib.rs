#[cfg(mri)]
mod mri_args;
#[cfg(mri)]
mod mri_errors;

#[cfg(mri)]
use crate::mri_args::{Args};
#[cfg(mri)]
use crate::mri_errors::{Error, type_error, runtime_error};

#[cfg(mri)]
use magnus::{Value, define_module, function, Module, Object};


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

fn map_xml2json_err(error: xml2json_rs::X2JError) -> Error {
    Error::new(runtime_error(), error.details())
}

fn build_xml(args: &[Value]) -> Result<String, Error> {
    build_xml_impl(args, false)
}

fn build_pretty_xml(args: &[Value]) -> Result<String, Error> {
    build_xml_impl(args, true)
}

fn build_xml_impl(args: &[Value], build_pretty: bool) -> Result<String, Error> {
    let parsed_args = Args::new(args)?;

    let mut xml_builder: XmlBuilder;
    if let Some(opts) = parsed_args.opts() { // yep, even if it's an empty hash
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
                    u8::try_from(indent_char.unwrap()).map_err(|error| Error::new(type_error(), error.to_string()))?
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

    xml_builder.build_from_json_string(&parsed_args.str_arg).map_err(|error| {
        map_xml2json_err(error)
    })
}

fn build_json(args: &[Value]) -> Result<String, Error> {
    build_json_impl(args, false)
}

fn build_pretty_json(args: &[Value]) -> Result<String, Error> {
    build_json_impl(args, true)
}

fn build_json_impl(args: &[Value], mut build_pretty: bool) -> Result<String, Error> {
    let parsed_args = Args::new(args)?;

    let json_builder: JsonBuilder;
    if let Some(opts) = parsed_args.opts() { // yep, even if it's an empty hash
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
        json_builder.build_pretty_string_from_xml(&parsed_args.str_arg)
    } else {
        json_builder.build_string_from_xml(&parsed_args.str_arg)
    }.map_err(|error| {
        map_xml2json_err(error)
    })
}

#[cfg(mri)]
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