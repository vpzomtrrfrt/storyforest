const express = require('express');
const expressProxy = require('express-http-proxy');
const historyFallback = require('connect-history-api-fallback');
const webpackDevMW = require('webpack-dev-middleware');
const webpack = require('webpack');

const compiler = webpack(require('./webpack.config'));

const app = express();
app.use('/api', expressProxy(process.env.API_PROXY_HOST || 'localhost:8000'));

app.use(historyFallback());

app.use(webpackDevMW(compiler));

app.listen(process.env.PORT || 8080);
