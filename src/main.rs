extern crate serde_json;

use evalexpr::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct RuleObject {
    r#if: String,
    then: String,
    r#else: Option<String>,
}

type RuleString = String;

#[derive(Serialize, Deserialize, Debug)]
enum RuleArrayContents {
    Object(RuleObject),
    String(RuleString),
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Rules {
    Object(RuleObject),
    String(RuleString),
    Array(Vec<RuleArrayContents>),
}

fn main() {
    let mut context = HashMapContext::new();
    let rule_json = r#" {
        "if": "fish == \"oneFish\"",
        "then": "fish = \"twoFish\"",
        "else": "fish = \"blueFish\""
    }"#;
    let rule = serde_json::from_str::<RuleObject>(rule_json).unwrap();
    context.set_value("fish".into(), "oneFish".into()).unwrap();
    assert_eq!(context.get_value("fish"), Some(&Value::from("oneFish")));

    assert_eq!(
        eval_with_context("fish == \"oneFish\"", &context),
        Ok(Value::from(true))
    );

    if eval_with_context(&rule.r#if, &context).unwrap() == Value::from(true) {
        eval_with_context_mut(&rule.then, &mut context).unwrap();
    } else {
        eval_with_context(&rule.r#else.unwrap(), &mut context).unwrap();
    }

    assert_eq!(context.get_value("fish"), Some(&Value::from("twoFish")));
}
