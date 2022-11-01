// extern crate priority_queue; 
// use priority_queue::PriorityQueue;
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
    // let mut pq = PriorityQueue::new();
    // for (key,val) in map.iter() {
    //     pq.push(key, *val);
    // }
    // println!("{:?}", pq) ;
}

fn main(){
    let test = "aabbccc".to_string();
    mapping(test) ;

}
