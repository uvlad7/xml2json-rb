package io.github.uvlad7.xml2json;

import org.jruby.*;
import org.jruby.anno.JRubyMethod;
import org.jruby.anno.JRubyModule;
import org.jruby.runtime.ThreadContext;
import org.jruby.runtime.builtin.IRubyObject;

@SuppressWarnings("serial")
@JRubyModule(name = "Xml")
public class Xml {

    private static native String buildNative(String input);

    static {
        System.load(Xml2JsonService.libPath);
    }

    //    https://github.com/jruby/jruby/wiki/JRubyMethod_Signatures
    @JRubyMethod(name = "build", module = true)
    public static IRubyObject build(ThreadContext context, IRubyObject self, RubyString json_s) {
        return RubyString.newString(context.getRuntime(), Xml.buildNative(json_s.asJavaString()));
    }
}