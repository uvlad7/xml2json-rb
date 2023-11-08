# frozen_string_literal: true

java_p = RUBY_PLATFORM.include?("java")

if java_p
  require_relative "jruby_cargo_builder"
else
  require "mkmf"
  require "rb_sys/mkmf"
end

# DLLIB = $(TARGET).#{RbConfig::CONFIG["DLEXT"]} in rb_sys/mkmf
dlext = RbConfig::CONFIG["DLEXT"]
RbConfig::CONFIG["DLEXT"] = RbConfig::MAKEFILE_CONFIG["DLEXT"]

create_rust_makefile("xml2json/xml2json") do |r|
  r.features = java_p ? %w[jruby] : %w[mri]
end

RbConfig::CONFIG["DLEXT"] = dlext
