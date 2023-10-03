# frozen_string_literal: true

require_relative "xml2json/version"
require_relative "xml2json/xml2json"

# @see https://docs.rs/xml2json-rs/1.0.1/xml2json_rs/index.html docs for the wrapped library
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
  #      # @note Default values are provided by xml2json-rs
  #      # @see https://docs.rs/xml2json-rs/1.0.1/xml2json_rs/struct.XmlConfig.html Docs for xml2json-rs XmlConfig
  #      # @option opts [String, #to_str] root_name ("root") Root key name to contain produced JSON object
  #      # @option opts [String, #to_str] attrkey ("$") The value of the JSON key used to store XML attributes under
  #      # @option opts [String, #to_str] charkey ("_") The value of the JSON key used to store XML character data under
  #      # @option opts [nil, "1.0", "1.1", #to_s] version (Xml2Json::Version::XML10) XML Declaration 'version' field,
  #      #  default if nil
  #      # @raise [RuntimeError] if +version.to_s+ returns unsupported string,
  #      #  for now only +"1.0"+ and +"1.1"+ are supported
  #      # @option opts [nil, "UTF-8", #to_s] encoding (nil) XML Declaration 'encoding' field, empty if nil
  #      # @raise [RuntimeError] if +encoding.to_s+ returns unsupported string, for now only +"UTF-8"+ is supported
  #      # @option opts [bool, nil] standalone (nil) XML Declaration 'standalone' field, empty if nil
  #      # @option opts [bool] indent (false) Output XML with line-breaks and indentations
  #      # @option opts [String, #to_str] indent_char (" ") Character used for indentation. This option will be ignored
  #      #  if +indent+ is false
  #      # @raise [TypeError] If +indent_char+ is more than one character or can not be encoded as UTF-8
  #      # @note It's not enough for the Ruby +indent_char+ string to be UTF-8 encoded, not sure
  #      #  if that's a bug or a feature of Magnus
  #      # @option opts [Integer, #to_int] indent_size (2) Number of characters used for indentation.
  #      #  This option will be ignored if +indent+ is false
  #      # @raise [RangeError] if +indent_size+ cannot be converted into +unsigned long long+
  #      # @return [String] XML string
  #      # @raise [RuntimeError] if +json_s+ is not valid JSON
  #      # @raise [TypeError] if +opts+ contain invalid value
  #      def self.build(json_s, opts: nil); end
  #      # @option opts [bool] indent (true)
  #      # @note Check +Xml2Json::Xml.build+ for other details, the only difference is that +indent+ is true by default
  #      # @see .build
  #      def self.build_pretty(json_s, opts: nil); end
  #    end
  #
  #    module Json
  #      # @param xml [String, #to_str] XML string
  #      # @param opts [Hash<Symbol, Object>] config params
  #      # @note Default values are provided by xml2json-rs
  #      # @see https://docs.rs/xml2json-rs/1.0.1/xml2json_rs/struct.JsonConfig.html Docs for xml2json-rs JsonConfig
  #      # @option opts [String, #to_str] attrkey ("$") Key to outer object containing tag attributes
  #      # @option opts [String, #to_str] charkey ("_") Key to store character content under
  #      # @option opts [String, #to_str] empty_tag ("") The value of empty nodes
  #      # @option opts [bool] indent (false) Output JSON with line-breaks and indentations
  #      # @option opts [bool] explicit_root (true) Sets the root node inside the resulting object
  #      # @option opts [bool] trim (false) Trim whitespace at the beginning and end of text nodes
  #      # @option opts [bool] ignore_attrs (false) Setting this to true will skip adding element attributes
  #      #  and create text nodes
  #      # @option opts [bool] merge_attrs (false) Merge all XML attributes and child elements as properties of
  #      #  the parent, instead of keying attributes off of the child attribute object. This option will be
  #      #  ignored if +ignore_attrs+ is true
  #      # @option opts [bool] normalize_text (false) Removes whitespace character data in text nodes
  #      # @option opts [bool] lowercase_tags (false) Normalize all tags by converting them to lowercase
  #      # @option opts [bool] explicit_array (true) Always put the child nodes in an array, otherwise an array is only
  #      #  created if there is more than one child
  #      # @option opts [bool] explicit_charkey (false) Always store character data under +charkey+ even if there are
  #      #  no other text elements stored inside the tag
  #      # @return [String] JSON string
  #      # @raise [RuntimeError] if +xml+ is not valid XML
  #      # @raise [TypeError] if +opts+ contain invalid value
  #      def self.build(xml, opts: nil); end
  #      # @option opts [bool] indent (true)
  #      # @note Check +Xml2Json::Json.build+ for other details, the only difference is that +indent+ is true by default
  #      # @see .build
  #      def self.build_pretty(xml, opts: nil); end
  #    end
end
