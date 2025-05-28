[![Gem Version](https://badge.fury.io/rb/xml2json-rb.svg)](https://badge.fury.io/rb/xml2json-rb)
![Gem](https://img.shields.io/gem/dt/xml2json-rb?style=plastic)
![Gem](https://img.shields.io/gem/dtv/xml2json-rb?style=plastic)
[![Tests](https://github.com/uvlad7/xml2json-rb/actions/workflows/main.yml/badge.svg)](https://github.com/uvlad7/xml2json-rb/actions/workflows/main.yml)
[![Docs](https://github.com/uvlad7/xml2json-rb/actions/workflows/docs.yml/badge.svg)](https://github.com/uvlad7/xml2json-rb/actions/workflows/docs.yml)
[![Release](https://github.com/uvlad7/xml2json-rb/actions/workflows/release.yml/badge.svg)](https://github.com/uvlad7/xml2json-rb/actions/workflows/release.yml)

# Xml2Json

A tiny Ruby wrapper for the Rust [xml2json](https://github.com/novcn/xml2json-rs) lib which allows
to convert xml strings to json and vise versa.

Table of Contents
-----------------

<!-- Generated with `markdown-toc -i README.md` -->

<!-- toc -->

- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage examples](#usage-examples)
- [Documentation](#documentation)
- [Development](#development)
- [Contributing](#contributing)
- [Releases](#releases)
- [License](#license)

<!-- tocstop -->

## Prerequisites

This gem is written in Rust and provides precompiled binary versions for the most common architectures and operation systems. If there is no suitable version or if you don't want to use precompiled libraries, you need
to [install](https://doc.rust-lang.org/cargo/getting-started/installation.html) `cargo` to compile
it from source.

> **Warning**  
> This won't work for local development with `rake compile`

If you have some weird setup with `cargo` named differently, of it's not in your `PATH`, you can
pass a command to be evaluated via environment variable `CARGO`, for example:

    $ export CARGO=cargo1_72_1

or simply

    $ CARGO=/path/to/cargo gem install xml2json-rb

## Installation

Install the gem and add to the application's Gemfile by executing:

    $ bundle add xml2json-rb --require xml2json

If bundler is not being used to manage dependencies, install the gem by executing:

    $ gem install xml2json-rb

## Usage examples

```ruby
require "xml2json"

puts Xml2Json::Xml.build_pretty('{"a": 1, "b": "2"}')
# <?xml version="1.0"?>
# <root>
#   <a>1</a>
#   <b>2</b>
# </root>
puts Xml2Json::Json.build("<root><a>1</a><b>2</b></root>", explicit_array: false)
# {"root":{"a":"1","b":"2"}}
```

See [specs](spec/xml2json_spec.rb) for more examples.

## Documentation

YARD docs are available on [rubydoc.info](https://rubydoc.info/gems/xml2json-rb/) for the latest
released version and on [pages.github.com](https://uvlad7.github.io/xml2json-rb/) for the most
recent changes in the `main` branch.

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake spec` to run
the tests. You can also run `bin/console` for an interactive prompt that will allow you to
experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new
version, update the version number in `version.rb`, and then run `bundle exec rake release`, which
will create a git tag for the version, push git commits and the created tag, and push the `.gem`
file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome [on GitHub](https://github.com/uvlad7/xml2json-rb).

## Releases

This project uses [Semantic Versioning](https://semver.org/) and adheres to
the [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) guidelines.

## License

The gem is available as open source under the terms of
the [MIT License](https://opensource.org/licenses/MIT).
