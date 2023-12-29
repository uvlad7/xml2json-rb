# frozen_string_literal: true

require "erb"

range = ("a".."z").to_a
template = ERB.new <<~ERB
      ($name:expr, <%= params.map.with_index { |name, i| "$param%s:ty, " % (i + 1) }.join %>$ret:ty, $java_name:expr) => {{
          unsafe extern "system" fn anon<'local>(mut env: robusta_jni::jni::JNIEnv<'local>,
                                         _this: robusta_jni::jni::objects::JObject<'local>,
  <%= params.map.with_index do |name, i|
  "                                       %s: <$param%s as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source," % [name, (i + 1)]
  end.join("\n") %>
          ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target {
              use $crate::{JNIFunction<%= params.size %>, JNIEnvFunction<%= params.size %>};
              $name.call_handle_error(<%= params.join ", " %>, &env)
          }
          let anon_func = anon as unsafe extern "system" fn(robusta_jni::jni::JNIEnv<'local>,
                                         robusta_jni::jni::objects::JObject<'local>,
  <%= params.map.with_index do |name, i|
  "                                       <$param%s as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source," % (i + 1)
  end.join("\n") %>
          ) -> <$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target;
          // TODO: impl and use func.as_ptr()
          let anon_ptr = anon_func as *mut std::ffi::c_void;
  
          robusta_jni::jni::NativeMethod {
              name: robusta_jni::jni::strings::JNIString::from($java_name),
              sig: robusta_jni::jni::strings::JNIString::from(format!("(<%= "{}" * params.size %>){}",
  <%= params.map.with_index do |name, i|
  "                <<$param%s as robusta_jni::convert::TryFromJavaValue<'local, 'local>>::Source as robusta_jni::convert::Signature>::SIG_TYPE," % (i + 1)
  end.join("\n") %>
                  <<$ret as robusta_jni::convert::TryIntoJavaValue<'local>>::Target as robusta_jni::convert::Signature>::SIG_TYPE)),
              fn_ptr: anon_ptr,
          }
      }};
ERB

(0..16).each do |len|
  params = range[0...len]
  puts template.result(binding)
end
