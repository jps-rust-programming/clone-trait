pub trait Summary {
    fn summarize(&self) -> String;
    fn hi(&self);
}

impl<T> Summary for T
where
    T: ToString,
{
    fn summarize(&self) -> String {
        self.to_string()
    }
    fn hi(&self) -> () {
        println!("Hi, Calling for the result;")
    }
}

pub struct Article {
    pub title: String,
    pub body: String,
}

impl ToString for Article {
    fn to_string(&self) -> String {
        format!("{} - {}", self.title, self.body)
    }
}
