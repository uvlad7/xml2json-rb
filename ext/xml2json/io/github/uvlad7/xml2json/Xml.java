package io.github.uvlad7.xml2json;

import jnr.ffi.LibraryLoader;
import org.jruby.*;
import org.jruby.anno.JRubyMethod;
import org.jruby.runtime.ThreadContext;
import org.jruby.runtime.builtin.IRubyObject;

@SuppressWarnings("serial")
public class Xml extends RubyObject {

    private static native String buildImpl(String input);

    static {
        System.load(Xml2JsonService.libPath);
    }

    private final Ruby ruby;

    public Xml(final Ruby ruby, RubyClass rubyClass) {
        super(ruby, rubyClass);
        this.ruby = ruby;
    }

    @JRubyMethod
    public IRubyObject build(ThreadContext context, RubyString json_s) {
        return RubyString.newString(ruby, Xml.buildImpl(json_s.asJavaString()));
    }
}