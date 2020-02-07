use rustpython_parser::ast::{Expression, ExpressionType, Statement, StatementType};

use crate::schema_filters::attribute_matches;

#[derive(Debug)]
pub struct FieldKind {
    pub name: String,
    pub kind: String,
}

impl FieldKind {
    pub fn new(name: String, kind: String) -> FieldKind {
        FieldKind { name, kind: FieldKind::remap_kind(&kind) }
    }

    pub fn to_json(&self) -> serde_json::Value {
        json!({
            self.name.clone(): {
                "type": self.kind.clone().to_ascii_lowercase()
            }
        })
    }

    fn remap_kind(input: &str) -> String {
        match input.to_lowercase().as_str() {
            "dict" => "object".to_owned(),
            "list" => "array".to_owned(),
            "integer" => "int".to_owned(),
            _ => input.to_lowercase()
        }
    }
}

pub fn is_assignment(s: &Statement) -> Option<FieldKind> {
    match &s.node {
        StatementType::Assign {
            targets,
            value
        } => {
            field_kind_from_assignment(targets, value)
        }
        _ => None,
    }
}

fn field_kind_from_assignment(targets: &Vec<Expression>, value: &Expression) -> Option<FieldKind> {
    is_value_call(value).and_then(|v| {
        is_single_target(targets).map(|t| FieldKind::new(t, v))
    })
}

pub fn is_value_call(e: &Expression) -> Option<String> {
    match &e.node {
        ExpressionType::Call { function, args: _, keywords: _ } => {
            attribute_matches(function, "fields", None)
        }
        _ => None,
    }
}

pub fn is_single_target(targets: &Vec<Expression>) -> Option<String> {
    if targets.len() != 1 {
        None
    } else {
        let target = targets.first().unwrap();
        match &target.node {
            ExpressionType::Identifier { name } => Some(name.to_owned()),
            _ => None
        }
    }
}
