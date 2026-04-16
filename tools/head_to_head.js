#!/usr/bin/env node
/**
 * Head-to-head test: Ratspeak's blockies vs LXMFace.
 *
 * Runs both the original Ratspeak blockies (copied verbatim from identity.js)
 * and the LXMFace library against 100 random LXMF-format hashes.
 * Verifies identical output (modulo clip ID).
 */

var crypto = require('crypto');
var path = require('path');
var lxmface = require(path.join(__dirname, '..', 'js', 'lxmface.js')).lxmface;

// --- Original Ratspeak blockies (verbatim from identity.js lines 64-133) ---

var blockies = (function() {
    function seedRand(seed) {
        var s = [0, 0, 0, 0];
        for (var i = 0; i < seed.length; i++) {
            s[i % 4] = (s[i % 4] << 5) - s[i % 4] + seed.charCodeAt(i);
        }
        return function() {
            var t = s[0] ^ (s[0] << 11);
            s[0] = s[1]; s[1] = s[2]; s[2] = s[3];
            s[3] = (s[3] ^ (s[3] >> 19) ^ t ^ (t >> 8)) >>> 0;
            return s[3] / ((1 << 31) >>> 0);
        };
    }

    function createColor(rand) {
        var h = Math.floor(rand() * 360);
        var s = ((rand() * 60) + 40);
        var l = ((rand() + rand() + rand() + rand()) * 25);
        return 'hsl(' + h + ',' + s + '%,' + l + '%)';
    }

    function createImageData(rand, gridSize) {
        var w = gridSize, h = gridSize;
        var halfW = Math.ceil(w / 2);
        var data = [];
        for (var y = 0; y < h; y++) {
            var row = [];
            for (var x = 0; x < halfW; x++) {
                row.push(Math.floor(rand() * 2.3));
            }
            var fullRow = row.slice();
            for (var x = Math.floor(w / 2) - 1; x >= 0; x--) {
                fullRow.push(row[x]);
            }
            data.push(fullRow);
        }
        return data;
    }

    var fn = function(seed, svgSize) {
        var gridSize = 8;
        var rand = seedRand(seed || '');
        var color = createColor(rand);
        var bgcolor = createColor(rand);
        var spotcolor = createColor(rand);
        var grid = createImageData(rand, gridSize);

        var rects = '';
        for (var y = 0; y < gridSize; y++) {
            for (var x = 0; x < gridSize; x++) {
                var val = grid[y][x];
                var fill = val === 0 ? bgcolor : val === 1 ? color : spotcolor;
                rects += '<rect x="' + x + '" y="' + y + '" width="1" height="1" fill="' + fill + '"/>';
            }
        }

        var r = gridSize / 2;
        var clipId = 'bc' + (++fn._uid);
        return '<svg xmlns="http://www.w3.org/2000/svg" width="' + svgSize +
            '" height="' + svgSize + '" viewBox="0 0 ' + gridSize + ' ' + gridSize +
            '" shape-rendering="crispEdges">' +
            '<clipPath id="' + clipId + '"><circle cx="' + r + '" cy="' + r + '" r="' + r + '"/></clipPath>' +
            '<g clip-path="url(#' + clipId + ')">' + rects + '</g></svg>';
    };
    fn._uid = 0;
    return fn;
})();

// --- Run test ---

var COUNT = 100;
var pass = 0;
var fail = 0;
var sizes = [20, 32, 40, 64];

for (var i = 0; i < COUNT; i++) {
    var hash = crypto.randomBytes(16).toString('hex');
    var size = sizes[i % sizes.length];

    var ratspeak = blockies(hash, size);
    var lib = lxmface(hash, size);

    // Normalize clip IDs for comparison
    var ratspeakNorm = ratspeak.replace(/bc\d+/g, 'CLIP');
    var libNorm = lib.replace(/lxmface-clip/g, 'CLIP');

    if (ratspeakNorm === libNorm) {
        pass++;
    } else {
        fail++;
        console.error('FAIL: hash=' + hash + ' size=' + size);
        for (var j = 0; j < Math.max(ratspeakNorm.length, libNorm.length); j++) {
            if (ratspeakNorm[j] !== libNorm[j]) {
                console.error('  first diff at position ' + j);
                console.error('  ratspeak: ' + JSON.stringify(ratspeakNorm.substring(j, j + 40)));
                console.error('  lxmface:  ' + JSON.stringify(libNorm.substring(j, j + 40)));
                break;
            }
        }
    }
}

console.log(pass + '/' + COUNT + ' random LXMF hashes match (clip ID normalized, sizes: ' + sizes.join('/') + 'px)');
if (fail > 0) {
    process.exit(1);
}
