use permutohedron::LexicalPermutation;
use std::cmp::Ord;

// An external iterator for this purpose is impractical in Rust. As such, we provide an internal
// iterator as an alternative.
//
// I should also note that, although such an iterator might be expressed in C#, it would exhibit
// soundness problems regarding the lifetime of the array references produced.

pub struct PermutationsIter<'a, T: 'a>(&'a mut [T]);

impl<'a, T: Ord> PermutationsIter<'a, T> {
    pub fn for_each<F: FnMut(&[T])>(&mut self, mut f: F) {
        f(self.0);
        while self.0.next_permutation() {
            f(self.0)
        }
    }

    pub fn map<U, F: FnMut(&[T]) -> U>(&'a mut self, mut f: F) -> IterMap<'a, T, U, F> {
        IterMap { source: self, complete: false, f }
    }
}

pub struct IterMap<'a, T: 'a, U, F: FnMut(&[T]) -> U> {
    source: &'a mut PermutationsIter<'a, T>,
    complete: bool,
    f: F,
}

impl<'a, T, U, F> Iterator for IterMap<'a, T, U, F>
where
    T: Ord + 'a,
    F: FnMut(&[T]) -> U,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.complete {
            let result = (self.f)(self.source.0);
            self.complete = self.source.0.next_permutation();
            Some(result)
        } else {
            None
        }
    }
}

pub trait Permutations<T> {
    fn permutations<'a>(&'a mut self) -> PermutationsIter<'a, T>;
}

impl<T> Permutations<T> for [T] {
    fn permutations<'a>(&'a mut self) -> PermutationsIter<'a, T> {
        PermutationsIter(self)
    }
}
