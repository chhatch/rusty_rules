const { wasm_rules } = require("./pkg");
const { ruleFactory } = require("@elite-libs/rules-machine");
const { performance, PerformanceObserver } = require("node:perf_hooks");
const { pick } = require("lodash");
const fs = require("fs");

const perfData = JSON.parse(fs.readFileSync("./perfData.json"));

// for (let i = 0; i < 5; i++) {
const iterations = 10 ** 5 * 4;
benchmark(iterations);
// }

fs.writeFileSync("./perfData.json", JSON.stringify(perfData, null, 2));

function benchmark(iterations) {
  const roteOperation = {
    if: {
      and: [
        "userId == 1",
        "cartId == IF(cartId != 0, 1, cartId)",
        'supplier == IF(supplier != "", "walmart", supplier)',
        'couponCode == IF(couponCode != "", "1234", couponCode)',
        'couponType == IF(couponType != "", "promotion", couponType)',
        'brand == IF(brand != "", "nike", brand)',
      ],
    },
    then: ["tax = tax + 1", 'source = "1998"'],
  };
  const returnObj = { return: "tax, source" };

  const getContext = () => ({
    userId: 1,
    cartId: 1,
    supplier: "walmart",
    couponCode: "1234",
    couponType: "promotion",
    brand: "nike",
    tax: 1,
    source: "mobile",
  });
  const contextString = JSON.stringify(getContext());

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
    rulesEngine(getContext());
  };

  var context = getContext();
  const littleJsFn = runTimes(() => {
    const rulesEngine = ruleFactory(littleJsonRules);
    rulesEngine(context);
  }, iterations);

  const bigJsFunctionFn = () => bigJsFunction(getContext());
  const littleJsFunctionFn = runTimes(
    () => littleJsFunction(getContext()),
    iterations
  );

  runAndMeasure(littleWasmLabel, littleWasmFn);
  runAndMeasure(littleJsFunctionLabel, littleJsFunctionFn);
  runAndMeasure(littleJsLabel, littleJsFn);

  runAndMeasure(bigWasmLabel, bigWasmFn);
  runAndMeasure(bigJsFunctionLabel, bigJsFunctionFn);
  runAndMeasure(bigJsLabel, bigJsFn);
  logMeasurements();
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
    `if (  userId == 1 &&  (!cartId || cartId == 1) &&  (supplier != "" || supplier == "walmart") &&  (couponCode != "" || couponCode == "1234") &&  (couponType != "" || couponType == "promotion") &&  (brand != "" || brand == "nike")) {  tax += ${x};  source = "1998";};`;
  const returnOperation = "return [ tax, source ];";

  const arguments =
    "{userId, cartId, supplier, couponCode, couponType, brand, tax, source = ''}";
  const bigJsFunctionRules = [];
  for (let i = 0; i < iterations; i++) {
    bigJsFunctionRules.push(roteOperation(1));
  }
  bigJsFunctionRules.push(returnOperation);

  const littleJsFunctionRules = `${roteOperation(1)}${returnOperation}`;

  const bigJsFunction = Function(arguments, bigJsFunctionRules.join(""));
  const littleJsFunction = Function(arguments, littleJsFunctionRules);
  littleJsFunctionRules;
  return { bigJsFunction, littleJsFunction };
}
