use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn parsing_file() -> io::Result<()> {
	let file_name = "../test_day_8.txt";
	let file = File::open(file_name)?;
	let reader = BufReader::new(file);
	let mut code_symbols: u32 = 0;
	let mut code_symbols_after_reg: u32 = 0;
	let mut strings: Vec<String> = Vec::new();
	// let re_ascii = Regex::new(r"\\x(\d|[abcdef])(\d|[abcdef])").unwrap();
	let re_escape1 = Regex::new(r#"""#).unwrap();
	let re_escape2 = Regex::new(r#"\\"#).unwrap();

	//FIXME можно за один проход обработать строку
	for line in reader.lines() {
		match line {
			Ok(st) => {
				for _ in st.chars() {
					code_symbols += 1;
				}

				let s = re_escape2.replace_all(&st, "\\\\").to_string();
				let s = re_escape1.replace_all(&s, "\\\"").to_string();
				// let s = re_ascii.replace_all(&s, "1").to_string();

				strings.push(s);
			},
			Err(_) => continue,
		}

	}

	//TODO коммент сверху
	for st in strings {
		for _ in st.chars() {
			code_symbols_after_reg += 1;
		}
		//прибавляем скобочки
		code_symbols_after_reg += 2;
		println!("{}", st);
	}

	println!("cs: {}, cs_after_reg: {}, div: {}", code_symbols, code_symbols_after_reg, code_symbols_after_reg - code_symbols);
	// println!("{:?}", strings);
	Ok(())
}