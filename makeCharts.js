const output = require("d3node-output");
const d3nLine = require("d3node-linechart");
const fs = require("fs");
const _ = require("lodash/fp");

const perfData = JSON.parse(fs.readFileSync("./perfData.json"));

const prepareData = (lang, type) =>
  _.flow(
    _.pickBy((v, k) => k.includes(lang)),
    _.pickBy((v, k) => k.includes(type)),
    // average the results
    _.mapValues((v, k) => {
      return v.reduce((a, b) => a + b, 0) / v.length;
    }),
    // map to seconds
    _.mapValues((v) => v / 1000),
    _.mapKeys((k) => k.split("::")[2]),
    Object.entries,
    _.map(([k, v]) => ({ key: k, value: v }))
  );

const bigWasm = prepareData("wasm", "many")(perfData);
const littleWasm = prepareData("wasm", "single")(perfData);
const bigJs = prepareData("js", "many")(perfData);
const littleJs = prepareData("js", "single")(perfData);
const bigJsFunction = prepareData("jsFn", "many")(perfData);
const littleJsFunction = prepareData("jsFn", "single")(perfData);

const manyData = [bigWasm, bigJs, bigJsFunction];
manyData.allKeys = bigWasm.map((d) => d.key);
const singleData = [littleWasm, littleJs, littleJsFunction];
singleData.allKeys = littleWasm.map((d) => d.key);

// create output files
output(
  "./charts/manyRules",
  d3nLine({
    data: manyData,
    container: `<div id="container"><h2>Many Rules</h2><div id="chart"></div></div>`,
    lineColors: ["steelblue", "darkorange", "green"],
    width: 1000,
    height: 800,
    isCurve: false,
  }),
  {}
);

output(
  "./charts/singleRule",
  d3nLine({
    data: singleData,
    container: `<div id="container"><h2>SingleRule</h2><div id="chart"></div></div>`,
    lineColors: ["steelblue", "darkorange", "green"],
    width: 1000,
    height: 800,
    isCurve: false,
  }),
  {}
);
