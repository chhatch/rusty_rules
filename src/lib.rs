extern crate serde_json;

use evalexpr::*;
mod rules_engine;
use crate::rules_engine::*;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use web_sys::console;

fn main() {
    let rule_object_json = r#" [{"return": ""hello node!""}]"#;
    let rule_object = serde_json::from_str::<Rules>(rule_object_json).unwrap();
    dbg!(&rule_object);

    let mut context = HashMapContext::new();

    context.set_value("fish".into(), "oneFish".into()).unwrap();
    context.set_value("cat".into(), "inHat".into()).unwrap();

    run_rules(&rule_object, &mut context);
}

#[wasm_bindgen]
pub fn wasm_rules(rule_string: String) -> String {
    let parsed_rules = serde_json::from_str::<Rules>(rule_string.as_str()).unwrap();
    let mut context = HashMapContext::new();

    context.set_value("foo".into(), Value::Int(1)).unwrap();
    context.set_value("bar".into(), Value::Int(1)).unwrap();

    if let Value::String(result) = run_rules(&parsed_rules, &mut context) {
        return result;
    } else {
        panic!("No result");
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn evaluate_string() {
        let rule_string = Rules::String("fish = \"redFish\"".to_string());
        let mut context = HashMapContext::new();

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
        let mut context = HashMapContext::new();

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
        let mut context = HashMapContext::new();

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
        let mut context = HashMapContext::new();

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
        let mut context = HashMapContext::new();

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
        let mut context = HashMapContext::new();

        context.set_value("fish".into(), "oneFish".into()).unwrap();
        context.set_value("cat".into(), "inHat".into()).unwrap();

        let result = run_rules(&rule_object, &mut context);

        assert_eq!(result, Value::String("twoFish".to_string()));
    }
}