use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str::FromStr;
use std::iter::FromIterator;

fn parsing_file() -> io::Result<Vec<Vec<i16>>> {
	let file_name = "../test_day_9.txt";
	let file = File::open(file_name)?;
	let reader = BufReader::new(file);

	let re = Regex::new(r"(\w+)\sto\s(\w+)\s=\s(\d+)").unwrap();
	let mut matrix: Vec<Vec<i16>> = vec![vec![0]];
	let mut current_city: String = "".to_string();
	let mut cnt: i16 = 0;

	for line in reader.lines() {
		match re.captures(&line.unwrap()) {
			Some(caps) => {
				let from = caps.get(1).unwrap().as_str();
				let weight = i16::from_str(&caps[3]).unwrap();

				if current_city == "".to_string() {
					current_city = from.to_string();
				}
				
				if from.to_string() != current_city {
					current_city = from.to_string();
					cnt += 1;
					let mut temp_vec: Vec<i16> = Vec::new();
					for i in 0..(cnt+1) {
						if i == cnt {
							temp_vec.push(0);
						} else {
							temp_vec.push(matrix[i as usize][cnt as usize]);
						}
					}
					matrix.push(temp_vec);
				}

				matrix[cnt as usize].push(weight);
			},
			None => continue
		}
	}

	Ok(matrix)
}

pub fn run(mat: io::Result<Vec<Vec<i16>>>) {
	let mut optimal_way: i16 = std::i16::MIN;
	let mut temp_vec: Vec<i16> = Vec::new();
	let mut matrix: Vec<Vec<i16>>;

	if let Ok(ma) = mat {
		matrix = ma;
	} else {
		panic!("Something gone wrong");
	}

	// println!("{:?}", matrix);
	let len = matrix[0].len() as i16;
	for i in &matrix {
		temp_vec.push(i[(len-1) as usize]);
	}
	temp_vec.push(0);
	matrix.push(temp_vec);

	let mut arr: Vec<i16> = vec![0i16; matrix[0].len()];
	for i in 0..arr.len() {
		arr[i] = i as i16;
	}

	let mut combinations: Vec<Vec<i16>> = vec![Vec::from_iter(arr[..].iter().cloned())];
	loop {
		let mut i: i16 = -1;
		for index in 0..arr.len()-1 {
			if arr[index] < arr[index+1] {
				i = index as i16;
			}
		}
		if i == -1 {
			break;
		}
		
		let mut j = i+1;
		let mut minimum: i16 = std::i16::MAX;
		for index in (i+1) as usize..arr.len() {
			if arr[index] < minimum && arr[index] > arr[i as usize] {
				j = index as i16;
				minimum = arr[index];
			}
		}
		
		// println!("i: {}, j: {}", i, j);
		arr.swap(i as usize, j as usize);

		let mut temp: Vec<i16> = arr[(i+1) as usize..].to_vec();
		temp.reverse();
		arr.truncate((i+1) as usize);
		arr.append(&mut temp);
		// println!("{:?}", arr);
		combinations.push(Vec::from_iter(arr[..].iter().cloned()));
	}

	// println!("{:?}", combinations);
	for val in combinations {
		let mut temp_way = 0;
		let mut i = val[0];
		
		for j in val[1..].into_iter() {
			// println!("i: {}, j: {}", i, *j);
			// println!("element: {}", matrix[i as usize][*j as usize]);
			temp_way += matrix[i as usize][*j as usize];
			i = *j
		}

		temp_way += matrix[i as usize][val[0] as usize];
		
		if temp_way > optimal_way {
			optimal_way = temp_way;
		}
	}

	println!("{}", optimal_way);
}