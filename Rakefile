# frozen_string_literal: true

require "bundler/setup"

require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

require "rubocop/rake_task"

RuboCop::RakeTask.new

require "rb_sys/extensiontask"

task build: :compile

gemspec = Gem::Specification.load("xml2json.gemspec")

RbSys::ExtensionTask.new("xml2json", gemspec) do |ext|
  ext.lib_dir = "lib/xml2json"
end

task :clippy do
  sh "cargo clippy -- -D warnings"
end

require "yard"

YARD::Rake::YardocTask.new do |t|
  # t.files = gemspec.files.select { |f| %w[.rb .json].include? File.extname(f) }
  t.options += ["--output-dir", ENV["YARD_OUTPUT_DIR"]] if ENV["YARD_OUTPUT_DIR"]
end

task default: %i[compile spec clippy rubocop yard]
