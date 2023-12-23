# frozen_string_literal: true

require_relative "lib/xml2json/version"

java_p = RUBY_PLATFORM.include?("java")

Gem::Specification.new do |spec|
  spec.name = "xml2json-rb"
  spec.version = Xml2Json::VERSION
  spec.authors = ["uvlad7"]
  spec.email = ["uvlad7@gmail.com"]

  spec.summary = "Ruby wrapper for xml2json-rs"
  spec.description = "This gem helps to convert xml strings to json and vise versa"
  spec.homepage = "https://github.com/uvlad7/xml2json-rb"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 2.6.0"
  spec.required_rubygems_version = ">= 3.3.11"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/uvlad7/xml2json-rb"
  spec.metadata["changelog_uri"] = "https://github.com/uvlad7/xml2json-rb/blob/main/CHANGELOG.md"
  spec.metadata["documentation_uri"] = "https://rubydoc.info/gems/xml2json-rb/#{spec.version}"
  spec.metadata["rubygems_mfa_required"] = "true"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  # spec.files = Dir.chdir(__dir__) do
  #   `git ls-files -z`.split("\x0").reject do |f|
  #     (File.expand_path(f) == __FILE__) ||
  #       f.start_with?(*%w[bin/ test/ spec/ features/ .git .circleci appveyor Gemfile])
  #   end
  # end
  ext_files = %w[rs rb toml]
  ext_files << "java" if java_p
  lib_files = %w[rb]
  lib_files << "jar" if java_p
  spec.files = [
    *Dir["ext/**/*.{#{ext_files.join(",")}}"], *Dir["lib/**/*.{#{lib_files.join(",")}}"], *Dir["sig/**/*"],
    "Cargo.lock", "Cargo.toml"
  ].reject { |f| File.directory?(f) }
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/xml2json/extconf.rb"]

  # Uncomment to register a new dependency of your gem
  # spec.add_dependency "example-gem", "~> 1.0"
  spec.add_development_dependency "rake", "~> 13.0"

  spec.add_development_dependency "rake-compiler"
  # TODO: remove in binary gems
  spec.add_dependency "rb_sys", "~> 0.9.63"

  spec.add_development_dependency "rspec", "~> 3.0"

  spec.add_development_dependency "rubocop", "~> 1.21"

  spec.add_development_dependency "yard"
  # # Cause I want to use rack adapter
  spec.add_development_dependency "rack"
  spec.add_development_dependency "rackup"
  # # Cause I want to use rack adapter with puma server
  spec.add_development_dependency "puma"
  # # For version validation
  spec.add_development_dependency "toml-rb"

  # For more information and examples about making a new gem, check out our
  # guide at: https://bundler.io/guides/creating_gem.html
end
