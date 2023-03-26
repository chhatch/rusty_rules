const { wasm_rules } = require("./pkg");
const { ruleFactory } = require("@elite-libs/rules-machine");
const { performance, PerformanceObserver } = require("node:perf_hooks");
const { pick } = require("lodash");
const fs = require("fs");

const perfData = JSON.parse(fs.readFileSync("./perfData.json"));

for (let i = 0; i < 5; i++) {
  const iterations = 10 ** i;
  benchmark(iterations);
}

fs.writeFileSync("./perfData.json", JSON.stringify(perfData, null, 2));

function benchmark(iterations) {
  const roteOperation = {
    if: { and: ["foo == foo ", "foo == foo"] },
    then: "bar = bar + 1",
    else: "bar = bar - 1",
  };

  const returnObj = { return: 'foo, bar, "hello wasm!"' };

  const context = { foo: 1, bar: 1 };
  const contextString = JSON.stringify(context);

  const bigWasmLabel = `wasm::many::${iterations}`;
  const littleWasmLabel = `wasm::single::${iterations}`;
  const bigJsLabel = `js::many::${iterations}`;
  const littleJsLabel = `js::single::${iterations}`;
  const bigJsFunctionLabel = `jsFn::many::${iterations}`;
  const littleJsFunctionLabel = `jsFn::single::${iterations}`;

  const bigJsonRules = [];
  for (let i = 0; i < iterations; i++) {
    bigJsonRules.push(roteOperation);
  }
  bigJsonRules.push(returnObj);

  const littleJsonRules = [roteOperation, returnObj];

  const littleRules = JSON.stringify(littleJsonRules);
  const bigRules = JSON.stringify(bigJsonRules);

  const { bigJsFunction, littleJsFunction } = buildFunctionRules(iterations);

  const bigWasmFn = () => JSON.parse(wasm_rules(bigRules, contextString));

  const littleWasmFn = runTimes(
    () => JSON.parse(wasm_rules(littleRules, contextString)),
    iterations
  );

  const bigJsFn = () => {
    const rulesEngine = ruleFactory(bigJsonRules);
    rulesEngine(context);
  };

  const littleJsFn = runTimes(() => {
    const rulesEngine = ruleFactory(littleJsonRules);
    rulesEngine(context);
  }, iterations);

  const bigJsFunctionFn = () => bigJsFunction(1, 1);
  const littleJsFunctionFn = runTimes(() => littleJsFunction(1, 1), iterations);

  runAndMeasure(bigWasmLabel, bigWasmFn);
  runAndMeasure(bigJsFunctionLabel, bigJsFunctionFn);
  runAndMeasure(bigJsLabel, bigJsFn);

  //   runAndMeasure(littleWasmLabel, littleWasmFn);
  //   runAndMeasure(littleJsFunctionLabel, littleJsFunctionFn);
  //   runAndMeasure(littleJsLabel, littleJsFn);
  //   logMeasurements();
  // addSpace();
  addPerfData();
  clearMeasurements();
}

function runAndMeasure(label, fn) {
  performance.mark(label);
  fn();
  performance.measure(label, label);
}

function addPerfData() {
  performance.getEntriesByType("measure").forEach(({ name, duration }) => {
    if (!perfData[name]) {
      perfData[name] = [];
    }
    perfData[name].push(duration);
  });
}

function logMeasurements() {
  console.table(
    performance
      .getEntriesByType("measure")
      .map((x) => pick(x, ["name", "duration"]))
      .map((x) => {
        x.duration = x.duration.toFixed(2);
        return x;
      })
  );
}

function clearMeasurements() {
  performance.clearMarks();
  performance.clearMeasures();
}

function addSpace() {
  console.log(`

`);
}

function runTimes(fn, times) {
  return () => {
    for (let i = 0; i < times; i++) {
      fn();
    }
  };
}

function buildFunctionRules(iterations) {
  const roteOperation = (x) =>
    `if (foo <= foo && bar > ${x}) { bar = bar + 1 } else { bar = bar - 1 };`;
  const returnOperation = "return [ foo, bar, 'hello wasm!' ];";

  const bigJsFunctionRules = [];
  for (let i = 0; i < iterations; i++) {
    bigJsFunctionRules.push(roteOperation(i));
  }
  bigJsFunctionRules.push(returnOperation);

  const littleJsFunctionRules = `${roteOperation(1)}${returnOperation}`;

  const bigJsFunction = Function("foo", "bar", bigJsFunctionRules.join(""));
  const littleJsFunction = Function("foo", "bar", littleJsFunctionRules);
  return { bigJsFunction, littleJsFunction };
}
