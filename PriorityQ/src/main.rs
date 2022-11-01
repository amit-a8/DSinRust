extern crate priority_queue; 
use priority_queue::PriorityQueue;
use std::collections::HashMap;

fn mapping(aa:String){
    let mut map: HashMap<char,i32> = HashMap::new();
    let mut freq ;
    for i in aa.chars(){
        if !map.contains_key(&i) {
            freq = 1 ;
        }else{
            freq = map[&i] + 1 ;
        }
        if !map.contains_key(&i){
            map.entry(i).or_insert(freq);
          }
          else
          {
            *map.get_mut(&i).unwrap() = freq;
          }
    }
    let mut pq = PriorityQueue::new();
    for (key,val) in map.iter() {
        pq.push(key, *val);
    }
    println!("{:?}", pq) ;
    let mut result = "".to_string();
    while !pq.is_empty() {
        let first = pq.pop().unwrap();
        println!("{:?}", first) ;
        if result.len() == 0 || *first.0 != result.chars().nth(result.len() - 1).unwrap() {
            result.push(*first.0);
            if first.1-1 > 0 {
                pq.push(first.clone().0,first.clone().1-1);
            }
        } 
        else {
            let second = pq.pop();
            result.push(*second.unwrap().0);
            if second.unwrap().1-1 > 0 {
                pq.push(second.clone().unwrap().0,second.clone().unwrap().1-1);
            }
            pq.push(first.clone().0,first.clone().1);
        }
    }
    println!("{}", result) ;
}

fn main(){
    let test = "aabbccc".to_string();
    mapping(test) ;

}

