pub fn parsing_file() {
	let mut test_text = "1321131112".to_string();
	let iteration = 50;

	for _ in 0..iteration {
		let mut temp_string: String = "".to_string();
		let mut current_char: String = "".to_string();
		let mut char_cnt = 0;

		for ch in test_text.chars() {
			if current_char == "".to_string() {
				current_char = ch.to_string();
				char_cnt += 1;
				continue;
			}

			if ch.to_string() != current_char {
				temp_string.push_str(&(char_cnt.to_string() + &current_char));
				
				char_cnt = 1;
				current_char = ch.to_string();
				continue;
			}
			char_cnt += 1;
		}
		temp_string.push_str(&(char_cnt.to_string() + &current_char));
		test_text = temp_string.clone();
	}

	println!("{}", test_text.len());
}