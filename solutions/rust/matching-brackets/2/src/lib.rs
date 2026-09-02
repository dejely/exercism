pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();

    for ket in string.chars() {
        match ket {
            '(' | '{' | '[' => {
                stack.push(ket);
            }

            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }

            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }

            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '"' => {
                if stack.pop() != Some('"') {
                    return false;
                }
            }

            _ => {} // anything else, do nothing
        }
    }

    stack.is_empty() // return true if empty
}