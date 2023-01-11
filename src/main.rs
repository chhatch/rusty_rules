extern crate serde_json;

use evalexpr::*;
mod rules_engine;
use crate::rules_engine::*;

fn main() {
    let rule_string = Rules::String("fish = \"redFish\"".to_string());
    let mut context = HashMapContext::new();

    run_rules(&rule_string, &mut context);
    dbg!(context.get_value("fish").unwrap());
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn evaluate_string() {
        let rule_string = Rules::String("fish = \"redFish\"".to_string());
        let mut context = HashMapContext::new();

        context.set_value("fish".into(), "oneFish".into());

        run_rules(&rule_string, &mut context);

        match context.get_value("fish").unwrap() {
            Value::String(s) => assert_eq!(s, "redFish"),
            _ => panic!("fish should be a string"),
        }
    }
    #[test]
    fn evaluate_array() {
        let rule_array_json = r#"["fish = \"twoFish\"", "cat = \"inHat\""]"#;
        let rule_array = serde_json::from_str::<Rules>(rule_array_json).unwrap();
        let mut context = HashMapContext::new();

        context.set_value("fish".into(), "oneFish".into());
        context.set_value("cat".into(), "noHat".into());

        run_rules(&rule_array, &mut context);

        match context.get_value("fish").unwrap() {
            Value::String(s) => assert_eq!(s, "twoFish"),
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

        context.set_value("fish".into(), "oneFish".into());
        context.set_value("cat".into(), "inHat".into());

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
    fn evaluate_conditional() {
        let rule_object_json = r#" {
        "if": {"and": ["fish == \"oneFish\"", "cat == \"inHat\""]},
          "then": "fish = \"twoFish\"",
          "else": "fish = \"blueFish\""
        }"#;
        let rule_object = serde_json::from_str::<Rules>(rule_object_json).unwrap();
        let mut context = HashMapContext::new();

        context.set_value("fish".into(), "oneFish".into());
        context.set_value("cat".into(), "inHat".into());

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
}
