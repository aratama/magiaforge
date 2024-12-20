pub fn random_select_mut<T: Copy>(xs: &mut Vec<T>) -> T {
    xs.remove((rand::random::<usize>() % xs.len()) as usize)
}
