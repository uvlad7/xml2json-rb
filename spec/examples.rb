# frozen_string_literal: true

require "node_task"

class Examples
  JSON_DEFAULT_OPTIONS = {
    charkey: "_",
    attrkey: "$",
    emptyTag: "",
    explicitRoot: true,
    explicitCharkey: false,
    trim: false,
    ignoreAttrs: false,
    mergeAttrs: false,
    normalize: false,
    normalizeTags: false,
    explicitArray: true
  }.freeze

  JSON_TRANSFORM_KEYS = {
    charkey: :charkey,
    attrkey: :attrkey,
    emptyTag: :empty_tag,
    explicitRoot: :explicit_root,
    explicitCharkey: :explicit_charkey,
    trim: :trim,
    ignoreAttrs: :ignore_attrs,
    mergeAttrs: :merge_attrs,
    normalize: :normalize_text,
    normalizeTags: :normalize_tags,
    explicitArray: :explicit_array
  }.freeze

  JSON_OPTIONS = {
    charkey: %w[c charkey],
    attrkey: %w[a attrkey],
    emptyTag: %w[e empty],
    explicitRoot: [false],
    explicitCharkey: [true],
    trim: [true],
    ignoreAttrs: [true],
    mergeAttrs: [true],
    normalize: [true],
    normalizeTags: [true],
    explicitArray: [false]
  }.freeze.each_value(&:freeze)

  def initialize
    @xml_task = NodeTask.new(File.join(__dir__ || ".", "lib/xml"))
    @json_task = NodeTask.new(File.join(__dir__ || ".", "lib/json"))
    @xml_input_data = {}
    @json_input_data = {}
    read_data
  end

  def to_h
    {
      json: option_variations(JSON_DEFAULT_OPTIONS, JSON_OPTIONS).flat_map do |options|
        @json_input_data.map do |fname, data|
          {
            fname: fname, options: options.transform_keys { |key| JSON_TRANSFORM_KEYS[key] },
            result: options.any? ? @json_task.run(data: data, options: options) : @json_task.run(data: data)
          }
        end
      end
    }
  end

  private

  def read_data
    Dir[File.join(__dir__ || ".", "data/*.json")].each do |fname|
      @xml_input_data[File.basename(fname)] = File.read(fname)
    end
    Dir[File.join(__dir__ || ".", "data/*.xml")].each do |fname|
      @json_input_data[File.basename(fname)] = File.read(fname)
    end
  end

  def option_variations(defaults, others)
    raise "Unsupported" unless defaults.keys == others.keys

    all_variations = others.merge(defaults) { |_key, old_val, new_val| [*old_val, new_val] }
    all_variations.values[0].product(*all_variations.values[1..]).map do |val|
      all_variations.keys.zip(val).to_h.compact
    end.push({})
  end
end
