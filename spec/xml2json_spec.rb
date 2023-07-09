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

  it "raises on incorrect input" do
    expect { Xml2Json::Xml.build('"a": 1, ') }.to raise_error(RuntimeError)
  end
end

RSpec.describe Xml2Json::Json do
  it "converts xml to json" do
    expect(Xml2Json::Json.build("<root><a>1</a><b>2</b></root>")).to eq '{"root":{"a":["1"],"b":["2"]}}'
  end

  it "raises on incorrect input" do
    expect { Xml2Json::Json.build("<root><a>1</") }.to raise_error(RuntimeError)
  end

  it "may ignore incorrect input" do
    expect(Xml2Json::Json.build("<root><a>1</a")).to eq "null"
  end
end
