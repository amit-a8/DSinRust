fn find_product(arr: Vec<i32>)->Vec<i32> {
    // write your code here
    let length = arr.len();
    println!("input array - {:?}", arr) ;
    let mut product: Vec<i32> = vec![0;arr.len()] ;
    product[0] = 1 ;
    for i in 1..length{
        product[i] = product[i-1] * arr[i-1] ;
    }
    let mut right = 1 ;
    for i in  (0..length).rev(){
        product[i] = product[i] * right ;
        right = right * arr[i] ;
    }
    println!("product till now {:?}", product) ;
    return product;
}

fn main(){
    let page_scores: Vec<i32> = vec![3, 5, 1, 1, 6, 7, 2, 3, 4, 1, 2];
    let ranking: Vec<i32> = find_product(page_scores);
    println!("{:?}",ranking);
}