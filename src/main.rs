fn main() {
    println!("Hello, world!");
}

struct Address {
    dist: String,
    str: String,
    num: u8,
}

impl Address {
    fn new(dist: String, str: String, num: u8) -> Self {
        Address { dist, str, num }
    }
    fn is_same(&self, add: &Address) -> bool {
        self.dist == add.dist && self.str== add.str && self.num == add.num
    }
}

struct Building {
    add: Address,
    build_year: u8,
}

impl Building {
    fn new(dist: String, str: String, num: u8, build_year: u8) -> Self {
        Building { add: Address::new(dist, str, num), build_year }
    }
}

fn is_unique(b: &Building, vec: &Vec<Building>) -> bool {
    for cur in vec {
        if cur.add.is_same(&b.add){
            return false;
        }
    }
    true
}
