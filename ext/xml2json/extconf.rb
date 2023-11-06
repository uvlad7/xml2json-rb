# frozen_string_literal: true

java_p = RUBY_PLATFORM.include?("java")

if java_p
  require_relative "jruby_cargo_builder"
else
  require "mkmf"
  require "rb_sys/mkmf"
end

create_rust_makefile("xml2json/xml2json") do |r|
  r.features = %w[jruby] if java_p
end
