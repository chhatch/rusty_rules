use evalexpr::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RuleHash {
    pub r#if: String,
    pub then: Box<Rules>,
    pub r#else: Option<Box<Rules>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RuleContents {
    Hash(RuleHash),
    String(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Rules {
    RuleContents(RuleContents),
    Array(Box<Vec<Rules>>),
}

pub fn run_rules(rules: Rules, context: &mut HashMapContext) {
    match rules {
        Rules::RuleContents(rule) => match rule {
            RuleContents::Hash(rule) => {
                println!(
                    "Hash Rule: {:?}, Result: {:?}",
                    rule,
                    context.get_value("fish").unwrap()
                );
                if eval_with_context(&rule.r#if, context).unwrap() == Value::from(true) {
                    run_rules(*rule.then, context);
                } else if let Some(else_rule) = rule.r#else {
                                        run_rules(*else_rule, context);

                }
            }
            RuleContents::String(rule) => {
                eval_with_context_mut(&rule, context).expect("Rule String failed");
                println!(
                    "String Rule: {:?}, Result: {:?}",
                    rule,
                    context.get_value("fish").unwrap()
                );
            }
        },
        Rules::Array(rules) => {
            println!("Rules Array: {:?}", rules,);
            for sub_rule in *rules {
                run_rules(sub_rule, context);
            }
        }
    }
}
