// Interpreter for "Magnificent Minsky Machines"
//

#[derive(Debug)]
pub enum ErrorCode {
    // a tape id listed in the clause is invalid
    BadClause,
    // a rule guard was not satisfied
    GuardNotSAT,
    // the current machine state does not match the rule cur_state
    WrongState,
}

// Machine states are non-negative integers
pub type State = usize;

// Tapes are identified using non-negative integers
pub type TapeId = usize;

// Tape state is a tape head position (non-negative integer) for each tape
#[derive(Debug)]
pub struct TapeState(pub Vec<i32>);

#[derive(Debug)]
pub struct Machine {
    machine_state: State,
    tape_state: TapeState,
}

#[derive(Debug)]
pub struct Rule {
    // current state that the rule applies to
    cur_state: State,
    // next state after the rule is applied
    next_state: State,
    // rule clause specifies decrements and increments to make to the tape state, provided that
    // the decrements can actually be made without passing bottom on any tape.
    rule: Vec<i32>,
}

// A program consists of a number of tapes and a list of rules
#[derive(Debug)]
pub struct Program {
    num_tapes: usize,
    rules: Vec<Rule>,
}

//////////////////////////////////////////////////////////////////////////////
// Implementations

impl Rule {
    pub fn new(cur_state: State, next_state: State, rule: Vec<i32>) -> Self {
        Rule {
            cur_state,
            next_state,
            rule,
        }
    }

    pub fn iter(&self) -> &Vec<i32> {
        &self.rule
    }
}

impl TapeState {
    // Examine a tape state and determine whether a rule is satisfied, i.e. can the tapes be
    // moved backwards by the amounts specified in the rule?
    //
    // This method assumes that the number of tapes and the size of the rule are equal.
    fn test_rule(&self, rule: &Rule) -> bool {
        assert!(self.0.len() == rule.rule.len());
        self.0
            .iter()
            .zip(rule.iter())
            .all(|(tp, amt)| *amt >= 0 || *tp >= -(*amt))
    }

    // Apply the decrements/increments given in `rule` to `self`.
    //
    // `test_rule` must always be called before `apply_rule`.
    //
    // This method assumes that self.0 and rule.rule are the same length, it does not examine the
    // current state of `rule`, and it does not check that the decrements can be made safely.
    fn apply_rule(&mut self, rule: &Rule) {
        for (t, amt) in self.0.iter_mut().zip(rule.iter()) {
            *t += *amt;
        }
    }

    // Check that the tape positions are all non-negative
    fn is_valid(&self) -> bool {
        self.0.iter().all(|tp| *tp >= 0)
    }
}

impl Program {
    pub fn new(num_tapes: usize, rules: Vec<Rule>) -> Self {
        Program { num_tapes, rules }
    }

    pub fn iter(&self) -> &Vec<Rule> {
        &self.rules
    }
}

impl Machine {
    pub fn new(machine_state: usize, tape_state: Vec<i32>) -> Self {
        Machine {
            machine_state,
            tape_state: TapeState(tape_state),
        }
    }

    // Try to apply the give rule to the machine.
    //
    // If the rule's guard is satisfied, move the tapes in the guard backwards and the tapes in
    // the action forward. Then update the machine's state. If successful, return Ok, else Err.
    pub fn apply_rule(&mut self, rule: &Rule) -> Result<(), ErrorCode> {
        if self.machine_state != rule.cur_state {
            return Err(ErrorCode::WrongState);
        }
        if self.tape_state.test_rule(&rule) {
            self.tape_state.apply_rule(&rule);
            assert!(self.tape_state.is_valid());
            self.machine_state = rule.next_state;
            Ok(())
        } else {
            return Err(ErrorCode::GuardNotSAT);
        }
    }

    pub fn tape_pos(&self, id: usize) -> i32 {
        self.tape_state.0[id]
    }
}

// Interpret the given program starting with the initial machine.
//
// Try to apply rules in the program in the order they appear.
//   - When a rule applies, apply it and start over from the first rule in the program.
//   - When no rules apply to a given machine, halt and return the machine.
pub fn interpret(initial_machine: Machine, program: &Program, fuel: i32) -> Result<Machine, ()> {
    let mut machine = initial_machine;
    let mut counter = 0;
    loop {
        let mut changed = false;
        for rule in program.iter() {
            if machine.apply_rule(&rule).is_ok() {
                changed = true;
                counter += 1;
                break;
            }
        }
        if !changed {
            return Ok(machine);
        }
        if counter >= fuel {
            return Err(());
        }
    }
}

