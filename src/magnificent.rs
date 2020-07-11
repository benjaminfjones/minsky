//! # Magnificent Minsky Machine Interpreter
//!
//! This module implements an interpreter for "Magnificent Minsky Machines".
//!
//! A Magnificent Minsky Machine is a Minsky Machine with a fixed number of tapes. Each tape has a
//! head whose position is a non-negative integer. The machine has a fixed set of states, denoted
//! by non-negative integers.
//!
//! A program for this machine is an ordered sequence of rules. The rules consist of a state in
//! which they may fire, a next state to transition to after the rule fires, and a vector of tape
//! head adjustments. A rule only fires if the machine is in the correct state and the tape head
//! adjustments can be made to the tape positions without causing any tape head position to fall
//! below zero. The negative entries in a rule adjustment are called the "guard" and the positive
//! entries are called the "action".
//!
//! The interpreter takes an initial machine (consisting of an initial machine state and tape head
//! positions) and a program, and produces a final machine (or an error). Starting from the initial
//! state, the interpreter iterates over the list rules in the program in order. The first rule to
//! apply is applied, the tape heads are adjusted, and the machine state is potentially changed.
//! The interpreter then repeats the process starting at the beginning of the rule list. This
//! process continues until either no rule in the program applies or the interpreter runs out of
//! fuel.

use std::slice::Iter;

/// Error conditions the interpreter may return
#[derive(Debug)]
pub enum ErrorCode {
    /// a tape id listed in the clause is invalid
    BadClause,
    /// interpreter out of fuel
    OutOfFuel,
}

/// Machine states are non-negative integers
pub type State = usize;

/// Tapes are identified using non-negative integers
pub type TapeId = usize;

/// Tape state is a tape head position (non-negative integer) for each tape
#[derive(Debug)]
pub struct TapeState(pub Vec<i32>);

/// A Magnificent Minsky Machine
#[derive(Debug)]
pub struct Machine {
    machine_state: State,
    tape_state: TapeState,
}

/// A Rule, part of a Minsky Machine program
#[derive(Debug, Eq, PartialEq)]
pub struct Rule {
    // current state that the rule applies to
    cur_state: State,
    // next state after the rule is applied
    next_state: State,
    // Rule clause specifies decrements and increments to make to the tape state, provided that
    // the decrements can actually be made without passing bottom on any tape.
    rule: Vec<i32>,
}

/// A program consists of a number of tapes and a list of rules
#[derive(Debug)]
pub struct Program {
    // Number of tapes used in the program. This value must match the size of the machine's
    // TapeState.
    num_tapes: usize,
    // Ordered sequence of rules that make up the program
    rules: Vec<Rule>,
}

//////////////////////////////////////////////////////////////////////////////
// Implementations

impl Rule {
    /// Create a new Rule by specifying the current state it should fire in, the next state the
    /// machine should transition to, and the tape head adjustments.
    pub fn new(cur_state: State, next_state: State, rule: Vec<i32>) -> Self {
        Rule {
            cur_state,
            next_state,
            rule,
        }
    }

    /// Return the state that this rule fires in.
    pub fn cur_state(&self) -> State {
        self.cur_state
    }

    /// Return the state that this rule transitions the machine to.
    pub fn next_state(&self) -> State {
        self.next_state
    }

    /// Return the number of tapes that the rule operators over (i.e. the number of tape head
    /// adjustments).
    pub fn len(&self) -> usize {
        self.rule.len()
    }

    /// Determine of the rule set is empty
    pub fn is_empty(&self) -> bool {
        self.rule.is_empty()
    }

    /// Iterate over the tape head adjustments that the rule specifies
    pub fn iter(&self) -> Iter<i32> {
        self.rule.iter()
    }
}

impl TapeState {
    /// Examine a tape state and determine whether a rule is satisfied, i.e. can the tapes be
    /// moved backwards by the amounts specified in the rule?
    ///
    /// This method assumes that the number of tapes and the size of the rule are equal.
    fn test_rule(&self, rule: &Rule) -> bool {
        assert!(self.0.len() == rule.rule.len());
        self.0
            .iter()
            .zip(rule.iter())
            .all(|(tp, amt)| *amt >= 0 || *tp >= -(*amt))
    }

    /// Apply the decrements/increments given in `rule` to `self`.
    ///
    /// `test_rule` must always be called before `apply_rule`.
    ///
    /// This method assumes that self.0 and rule.rule are the same length, it does not examine the
    /// current state of `rule`, and it does not check that the decrements can be made safely.
    fn apply_rule(&mut self, rule: &Rule) {
        for (t, amt) in self.0.iter_mut().zip(rule.iter()) {
            *t += *amt;
        }
    }

