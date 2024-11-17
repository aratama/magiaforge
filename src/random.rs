pub fn random_select<T: Copy>(xs: &mut Vec<T>) -> T {
    xs.remove((rand::random::<usize>() % xs.len()) as usize)
}
