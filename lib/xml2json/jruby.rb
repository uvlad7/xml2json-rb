# frozen_string_literal: true

require "jruby"

# Java::IoGithubUvlad7Xml2json.Xml2JsonService.new.basicLoad(JRuby.runtime)

def io
  Java::Io
end

io.github.uvlad7.xml2json.Xml2JsonService.new.basicLoad(JRuby.runtime)
