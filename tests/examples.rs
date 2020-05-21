extern crate minsky;

use minsky::m3_parser;
use minsky::magnificent;

// Test parsing / interpreting of the adder
#[test]
pub fn adder() {
    let program = m3_parser::read_program("examples/adder.m3");
    let x = 1;
    let y = 3;
    let machine = magnificent::Machine::new(0, vec![x, y]);
    let end_machine = magnificent::interpret(machine, &program, 100);
    assert!(end_machine.is_ok());
    let (_, end_machine) = end_machine.unwrap();
    assert_eq!(end_machine.tape_pos(0), x + y);
}

// Test parsing / interpreting of the 4-tape multiplier
#[test]
pub fn mult() {
    let program = m3_parser::read_program("examples/mult.m3");
    let x = 2;
    let y = 3;
    let machine = magnificent::Machine::new(0, vec![0, x, 0, y - 1]);
    let end_machine = magnificent::interpret(machine, &program, 100);
    assert!(end_machine.is_ok());
    let (_, end_machine) = end_machine.unwrap();
    assert_eq!(end_machine.tape_pos(0), x * y);
}

// Test parsing / interpreting of the 6-rule, 4-tape multiplier
#[test]
pub fn six_rule_mult() {
    let program = m3_parser::read_program("examples/6-rule-mult.m3");
    let x = 7;
    let y = 11;
    let machine = magnificent::Machine::new(0, vec![0, x, y, 0]);
    let end_machine = magnificent::interpret(machine, &program, 1000);
    assert!(end_machine.is_ok());
    let (_, end_machine) = end_machine.unwrap();
    assert_eq!(end_machine.tape_pos(0), x * y);
}

// Test parsing / interpreting of the marvellous multiplier
#[test]
pub fn marvellous_mult() {
    let program = m3_parser::read_program("examples/marvellous-mult.m3");
    let x = 7;
    let y = 13;
    let machine = magnificent::Machine::new(0, vec![0, x, 0, y - 1, 1, 0, 0, 0]);
    let end_machine = magnificent::interpret(machine, &program, 1000);
    match end_machine {
        Ok((cycles, end_machine)) => {
            assert_eq!(end_machine.tape_pos(0), x * y);
            println!("interpreter took {} cycles", cycles); // 7*13 takes 390 cycles
        }
        Err(e) => panic!("Interpreter error: {:?}", e),
    }
}
