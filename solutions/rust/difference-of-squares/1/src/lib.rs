pub fn square_of_sum(n: u32) -> u32 {
    let x: u32 = (1 .. n+1).sum();
    x * x
}

pub fn sum_of_squares(n: u32) -> u32 {
    (1 .. n+1).map(|a| a*a).sum()
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}