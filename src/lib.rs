use std::{collections::HashMap, any::Any};

#[derive(Debug, PartialEq, Clone)]
pub enum JsonNumber {
    Int(i64),
    Float(f64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue {
 Unknown,
 Null,
 Bool(bool),
 Number(JsonNumber),
 String(String),
 Array(Vec<JsonValue>),
 Object(HashMap<String, JsonValue>),
}

#[derive(Debug, PartialEq)]
enum ObjectFieldType {
    Unknown,
    DoubleQuoted,
    SingleQuoted,
    Unquoted,
    Numeric,
}

#[derive(Debug, PartialEq)]
enum ParseType {
    Unknown,
    Object,
    // ObjectField(type, has key, has value)
    ObjectField(ObjectFieldType, bool, bool),
    Array,
    String,
    Number,
    Boolean,
    Null,
}

struct ParseState {
    pub parse_type: ParseType,
    pub value: JsonValue,
}

pub fn parse_str(s: &str) -> Result<JsonValue, String> {
    let mut states = Vec::new();
    let new_state = |parse_type: ParseType, value: JsonValue| {
        states.push(ParseState {
            parse_type: parse_type,
            value: value,
        });
    };
    let pop_state = || {
        states.pop().unwrap()
    };
    let is_state = |state: ParseType| -> bool {
        if let Some(last) = states.last() {
            last.parse_type.type_id() == state.type_id()
        } else {
            false
        }
    };
    let is_state_object_field = || -> bool {
        match states.last() {
            Some(s) => match s.parse_type { ParseType::ObjectField(_, _, _) => true, _ => false },
            _ => false
        }
    };
    // Initial state
    new_state(ParseType::Unknown, JsonValue::Null);

    for c in s.chars() {
        match c {
            '{' if is_state(ParseType::Unknown) => {
                new_state(ParseType::Object, JsonValue::Object(HashMap::new()));
                new_state(ParseType::ObjectField(ObjectFieldType::Unknown, false, false), JsonValue::Null);
            },
            '"' if is_state_object_field() => {

            },
            '"' if is_state(ParseType::Unknown) => {
                new_state(ParseType::String, JsonValue::String(String::new()));
            },
            '}' if is_state_object_field() => {
                
                let mut object = pop_state().value.clone().into_object().unwrap();
                let field = pop_state().value.clone().into_string().unwrap();
                let value = pop_state().value.clone();
                object.insert(field, value);
                let mut object_value = JsonValue::Object(object);
                states.pop();
                states.pop();
                states.pop();
                new_state(ParseType::Object, object_value);
            },
            _ if c.is_whitespace() && !is_state(ParseType::String) => continue,
            _ => return Err(format!("Unexpected character: {}", c)),
        }
    }
    // Must be one value left on the stack
    assert!(states.len() == 1, "Only one value can be parsed at a time");
    Ok(pop_state().value)
}

pub fn parse_file(path: &str) -> Result<JsonValue, String> {
    unimplemented!()
}

// Convert a JsonValue to a Rust value
pub fn to_rust<T>(value: &JsonValue) -> Result<T, String>
where T : Any {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn empty_object() {
        let json = "{}";
        let parsed = super::parse_str(json).expect("Failed to parse json");
        match parsed {
            super::JsonValue::Object(map) => {
                assert!(map.is_empty());
            }
            _ => panic!("Expected object"),
        }
    }
}
