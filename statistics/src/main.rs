use std::collections::HashMap;
fn main() {
    let list: Vec<i32> = vec![1, 2, 3, 4, 5];
    let list1 = vec![1, 2, 3];
    let list2 = vec![-1, 1, 3];
    println!("{}", median(&list));
    println!("{}", median(&list1));
    println!("{}", median(&list2));
    println!("{}", mean(&list));
    println!("{}", mean(&list1));
    println!("{}", mean(&list2));
    println!("{}", mode(&vec![1, 2, 2, 3, 4]));
}

fn median(list: &Vec<i32>) -> i32 {
    list[list.len()/2]
}

fn mean(list: &Vec<i32>) -> f32 {
    let mut sum: i32 = 0;
    for i in list.iter() {
        sum += i;
    }
    sum as f32 / list.len() as f32
}

fn mode(list: &Vec<i32>) -> i32 {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for v in list.iter() {
        let a = counts.entry(*v).or_insert(0);
        *a = *a + 1;
    } 
    let mut max_val = 0;
    let mut max_key = 0;
    for (k, v) in counts.iter() {
        if *v > max_val {
            max_val = *v;
            max_key = *k;
        }
    }
    max_key
}
