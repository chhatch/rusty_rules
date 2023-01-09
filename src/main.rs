extern crate serde_json;

use evalexpr::*;
mod rules_engine;
use crate::rules_engine::*;

fn main() {
    let rule_object_json = r#" {
        "if": "fish == \"oneFish\"",
        "then": {
        "if": "fish == \"oneFish\"",
        "then": "fish = \"twoFish\"",
        "else": "fish = \"blueFish\""
    },
        "else": "fish = \"blueFish\""
    }"#;
    let rule_object = serde_json::from_str::<Rules>(rule_object_json).unwrap();

    let rule_string = Rules::RuleContents(RuleContents::String("fish = \"redFish\"".to_string()));

    let rule_array_json = r#" [{
        "if": "fish == \"oneFish\"",
        "then": "fish = \"twoFish\"",
        "else": "fish = \"blueFish\""
    }, "fish == \"twoFish\""]"#;
    let rule_array = serde_json::from_str::<Rules>(rule_array_json).unwrap();

    let mut context = HashMapContext::new();
    context.set_value("fish".into(), "oneFish".into()).unwrap();
    // assert_eq!(context.get_value("fish"), Some(&Value::from("oneFish")));
    // assert_eq!(
    //     eval_with_context("fish == \"oneFish\"", &context),
    //     Ok(Value::from(true))
    // );

    run_rules(rule_object, &mut context);
    // run_rules(rule_string, &mut context);
    // run_rules(rule_array, &mut context);
}
