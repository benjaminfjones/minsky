use crate::magnificent;
use std::fs;

lalrpop_mod!(pub m3); // generated parser

pub fn parse_m3(_input: &str) -> Result<magnificent::Program, String> {
    // let raw_program = grammer::PredParser::new().parse(input)
    //     .or_else(|e| panic!("m3 parser failed: {}", e))
    Err("unimplemented".to_string())
}

pub fn validate_raw_program(prog: &magnificent::Program) -> Result<(), String> {
    for r in prog.iter() {
        if r.len() != prog.num_tapes() {
            return Err(format!("Rule {:?} specifies incorrect number of tapes", r));
        }
    }
    Ok(())
}

// Helper function to read / parse programs
pub fn read_program(filepath: &str) -> magnificent::Program {
    let input = fs::read_to_string(filepath).expect("failed to read program file");
    let program = m3::ProgramParser::new()
        .parse(&input)
        .expect("failed to parse program file");
    validate_raw_program(&program).expect("invalid program");
    program
}

#[cfg(test)]
mod test {

    use super::m3;
    use super::validate_raw_program;
    use crate::magnificent;
    use std::fs;

    #[test]
    pub fn test_parse_m3() {
        let input = r"
            tapes: 2
            0 [1, -1] 0";
        let program = m3::ProgramParser::new()
            .parse(input)
            .expect("m3 parser failed");
        validate_raw_program(&program).expect("Invalid program");
        assert_eq!(program.num_tapes(), 2);
        let mut rules_iter = program.iter();
        assert_eq!(
            rules_iter.next().unwrap(),
            &magnificent::Rule::new(0, 0, vec![1, -1])
        );

        let input = r"
            tapes: 3
            0 [1, -1, 2] 1
            1 [0, 1, 0] 2";
        let program = m3::ProgramParser::new()
            .parse(input)
            .expect("m3 parser failed");
        validate_raw_program(&program).expect("Invalid program");
        assert_eq!(program.num_tapes(), 3);

        let mut rules_iter = program.iter();
        assert_eq!(
            rules_iter.next().unwrap(),
            &magnificent::Rule::new(0, 1, vec![1, -1, 2])
        );
        assert_eq!(
            rules_iter.next().unwrap(),
            &magnificent::Rule::new(1, 2, vec![0, 1, 0])
        );

        let input = r"
            tapes: 5
            0 [1, 2, 3, 4, 5] 1
            1 [-1, 2, 3, 4, 5] 2
            2 [-1, -2, 3, 4, 5] 0
            0 [0, 0, 0, 4, 5] 4
            4 [1, 2, 3, -4, -5] 5
            5 [1, 2, 3, 0, 0] 0
            ";
        let program = m3::ProgramParser::new()
            .parse(input)
            .expect("m3 parser failed");
        validate_raw_program(&program).expect("Invalid program");
    }

    // Test parsing a program with malformed tapes statement
    #[test]
    #[should_panic(expected = "m3 parser failed")]
    pub fn test_bad_parse1() {
        let input = r"
            tape 3
            0 [1, -1, 2] 1";
        let program = m3::ProgramParser::new()
            .parse(input)
            .expect("m3 parser failed");
        validate_raw_program(&program).expect("Invalid program");
    }

    // Test parsing a program with missing next state
    #[test]
    #[should_panic(expected = "m3 parser failed")]
    pub fn test_bad_parse2() {
        let input = r"
            tapes: 3
            0 [1, -1, 2]";
        let program = m3::ProgramParser::new()
            .parse(input)
            .expect("m3 parser failed");
        validate_raw_program(&program).expect("Invalid program");
    }

    // Test parsing a program with invalid rule width
    #[test]
    #[should_panic(expected = "Invalid program")]
    pub fn test_bad_parse3() {
        let input = r"
            tapes: 1
            0 [1, -1, 2, 0] 0";
        let program = m3::ProgramParser::new().parse(input).unwrap(); //.expect("m3 parser failed");
        validate_raw_program(&program).expect("Invalid program");
    }

    // Test parsing of a file on disk
    #[test]
    pub fn test_parse_adder() {
        const ADDER_PROGRAM: &str = "examples/adder.m3";
        let input = fs::read_to_string(ADDER_PROGRAM).expect("failed to read program file");
        let program = m3::ProgramParser::new()
            .parse(&input)
            .expect("failed to parse program file");
        validate_raw_program(&program).expect("invalid program");

        // Interpret the parsed program to make sure it works
        let machine = magnificent::Machine::new(0, vec![1, 1]);
        let end_machine = magnificent::interpret(machine, &program, 100);
        assert!(end_machine.is_ok());
        let (_, end_machine) = end_machine.unwrap();
        assert_eq!(end_machine.tape_pos(0), 2);
    }

    // Test parsing of a file on disk
    #[test]
    pub fn test_parse_mult() {
        const MULT_PROGRAM: &str = "examples/mult.m3";
        let input = fs::read_to_string(MULT_PROGRAM).expect("failed to read program file");
        let program = m3::ProgramParser::new()
            .parse(&input)
            .expect("failed to parse program file");
        validate_raw_program(&program).expect("invalid program");

        // Interpret the parsed program to make sure it works
        let machine = magnificent::Machine::new(0, vec![0, 2, 0, 3 - 1]);
        let end_machine = magnificent::interpret(machine, &program, 100);
        assert!(end_machine.is_ok());
        let (_, end_machine) = end_machine.unwrap();
        assert_eq!(end_machine.tape_pos(0), 6);
    }

    // Test parsing of the 6-rule multiplier
    #[test]
    pub fn test_parse_6_rule_mult() {
        const MULT_PROGRAM: &str = "examples/6-rule-mult.m3";
        let input = fs::read_to_string(MULT_PROGRAM).expect("failed to read program file");
        let program = m3::ProgramParser::new()
            .parse(&input)
            .expect("failed to parse program file");
        validate_raw_program(&program).expect("invalid program");

        // Interpret the parsed program to make sure it works
        let x = 7;
        let y = 11;
        let machine = magnificent::Machine::new(0, vec![0, x, y, 0]);
        let end_machine = magnificent::interpret(machine, &program, 1000);
        assert!(end_machine.is_ok());
        let (_, end_machine) = end_machine.unwrap();
        assert_eq!(end_machine.tape_pos(0), x * y);
    }
}
