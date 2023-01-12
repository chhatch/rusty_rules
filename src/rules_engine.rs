use evalexpr::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]

pub struct And {
    pub and: Box<Rules>,
}

#[derive(Serialize, Deserialize, Debug)]

pub struct Or {
    pub or: Box<Rules>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct If {
    pub r#if: Box<Rules>,
    pub then: Box<Rules>,
    pub r#else: Option<Box<Rules>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum RuleObject {
    If(If),
    And(And),
    Or(Or),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Rules {
    String(String),
    Object(RuleObject),
    Array(Box<Vec<Rules>>),
}

pub fn run_rules(rules: &Rules, context: &mut HashMapContext) -> evalexpr::Value {
    match rules {
        Rules::Object(rule_object) => match rule_object {
            RuleObject::If(if_object) => {
                let condition_result = run_rules(&if_object.r#if, context) == Value::from(true);
                if condition_result {
                    return run_rules(&*if_object.then, context);
                } else if let Some(else_rule) = &if_object.r#else {
                    return run_rules(&*else_rule, context);
                } else {
                    return evalexpr::Value::from(());
                }
            }
            RuleObject::And(and_object) => {
                let results = run_rules(&*and_object.and, context);
                if let Value::Tuple(arr) = results {
                    return Value::from(
                        arr.iter()
                            .fold(true, |acc, x| x == &Value::from(true) && acc == true),
                    );
                } else {
                    panic!("And rule must contain rules array")
                };
            }
            RuleObject::Or(or_object) => {
                let results = run_rules(&*or_object.or, context);
                if let Value::Tuple(arr) = results {
                    return Value::from(
                        arr.iter()
                            .fold(false, |acc, x| x == &Value::from(true) || acc == true),
                    );
                } else {
                    panic!("And rule must contain rules array")
                };
            }
            _ => panic!("Object rule type not supported"),
        },
        Rules::Array(rules) => {
            Value::Tuple(rules.iter().map(|rule| run_rules(rule, context)).collect())
        }
        Rules::String(rule) => eval_with_context_mut(&rule, context).expect("Rule String failed"),
        _ => panic!("Rule type not supported"),
    }
}
