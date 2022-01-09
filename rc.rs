use std::cell::RefCell;

fn inc(counter: &RefCell<u8>) {
    *counter.borrow_mut() += 1;
}

fn dec(counter: &RefCell<u8>) {
    *counter.borrow_mut() -= 1;
}

fn main() {
    let counter = RefCell::new(0);
    inc(&counter);
    dec(&counter);
    println!("{}", *counter.borrow());
}
