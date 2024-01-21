struct Item {
    number: u8,
}

impl Item {
    fn compare_number(&self, other_number: u8) {
        println!("Are {} and {} equal? {}", self.number, other_number, self.number == other_number);
    }
}

fn main() {
    let item = Item {
        number: 8,
    };

    let ref_item = &item;
    let ref_item_two = &ref_item;

    item.compare_number(9);
    ref_item.compare_number(8);
    ref_item_two.compare_number(8);
}
