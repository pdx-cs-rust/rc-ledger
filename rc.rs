use std::cell::RefCell;
use std::io::{Write, stdout};
use std::rc::Rc;

#[derive(Default)]
struct Count(i64);

impl Drop for Count {
    fn drop(&mut self) {
        let _ = writeln!(stdout(), "counter dropped");
    }
}

impl Count {
    fn adjust(&mut self, adj: i64) {
        self.0 += adj;
    }

    fn value(&self) -> i64 {
        self.0
    }
}

#[derive(Default)]
struct Counter(Rc<RefCell<Count>>);

impl Counter {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }

    fn adjust(&self, adj: i64) {
        self.0.borrow_mut().adjust(adj);
    }

    fn value(&self) -> i64 {
        self.0.borrow_mut().value()
    }
}

struct Assets {
    count: u64,
    total: Counter,
}

struct Debits {
    count: u64,
    total: Counter,
}

impl Assets {
    fn acquire(&mut self) {
        self.total.adjust(1);
        self.count += 1;
    }
}

impl Debits {
    fn loan(&mut self) {
        self.total.adjust(-1);
        self.count += 1;
    }
}

fn main() {
    let counter = Counter::default();
    let mut debits = Debits { count: 0, total: counter.clone() };
    let mut assets = Assets { count: 0, total: counter.clone() };
    assets.acquire();
    debits.loan();
    println!("{}", counter.value());
    drop(assets);
    println!("assets dropped");
    drop(debits);
    println!("debits dropped");
    drop(counter);
    println!("finished");
}
