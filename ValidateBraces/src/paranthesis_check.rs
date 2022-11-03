fn is_valid(s: String)->bool {
    // Write your code here
    let mut stack:Vec<char> = Vec::new();
    let mut flag = true ;
    for c in s.chars(){
        println!("{}", c) ;
        if c == '(' || c=='{' || c=='['{
            stack.push(c);
        }
        else {
            if stack.is_empty(){
                flag = false ;
            }else{
                stack.pop();
            }
        }
    }
    return stack.is_empty() & flag;
}

fn main(){
    let s:String = "{()}}".to_string();
    let resp = is_valid(s);
    println!("{}", resp) ;
}