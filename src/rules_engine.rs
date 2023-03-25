use evalexpr::{eval_with_context_mut, HashMapContext, Value};
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

pub struct Return {
    pub r#return: String,
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
    Return(Return),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Rules {
    String(String),
    // Array needs to come before Object oterhwise Serde parses length 3 arrays as If Object
    Array(Box<Vec<Rules>>),
    Object(RuleObject),
}

#[derive(Debug)]
pub enum Exit {
    Return(Value),
    Continue(Value),
}

pub fn run_rules(rules: &Rules, context: &mut HashMapContext) -> Value {
    if let Exit::Return(value) = rule_recursion(rules, context) {
        println!("Return value: {:?}", value);
        return value;
    } else {
        println!("No return value");
        return Value::from(());
    }
}

pub fn rule_recursion(rules: &Rules, context: &mut HashMapContext) -> Exit {
    let result: Exit = match rules {
        Rules::Object(rule_object) => match rule_object {
            RuleObject::If(if_object) => {
                let condition_result =
                    unwrap_continue(rule_recursion(&if_object.r#if, context)) == Value::from(true);
                if condition_result {
                    return rule_recursion(&*if_object.then, context);
                } else if let Some(else_rule) = &if_object.r#else {
                    return rule_recursion(&*else_rule, context);
                } else {
                    return Exit::Continue(Value::from(()));
                }
            }
            RuleObject::And(and_object) => {
                let results = rule_recursion(&*and_object.and, context);
                if let Value::Tuple(arr) = unwrap_continue(results) {
                    return Exit::Continue(Value::from(
                        arr.iter()
                            .fold(true, |acc, x| x == &Value::from(true) && acc == true),
                    ));
                } else {
                    panic!("And rule must contain rules array")
                };
            }
            RuleObject::Or(or_object) => {
                let results = rule_recursion(&*or_object.or, context);
                if let Value::Tuple(arr) = unwrap_continue(results) {
                    return Exit::Continue(Value::from(
                        arr.iter()
                            .fold(false, |acc, x| x == &Value::from(true) || acc == true),
                    ));
                } else {
                    panic!("And rule must contain rules array")
                };
            }
            RuleObject::Return(return_object) => {
                let value = eval_with_context_mut(&return_object.r#return, context)
                    .expect("Rule String failed");
                return Exit::Return(value);
            }
        },
        Rules::Array(rules) => {
            let mut r#return: Option<Value> = None;

            let results: Vec<Value> = rules
                .iter()
                .map_while(|rule| {
                    return match rule_recursion(rule, context) {
                        Exit::Return(value) => {
                            r#return = Some(value);
                            None
                        }
                        Exit::Continue(value) => Some(value),
                    };
                })
                .collect();

            if let Some(return_value) = r#return {
                return Exit::Return(return_value);
            } else {
                return Exit::Continue(Value::Tuple(results));
            }
        }

        Rules::String(rule) => {
            Exit::Continue(eval_with_context_mut(&rule, context).expect("Rule String failed"))
        }
    };

    result
}

fn unwrap_continue(exit: Exit) -> Value {
    match exit {
        Exit::Continue(value) => return value,
        _ => panic!("Expected Continue"),
    }
}
