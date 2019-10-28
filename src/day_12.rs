use serde_json::{Result, Value};
use std::fs;

fn contains_red(obj: &Value) -> bool {
	for (_, val) in obj.as_object().unwrap() {
		if val.is_string() {
			if let Some("red") = val.as_str() {
				return true;
			}
		}
	}

	false
}

fn array_iter(array: &Value) -> i64 {
	let mut summ = 0i64;
	if let Value::Array(value) = array {
		for val in value {
			if val.is_number() {
				summ += val.as_i64().unwrap();
			}

			if val.is_array() {
				summ += array_iter(&val);
			}

			if val.is_object() {
				if !contains_red(&val) {
					summ += obj_iter(&val);
				}
			}
		}	
	}

	summ
}

fn obj_iter(json: &Value) -> i64 {
	let mut summ = 0i64;

	if json.is_object() {
		for (_, val) in json.as_object().unwrap() {
			if val.is_number() {
				summ += val.as_i64().unwrap();
			}

			if val.is_array() {
				summ += array_iter(&val);
			}
				
			if val.is_object() {
				if !contains_red(&val) {
					summ += obj_iter(&val);
				}
			}
		}
	}
	
	summ
}

pub fn run() -> Result<()> {
	let file_data = fs::read_to_string("../test_day_12.txt").expect("Error with readin file.");
	let v: Value = serde_json::from_str(&file_data)?;

	println!("{:?}", obj_iter(&v));

	Ok(())
}