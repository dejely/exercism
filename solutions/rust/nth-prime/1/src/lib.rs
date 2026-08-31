pub fn nth(n: u32) -> u32 {

    let mut i: u32 = 2;
    let mut primes = Vec::<u32>::new();

    while primes.len()  <= n as usize{
        if is_prime(i){
            primes.push(i);
        }
        i += 1;
    }
    let nth = primes[n as usize];

    dbg!(nth)
}

pub fn is_prime(n: u32) -> bool {
    
    let mut i = 2;

    while i * i <= n {  // i * i since pairs repeat
        if n % i == 0 {
            return false;
        }
        i += 1;
    }

    true
}