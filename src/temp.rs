use evalexpr::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RuleHash {
    pub r#if: String,
    pub then: String,
    pub r#else: Option<String>,
}

type RuleString = String;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RuleArrayContents {
    Hash(RuleHash),
    String(RuleString),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Rules {
    Hash(RuleHash),
    String(RuleString),
    Array(Vec<Box<self>>),
}

pub fn run_rules(rules: Rules, context: &mut HashMapContext) {
    match rules {
        Rules::Hash(rule) => {
            if eval_with_context(&rule.r#if, context).unwrap() == Value::from(true) {
                eval_with_context_mut(&rule.then, context).unwrap();
            } else {
                // cloned to avoid borrowing issue with println! macro
                eval_with_context_mut(&rule.r#else.clone().unwrap(), context).unwrap();
            }

            assert_eq!(context.get_value("fish"), Some(&Value::from("twoFish")));
            println!(
                "Hash Rule: {:?}, Result: {:?}",
                rule,
                context.get_value("fish").unwrap()
            );
        }
        Rules::String(rule) => {
            eval_with_context_mut(&rule, context).expect("Rule String failed");
            println!(
                "String Rule: {:?}, Result: {:?}",
                rule,
                context.get_value("fish").unwrap()
            );
        }
        Rules::Array(rule) => {
            println!("Rules Array: {:?}", rule,);
            for sub_rule in rule {
                run_rules(sub_rule, &mut context);
            }
        }
    }
}
