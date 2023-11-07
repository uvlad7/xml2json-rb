package io.github.uvlad7.xml2json;

import org.jruby.*;
import org.jruby.anno.JRubyMethod;
import org.jruby.runtime.ThreadContext;
import org.jruby.runtime.builtin.IRubyObject;
import org.jruby.runtime.load.BasicLibraryService;
import jnr.ffi.LibraryLoader;

public class Xml2JsonService implements BasicLibraryService {

    @Override
    public boolean basicLoad(Ruby ruby) {
        RubyModule module = ruby.defineModule("Xml2Json");
        RubyClass xml = module.defineClassUnder("Xml", ruby.getObject(), Xml::new);
        xml.defineAnnotatedMethods(Xml.class);
        return true;
    }
}

@SuppressWarnings("serial")
class Xml extends RubyObject {
    public interface LibC { // A representation of libC in Java
        int puts(String s); // mapping of the puts function, in C `int puts(const char *s);`
    }

    LibC libc = LibraryLoader.create(LibC.class).load("c");

    private final Ruby ruby;

    public Xml(final Ruby ruby, RubyClass rubyClass) {
        super(ruby, rubyClass);
        this.ruby = ruby;
    }

    @JRubyMethod
    public IRubyObject build(ThreadContext context, RubyString json_s) {
        return RubyString.newString(ruby, json_s.asJavaString().toLowerCase());
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
