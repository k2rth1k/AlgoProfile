pub mod utils {
    pub fn new_numeric_vector_with_size<T: Default>(size: usize) -> Vec<T> {
        let mut vec: Vec<T> = Vec::with_capacity(size);
        vec.resize_with(size, || Default::default());
        vec
    }

    pub fn new_numeric_vector_with_size_and_range<
        T: Copy + PartialOrd + rand::distr::uniform::SampleUniform,
    >(
        size: usize,
        range_start: T,
        range_end: T,
    ) -> Vec<T> {
        use rand::RngExt;
        let mut rng = rand::rng();
        let mut vec: Vec<T> = Vec::with_capacity(size);
        vec.resize_with(size, || rng.random_range(range_start..=range_end));
        vec
    }

    pub fn random_numeric<T: Copy + PartialOrd + rand::distr::uniform::SampleUniform + Default>(
        range_start: T,
        range_end: T,
    ) -> T {
        use rand::RngExt;
        let mut rng = rand::rng();
        rng.random_range(range_start..=range_end)
    }
}
