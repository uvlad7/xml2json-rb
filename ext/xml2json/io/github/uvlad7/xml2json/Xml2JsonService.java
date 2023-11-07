package io.github.uvlad7.xml2json;

import io.github.uvlad7.xml2json.Xml;
import org.jruby.*;
import org.jruby.runtime.load.BasicLibraryService;

public class Xml2JsonService implements BasicLibraryService {

    @Override
    public boolean basicLoad(Ruby ruby) {
        RubyModule module = ruby.defineModule("Xml2Json");
        RubyClass xml = module.defineClassUnder("Xml", ruby.getObject(), Xml::new);
        xml.defineAnnotatedMethods(Xml.class);
        return true;
    }
}

