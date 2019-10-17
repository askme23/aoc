use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;
// use std::str::FromStr;

//FIXME все значения должны хранить ссылки на другие переменные
fn smt1(line: &str, map: &mut HashMap<String, u16>) {
	let re = Regex::new(r"(\w+|\d+)\s->\s(\w+)").unwrap();

	let caps = re.captures(line).unwrap();
	let key = caps.get(2).unwrap().as_str();
	let operand = caps.get(1).unwrap().as_str();


	let value = if let Err(_) = operand.parse::<u16>() {
		*map.entry(operand.to_string()).or_insert(0)
	} else if let Ok(r) = operand.parse::<u16>() {
		r
	} else {
		0
	};
	map.entry(key.to_string()).or_insert(value);
}

fn smt2(line: &str, map: &mut HashMap<String, u16>) {
	let re1 = Regex::new(r"(\w+|\d+)\s(AND|OR|LSHIFT|RSHIFT)\s(\w+|\d+)\s->\s(\w+)").unwrap();
	let re2 = Regex::new(r"NOT\s(\w+|\d+)\s->\s(\w+)").unwrap();

	match re2.captures(line) {
		Some(caps) => {
			let operand = caps.get(1).unwrap().as_str();
			let key = caps.get(2).unwrap().as_str();

			 let value = if let Err(_) = operand.parse::<u16>() {
				!*map.entry(operand.to_string()).or_insert(0)
			} else if let Ok(r) = operand.parse::<u16>() {
				!r
			} else {
				0
			};

			map.insert(key.to_string(), value);
			// *map.entry(String::from(key)).or_insert(0)
		},
		None => {
			match re1.captures(line) {
				Some(caps) => {
					let f_operand = caps.get(1).unwrap().as_str();
					let s_operand = caps.get(3).unwrap().as_str();
					let command = caps.get(2).unwrap().as_str();
					let key = caps.get(4).unwrap().as_str();

					let operand1 = if let Err(_) = f_operand.parse::<u16>() {
						map.entry(f_operand.to_string()).or_insert(0);
						*map.get(f_operand).unwrap()
					} else if let Ok(r) = f_operand.parse::<u16>() {
						r
					} else {
						0
					};

					let operand2 = if let Err(_) = s_operand.parse::<u16>() {
						map.entry(s_operand.to_string()).or_insert(0);
						*map.get(s_operand).unwrap()
					} else if let Ok(r) = s_operand.parse::<u16>() {
						r
					} else {
						0
					};

					*map.entry(String::from(key)).or_insert(0) = match command {
						"AND" => operand1 & operand2,
						"OR" => operand1 | operand2,
						"LSHIFT" => operand1 << operand2,
						"RSHIFT" => operand1 >> operand2,
						_ => 0,
					};
				},
				None => (),
			}
		},
	}
}

pub fn parsing_file() -> io::Result<()> {
	let file_name = "../test_day_7.txt";
	let file = File::open(file_name)?;
	let reader = BufReader::new(file);

	let re_op = Regex::new(r"(AND|OR|LSHIFT|RSHIFT|NOT)").unwrap();	

	let mut instruction: HashMap<String, u16> = HashMap::new();
	for line in reader.lines() {
		let l = match line {
			Ok(st) => st,
			Err(_) => continue,
		};

		match re_op.captures(&l) {
			None => {
				smt1(&l, &mut instruction);
				continue
			},
			Some(_) => {
				smt2(&l, &mut instruction);
			}, 
		}
	}

	println!("{:?}", instruction);

	Ok(())
}