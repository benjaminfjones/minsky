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

#[cfg(test)]
mod test {
    use super::adder;

    #[test]
    fn add_1_1() {
        assert_eq!(adder(1, 1), 2);
    }

    #[test]
    fn add_1_10() {
        assert_eq!(adder(1, 10), 11);
    }

    #[test]
    fn add_100_11() {
        assert_eq!(adder(100, 11), 111);
    }
}
