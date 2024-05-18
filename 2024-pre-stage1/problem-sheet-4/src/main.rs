use tracing::*;

fn main() {
	tracing_subscriber::fmt::init();

	problem_3();
}

/// Reading from left to right, a sequence consists of 6 A’s,
/// followed by 24 B’s,
/// followed by 96 A’s.
/// After the first n letters (from left to right), one letter has occurred twice as many times as the other letter. What is the sum of all the possible values of n?
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