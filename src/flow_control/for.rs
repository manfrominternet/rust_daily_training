pub fn sum_of_evens(start: i32, end: i32) -> i32 {
    let mut sum: i32 = 0;
    if start > end {
        return sum;
    }
    for n in start..=end {
        if n % 2 == 0 {
            sum += n;
        }
    }
    sum
}
