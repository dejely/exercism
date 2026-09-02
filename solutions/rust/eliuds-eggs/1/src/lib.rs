pub fn egg_count(display_value: u32) -> usize {
    let mut eggs = 0;
    let mut number = display_value;
    
    while number != 0{
        if number & 1 == 1{
            eggs += 1
        }

        number >>= 1; // shift right
    }

        dbg!(eggs)
    
}
