enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Color {
    fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        }
        return false;
    }
    fn is_parts(&self) -> bool {
        match self {
            Color::Blue => return true,
            Color::Green => return true,
            Color::Yellow => return true,
            _ => return false,
        }
    }
}

struct Custom {
    age: usize,
    name: String,
}

enum Item {
    Number(usize),
    String(String),
    MyCustom(Custom),
}

fn main() {
    iterator_example();

    read_fs_lines();

    print_color(Color::Red);
    println!("{:?}", Color::Green.is_green());
    println!("{:?}", Color::Red.is_parts());

    let mut items: Vec<Item> = vec![];
    append_string(&mut items);

    // Not allowed, Strings are not numbers... duh.
    //let mut items: Vec<usize> = vec![];
    //append_string(&mut items);
    
    let my_num = Some(5);
    multiply(my_num);
}

fn practice(list: Vec<usize>, index: usize) -> usize {
    return list.get(index).unwrap_or(&index) * 5;
}

fn multiply(num: Option<usize>) -> Option<usize> {
    // When taking & return an Option the ? operator can be used to return None early
    return Some(num? * 5);
}

fn append_string(items: &mut Vec<Item>) {
    items.push(Item::String("hello world".into()));
}

fn print_color(color:Color){
    match color {
        Color::Red => println!("red"),
        Color::Blue => println!("blue"),
        Color::Green => println!("green"),
        _ => println!("unhandled color"),
    }
}

fn read_fs_lines(){
    let file = std::fs::read_to_string("lines").unwrap();
    file
        .lines()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .skip(2)
        .take(2)
        .for_each(|(_, line)| println!("{}", line));
}

fn iterator_example() {
    let data = vec![1, 2, 3];

    // [Type] -> [Iterator] -> [Type]
    let mut foo = data
                .iter()
                .map(|x| x + 1);

    let mut new_vec = vec![];

    while let Some(x) = foo.next(){
        new_vec.push(x);
    }

    println!("{:?}", new_vec);
}
