use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;
use std::str::FromStr;
use std::fs::File;
use regex::Regex;

use super::day_9;

pub fn parsing_file() -> io::Result<Vec<Vec<i16>>> {
	let file_name = "../test_day_13.txt";
	let file = File::open(file_name)?;
	let reader = BufReader::new(file);

	let re = Regex::new(r"(\w+) would (\w+) (\d+) happiness units by sitting next to (\w+)").unwrap();
	let mut matrix: Vec<Vec<i16>> = vec![vec![0]];
	let mut cur_person: String = String::from("");
	let mut cnt: i16 = 0;

	for line in reader.lines() {
		match re.captures(&line.unwrap()) {
			Some(caps) => {
				let person = caps.get(1).unwrap().as_str();
				let mut happiness = i16::from_str(&caps[3]).unwrap();
				let action = caps.get(2).unwrap().as_str();

				if cur_person == String::from("") {
					cur_person = person.to_string();
				}

				if person.to_string() != cur_person {
					cur_person = person.to_string();
					cnt += 1;
					matrix.push(Vec::new());
				}

				if action == "lose" {
					happiness *= -1;
				}

				matrix[cnt as usize].push(happiness);

				if matrix[cnt as usize].len() == cnt as usize {
					matrix[cnt as usize].push(0i16);
				}
			},
			None => continue
		}
	}

	for i in 0..matrix.len() {
		for j in i..matrix[i].len() {
			matrix[i][j] += matrix[j][i];
			matrix[j][i] = matrix[i][j];
		}
	}

	// part 2
	for i in 0..matrix.len() {
		matrix[i].push(0);
	}
	matrix.push(vec![0; matrix[0].len()]);
	println!("{:?}", matrix);

	Ok(matrix)
}

pub fn run() {
	// Данная задача является еще одной формулировкой задачи о коммивояжере.
	day_9::run(parsing_file());
}