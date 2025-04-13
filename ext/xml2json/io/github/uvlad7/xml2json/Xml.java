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

    // https://github.com/jruby/jruby/wiki/JRubyMethod_Signatures
    // https://github.com/jruby/jruby/wiki/Method-Signatures-and-Annotations-in-JRuby-extensions
    @JRubyMethod(name = "build", meta = true)
    public static IRubyObject build(ThreadContext context, IRubyObject self, RubyString json_s, IRubyObject opts) {
        // opts.convertToHash();
        return RubyString.newString(context.getRuntime(), Xml.buildNative(json_s.asJavaString()));
    }

    @JRubyMethod(name = "build", meta = true)
    public static IRubyObject build(ThreadContext context, IRubyObject self, RubyString json_s) {
//        throw context.getRuntime().newArgumentError(new IRubyObject[]{}.length, 1, 2);
        // https://github.com/jruby/jruby/issues/6203#issuecomment-624351412
        // throw RaiseException.from(context.getRuntime(), context.getRuntime().getFatal(), "fatal");
        return Xml.build(context, self, json_s, null);
    }
}