//////////////////////////////////////////////////////////////////////////////
// Tests

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic() {
        let rule = Rule::new(0, 0, vec![-1, -2, 3, 4]);
        assert_eq!(rule.cur_state, 0);
        assert_eq!(rule.next_state, 0);
        assert_eq!(rule.rule, vec![-1, -2, 3, 4]);
    }

    #[test]
    fn test_test_rule() {
        let tape_state: TapeState = TapeState(vec![2, 2]);
        let rule0 = Rule::new(0, 0, vec![0, -1]);
        let rule1 = Rule::new(0, 0, vec![0, 5]);
        let rule2 = Rule::new(0, 0, vec![2, 0]);
        let rule3 = Rule::new(0, 0, vec![3, 3]);
        let false_rule = Rule::new(0, 0, vec![-7, 0]);
        assert!(tape_state.test_rule(&rule0));
        assert!(tape_state.test_rule(&rule1));
        assert!(tape_state.test_rule(&rule2));
        assert!(tape_state.test_rule(&rule3));
        assert!(!tape_state.test_rule(&false_rule));
    }

    #[test]
    #[should_panic]
    fn test_bad_rule() {
        let tape_state: TapeState = TapeState(vec![2, 2]);
        let wide_rule = Rule::new(0, 0, vec![0, 1, 2]);
        let _b = tape_state.test_rule(&wide_rule);
    }

    #[test]
    fn test_move_tapes() {
        // Test move_backwards
        let mut tape_state: TapeState = TapeState(vec![2, 2]);
        let rule = Rule::new(0, 0, vec![-1, -1]);
        tape_state.apply_rule(&rule);
        assert_eq!(tape_state.0[0], 1);
        assert_eq!(tape_state.0[1], 1);

        // Test move_forwards
        let rule = Rule::new(0, 0, vec![1, 9]);
        tape_state.apply_rule(&rule);
        assert_eq!(tape_state.0[0], 2);
        assert_eq!(tape_state.0[1], 10);
    }

    #[test]
    fn test_apply_rule() {
        let tape_state: TapeState = TapeState(vec![0, 0]);
        // Rule 0: no guard, no machine state transition
        let rule0 = Rule::new(0, 0, vec![1, 1]);
        // Rule 1: move tape 0 back 1, tape 1 forward 2, transition to new machine state
        let rule1 = Rule::new(0, 1, vec![-1, 2]);
        // Rule 2: move backwards only, preserve machine state
        let rule2 = Rule::new(1, 1, vec![-2, -2]);

        // Initial machine state
        let mut machine = Machine {
            machine_state: 0,
            tape_state: tape_state,
        };
        // tape_state: (0, 0), (1, 0), machine_state: 0
        assert_eq!(machine.machine_state, 0);
        assert_eq!(machine.tape_state.0, vec![0, 0]);

        // Rule 1 doesn't apply b/c of the guard
        assert!(machine.apply_rule(&rule1).is_err());
        // Rule 2 doesn't apply b/c the machine state is wrong
        assert!(machine.apply_rule(&rule2).is_err());
        // tape_state: [0, 0], machine_state: 0
        assert_eq!(machine.machine_state, 0);
        assert_eq!(machine.tape_state.0, vec![0, 0]);

        // Rule 0 always applies
        assert!(machine.apply_rule(&rule0).is_ok());
        // tape_state: [1, 1], machine_state: 0
        assert_eq!(machine.machine_state, 0);
        assert_eq!(machine.tape_state.0, vec![1, 1]);

        assert!(machine.apply_rule(&rule0).is_ok());
        assert_eq!(machine.machine_state, 0);
        assert_eq!(machine.tape_state.0, vec![2, 2]);

        assert!(machine.apply_rule(&rule0).is_ok());
        assert!(machine.apply_rule(&rule0).is_ok());

        // Try rule1:
        assert!(machine.apply_rule(&rule1).is_ok());
        assert_eq!(machine.machine_state, 1);
        assert_eq!(machine.tape_state.0, vec![3, 6]);

        // Rule 1 doesn't apply anymore b/c we're in state 1
        assert!(machine.apply_rule(&rule1).is_err());

        // Rule 2 applies:
        assert!(machine.apply_rule(&rule2).is_ok());
        assert_eq!(machine.machine_state, 1);
        assert_eq!(machine.tape_state.0, vec![1, 4]);

        // Rule 2 doesn't apply anymore b/c tape 0 can't move back 2
        assert!(machine.apply_rule(&rule2).is_err());
    }

    #[test]
    fn test_interpret() {
        let rule0 = Rule::new(0, 0, vec![1, 1, -1]);
        let rule1 = Rule::new(0, 1, vec![0, -5, 0]);
        let rule2 = Rule::new(1, 1, vec![-1, 1, 2]);
        let program = Program::new(3, vec![rule0, rule1, rule2]);

        let machine = Machine {
            machine_state: 0,
            tape_state: TapeState(vec![0, 0, 5]),
        };

        // machine transitions:
        //   - rule 0 will fire 5 times, moving tape 0 and 1 ahead to 5 and tape 2 back to 0
        //   - rule 1 will then fire once, moving tape 1 to 5 and changing machine state to 1
        //   - rule 2 will then fire 5 times, moving tape 0 to 0, tape 1 to 5, and tape 2 to 10
        let end_machine = interpret(machine, &program, 1000);
        assert!(end_machine.is_ok());
        let end_machine = end_machine.unwrap();
        println!("end machine: {:?}", end_machine);
        assert_eq!(end_machine.machine_state, 1);
        assert_eq!(end_machine.tape_state.0, vec![0, 5, 10]);
    }
}
