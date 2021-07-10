struct TwoQueue {
    // the last added element
    last: u32,
    // the next to last element
    next: u32,

}

impl TwoQueue {
    fn put(&mut self, new: u32) {
        self.next = self.last;
        self.last = new;
    }
}

fn new_queue() -> TwoQueue {
    TwoQueue {
        last: 1,
        next: 0,
    }
}

fn main() {
    let mut path = new_queue();
    path.put(3);
    println!("{}", path.last);
    println!("{}", path.next);
    println!("Hello, world!");
}
