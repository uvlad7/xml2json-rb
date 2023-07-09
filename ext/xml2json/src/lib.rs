use magnus::{define_module, function, prelude::*, Error};
use xml2json_rs::{XmlBuilder, JsonBuilder};

// fn hello(subject: String) -> String {
//     format!("Hello from Rust, {}!", subject)
// }

// struct RbXmlConfig {
//     val: XmlConfig,
// }

// impl RbXmlConfig {
//     fn root_name(&mut self, key: Option<String>) -> &mut RbXmlConfig {
//         self.val.root_name(key);
//         self
//     }
//
//     fn build(self, json_s: String) -> Result<String, Error> {
//         build_xml_impl(json_s, Some(self))
//     }
// }

// fn build_xml(json_s: String) -> Result<String, Error> {
//     build_xml_impl(json_s, None)
// }

// fn build_xml_impl(json_s: String, config: Option<RbXmlConfig>) -> Result<String, Error> {
//     let mut xml_builder = config.map_or_else(|| XmlBuilder::default(), |conf| conf.val.finalize());
//     xml_builder.build_from_json_string(&json_s).or_else(|error| {
//         Err(Error::new(magnus::exception::arg_error(), error.to_string()))
//     })
// }

fn build_xml(json_s: String) -> Result<String, Error> {
    let mut xml_builder = XmlBuilder::default();
    xml_builder.build_from_json_string(&json_s).or_else(|error| {
        Err(Error::new(magnus::exception::runtime_error(), error.details()))
    })
}

fn build_json(xml: String) -> Result<String, Error> {
    let json_builder = JsonBuilder::default();
    json_builder.build_string_from_xml(&xml).or_else(|error| {
        Err(Error::new(magnus::exception::runtime_error(), error.details()))
    })
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("Xml2Json")?;
    // module.define_singleton_method("hello", function!(hello, 1))?;
    // It's not possible to wrap XmlBuilder
    let xml = module.define_class("Xml", magnus::class::object())?;
    xml.define_singleton_method("build", function!(build_xml, 1))?;
    let json = module.define_class("Json", magnus::class::object())?;
    json.define_singleton_method("build", function!(build_json, 1))?;
    Ok(())
}
