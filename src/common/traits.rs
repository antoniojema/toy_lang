pub trait WithSize<T:Clone> {
    fn with_size(size : usize, val : &T) -> Self;
}

impl<T:Clone> WithSize<T> for Vec<T> {
    fn with_size(size : usize, val : &T) -> Vec<T> {
        let mut vec = Vec::<T>::with_capacity(size);
        for _ in 0..size {
            vec.push(val.clone());
        }
        vec
    }
}
