package io.github.uvlad7.xml2json;

import org.jruby.*;
import org.jruby.runtime.load.BasicLibraryService;

public class Xml2JsonService implements BasicLibraryService {
    public static void systemLoad(String libPath) {
        System.load(libPath);
    }

    @Override
    public native boolean basicLoad(final Ruby ruby);
}
