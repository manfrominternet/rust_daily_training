use std::fmt::{self, Display, Formatter, Write};

/// Build a greeting message using format!
pub fn build_greeting(name: &str, age: u32) -> String {
    // TODO: Use format!() to create a greeting
    // Format: "Hello, {name}! You are {age} years old."
    let greeting = format!("Hello, {}! You are {} years old.", name, age);
    greeting
}

/// Build a numbered list from items using write!
pub fn build_list(items: &[&str]) -> String {
    // TODO: Create a numbered list using std::fmt::Write
    // Format: "1. item1\n2. item2\n3. item3"
    let mut output = String::new();
    for (i, item) in items.iter().enumerate() {
        write!(output, "{}. {}\n", i + 1, item).unwrap();
    }
    output
}

/// A person with a name and age.
#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

// TODO: Implement Display for Person
// Format: "Name (Age years old)"
impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // TODO: Write the person's info in the format
        write!(f, "{} ({} years old)", self.name, self.age)
    }
}
