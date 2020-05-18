use crate::magnificent;

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

#[cfg(test)]
mod test {

    use super::m3;
    use super::validate_raw_program;
    use crate::magnificent;

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
}
