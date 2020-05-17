use crate::m3_ast;
use crate::magnificent;

lalrpop_mod!(pub m3); // generated parser

pub fn parse_m3(_input: &str) -> Result<magnificent::Program, String> {
    // let raw_program = grammer::PredParser::new().parse(input)
    //     .or_else(|e| panic!("m3 parser failed: {}", e))
    Err("unimplemented".to_string())
}

pub fn validate_raw_program(raw_prog: &m3_ast::RawProgram) -> Result<(), String> {
    for r in raw_prog.rules.iter() {
        if r.rule.len() != raw_prog.num_tapes {
            return Err(format!("Rule {:?} specifies incorrect number of tapes", r));
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {

    use super::m3;
    use super::validate_raw_program;

    #[test]
    pub fn test_parse_m3() {
        let input = r"
            tapes: 2
            0 [1, -1] 0";
        let raw_program = m3::RawProgramParser::new()
            .parse(input)
            .expect("m3 parser failed");
        validate_raw_program(&raw_program).expect("Invalid program");
        assert_eq!(raw_program.num_tapes, 2);
        assert_eq!(raw_program.rules[0].cur_state, 0);
        assert_eq!(raw_program.rules[0].next_state, 0);
        assert_eq!(raw_program.rules[0].rule, vec![1, -1]);

        let input = r"
            tapes: 3
            0 [1, -1, 2] 1
            1 [0, 1, 0] 2";
        let raw_program = m3::RawProgramParser::new()
            .parse(input)
            .expect("m3 parser failed");
        assert_eq!(raw_program.num_tapes, 3);
        assert_eq!(raw_program.rules[0].cur_state, 0);
        assert_eq!(raw_program.rules[0].next_state, 1);
        assert_eq!(raw_program.rules[0].rule, vec![1, -1, 2]);
        assert_eq!(raw_program.rules[1].cur_state, 1);
        assert_eq!(raw_program.rules[1].next_state, 2);
        assert_eq!(raw_program.rules[1].rule, vec![0, 1, 0]);
    }
}
