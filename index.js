const { wasm_rules } = require("./pkg");
const { ruleFactory } = require("@elite-libs/rules-machine");
const { performance, PerformanceObserver } = require("node:perf_hooks");
const { pick } = require("lodash");

for (let i = 0; i < 7; i++) {
  const iterations = 10 ** i;
  benchmark(iterations);
}

function benchmark(iterations) {
  const roteOperation = {
    if: { and: ["foo == foo ", "foo == foo"] },
    then: "bar = bar + 1",
    else: "bar = bar - 1",
  };

  const returnObj = { return: '"hello node!"' };

  const bigWasmLabel = `wasm: ${iterations} rules`;
  const littleWasmLabel = `wasm: 1 rule ${iterations} times`;
  const bigJsLabel = `js: ${iterations} rules`;
  const littleJsLabel = `js: 1 rule ${iterations} times`;

  const bigJsonRules = [];
  for (let i = 0; i < iterations; i++) {
    bigJsonRules.push(roteOperation);
  }
  bigJsonRules.push(returnObj);

  const littleJsonRules = [roteOperation, returnObj];

  const littleRules = JSON.stringify(littleJsonRules);
  const bigRules = JSON.stringify(bigJsonRules);

  performance.mark(bigWasmLabel);
  wasm_rules(bigRules);
  performance.measure(bigWasmLabel, bigWasmLabel);

  performance.mark(bigJsLabel);
  const rulesEngine = ruleFactory(littleJsonRules);
  rulesEngine({ foo: 1, bar: 1 });
  performance.measure(bigJsLabel, bigJsLabel);

  performance.mark(littleWasmLabel);
  for (let i = 0; i < iterations; i++) {
    wasm_rules(littleRules);
  }
  performance.measure(littleWasmLabel, littleWasmLabel);

  performance.mark(littleJsLabel);
  for (let i = 0; i < iterations; i++) {
    const rulesEngine = ruleFactory(littleJsonRules);
    rulesEngine({ foo: 1, bar: 1 });
  }
  performance.measure(littleJsLabel, littleJsLabel);

  console.table(
    performance
      .getEntriesByType("measure")
      .map((x) => pick(x, ["name", "duration"]))
  );

  performance.clearMarks();
  performance.clearMeasures();
}
