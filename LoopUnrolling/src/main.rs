fn decode_string( s: String)->String {
    // write your code here
    let mut count_stack:Vec<i32> = Vec::new();
    let mut stmt_stack:Vec<String> = Vec::new();
    let mut current_stmt = String::from("") ;
    let mut n = 0 ;
    for i in 0..s.len(){
        let ch = s.chars().nth(i).unwrap();
        if ch.is_digit(10){
            n = n * 10 + ch.to_digit(10).unwrap() as i32 ;

        }else if ch == '['{
            count_stack.push(n);
            stmt_stack.push(current_stmt) ;
            current_stmt = "".to_string();
            n = 0 
        }else if ch==']'{
            let mut unroll_stmt:String = stmt_stack.last().unwrap().to_string();
            stmt_stack.pop();
            let mut current_n:i32 = *count_stack.last().unwrap() ;
            count_stack.pop();
            while current_n > 0{
                unroll_stmt += &current_stmt ;
                current_n -= 1 ;
            }
            current_stmt = unroll_stmt ;
        }
        else{
            current_stmt.push(ch);
        }
    }
    return current_stmt;
}

fn main(){
    let code_block: String =  String::from("2[sum = sum + i; 2[i++; ]]");
    println!("{}",decode_string(code_block));
}
