use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;
use std::str::FromStr;
use std::fs::File;
use regex::Regex;

fn parsing_file() -> io::Result<Vec<HashMap<String, i16>>> {
	let file_name = "../test_day_15.txt";
	let file = File::open(file_name)?;
	let reader = BufReader::new(file);
	let re = Regex::new(r"\w+: capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (-?\d+)").unwrap();
	
	let mut ingridient: HashMap<String, i16>;
	let mut ingridients: Vec<HashMap<String, i16>> = Vec::new();

	for line in reader.lines() {
		match re.captures(&line.unwrap()) {
			Some(caps) => {
				ingridient = HashMap::new();
				let indicators = ["capacity", "durability", "flavor", "texture", "calories"];
				for i in 0..indicators.len() {
					ingridient.insert(indicators[i].to_string(), i16::from_str(&caps[i+1]).unwrap());	
				}

				ingridients.push(ingridient);
			},
			None => continue
		}
	}
	Ok(ingridients)
}

pub fn run() {
	let ingridients: Vec<HashMap<String, i16>>;
	let n = 100;

	if let Ok(i) = parsing_file() {
		ingridients = i;
	} else {
		panic!("something gone wrong");
	}
	let mut nums: Vec<Vec<i16>> = vec![Vec::new()];
	// println!("{:?}", ingridients);
	
	

	//генерим всевозможные варианты массива из 4 элементов сумма которых будет равна 100, и значение каждого по крайней мере не меньше 1
	//FIXME not working
	for i in 1..n {
		let mut temp_array: Vec<i16> = [0; ingridients.len()];
		temp_array[0] = i;
		
		for j in 1..(n-i-2) {
			temp_array[1] = j;
			
			for k in 1..(n-i-j-1) {
				temp_array[2] = k;
				let summ: i32 = temp_array.iter().sum();
				temp_array[3] = 100 - summ;	

				nums.push(temp_array.clone());
				temp_array[3] = 0;		
			}
		}
	}

	// println!("{:?}", nums);
}