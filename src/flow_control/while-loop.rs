pub fn countdown(n: u32) -> Vec<u32> {
    // TODO: Implement the countdown using a while loop
    // A counter variable
    let mut nm: u32 = n;
    let mut vec: Vec<u32> = Vec::new();
    while nm != 0 {
        vec.push(nm);
        nm -= 1;
    }
    let zero: u32 = 0;
    vec.push(zero);
    vec
}
