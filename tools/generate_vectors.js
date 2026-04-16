#!/usr/bin/env node
/**
 * Generate test vectors from the reference blockies implementation.
 * This extracts the exact algorithm from Ratspeak's identity.js and
 * produces vectors.json for cross-language compatibility testing.
 */

// --- Reference implementation (verbatim from ratspeak-rs/dashboard/static/js/identity.js) ---

function seedRand(seed) {
    var s = [0, 0, 0, 0];
    for (var i = 0; i < seed.length; i++) {
        s[i % 4] = (s[i % 4] << 5) - s[i % 4] + seed.charCodeAt(i);
    }
    // Capture seeded state before any stepping
    var seededState = [s[0] | 0, s[1] | 0, s[2] | 0, s[3] | 0];
    var rand = function() {
        var t = s[0] ^ (s[0] << 11);
        s[0] = s[1]; s[1] = s[2]; s[2] = s[3];
        s[3] = (s[3] ^ (s[3] >> 19) ^ t ^ (t >> 8)) >>> 0;
        return s[3] / ((1 << 31) >>> 0);
    };
    rand._seededState = seededState;
    return rand;
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

function generateSvg(seed, svgSize, clipId) {
    var gridSize = 8;
    var rand = seedRand(seed || '');
    var seededState = rand._seededState;
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
    var svg = '<svg xmlns="http://www.w3.org/2000/svg" width="' + svgSize +
        '" height="' + svgSize + '" viewBox="0 0 ' + gridSize + ' ' + gridSize +
        '" shape-rendering="crispEdges">' +
        '<clipPath id="' + clipId + '"><circle cx="' + r + '" cy="' + r + '" r="' + r + '"/></clipPath>' +
        '<g clip-path="url(#' + clipId + ')">' + rects + '</g></svg>';

    return {
        seeded_state: seededState,
        color: color,
        bgcolor: bgcolor,
        spotcolor: spotcolor,
        grid: grid,
        svg: svg
    };
}

// --- Generate vectors ---
// All seeds are 32-char lowercase hex strings — the exact format of real LXMF
// destination hashes (hex::encode of 16-byte Reticulum destination hash).

var seeds = [
    // Boundary cases
    "00000000000000000000000000000000",
    "ffffffffffffffffffffffffffffffff",
    // Sequential pattern
    "0123456789abcdef0123456789abcdef",
    // From LXMF repo documentation
    "6b3362bd2c1dbf87b66a85f79a8d8c75",
    // Realistic random LXMF hashes (crypto.randomBytes(16).toString('hex'))
    "a7b3c9d1e5f20681943ab2de77fc8e01",
    "c65f8642f4ff9a4de0a088851f6f1f12",
    "39dd37d6418cd2eea2bd4e5d90a74e80",
    "6a50d06c53cb252c0677b5e90ebee052",
    "930a901c829697ee9b8c5cb452e24a0e",
    "8c06b2f01b0756dc2f5508db5bbfad65",
    "e860b957685fee0b4505225f7ce5df08",
    "607986ae656ebdd48c656b46009ff372",
    "ce1e23f02ef7883d2280c1fd3b715e44",
    "b001ea40876ff667a5cc1d36bd402680",
    "b512663c2edd60689c6d180b9f7f969f",
    "327714e89df5cdd722e2dbdbe6e2fe32",
    "12bcf7b44fe3a174558259725023d27a",
    "8caea14ce1c0785bef6c81f80353be37",
    "ae52e55a2153f796b847cd1c72d08ba3",
    "df308ab4abb377e2461d919f15f52183",
    "838a08b6416598946f195962abefd7dc",
    "c2c0f2d7ebaabca84ba0717db9d60579",
    "653dee7007462d7c2dfb0fc74c42c1c9",
    "2b5b259e1e2270c0f2202334635f61ed",
];

var vectors = [];
for (var i = 0; i < seeds.length; i++) {
    var seed = seeds[i];
    var result = generateSvg(seed, 64, "lxmface-clip");
    vectors.push({
        seed: seed,
        size: 64,
        seeded_state: result.seeded_state,
        color: result.color,
        bgcolor: result.bgcolor,
        spotcolor: result.spotcolor,
        grid: result.grid,
        svg: result.svg
    });
}

var json = JSON.stringify({ vectors: vectors }, null, 2);

var fs = require('fs');
var path = require('path');
var outPath = path.join(__dirname, '..', 'tests', 'vectors.json');
fs.writeFileSync(outPath, json + '\n');
console.log('Wrote ' + vectors.length + ' vectors to ' + outPath);
