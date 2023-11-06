# rubocop:disable

require "mkmf"
require "rb_sys/mkmf"

class RbSys::CargoBuilder
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

# rubocop:enable
