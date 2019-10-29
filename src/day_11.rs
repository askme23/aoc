use fancy_regex::Regex;

fn is_contains_straight_increasing_symbols(password: &String) -> bool {
	let mut ascii_vec: Vec<u8> = Vec::new();
	let mut is_contain = false;

	for ch in password.chars() {
		ascii_vec.push(ch as u8)
	}

	for index in 0..(ascii_vec.len()-3) {
		if ascii_vec[index] == ascii_vec[index+1] - 1 && ascii_vec[index+1] == ascii_vec[index+2] - 1 {
			is_contain = true;
			break;
		}
	}
	// println!("{}", is_contain);
	is_contain
}

fn increment_password(password: String) -> String {
	let mut ascii_vec: Vec<u8> = password.chars().map(|x| x as u8).collect();
	let restricted_sybmols: Vec<u8> = vec![105, 108, 111];
	let vec_len = ascii_vec.len();
	let mut index = 1;

	loop {
		if ascii_vec[vec_len-index] == 122 {
			ascii_vec[vec_len-index] = 97;
			index += 1;
			continue;
		} else {
			if restricted_sybmols.contains(&(ascii_vec[vec_len-index] + 1)) {
				ascii_vec[vec_len-index] += 2;	
			} else {
				ascii_vec[vec_len-index] += 1;
			}
			break;
		}
	}

	ascii_vec.iter().map(|x| *x as char).collect::<Vec<char>>().iter().collect()
}

pub fn run() {
	let new_password;
	let mut password = "cqjxxyzz".to_string();
	let re = Regex::new(r"(\w)\1.*(\w)\2").unwrap();
	
	loop {
		password = increment_password(password);
		println!("{}", password);
		if is_contains_straight_increasing_symbols(&password) && re.is_match(&password).unwrap() {
			new_password = password.clone();
			break;							
		}
	}

	println!("{:?}", new_password);
}