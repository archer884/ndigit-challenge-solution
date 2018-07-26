pub struct Digits(u64);

impl Digits {
    pub fn new(n: u64) -> Self {
        Digits(n)
    }
}

impl Iterator for Digits {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            0 => None,
            n => {
                self.0 = n / 10;
                Some(n % 10)
            }
        }
    }
}
