extern crate serde_json;

use evalexpr::{
    context_map, Context, ContextWithMutableVariables, EvalexprError, HashMapContext, Value,
};
use serde_json::Value as JsonValue;
mod rules_engine;
use crate::rules_engine::*;
use std::collections::HashMap;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::console;

fn main() {
    let rule_object_json = r#" [{"if":{"and":["userId == 1","cartId == IF(cartId != 0, 1, cartId)","supplier == IF(supplier != \"\", \"walmart\", supplier)","couponCode == IF(couponCode != \"\", \"1234\", couponCode)","couponType == IF(couponType != \"\", \"promotion\", couponType)","brand == IF(brand != \"\", \"nike\", brand)"]},"then":["tax += 1","source = \"1998\""]},{"return":"tax, source"}]"#;
    let rule_object = serde_json::from_str::<Rules>(rule_object_json).unwrap();

    let mut context = get_context();

    context.set_value("userId".into(), 1.into()).unwrap();
    context.set_value("cartId".into(), 1.into()).unwrap();
    context
        .set_value("supplier".into(), "walmart".into())
        .unwrap();
    context
        .set_value("couponCode".into(), "1234".into())
        .unwrap();
    context
        .set_value("couponType".into(), "promotion".into())
        .unwrap();
    context.set_value("brand".into(), "nike".into()).unwrap();
    context.set_value("source".into(), "mobile".into()).unwrap();
    context.set_value("tax".into(), 1.into()).unwrap();

    run_rules(&rule_object, &mut context);
}

#[wasm_bindgen]
pub fn wasm_rules(rule_string: String, context_string: String) -> String {
    // console::log_1(&rule_string.clone().into());
    // console::log_1(&context_string.clone().into());
    let parsed_rules = serde_json::from_str::<Rules>(rule_string.as_str()).unwrap();
    let mut context = get_context();

    let context_struct: HashMap<String, JsonValue> =
        serde_json::from_str(context_string.as_str()).unwrap();
    for (key, value) in context_struct {
        match value {
            JsonValue::String(s) => context.set_value(key.into(), s.into()).unwrap(),

            JsonValue::Number(n) => context.set_value(key.into(), 1.into()).unwrap(),
            JsonValue::Bool(b) => context.set_value(key.into(), Value::from(b)).unwrap(),
            _ => panic!("Unsupported type in context"),
        }
    }

    match run_rules(&parsed_rules, &mut context) {
        Value::String(result) => {
            return serde_json::to_string(&result).expect("Failed to serialize result")
        }
        Value::Int(result) => {
            return serde_json::to_string(&result).expect("Failed to serialize result")
        }
        Value::Float(result) => {
            return serde_json::to_string(&result).expect("Failed to serialize result")
        }
        Value::Boolean(result) => {
            return serde_json::to_string(&result).expect("Failed to serialize result")
        }
        Value::Tuple(result) => {
            let mut result_string = result
                .iter()
                .fold("[".to_string(), |acc, x| acc + x.to_string().as_str() + ",");
            result_string.pop();

            result_string + "]"
        }
        Value::Empty => "null".to_string(),
    }
}

