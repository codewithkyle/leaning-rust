#[derive(Debug)]
struct Item {
    count: usize,
}

fn add_one(item: &mut Item){
    item.count += 1;
}

fn print_all(items: &Vec<Item>) {
    for item in items {
        println!("{:?}", item);
    }
}

fn main(){
    let mut items = vec![Item { count: 1 }];
    let mut first = items.get_mut(0);
    println!("{:?}", first);
    match first {
        Some(ref mut first) => add_one(first),
        _ => {}
    }
    println!("{:?}", first);
    let second = items.get_mut(1);
    println!("{:?}", second);
}
