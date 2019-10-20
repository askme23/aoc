use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;
// use std::str::FromStr;

fn get_wire_signal(wire: &str, map: &mut HashMap<String, String>) -> u16 {
	let re1 = Regex::new(r"^(\w+|\d+)$").unwrap();
	let re2 = Regex::new(r"(\w+|\d+)\s(AND|OR|LSHIFT|RSHIFT)\s(\w+|\d+)").unwrap();
	let re3 = Regex::new(r"NOT\s(\w+|\d+)").unwrap();

	let value = String::from(map.get(wire).unwrap());
	// println!("{:?} => {:?}", wire, value);
	if re1.is_match(&value) {
		let value = match value.parse::<u16>() {
			Err(_) => get_wire_signal(&value, map),//recursion
			Ok(r) => r,
		};
		map.entry(String::from(wire)).and_modify(|e| *e = value.to_string());

		return value;
	} else if let Some(caps) = re3.captures(&value) {
		let operand = caps.get(1).unwrap().as_str();
		let operand_value = match operand.parse::<u16>() {
			Err(_) => get_wire_signal(operand, map),//recursion
			Ok(r) => r,
		};

		let value = !operand_value;
		map.entry(String::from(wire)).and_modify(|e| *e = value.to_string());

		return value;
	} else if let Some(caps) = re2.captures(&value) {
		let operand1 = caps.get(1).unwrap().as_str();
		let operand2 = caps.get(3).unwrap().as_str();
		let op = caps.get(2).unwrap().as_str();

		let op1_value = match operand1.parse::<u16>() {
			Err(_) => get_wire_signal(operand1, map),//recursion,
			Ok(r) => r,
		};

		let op2_value = match operand2.parse::<u16>() {
			Err(_) => get_wire_signal(operand2, map),//recursion,
			Ok(r) => r,
		};

		// println!("{}", op);

		let value = match op {
			"AND" => op1_value & op2_value,
			"OR" => op1_value | op2_value,
			"LSHIFT" => op1_value << op2_value,
			"RSHIFT" => op1_value >> op2_value,
			_ => 0,
		};
		map.entry(String::from(wire)).and_modify(|e| *e = value.to_string());
		return value
	}

	0
}

pub fn parsing_file() -> io::Result<()> {
	let file_name = "../test_day_7.txt";
	let file = File::open(file_name)?;
	let reader = BufReader::new(file);
	let mut map: HashMap<String, String> = HashMap::new();
	let re_key = Regex::new(r"(.+)\s->\s(\w+)").unwrap();

	for line in reader.lines() {
		let l = match line {
			Ok(st) => st,
			Err(_) => continue,
		};

		match re_key.captures(&l) {
			Some(caps) => {
				let key = caps.get(2).unwrap().as_str();
				map.entry(key.to_string()).or_insert(caps.get(1).unwrap().as_str().to_string());
			},
			None => continue,
		}
	}

	println!("{:?}", get_wire_signal("a", &mut map));

	Ok(())
}