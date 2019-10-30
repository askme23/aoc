use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;
use std::str::FromStr;
use std::fs::File;
use regex::Regex;

fn parsing_file() -> io::Result<HashMap<String, [u16; 8]>> {
	let file_name = "../test_day_14.txt";
	let file = File::open(file_name)?;
	let reader = BufReader::new(file);
	let re = Regex::new(r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+)").unwrap();
	let mut reindeer: HashMap<String, [u16; 8]> = HashMap::new();

	for line in reader.lines() {
		match re.captures(&line.unwrap()) {
			Some(caps) => {
				let deer = caps.get(1).unwrap().as_str();
				let distance = u16::from_str(&caps[2]).unwrap();
				let time = u16::from_str(&caps[3]).unwrap();
				let rest = u16::from_str(&caps[4]).unwrap();

				let deer_speed_info: [u16; 8] = [distance, time, time, rest, rest, 0, 0, 0];
				reindeer.entry(deer.to_string()).or_insert(deer_speed_info); 
			},
			None => continue
		}
	}

	Ok(reindeer)
}

fn calc_points(reindeer: &mut HashMap<String, [u16; 8]>) {
	let mut max_distance = 0u16;

	for (_, deer) in &mut *reindeer {
		if deer[6] > max_distance {
			max_distance = deer[6];
		}
	}

	for (_, deer) in &mut *reindeer {
		if deer[6] == max_distance {
			deer[7] += 1;
		}
	}
}

pub fn run() {
	let n = 2503u16;
	let mut reindeer: HashMap<String, [u16; 8]>;
	
	if let Ok(deers) = parsing_file() {
		reindeer = deers;
	} else {
		panic!("something gone wrong");
	}

	for _ in 0..n {
		for (_, deer) in &mut reindeer {
			if deer[5] == 0 {
				deer[6] += deer[0];
				deer[1] -= 1;
				if deer[1] == 0 {
					deer[1] = deer[2];
					deer[5] = 1;
				}
			} else {
				deer[3] -= 1;	
				if deer[3] == 0 {
					deer[3] = deer[4];
					deer[5] = 0;
				}
			}
		}

		calc_points(&mut reindeer);
	}
	println!("{:?}", reindeer);
}