use std::iter::repeat;

pub fn string_of_len(len: usize) -> String {
    repeat(' ').take(len).collect::<String>()
}

pub fn vec_of_len<T: Clone>(item: T, len: usize) -> Vec<T> {
    repeat(item).take(len).collect::<Vec<T>>()
}
