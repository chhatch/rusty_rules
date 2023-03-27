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

const { bigJsFunction, littleJsFunction } = buildFunctionRules(400000);

let result;
const start = performance.now();
for (let i = 0; i < 400000; i++) {
  result = littleJsFunction(getContext());
}
const end = performance.now();
console.log("JsFunction", end - start, result);
