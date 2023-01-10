use evalexpr::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

pub struct AndObject {
    pub and: Vec<Conditional>,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct OrObject {
    pub or: Vec<Conditional>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Conditional {
    String(String),
    AndObject(AndObject),
    OrObject(OrObject),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RuleObject {
    pub r#if: Conditional,
    pub then: Box<Rules>,
    pub r#else: Option<Box<Rules>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Rules {
    Object(RuleObject),
    String(String),
    Array(Box<Vec<Rules>>),
    AndObject(AndObject),
    OrObject(OrObject),
}

pub fn run_rules(rules: &Rules, context: &mut HashMapContext) -> evalexpr::Value {
    match rules {
        Rules::Object(rule) => {
            let mut condition_result = false;
            match &rule.r#if {
                Conditional::String(condition) => {
                    condition_result =
                        eval_with_context(&condition, context).unwrap() == Value::from(true);
                }
                Conditional::AndObject(and) => {
                    for rule in &and.and {
                        match rule {
                            Conditional::String(condition) => {
                                condition_result = eval_with_context(&condition, context).unwrap()
                                    == Value::from(true);
                                if !condition_result {
                                    break;
                                }
                            }
                            _ => {},
                        }
                    }
                }
                _ => {}
            }
            if condition_result {
                return run_rules(&*rule.then, context);
            } else if let Some(else_rule) = &rule.r#else {
                return run_rules(&*else_rule, context);
            } else {
                return evalexpr::Value::from(());
            }
        }
        Rules::Array(rules) => {
            for rule in rules.iter() {
                run_rules(rule, context);
            };
            evalexpr::Value::from(())
        }
        Rules::String(rule) => eval_with_context_mut(&rule, context).expect("Rule String failed"),
        _ => evalexpr::Value::from(()),
    }
}
