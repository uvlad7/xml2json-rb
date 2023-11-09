package io.github.uvlad7.xml2json;

import jnr.ffi.LibraryLoader;
import org.jruby.*;
import org.jruby.anno.JRubyMethod;
import org.jruby.runtime.ThreadContext;
import org.jruby.runtime.builtin.IRubyObject;

@SuppressWarnings("serial")
public class Xml extends RubyObject {
    public interface LibC { // A representation of libC in Java
        int puts(String s); // mapping of the puts function, in C `int puts(const char *s);`
    }

    public static native float nativeGetJniVersion();

    public static float getJniVersion() {
        System.out.println(1.4f);
        return 1.4f;
    }

    private static native String buildImpl(String input);

    static {
        System.load(Xml2JsonService.libPath);
    }

    LibC libc = LibraryLoader.create(LibC.class).load("c");

    private final Ruby ruby;

    public Xml(final Ruby ruby, RubyClass rubyClass) {
        super(ruby, rubyClass);
        this.ruby = ruby;
    }

    @JRubyMethod
    public IRubyObject build(ThreadContext context, RubyString json_s) {
        return RubyString.newString(ruby, Xml.buildImpl(json_s.asJavaString()));
    }

    @JRubyMethod
    public IRubyObject hello_c(ThreadContext context) {
        return RubyFixnum.newFixnum(ruby, libc.puts("Hello C!"));
    }

    @JRubyMethod
    public void hello_java(ThreadContext context) {
        System.out.println("Hello Java!");
    }
}