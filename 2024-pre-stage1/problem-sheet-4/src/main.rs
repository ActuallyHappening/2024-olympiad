use tracing::*;

fn main() {
	tracing_subscriber::fmt::init();

	// problem_3();
	problem_4();
	// problem_6();
}

/// Reading from left to right, a sequence consists of 6 A’s,
/// followed by 24 B’s,
/// followed by 96 A’s.
/// After the first n letters (from left to right), one letter has occurred twice as many times as the other letter. What is the sum of all the possible values of n?
#[instrument]
pub fn problem_3() {
	let sequence: String = {
		let mut sequence = String::new();

		for _ in 0..6 {
			sequence.push('A');
		}

		for _ in 0..24 {
			sequence.push('B');
		}

		for _ in 0..96 {
			sequence.push('A');
		}

		trace!(%sequence);

		sequence
	};

	let mut count = 0;
	for n in 0..sequence.len() {
		let a_count = sequence.chars().take(n).filter(|&c| c == 'A').count();
		let b_count = sequence.chars().take(n).filter(|&c| c == 'B').count();

		if a_count == 2 * b_count || b_count == 2 * a_count {
			info!(n, a_count, b_count);
			count += n;
		}
	}

	info!("Total: {}", count);
}

/// ** Alan using identical square cards makes a rectangle measuring m cards by n cards.
/// After Alan is done, Cath comes and adds a border of width one card,
/// and the area of the border is equal to the original area of Alan’s rectangle.
/// What is the sum of all possible values of m?
#[instrument]
pub fn problem_6() {
	let mut m_count = 0;
	const MAX: u32 = 100;
	for m in 0..MAX {
		for n in 0..MAX {
			let original_area = m * n;
			let border_area = (m + 2) * (n + 2) - original_area;

			if original_area == border_area {
				info!(m, n, original_area, border_area);
				m_count += m;
			}
		}
	}

	info!(
		"The total m count is: {}, which counted from 0 to {} for both m and n",
		m_count, MAX
	);
}

/// In a sequence of numbers, each term after the first two terms
/// is the average of all the terms which come before that term.
/// If the first term is 10 and the twelfth term is 28,
/// what is the sum of the second and the fifth term?
///
/// Deduced example:
/// 1, 2, 1.5, 1.5, 1.5 ...
#[instrument]
pub fn problem_4() {
	use num_rational::Ratio;

	type Num = i32;
	type List = Vec<Ratio<Num>>;
	type ListRef<'a> = &'a [Ratio<Num>];


	fn average_of(list: ListRef) -> Ratio<Num> {
		let sum: Ratio<Num> = list.iter().sum();
		sum / Ratio::new(list.len() as Num, 1)
	}

	const NUM_MAX: Num = 50;
	const DENOM_MAX: Num = 50;
	for guess_num in 0..NUM_MAX {
		for guess_denom in 1..DENOM_MAX {
			let guess = Ratio::new(guess_num, guess_denom);

			let mut list = vec![Ratio::from(12), guess];
			for _len in 3..=12 {
				let average = average_of(&list);
				list.push(average);
			}

			let twelfth_term = list[11];
			if twelfth_term == Ratio::from(28) {
				info!(?twelfth_term, "Found correct guess: {:?}", guess);
				trace!(?list, ?guess);

				{
					let mut debug_str = String::from("[");

					for term in list {
						debug_str.push_str(&format!("{}, ", term));
					}

					debug_str.push(']');

					trace!(%debug_str);
				}
			}
		}
	}
}
