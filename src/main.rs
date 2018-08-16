#![feature(rust_2018_preview, uniform_paths)]

mod digits;
mod permutations;

fn main() {
    use std::env;

    let inputs = env::args().skip(1).filter_map(as_integers);
    for input in inputs {
        println!("{}", sum_of(&input));
    }
}

fn sum_of(s: &[u64]) -> u64 {
    use permutations::Permutations;

    let mut s: Vec<_> = s.into_iter().cloned().collect();

    s.sort();
    s.permutations().map(|s| build_number(s)).sum()
}

fn as_integers<S: AsRef<str>>(input: S) -> Option<Vec<u64>> {
    use digits::Digits;

    let n: u64 = input.as_ref().parse().ok()?;
    Some(Digits::new(n).collect())
}

fn build_number(s: &[u64]) -> u64 {
    s.iter().fold(0, |mut a, &b| {
        a *= 10;
        a += b;
        a
    })
}
