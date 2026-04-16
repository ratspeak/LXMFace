#!/usr/bin/env node
/**
 * Verify the JS lxmface library produces output identical to test vectors.
 * Run: node tests/compatibility.js
 */

var fs = require('fs');
var path = require('path');
var lib = require(path.join(__dirname, '..', 'js', 'lxmface.js'));

var data = fs.readFileSync(path.join(__dirname, 'vectors.json'), 'utf8');
var vectors = JSON.parse(data).vectors;

var pass = 0;
var fail = 0;

vectors.forEach(function(vec) {
    var svg = lib.lxmface(vec.seed, vec.size);
    if (svg === vec.svg) {
        pass++;
    } else {
        fail++;
        console.error('FAIL: seed=' + JSON.stringify(vec.seed));
        for (var i = 0; i < Math.max(svg.length, vec.svg.length); i++) {
            if (svg[i] !== vec.svg[i]) {
                console.error('  first diff at position ' + i);
                console.error('  got:      ' + JSON.stringify(svg.substring(i, i + 40)));
                console.error('  expected: ' + JSON.stringify(vec.svg.substring(i, i + 40)));
                break;
            }
        }
    }
});

console.log(pass + ' passed, ' + fail + ' failed');
if (fail > 0) {
    process.exit(1);
}
