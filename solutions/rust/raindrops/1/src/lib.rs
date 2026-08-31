pub fn raindrops(n: u32) -> String {
    let numbers = [3,5,7];
    let mut text = String::from("");

    for i in 0..3{
        if n % numbers[i] == 0{
            if numbers[i] == 3{
                text.push_str("Pling");
            }
            if numbers[i] == 5{
                text.push_str("Plang");
            }
            if numbers[i] == 7{
                text.push_str("Plong");
            } 
        }
    }

    if text.is_empty(){
                text.push_str(&n.to_string());
    }

    dbg!(text)
    
}
