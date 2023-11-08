# frozen_string_literal: true

require "jruby"

# Java::IoGithubUvlad7Xml2json.Xml2JsonService.new.basicLoad(JRuby.runtime)

def io
  Java::Io
end

# like require_relative
io.github.uvlad7.xml2json.Xml2JsonService.libPath = File.join(
  File.absolute_path(File.dirname(__FILE__)),
  "xml2json.#{RbConfig::MAKEFILE_CONFIG["DLEXT"]}"
)
io.github.uvlad7.xml2json.Xml2JsonService.new.basicLoad(JRuby.runtime)
