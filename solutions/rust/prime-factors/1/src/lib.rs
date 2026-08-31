pub fn factors(n: u64) -> Vec<u64> {

    let mut divisors = Vec::<u64>::new();
    let mut num = n;
    let mut i = 2;

    if n == 2_u64{
        return [2].to_vec();
    }else if n == 3_u64{
       return [3].to_vec();
    }

    while i <= n{
        if n % i == 0{
            divisors.push(i);
            num = num / i;  // implies n is divisible by i
            i = 2;
            
        }
    }
    dbg!(divisors)

}