fn convert(x: f64) -> i32 {
    x.round().rem_euclid(2f64.powi(32)) as u32 as i32
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn evaluate_string() {
        let rule_string = Rules::String("fish = \"redFish\"".to_string());
        let mut context = get_context();

        context.set_value("fish".into(), "oneFish".into()).unwrap();

        run_rules(&rule_string, &mut context);

        match context.get_value("fish").unwrap() {
            Value::String(s) => assert_eq!(s, "redFish"),
            _ => panic!("fish should be a string"),
        }
    }
    #[test]
    fn evaluate_array() {
        // this test is also a regression test for bug where length 3 arrays were parsed as If objects
        let rule_array_json = r#"["fish = \"twoFish\"", "cat = \"inHat\"", "fish = \"blueFish\""]"#;
        let rule_array = serde_json::from_str::<Rules>(rule_array_json).unwrap();
        let mut context = get_context();

        context.set_value("fish".into(), "oneFish".into()).unwrap();
        context.set_value("cat".into(), "noHat".into()).unwrap();

        run_rules(&rule_array, &mut context);

        match context.get_value("fish").unwrap() {
            Value::String(s) => assert_eq!(s, "blueFish"),
            _ => panic!("fish should be a string"),
        }
        match context.get_value("cat").unwrap() {
            Value::String(s) => assert_eq!(s, "inHat"),
            _ => panic!("fish should be a string"),
        }
    }
    #[test]
    fn evaluate_if() {
        let rule_object_json = r#" {
        "if": "fish == \"oneFish\"",
          "then": {
            "if": "cat == \"inHat\"",
            "then": "fish = \"twoFish\""
          },
        "else": "fish = \"blueFish\""
        }"#;
        let rule_object = serde_json::from_str::<Rules>(rule_object_json).unwrap();
        let mut context = get_context();

        context.set_value("fish".into(), "oneFish".into()).unwrap();
        context.set_value("cat".into(), "inHat".into()).unwrap();

        run_rules(&rule_object, &mut context);

        match context.get_value("fish").unwrap() {
            Value::String(s) => assert_eq!(s, "twoFish"),
            _ => panic!("fish should be a string"),
        }

        run_rules(&rule_object, &mut context);

        match context.get_value("fish").unwrap() {
            Value::String(s) => assert_eq!(s, "blueFish"),
            _ => panic!("fish should be a string"),
        }
    }
    #[test]
    fn evaluate_and() {
        let rule_object_json = r#" {
        "if": {"and": ["fish == \"oneFish\"", "cat == \"inHat\""]},
          "then": "fish = \"twoFish\"",
          "else": "fish = \"blueFish\""
        }"#;
        let rule_object = serde_json::from_str::<Rules>(rule_object_json).unwrap();
        let mut context = get_context();

        context.set_value("fish".into(), "oneFish".into()).unwrap();
        context.set_value("cat".into(), "inHat".into()).unwrap();

        run_rules(&rule_object, &mut context);

        match context.get_value("fish").unwrap() {
            Value::String(s) => assert_eq!(s, "twoFish"),
            _ => panic!("fish should be a string"),
        };

        run_rules(&rule_object, &mut context);

        match context.get_value("fish").unwrap() {
            Value::String(s) => assert_eq!(s, "blueFish"),
            _ => panic!("fish should be a string"),
        };
    }
    #[test]
    fn evaluate_or() {
        let rule_object_json = r#" {
        "if": {"or": ["fish == \"oneFish\"", "cat == \"inHat\""]},
          "then": "fish = \"twoFish\"",
          "else": "fish = \"blueFish\""
        }"#;
        let rule_object = serde_json::from_str::<Rules>(rule_object_json).unwrap();
        let mut context = get_context();

        context.set_value("fish".into(), "oneFish".into()).unwrap();
        context.set_value("cat".into(), "noHat".into()).unwrap();

        run_rules(&rule_object, &mut context);

        match context.get_value("fish").unwrap() {
            Value::String(s) => assert_eq!(s, "twoFish"),
            _ => panic!("fish should be a string"),
        };

        run_rules(&rule_object, &mut context);

        match context.get_value("fish").unwrap() {
            Value::String(s) => assert_eq!(s, "blueFish"),
            _ => panic!("fish should be a string"),
        };
    }

    #[test]
    fn evaluate_return() {
        let rule_object_json = r#" {
  "if": "fish == \"oneFish\"",
  "then": {
    "if": "cat == \"inHat\"",
    "then": ["fish = \"twoFish\"", { "return": "fish" }]
  },
  "else": { "return": "\"blueFish\"" }
}"#;
        let rule_object = serde_json::from_str::<Rules>(rule_object_json).unwrap();
        let mut context = get_context();

        context.set_value("fish".into(), "oneFish".into()).unwrap();
        context.set_value("cat".into(), "inHat".into()).unwrap();

        let result = run_rules(&rule_object, &mut context);

        assert_eq!(result, Value::String("twoFish".to_string()));
    }

    #[test]
    fn evaluate_IF_Fn() {
        let rule_object_json = r#" {"return": "IF(1 == 1, 1, 0)"}"#;
        let rule_object = serde_json::from_str::<Rules>(rule_object_json).unwrap();
        let mut context = get_context();

        let result = run_rules(&rule_object, &mut context);

        assert_eq!(result, Value::Int(1));

        let rule_object_json = r#" {"return": "IF(1 == 0, 1, 0)"}"#;
        let rule_object = serde_json::from_str::<Rules>(rule_object_json).unwrap();

        let result = run_rules(&rule_object, &mut context);

        assert_eq!(result, Value::Int(0));
    }
}
