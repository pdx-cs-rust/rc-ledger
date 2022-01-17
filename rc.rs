use std::rc::Rc;
use std::cell::RefCell;

struct Assets {
    count: u64,
    total: Rc<RefCell<i64>>,
}

struct Debits {
    count: u64,
    total: Rc<RefCell<i64>>,
}

impl Assets {
    fn acquire(&mut self) {
        *self.total.borrow_mut() += 1;
        self.count += 1;
    }
}

impl Debits {
    fn loan(&mut self) {
        *self.total.borrow_mut() -= 1;
        self.count += 1;
    }
}

fn main() {
    let counter = Rc::new(RefCell::new(0));
    let mut debits = Debits { count: 0, total: Rc::clone(&counter) };
    let mut assets = Assets { count: 0, total: Rc::clone(&counter) };
    assets.acquire();
    debits.loan();
    println!("{}", *counter.borrow());
}
