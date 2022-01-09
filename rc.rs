use std::cell::RefCell;

static COUNTER: RefCell<u8> = RefCell::new(0);

fn inc() {
    *COUNTER.borrow_mut() += 1;
}

fn dec() {
    *COUNTER.borrow_mut() -= 1;
}

fn main() {
    inc();
    dec();
    println!("{}", *COUNTER.borrow());
}
