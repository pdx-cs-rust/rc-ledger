use std::cell::RefCell;

struct Assets<'a> {
    count: u64,
    total: &'a RefCell<i64>,
}

struct Debits<'a> {
    count: u64,
    total: &'a RefCell<i64>,
}

impl Assets<'_> {
    fn acquire(&mut self) {
        *self.total.borrow_mut() += 1;
        self.count += 1;
    }
}

impl Debits<'_> {
    fn loan(&mut self) {
        *self.total.borrow_mut() -= 1;
        self.count += 1;
    }
}

fn main() {
    let counter = RefCell::new(0);
    let mut debits = Debits { count: 0, total: &counter };
    let mut assets = Assets { count: 0, total: &counter };
    assets.acquire();
    debits.loan();
    println!("{}", *counter.borrow());
}
