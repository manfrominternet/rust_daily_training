pub fn parse_int(s: &str) -> Result<i32, String> {
    // TODO: Parse the string as an i32
    s.trim().parse::<i32>().map_err(|e| e.to_string())
}