    /// Check that the tape positions are all non-negative
    fn is_valid(&self) -> bool {
        self.0.iter().all(|tp| *tp >= 0)
    }
}

impl Program {
    /// Create a new program by specifying the number of tapes it operates on andf the ordered
    /// sequence of rules to apply.
    pub fn new(num_tapes: usize, rules: Vec<Rule>) -> Self {
        Program { num_tapes, rules }
    }

    /// Return the number of tapes the program operates on.
    pub fn num_tapes(&self) -> usize {
        self.num_tapes
    }

    /// Return the number of rules.
    pub fn num_rules(&self) -> usize {
        self.rules.len()
    }

    /// Iterate over the rules in the program in order.
    pub fn iter(&self) -> Iter<Rule> {
        self.rules.iter()
    }
}

impl Machine {
    /// Create a new machine given an initial machine state and tape head positions.
    pub fn new(machine_state: usize, tape_state: Vec<i32>) -> Self {
        Machine {
            machine_state,
            tape_state: TapeState(tape_state),
        }
    }

    /// Try to apply the give rule to the machine.
    ///
    /// If the rule's guard is satisfied, move the tapes in the guard backwards and the tapes in
    /// the action forward. Then update the machine's state. If successful, return `true`,
    /// otherwise `false`.
    pub fn apply_rule(&mut self, rule: &Rule) -> bool {
        if self.machine_state == rule.cur_state && self.tape_state.test_rule(&rule) {
            self.tape_state.apply_rule(&rule);
            assert!(self.tape_state.is_valid());
            self.machine_state = rule.next_state;
            return true;
        }
        false
    }

    /// Return the current tape head position for the indicated tape.
    pub fn tape_pos(&self, id: usize) -> i32 {
        self.tape_state.0[id]
    }
}

/// Interpret the given program starting with the initial machine.
///
/// Try to apply rules in the program in the order they appear.
///   - When a rule applies, apply it and start over from the first rule in the program.
///   - When no rules apply to a given machine, halt and return the machine.
pub fn interpret(
    initial_machine: Machine,
    program: &Program,
    fuel: u64,
) -> Result<(u64, Machine), ErrorCode> {
    let mut machine = initial_machine;
    let mut counter: u64 = 0;
    loop {
        let mut changed = false;
        println!("{}: {:?}", machine.machine_state, machine.tape_state);
        for rule in program.iter() {
            if machine.apply_rule(&rule) {
                changed = true;
                counter += 1;
                break;
            }
        }
        if !changed {
            return Ok((counter, machine));
        }
        if counter >= fuel {
            return Err(ErrorCode::OutOfFuel);
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
            tape_state,
        };
        // tape_state: (0, 0), (1, 0), machine_state: 0
        assert_eq!(machine.machine_state, 0);
        assert_eq!(machine.tape_state.0, vec![0, 0]);

        // Rule 1 doesn't apply b/c of the guard
        assert!(!machine.apply_rule(&rule1));
        // Rule 2 doesn't apply b/c the machine state is wrong
        assert!(!machine.apply_rule(&rule2));
        // tape_state: [0, 0], machine_state: 0
        assert_eq!(machine.machine_state, 0);
        assert_eq!(machine.tape_state.0, vec![0, 0]);

        // Rule 0 always applies
        assert!(machine.apply_rule(&rule0));
        // tape_state: [1, 1], machine_state: 0
        assert_eq!(machine.machine_state, 0);
        assert_eq!(machine.tape_state.0, vec![1, 1]);

        assert!(machine.apply_rule(&rule0));
        assert_eq!(machine.machine_state, 0);
        assert_eq!(machine.tape_state.0, vec![2, 2]);

        assert!(machine.apply_rule(&rule0));
        assert!(machine.apply_rule(&rule0));

        // Try rule1:
        assert!(machine.apply_rule(&rule1));
        assert_eq!(machine.machine_state, 1);
        assert_eq!(machine.tape_state.0, vec![3, 6]);

        // Rule 1 doesn't apply anymore b/c we're in state 1
        assert!(!machine.apply_rule(&rule1));

        // Rule 2 applies:
        assert!(machine.apply_rule(&rule2));
        assert_eq!(machine.machine_state, 1);
        assert_eq!(machine.tape_state.0, vec![1, 4]);

        // Rule 2 doesn't apply anymore b/c tape 0 can't move back 2
        assert!(!machine.apply_rule(&rule2));
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
        let (_, end_machine) = end_machine.unwrap();
        println!("end machine: {:?}", end_machine);
        assert_eq!(end_machine.machine_state, 1);
        assert_eq!(end_machine.tape_state.0, vec![0, 5, 10]);
    }
}
