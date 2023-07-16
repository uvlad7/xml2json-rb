const xml2js = require("xml2js");

module.exports = function (opts, done) {
    let parser = opts.options ? new xml2js.Parser(opts.options) : new xml2js.Parser();
    parser.parseString(opts.data, function (err, result) {
        if (err) {
            done(err, null);
            return;
        }
        done(null, JSON.stringify(result));
    });
}
