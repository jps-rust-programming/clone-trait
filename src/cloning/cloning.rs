#[derive(Debug)]
pub struct Container<T> {
    items: Vec<T>,
}

impl<T: Clone + std::fmt::Debug> Container<T> {
   pub fn new(items: Vec<T>) -> Self {
        Container { items }
    }

    pub fn clone_elements(&self) -> Vec<T> {
        self.items.clone()
    }

    pub fn display(&self) {
        println!("{:?}", self.items);
    }
}
