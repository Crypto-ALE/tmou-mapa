import typescript from '@rollup/plugin-typescript';
import nodeResolve from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import copy from 'rollup-plugin-copy';
import { terser } from 'rollup-plugin-terser';

export default [{
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
                { src: 'not_started.html', dest: '../tmou-mapa-backend/templates/', rename: 'not_started.html.tera' },
            ],
        }),
        typescript(),
        nodeResolve({browser: true}),
        commonjs(),
        terser(),
    ],
},{
    input: 'src/admin.ts',
    external: [ 'leaflet' ],
    output: [
        {
            file: '../tmou-mapa-backend/static/admin.js',
            format: 'iife',
            sourcemap: true,
            globals: { leaflet: 'L' },
        }],
    plugins: [
        copy({
            targets: [
                { src: 'admin.html', dest: '../tmou-mapa-backend/templates/', rename: 'admin.html.tera' },
            ],
        }),
        typescript(),
        nodeResolve({browser: true}),
        commonjs(),
        terser(),
    ],
}
];
