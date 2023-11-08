# rubocop:disable all

require "mkmf"
require "rb_sys/mkmf"

# @private
class RbSys::CargoBuilder
  def cargo_command(dest_path, args = [])
    cmd = []
    cmd += ["cargo", "rustc"]
    cmd += ["--target", target] if target
    cmd += ["--target-dir", dest_path]
    cmd += ["--features", features.join(",")] unless features.empty?
    cmd += ["--lib"]
    cmd += ["--profile", profile.to_s]
    cmd += Gem::Command.build_args
    # cmd += args -- rake --trace fails
    cmd += ["--"]
    cmd += [*rustc_args(dest_path)]
    cmd += extra_rustc_args
    cmd
  end

  private

  def config(var_name)
    RbConfig::CONFIG[var_name]
  end

  def msvc_target?
    config("target_os").include?("msvc")
  end

  def darwin_target?
    config("target_os").include?("darwin")
  end

  def mingw_target?
    config("target_os").include?("mingw")
  end

  # def mswin_link_args
  # Not sure if LIBRUBYARG_SHARED and LOCAL_LIBS work on mswin
  #   args = []
  #   args += ["-l", makefile_config("LIBRUBYARG_SHARED").chomp(".lib")]
  #   args += split_flags("LIBS").flat_map { |lib| ["-l", lib.chomp(".lib")] }
  #   args += split_flags("LOCAL_LIBS").flat_map { |lib| ["-l", lib.chomp(".lib")] }
  #   args
  # end
end

# rubocop:enable all
