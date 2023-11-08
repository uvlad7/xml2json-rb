# frozen_string_literal: true

require "bundler/setup"

require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

require "rubocop/rake_task"

RuboCop::RakeTask.new

task build: :compile

gemspec = Gem::Specification.load("xml2json.gemspec")

if RUBY_PLATFORM.include?("java_skip_this_now")
  # Run rb_sys task and mv tmp/java/xml2json/3.1.4/xml2json.jar ./xml2json_java.so
  require "rake/javaextensiontask"

  Rake::JavaExtensionTask.new("xml2json", gemspec) do |ext|
    ext.lib_dir = "lib/xml2json"
    ext.source_version = "11" # or "8"
    ext.target_version = "11" # or "8"
  end
else
  require "rb_sys/extensiontask"

  RbSys::ExtensionTask.new("xml2json", gemspec) do |ext|
    ext.lib_dir = "lib/xml2json"
    ext.cross_compile = true
  end
end

task :clippy do
  # sh "cargo clippy -- -D warnings"
end

require "yard"

YARD::Rake::YardocTask.new do |t|
  # t.files = gemspec.files.select { |f| %w[.rb .json].include? File.extname(f) }
  t.options += ["--output-dir", ENV["YARD_OUTPUT_DIR"]] if ENV["YARD_OUTPUT_DIR"]
end

task default: %i[compile spec clippy rubocop yard]
