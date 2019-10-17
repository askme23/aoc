use regex::Regex;
use std::fs;
use std::str::FromStr;

pub fn parsing_file() {
	let file_name = "../test_day_6.txt";
	let contents = fs::read_to_string(file_name)
		.expect("Something went wrong reading the file");
	let re = Regex::new(r"(turn on|turn off|toggle)\s(\d+),(\d+)\sthrough\s(\d+),(\d+)").unwrap();
	let mut matrix = [[0i32; 1000]; 1000];

	for cap in re.captures_iter(&contents) {
		for x in i32::from_str(&cap[2]).unwrap()..i32::from_str(&cap[4]).unwrap() + 1 {
			for y in i32::from_str(&cap[3]).unwrap()..i32::from_str(&cap[5]).unwrap() + 1 {
				matrix[x as usize][y as usize] += match &cap[1] {
					"turn on" => 1,
				 	"turn off" => if matrix[x as usize][y as usize] <= 0 { 0 } else { -1 },
				 	"toggle" => 2,
				 	_ => 0	
				};
			}
		}
	}

	let new_matrix = matrix.into_iter().map(|x| { x.into_iter().sum::<i32>()}).sum::<i32>();
	println!("{}", new_matrix);
}