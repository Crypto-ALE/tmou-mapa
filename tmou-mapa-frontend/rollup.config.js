import typescript from '@rollup/plugin-typescript';
import serve from 'rollup-plugin-serve';
import nodeResolve from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import copy from 'rollup-plugin-copy'

export default {
    input: 'src/index.ts',
    output: [
        {
            file: '../tmou-mapa-backend/static/index.js',
            format: 'cjs',
            sourcemap: true
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
        serve('.'),
    ]
};
