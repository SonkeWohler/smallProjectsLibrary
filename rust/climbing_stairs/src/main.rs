struct TwoQueue {
    // the last added element
    last: i32,
    // the next to last element
    next: i32,

}

impl TwoQueue {
    fn put(&mut self, new: i32) {
        self.next = self.last;
        self.last = new;
    }
}

impl Clone for TwoQueue {
    fn clone(&self) -> Self {
        *self
    }
}

fn new_queue() -> TwoQueue {
    TwoQueue {
        last: 1,
        next: 0,
    }
}

fn main() {
    // define the stairs
//    let args: Vec<String> = std::env::args().collect();
//    let stairs = &args[1].parse::<i32>().unwrap();
    let stairs: i32 = 10;
    println!("Calculating for {} stairs", stairs);

    // base case defined in new_queue, for one step
    //let mut current: i32 = 2;
    let mut paths: Vec<TwoQueue> = Vec::new();
    paths.push(new_queue());

    // induction steps until reaching the end of the stairs
    let mut new_path: TwoQueue;
    for current in 0..stairs {
        println!("calculating for step number {}", &current);
        for path in &mut paths {
            // current-1 is last for all paths
            if path.next == current - 1 {
                new_path = path.clone();
                new_path.last = current;
                // well damn it.  will have to find another way then.
                paths.push(new_path);
                path.put(current);
            } else {
                path.put(current);
            }
        }
    }

    println!("There are {} different paths to climb the stairs", paths.len());

    let mut path = new_queue();
    path.put(3);
    println!("{}", path.last);
    println!("{}", path.next);

    println!("Hello, world!");
}
