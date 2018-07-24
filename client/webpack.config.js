const ForkTsCheckerPlugin = require('fork-ts-checker-webpack-plugin');
const HtmlPlugin = require('html-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
const webpack = require('webpack');

module.exports = {
	module: {
		rules: [
			{
				test: /\.tsx?$/,
				exclude: /node_modules/,
				use: {
					loader: 'ts-loader',
					options: {
						transpileOnly: true
					}
				}
			},
			{
				test: /\.scss$/,
				exclude: /node_modules/,
				use: [
					{
						loader: MiniCssExtractPlugin.loader
					},
					"css-loader",
					"sass-loader"
				]
			}
		]
	},
	resolve: {
		extensions: [".ts", ".tsx", ".js", ".jsx"]
	},
	plugins: [
		new ForkTsCheckerPlugin(),
		new HtmlPlugin(),
		new webpack.DefinePlugin({
			API_HOST: JSON.stringify(process.env.API_HOST || "/api")
		}),
		new MiniCssExtractPlugin({
			filename: "[hash].css"
		})
	],
	entry: './src/main.tsx',
	output: {
		publicPath: "/"
	},
	mode: process.env.NODE_ENV || "development",
	devServer: {
		historyApiFallback: true
	}
};
