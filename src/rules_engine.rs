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
pub enum RuleContents {
    Hash(RuleHash),
    String(RuleString),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Rules {
    RuleContents(RuleContents),
    Array(Vec<RuleContents>),
}

pub fn run_rules(rules: Rules, context: &mut HashMapContext) {
    match rules {
        Rules::RuleContents(rule) => {
            match rule {
                RuleContents::Hash(rule) => {
                    if eval_with_context(&rule.r#if, context).unwrap() == Value::from(true) {
                        eval_with_context_mut(&rule.then, context).unwrap();
                    } else if let Some(else_rule) = &rule.r#else{
                        eval_with_context_mut(else_rule, context).unwrap();
                    }

                    assert_eq!(context.get_value("fish"), Some(&Value::from("twoFish")));
                    println!(
                        "Hash Rule: {:?}, Result: {:?}",
                        rule,
                        context.get_value("fish").unwrap()
                    );
                }
                RuleContents::String(rule) => {
                    eval_with_context_mut(&rule, context).expect("Rule String failed");
                    println!(
                        "String Rule: {:?}, Result: {:?}",
                        rule,
                        context.get_value("fish").unwrap()
                    );
                }
            }
        }
        Rules::Array(rules) => {
            println!("Rules Array: {:?}", rules,);
            for sub_rule in rules {
                run_rules(Rules::RuleContents(sub_rule), context);
            }
        }
    }
}
