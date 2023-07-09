# frozen_string_literal: true

require_relative "xml2json/version"
require_relative "xml2json/xml2json"

# @see https://github.com/novcn/xml2json-rs docs for the wrapped library
module Xml2Json
  # @!parse [ruby]
  #    module Xml
  #      # @param json_s [String, #to_str] JSON string
  #      # @return [String] XML string
  #      # @raise [RuntimeError] if the input string is invalid
  #      def self.build(json_s); end
  #    end
  #    module Json
  #      # @param xml [String, #to_str] XML string
  #      # @return [String] JSON string
  #      # @raise [RuntimeError] if the input string is invalid
  #      def self.build(xml); end
  #    end
end
