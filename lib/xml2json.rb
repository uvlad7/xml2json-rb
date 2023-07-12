# frozen_string_literal: true

require_relative "xml2json/version"
require_relative "xml2json/xml2json"

# @see https://github.com/novcn/xml2json-rs docs for the wrapped library
module Xml2Json
  # @!parse [ruby]
  #    module Xml
  #      module Version
  #        XML10 = "1.0"
  #        XML11 = "1.1"
  #      end
  #      module Encoding
  #        UTF8 = "UTF-8"
  #      end
  #
  #      # @param json_s [String, #to_str] JSON string
  #      # @param opts [Hash<Symbol, Object>] config params
  #      # @see https://docs.rs/xml2json-rs/1.0.1/xml2json_rs/struct.XmlConfig.html rust doc
  #      # @return [String] XML string
  #      # @raise [RuntimeError] if the input string is invalid
  #      def self.build(json_s, opts: nil); end
  #      def self.build_pretty(json_s, opts: nil); end
  #    end
  #
  #    module Json
  #      # @param xml [String, #to_str] XML string
  #      # @param opts [Hash<Symbol, Object>] config params
  #      # @see https://docs.rs/xml2json-rs/1.0.1/xml2json_rs/struct.JsonConfig.html rust doc
  #      # @return [String] JSON string
  #      # @raise [RuntimeError] if the input string is invalid
  #      def self.build(xml, opts: nil); end
  #      def self.build_pretty(xml, opts: nil); end
  #    end
end
