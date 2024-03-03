fn returns<'a>(input: &'a Vec<char>) -> std::iter::Take<std::iter::Skip<std::slice::Iter<'a, char>>> {
    input.iter().skip(4).take(5)
}




type SkipFourTakeFive<'a> = std::iter::Take<std::iter::Skip<std::slice::Iter<'a, char>>;

fn returns<'a>(input: &'a Vec<char>) -> SkipFourTakeFive {
    input.iter().skip(4).take(5)
}




use std::iter::{Take, Skip};
use std::slice::Iter;

fn returns<'a>(input: &'a Vec<char>) -> Take<Skip<Iter<'a, char>>> {
    input.iter().skip(4).take(4)
}
