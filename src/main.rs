#[derive(Debug)]
struct Item {
    count: usize,
}

fn add_one(mut item: &Item){
    item.count += 1;
}

fn main(){
    let item = Item { count: 1 };
    println!("{:?}", item);
    add_one(item);
    println!("{:?}", item);
}
