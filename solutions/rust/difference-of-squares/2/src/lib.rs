pub fn square_of_sum(n: u32) -> u32 {
    let mut sum = 0;
    for i in 1..n+1 { // loops to n and adds it
        sum = sum + i;
        dbg!(sum);
    }

    return sum * sum;
    
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut squares = 0;

    for i in 1..n+1 {
        squares += i * i;

        dbg!(squares);
    }
    return squares;
}

pub fn difference(n: u32) -> u32 {

    return square_of_sum(n) - sum_of_squares(n);
}
