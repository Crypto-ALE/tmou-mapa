import typescript from '@rollup/plugin-typescript';
import nodeResolve from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import copy from 'rollup-plugin-copy';
import { terser } from 'rollup-plugin-terser';

export default {
    input: 'src/index.ts',
    external: [ 'leaflet' ],
    output: [
        {
            file: '../tmou-mapa-backend/static/index.js',
            format: 'iife',
            sourcemap: true,
            globals: { leaflet: 'L' },
        }],
    plugins: [
        copy({
            targets: [
                { src: 'static/tmou.css', dest: '../tmou-mapa-backend/static/' },
                { src: 'index.html', dest: '../tmou-mapa-backend/templates/', rename: 'index.html.tera' },
            ],
        }),
        typescript(),
        nodeResolve({browser: true}),
        commonjs(),
        terser(),
    ],
};
