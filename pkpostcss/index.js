var bench = require('nanobench')
var postcss = require('postcss');
var fs = require('fs');
const path = require('path');

const css = fs.readFileSync(path.join(__dirname, '../test.css'))

bench('sha256 200.000 times', function (b) {


    b.start()

    postcss.parse(css)
    b.end()
})
