pub fn square(s: u32) -> u64 {
    let square;
    
    square = 2_u64.pow(s-1);

    dbg!(square);
    square
}

pub fn total() -> u64 {
    let mut sum = 0;

    for i in 0..64{
        sum += 2_u64.pow(i);
    }

    sum
    
}
