// This module demonstrates some basic Magnificent Minsky Machines that perform arithmetic.
//

use crate::magnificent::{interpret, Machine, Program, Rule};

// Construct a Magnificent Minsky Machine that adds two non-negative integers and run it on the
// inputs.
pub fn adder(x: i32, y: i32) -> i32 {
    assert!(x >= 0 && y >= 0);
    let rule = Rule::new(0, 0, vec![1, -1]);
    let program = Program::new(2, vec![rule]);
    let machine = Machine::new(0, vec![x, y]);

    // machine transitions:
    //   - rule will fire y times, moving tape 0 to x+y and tape 1 to 0
    let end_machine = interpret(machine, &program, 2 * y);
    assert!(end_machine.is_ok());
    let end_machine = end_machine.unwrap();
    end_machine.tape_pos(0)
}

// Basic multiplier machine
//
// Initial machine state:
//
// 0: 0   x   0   y   --> rule0
// 0: 1   x-1 1   y   --> rule0
// 0: ...             --> ...
// 0: x   0   x   y   --> rule0 doesn't apply, rule1 fires st' = 1
// 1: x   0   x   y   --> rule0 & 1 don't apply, rule2 fires
// 1: x   1   x-1 y   --> rule2
// 1: ...             --> ...
// 1: x   x   0   y   --> rule2 doesn't apply, rule 3 resets state st' = 0
// 0: x   x   0   y-1
// .. ...
// 1: x*y x   0   0   --> HALT
//
pub fn mult(x: i32, y: i32) -> i32 {
    let rule0 = Rule::new(0, 0, vec![1, -1, 1, 0]);
    let rule1 = Rule::new(0, 1, vec![0, 0, 0, 0]);
    let rule2 = Rule::new(1, 1, vec![0, 1, -1, 0]);
    let rule3 = Rule::new(1, 0, vec![0, 0, 0, -1]);
    let program = Program::new(4, vec![rule0, rule1, rule2, rule3]);
    let machine = Machine::new(0, vec![0, x, 0, y - 1]);

    let end_machine = interpret(machine, &program, 2 * (x + 1) * y);
    assert!(end_machine.is_ok());
    let end_machine = end_machine.unwrap();
    end_machine.tape_pos(0)
}

#[cfg(test)]
mod test {
    use super::{adder, mult};

    #[test]
    fn add_x_y() {
        for x in 1..10 {
            for y in 1..10 {
                assert_eq!(adder(x, y), x + y);
            }
        }
    }

    #[test]
    fn mult_x_y() {
        for x in 1..10 {
            for y in 1..10 {
                assert_eq!(mult(x, y), x * y);
            }
        }
    }

    #[test]
    fn big_mult() {
        assert_eq!(mult(100, 100), 10_000); // 20200 steps
    }
}
