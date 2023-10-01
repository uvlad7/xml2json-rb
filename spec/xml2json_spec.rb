# frozen_string_literal: true

require "xml2json"

RSpec.describe Xml2Json do
  it "has a version number" do
    expect(Xml2Json::VERSION).not_to be nil
  end
end

RSpec.describe Xml2Json::Xml do
  it "converts json to xml" do
    expect(Xml2Json::Xml.build('{"a": 1, "b": ["2"]}')).to eq "<?xml version=\"1.0\"?><root><a>1</a><b>2</b></root>"
  end

  it "converts json into pretty xml" do
    expect(Xml2Json::Xml.build_pretty('{"a": 1, "b": ["2"]}')).to eq <<-XML.chomp
<?xml version="1.0"?>
<root>
  <a>1</a>
  <b>2</b>
</root>
    XML
  end

  it "raises on incorrect input" do
    expect { Xml2Json::Xml.build('{"a": 1, ') }.to raise_error(RuntimeError)
    expect { Xml2Json::Xml.build('{"a": 1}', indent_char: "🐈") }.to raise_error(TypeError)
  end

  it "accepts config options" do
    expect(
      Xml2Json::Xml.build(
        '{"book":{"^":{"category":"fantasy"},"title":{"_":"The Name of the Wind","^":{"lang":"en"}},' \
        '"author":"Patrick Rothfuss","year":"2007"}}',
        indent_char: " ", indent_size: 2,
        version: "1.0", encoding: :UTF8, standalone: true,
        attrkey: "^", root_name: "store"
      )
    ).to eq '<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<store>
  <book category="fantasy">
    <title lang="en">The Name of the Wind</title>
    <author>Patrick Rothfuss</author>
    <year>2007</year>
  </book>
</store>'
  end
end

RSpec.describe Xml2Json::Xml::Encoding do
  it "stores constants" do
    should be_const_defined(:UTF8)
  end
end

RSpec.describe Xml2Json::Xml::Version do
  it "stores constants" do
    should be_const_defined(:XML10)
    should be_const_defined(:XML11)
  end
end

RSpec.describe Xml2Json::Json do
  it "converts xml to json" do
    expect(Xml2Json::Json.build("<root><a>1</a><b>2</b></root>")).to eq '{"root":{"a":["1"],"b":["2"]}}'
  end

  it "converts xml into pretty json" do
    expect(Xml2Json::Json.build_pretty("<root><a>1</a><b>2</b></root>")).to eq <<-JSON.chomp
{
  "root": {
    "a": [
      "1"
    ],
    "b": [
      "2"
    ]
  }
}
    JSON
  end

  it "raises on incorrect input" do
    expect { Xml2Json::Json.build("<root><a>1</") }.to raise_error(RuntimeError)
  end

  it "may ignore incorrect input" do
    expect(Xml2Json::Json.build("<root><a>1</a")).to eq "null"
  end

  it "accepts config options" do
    expect(
      Xml2Json::Json.build(
        '<?xml version="1.0"?><book category="fantasy"><title lang="en">The Name of the Wind</title>' \
        "<author>Patrick Rothfuss</author><year>2007</year></book>",
        ignore_attrs: true,
        explicit_array: false
      )
    ).to eq '{"book":{"title":"The Name of the Wind","author":"Patrick Rothfuss","year":"2007"}}'
  end
end
