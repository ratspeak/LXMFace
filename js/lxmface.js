/**
 * LXMFace — Deterministic identicon generator for LXMF/Reticulum addresses.
 * Algorithm adapted from https://github.com/download13/blockies (MIT license).
 *
 * @param {string} seed - Hash string (typically an LXMF hex address)
 * @param {number} size - SVG width/height in pixels
 * @returns {string} Self-contained SVG string
 */
(function(root) {
    'use strict';

    // Xorshift128 PRNG seeded from a string
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
            // mirror
            var fullRow = row.slice();
            for (var x = Math.floor(w / 2) - 1; x >= 0; x--) {
                fullRow.push(row[x]);
            }
            data.push(fullRow);
        }
        return data;
    }

    function lxmface(seed, size) {
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
        return '<svg xmlns="http://www.w3.org/2000/svg" width="' + size +
            '" height="' + size + '" viewBox="0 0 ' + gridSize + ' ' + gridSize +
            '" shape-rendering="crispEdges">' +
            '<clipPath id="lxmface-clip"><circle cx="' + r + '" cy="' + r + '" r="' + r + '"/></clipPath>' +
            '<g clip-path="url(#lxmface-clip)">' + rects + '</g></svg>';
    }

    // Export for various module systems
    if (typeof module !== 'undefined' && module.exports) {
        module.exports = { lxmface: lxmface };
        module.exports.default = lxmface;
    }
    if (typeof root !== 'undefined') {
        root.lxmface = lxmface;
    }
    if (typeof define === 'function' && define.amd) {
        define(function() { return { lxmface: lxmface }; });
    }

})(typeof globalThis !== 'undefined' ? globalThis : typeof self !== 'undefined' ? self : this);
