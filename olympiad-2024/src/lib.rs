use std::collections::HashSet;

use tracing::info;

pub fn question_5() {
	let height = 0..(2023 / 6);
	for h in height {
		let hf = h * 6;
		let width = (1 + hf)..=(6 + hf);

		let mut str = String::new();
		for n in width {
			let mut available_pairs = 3;
			let nstr = {
				if n % 7 != 0 {
					format!(" {:4},", n)
				} else {
					// available_pairs -= 1;
					"     ,".into()
				}
			};
			str.push_str(&nstr);
		}
		info!(%str, %h, "Combinations = ");
		// info!("Total combinations = 336 * 6 = {}", 336 * 6);
	}
}

pub fn question_4() {
	// How many even integers between 6000 and 9000 have four different digits?
	let numbers = 6000..=9000;
	let mut sum = 0u32;
	for n in numbers {
		if n % 2 == 0 {
			let decomposition = decompose_digits(n);
			let are_all_different = digits_are_all_different(decomposition);
			if are_all_different {
				tracing::info!("{} has all different digits", n);
				sum += 1;
			}
		}
	}

	tracing::info!(%sum, "Found total");
}

fn decompose_digits(num: u32) -> [u32; 4] {
	let mut digits = [0u32; 4];
	let mut n = num;
	for i in (0..4).rev() {
		digits[i] = n % 10;
		n /= 10;
	}
	digits
}

fn digits_are_all_different(digits: [u32; 4]) -> bool {
	let original_len = digits.len();
	let de_dupped = HashSet::from(digits);
	original_len == de_dupped.len()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn decompose_digits_examples() {
		let inputs = [1000, 1234];
		let expected_outputs = [[1, 0, 0, 0], [1, 2, 3, 4]];

		for i in 0..inputs.len() {
			assert_eq!(decompose_digits(inputs[i]), expected_outputs[i]);
		}
	}

	#[test]
	fn digits_different() {
		let digits = [1000, 1234];
		let expected_outputs = [false, true];

		for i in 0..digits.len() {
			assert_eq!(
				digits_are_all_different(decompose_digits(digits[i])),
				expected_outputs[i]
			);
		}
	}
}
