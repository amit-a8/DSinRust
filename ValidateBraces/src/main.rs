use std::collections::HashMap;
fn valid( s: String)->bool {
    let mut matches: HashMap<char,char> = HashMap::new();
    matches.insert(')','(');
    matches.insert('}','{');
    matches.insert(']','[');

    let mut starting_matches: HashMap<char,char> = HashMap::new();
    starting_matches.insert('{','}');
    starting_matches.insert('[',']');
    starting_matches.insert('(',')');

    let mut stack: Vec<char> = Vec::new();

    for i in 0..s.len() {
        let c: char = s.chars().nth(i).unwrap();
        
        if stack.is_empty() && matches.contains_key(&c) {
            stack.push('0'); // If a closing bracket is received first
            break;
        }
        if !stack.is_empty() && stack.last() == matches.get(&c) {
            stack.pop();
        } 

        if starting_matches.contains_key(&c) {
            stack.push(c);
        }
    }

    return stack.is_empty();
}

  //Driver code
fn main(){
  let s = "let output = function () { console.log('Anonymous function'); };".to_string();

  let is_valid = valid(s);
  println!("{:}",is_valid);
}