# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

require "rubocop/rake_task"

RuboCop::RakeTask.new

require "rb_sys/extensiontask"

task build: :compile

RbSys::ExtensionTask.new("xml2json") do |ext|
  ext.lib_dir = "lib/xml2json"
end

require "yard"

YARD::Rake::YardocTask.new do |t|
  t.files = ["lib/**/*.rb"]
end

require_relative "yard-ghpages/yard-ghpages"
Yard::GHPages::Tasks.install_tasks

task default: %i[compile spec rubocop yard]
