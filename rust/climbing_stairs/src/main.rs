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

//impl Clone for TwoQueue {
//    fn clone(&self) -> Self {
//        *self
//    }
//}

fn new_queue() -> TwoQueue {
    TwoQueue {
        last: 1,
        next: 0,
    }
}

/// calculates by walking through the paths in, I think, the most efficient way possible.  Can be
/// extended to also store these paths, if you want to.
/// O(stiars * x) where x is the number of paths per stair
fn calc_all_paths(stairs: i32) {
    // define the stairs
//    let args: Vec<String> = std::env::args().collect();
//    let stairs = &args[1].parse::<i32>().unwrap();
    //let stairs: i32 = 10;
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
                //new_path = path.clone();
                //new_path.last = current;
                // well damn it.  will have to find another way then
                //paths.push(new_path);
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

/// calculates only the number of possible paths to stairs, not the actual paths themselves.
fn calc_number_of_solutions_only(stairs: i32) -> i32 {
    println!("Calculating for {} stairs", stairs);
    
    // base case for 0 and 1 stairs
    if stairs == 0 {
        return 1;
    } else if stairs == 1{
        return 1;
    } else {
        return calc_number_of_solutions_only(stairs - 1) + calc_number_of_solutions_only(stairs -
            2);
    }
}

fn calc_more_efficiently(stairs: i32) -> i32 {
    println!("---");
    println!("Calculating for {} stairs", stairs);
    
    // base cases for 0 and 1 stairs, both 1 possible paths.
    // next to last stair, base case 0
    let mut next = 1;
    // last stair, base case 1
    let mut last = 1;
    // current paths, no case yet
    let mut current: i32 = 2;

    for step in 2..stairs {
        println!("At stair {}", step);
        println!("{} paths to last stair", last);
        current = last + next;
        next = last;
        last = current;
    }
    
    println!("---");
    return current;
}

fn main() {
    let stairs = 10;
    println!("For {} stairs, the number of paths is: {}", stairs, calc_more_efficiently(stairs));
}
