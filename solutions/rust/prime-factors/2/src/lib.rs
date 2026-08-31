pub fn factors(n: u64) -> Vec<u64> {
    let mut divisors = Vec::<u64>::new();
    let mut num = n;
    let mut i = 2;
    while i <= num{
        if num % i == 0{
            divisors.push(i);
            num = num / i;  // implies n is divisible by i
            i = 2;
        }else{
            i += 1
        }
    }
    dbg!(divisors)
}