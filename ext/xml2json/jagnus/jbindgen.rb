# frozen_string_literal: true

require "erb"

# bindgen for robusta-jni
module JBindgen
  # Wrapper around java.lang.class
  class JavaClass
    def initialize(java_class)
      @java_class = java_class
    end

    def package_name
      @java_class.get_package.get_name
    end

    def name
      @java_class.get_name
    end

    def simple_name
      @java_class.get_simple_name
    end
  end

  # Wrapper around java.lang.reflect.Method
  class JavaMethod
    def initialize(java_method)
      @java_method = java_method
    end
  end

  # Generates rust code
  class Codegen
    STRUCT_TEMPLATE = ERB.new(<<~TEMPLATE)
      #[robusta_jni::bridge]
      mod bridge_mod {
          use robusta_jni::{
              convert::{
                  Signature, TryFromJavaValue, TryIntoJavaValue,
              },
              jni::errors::Result as JniResult,
              jni::objects::AutoLocal,
              jni::JNIEnv,
              jni::objects::JObject,
          };

          #[derive(Signature, TryIntoJavaValue, TryFromJavaValue)]
          #[package(org.jruby)]
          pub struct RubyModule<'env: 'borrow, 'borrow> {
              #[instance]
              raw: AutoLocal<'env, 'borrow>,
          }

          impl<'env: 'borrow, 'borrow> RubyModule<'env, 'borrow> {
              pub extern "java" fn defineAnnotatedMethods(&self,
                                                          env: &JNIEnv,
                                                          #[input_type("Ljava/lang/Class;")] cls: JObject,
              ) -> JniResult<()> {}
          }
      }
      // TODO: `From` trait for descendants
      pub use bridge_mod::*;

    TEMPLATE

    FINAL_MOD_TEMPLATE = ERB.new(<<~TEMPLATE)
      mod ruby_module;

      pub use ruby_module::*;

      mod ruby;

      pub use ruby::*;

    TEMPLATE

    INTERMEDIATE_MOD_TEMPLATE = ERB.new(<<~TEMPLATE)
      pub mod org;

    TEMPLATE
  end
end
