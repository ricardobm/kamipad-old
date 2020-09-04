const webpack = require('webpack')
const path = require('path')

const CopyPlugin = require('copy-webpack-plugin')
const HtmlPlugin = require('html-webpack-plugin')
const TSLintPlugin = require('tslint-webpack-plugin')

const DIST = path.resolve(__dirname, 'dist')

const config = {
	entry: [
		'./src/index.ts'
	],
	output: {
		path: DIST,
		filename: 'bundle.js',
	},
	module: {
		rules: [
			{
				test: /\.ts(x)?$/,
				use: ['awesome-typescript-loader'],
				exclude: /node_modules/,
			},
			{
				test: /\.scss$/,
				use: [
					'style-loader',
					'css-loader',
					'sass-loader',
				],
			},
		],
	},
	resolve: {
		extensions: [
			'.tsx',
			'.ts',
			'.js',
		],
	},
	plugins: [
		new CopyPlugin({
			patterns: [
				{ from: 'static' },
			],
		}),
		new HtmlPlugin({
			template: 'src/main.html',
		}),
		new TSLintPlugin({ files: ['./src/**/*.ts'] }),
	],
	devServer: {
		contentBase: './dist'
	},
}

module.exports = config
