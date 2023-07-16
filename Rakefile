# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

# so it's a file task, it works well as a prerequisite, but it always is executed
yarn_task = file "spec/lib/" do |task|
  cd task.name do
    sh "yarn install"
  end
end

def yarn_task.needed?
  true
end

task yarn: "spec/lib/"

require_relative "spec/examples"

file "spec/examples.json" => "spec/lib/" do |task|
  File.write task.name, JSON.pretty_generate(Examples.new.to_h)
  # task.instance_eval do
  #   # copied from out_of_date? method
  #   p timestamp
  #   p(all_prerequisite_tasks.map do |prereq|
  #     prereq_task = application[prereq, @scope]
  #     [prereq_task.timestamp, prereq_task.name]
  #   end)
  # end
end

RSpec::Core::RakeTask.new(:spec)

task spec: "spec/examples.json"

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

task default: %i[compile spec rubocop yard]

require "rake/clean"

CLEAN.concat(Dir["**/ruby_node_task*"])
