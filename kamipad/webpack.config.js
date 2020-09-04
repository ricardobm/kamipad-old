const webpack = require('webpack')
const path = require('path')

const TSLintPlugin = require('tslint-webpack-plugin')
const HtmlWebpackPlugin = require('html-webpack-plugin')

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
		new TSLintPlugin({ files: ['./src/**/*.ts'] }),
		new HtmlWebpackPlugin({
			template: 'src/main.html',
		}),
	],
	devServer: {
		contentBase: './dist'
	},
}

module.exports = config
