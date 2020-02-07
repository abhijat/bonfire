use rustpython_parser::ast::{Expression, ExpressionType, Statement, StatementType, Suite};
use serde_json::Value;
use crate::statement_filters::is_assignment;
use crate::utils::snakeify;

pub struct ClassDefinition<'a> {
    pub name: String,
    pub body: &'a Suite,
}

impl<'a> ClassDefinition<'a> {
    pub fn new(name: &str, body: &'a Suite) -> Self {
        ClassDefinition { name: name.to_owned(), body }
    }

    pub fn to_json(&self) -> Value {
        let properties: Vec<Value> = self
            .body
            .iter()
            .flat_map(|s| is_assignment(s))
            .map(|s| s.to_json())
            .collect();
        
        json!({
        "name": snakeify(&self.name),
        "properties": properties
    })
    }
}

pub fn schema_class_definitions(s: &Statement) -> Option<ClassDefinition> {
    match &s.node {
        StatementType::ClassDef {
            name,
            body,
            bases,
            keywords: _,
            decorator_list: _
        } => {
            let is_schema_def = bases
                .iter()
                .fold(false, |acc, expr| acc || attribute_matches(expr, "marshmallow", Some("Schema")).is_some());
            if is_schema_def {
                Some(ClassDefinition::new(name.as_str(), body))
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn attribute_matches(e: &Expression, key: &str, v: Option<&str>) -> Option<String> {
    match &e.node {
        ExpressionType::Attribute {
            value,
            name
        } if identifier_matches(value, key) => {
            let matched_value = Some(name.to_owned());

            if v.is_none() {
                matched_value
            } else if v.unwrap() == name.as_str() {
                matched_value
            } else {
                None
            }
        }
        _ => None,
    }
}

pub fn identifier_matches(e: &Expression, key: &str) -> bool {
    match &e.node {
        ExpressionType::Identifier { name } => name.as_str() == key,
        _ => false,
    }
}
