package io.github.uvlad7.xml2json;

import io.github.uvlad7.xml2json.Xml;
import org.jruby.*;
import org.jruby.runtime.load.BasicLibraryService;

public class Xml2JsonService implements BasicLibraryService {
    public static String libPath = null;

    @Override
    public boolean basicLoad(final Ruby ruby) {
        RubyModule module = ruby.defineModule("Xml2Json");
        RubyModule xml = module.defineModuleUnder("Xml");
        xml.defineAnnotatedMethods(Xml.class);
        return true;
    }
}

