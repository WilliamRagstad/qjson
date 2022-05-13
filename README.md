# The `qJSON`-parser
> ### A quick and easy JSON parser.
> This library should not be used in production, it is purely for learning rust and how its library system works.

## Introduction
This is a simple JSON parser written in **Rust** with full support for [**JSON5**](https://json5.org/) serialization and deserialization.

## Usage
The following code snippet shows an example of how to use the parser:
```rust
use qjson::{parse_file, parse_str};

fn main() {
    let json = r#"{
        name: "John Doe",
        'age': 42,
        "married": true,
        "children": ['Jane', "Jack"],
        "address": {
            "street": "Main St.",
            "city": "New York",
            "state": "NY"
        }
    }"#;

    let parsed = parse_str(json).unwrap();
    println!("{:?}", parsed);
}
```

The output is the parsed JSON object:
```json
{
    "name": "John Doe",
    "age": 42,
    "married": true,
    "children": [
        "Jane",
        "Jack"
    ],
    "address": {
        "street": "Main St.",
        "city": "New York",
        "state": "NY"
    }
}
```

## Features
The following features are provided by the library.

### Definition
A JSON value is defined by the following:
```rust
#[derive(Debug, PartialEq)]
pub enum JsonValue {
 Null,
 Bool(bool),
 Number(f64),
 String(String),
 Array(Vec<Box<JsonValue>>),
 Object(HashMap<String, Box<JsonValue>>),
}
```

### Single values

It is also possible to parse single JSON values:
```rust
use qjson::{parse_str};

fn main() {
    let json = r#"[1, 2, 3]"#;
    let parsed = parse_str(json).unwrap();
    println!("{:?}", parsed);
}
```

The output is the parsed JSON array:
```json
[ 1, 2, 3 ]
```