use std::str::FromStr;

/// Parse a string into an i32, returning a descriptive error message on failure.
pub fn parse_int(s: &str) -> Result<i32, String> {
    // TODO: Parse the string as an i32
    s.trim().parse::<i32>().map_err(|e| e.to_string())
}

/// Parse common boolean representations (case-insensitive).
/// Accepts: "true", "false", "1", "0", "yes", "no"
pub fn parse_bool(s: &str) -> Result<bool, String> {
    // TODO: Match on the lowercase version of the string
    let result = match s.trim().to_lowercase().as_str() {
        "true" | "1" | "yes" => Ok(true),
        "false" | "0" | "no" => Ok(false),
        _ => Err(String::from("String as Error")),
    };
    result
}

/// Parse a "key=value" string into a tuple.
pub fn parse_key_value(s: &str) -> Result<(String, String), String> {
    // TODO: Split the string at '=' and return (key, value)
    let result = match s.trim().split_once("=") {
        None => Err(String::from("error")),
        Some((key, value)) => Ok((key.trim().to_string(), value.trim().to_string())),
    };
    result
}

/// A color represented by red, green, and blue components.
#[derive(Debug, PartialEq, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// TODO: Implement FromStr for Color
// The format is "r,g,b" (e.g., "255,128,0")
impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: Parse the "r,g,b" format
        let vec: Vec<&str> = s.split(",").collect();
        if vec.len() != 3 {
            return Err(String::from("wrong input"));
        }
        let r = vec[0].trim().parse::<u8>().map_err(|e| e.to_string())?;
        let g = vec[1].trim().parse::<u8>().map_err(|e| e.to_string())?;
        let b = vec[2].trim().parse::<u8>().map_err(|e| e.to_string())?;
        Ok(Color { r, g, b })
    }
}

/// Parse a delimited list of values into a Vec.
pub fn parse_list<T: FromStr>(s: &str, delimiter: char) -> Result<Vec<T>, String> {
    // TODO: Split by delimiter, parse each part, collect into Result<Vec<T>, String>
    let mut vec = Vec::new();
    for el in s.split(delimiter) {
        // 1. Parsujemy i od razu wyciągamy wartość operatorem ?
        // Jeśli parse się nie uda, funkcja tutaj przerywa (return) i zwraca Err
        let parsed: T = el
            .trim()
            .parse::<T>()
            .map_err(|_| "Error parsing".to_string())?;

        // 2. Do wektora trafia już CZYSTE T, a nie Result
        vec.push(parsed);
    }
    Ok(vec)
}
