mod blank;
mod cloning;
use crate::blank::blank::Summary;
mod inheritance;

use crate::inheritance::traits::colorful::Colorful;
use crate::inheritance::traits::shape::Shape;


#[derive(Clone, Debug)]
struct AppConfig {
    username: String,
    theme: String,
    language: String,
    max_connections: u32,
}

impl AppConfig {
    fn new(username: &str, theme: &str, language: &str, max_connections: u32) -> Self {
        AppConfig {
            username: username.to_string(),
            theme: theme.to_string(),
            language: language.to_string(),
            max_connections,
        }
    }

    fn display(&self) {
        println!(
            "App Config - Username: {}, Theme: {} Language: {}, Max Connections: {}",
            self.username, self.theme, self.language, self.max_connections
        )
    }
}

fn apply_config(config: AppConfig) {
    println!("Applying configuration: {:?}", config);
}

fn main() {
    // println!("Hello, world!");
    let default_config = AppConfig::new("user name123", "dark", "english", 100);

    default_config.display();

    let mut module_config = default_config.clone();
    module_config.theme = "light".to_string();
    module_config.max_connections = 50;

    module_config.display();

    apply_config(default_config.clone());

    apply_config(module_config.clone());

    // cloning

    let int_container = cloning::cloning::Container::new(vec![1, 2, 3, 4, 5]);

    println!("Original container:");
    int_container.display();

    let article = blank::blank::Article {
        title: String::from("Rust Programming"),
        body: String::from("Rust Programming"),
    };

    println!("{}", article.summarize());
    // println!("{}", article.hi());
    article.hi();

    let circle = inheritance::heritance::Circle{
        radius: 5.0,
        color: String::from("Red")
    };

    println!("Area of the circle: {}", circle.area());
    println!("Color of the circle: {}", circle.color());
}
