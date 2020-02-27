use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

/// Problem state. Whether the boat is on the other side, and how many {missionaries,cannibals} are
/// there in the original side.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State(bool, i32, i32);

/// An operation. Whether the boat is moving from the original to the other side, and how the
/// number of missionaries and cannibals on the original side change.
type Operation = (bool, Box<dyn Fn(i32) -> i32>, Box<dyn Fn(i32) -> i32>);

impl State {
    /// Returns the state when applying operation on self, an Err(()) is returned if the resulted
    /// state is invalid.
    fn apply(&self, (b, miss, cann): &Operation) -> Result<State, ()> {
        if self.0 == *b {
            // Boat is on the wrong side
            return Err(());
        }

        let miss0 = miss(self.1);
        let cann0 = cann(self.2);
        let miss1 = 3 - miss0;
        let cann1 = 3 - cann0;

        // Make sure the numbers are positive and the number of missionaries is not less than the
        // number of cannibals on each side, or the number of missionaries is zero.
        if (miss0 >= 0 && miss0 <= 3)
            && (cann0 >= 0 && cann0 <= 3)
            && (miss0 >= cann0 || miss0 == 0)
            && (miss1 >= cann1 || miss1 == 0)
        {
            Ok(State(*b, miss0, cann0))
        } else {
            Err(())
        }
    }

    fn at_goal(&self) -> bool {
        matches!(self, State(_, 0, 0))
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_assert!(
            self.1 >= 0 && self.1 <= 3 && self.2 >= 0 && self.2 <= 3,
            "Invalid state: {:?}",
            self
        );
        let boat = if self.0 { "|   B|" } else { "|B   |" };
        let miss0 = "M".repeat(self.1 as usize);
        let cann0 = "C".repeat(self.2 as usize);
        let left = format!("{:>6}", miss0 + &cann0);
        let miss1 = "M".repeat((3 - self.1) as usize);
        let cann1 = "C".repeat((3 - self.2) as usize);
        write!(f, "{}{}{}{}", left, boat, miss1, cann1)
    }
}

fn add(a: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |b| a + b)
}

fn main() {
    // Possible operations
    let ops = [
        (true, add(-1), add(-1)),
        (true, add(0), add(-1)),
        (true, add(-1), add(0)),
        (true, add(0), add(-2)),
        (true, add(-2), add(0)),
        (false, add(1), add(1)),
        (false, add(0), add(1)),
        (false, add(1), add(0)),
        (false, add(0), add(2)),
        (false, add(2), add(0)),
    ];

    let init = State(false, 3, 3);
    let goal = State(true, 0, 0);
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut parent = HashMap::new();
    visited.insert(init);
    queue.push_back(init);

    while let Some(state) = queue.pop_front() {
        if state.at_goal() {
            break;
        }
        // Try all possible operations
        for o in ops.iter() {
            if let Ok(s) = state.apply(o) {
                // Discovered a valid state
                if !visited.contains(&s) {
                    visited.insert(s);
                    queue.push_back(s);
                    parent.insert(s, state);
                }
            }
        }
    }

    // Construct the path to goal
    let mut curr = goal;
    let mut path = VecDeque::new();
    path.push_front(goal);
    while let Some(&prev) = parent.get(&curr) {
        path.push_front(prev);
        curr = prev;
    }
    for s in path {
        println!("{}", s);
    }
}
