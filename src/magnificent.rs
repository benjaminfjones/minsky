// Interpreter for "Magnificent Minsky Machines"
//

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
pub enum ErrorCode {
    // a tape id listed in the clause is invalid
    BadClause,
    // a rule guard was not satisfied
    GuardNotSAT,
    // the current machine state does not match the rule cur_state
    WrongState,
}

// Machine states are non-negative integers. By convention, 0 represents the halting state.
pub type State = i32;

// Tapes are identified using non-negative integers
pub type TapeId = i32;

// Tape state is a tape head position (non-negative integer) for each tape
#[derive(Debug)]
pub struct TapeState(pub HashMap<TapeId, i32>);

#[derive(Debug)]
pub struct Machine {
    machine_state: State,
    tape_state: TapeState,
}

// An Atom is a tape identifier paired with an amount to move (either backwards or forwards
// depending on context).
pub type Atom = (TapeId, i32);

// A clause is a list of tape identifiers along with amounts to move each tape head (either forward
// or backward depending on how the clause is interpreted).
pub struct Clause(pub Vec<Atom>);
impl Clause {
    pub fn into_iter(self) -> Vec<Atom> {
        self.0
    }

    pub fn iter(&self) -> &Vec<Atom> {
        &self.0
    }
}

pub struct Rule {
    // guard clause to check (and amounts to move tapes backward)
    guard: Clause,
    // action clause with amounts to move tapes forwards
    action: Clause,
    // current state that the rule applies to
    cur_state: State,
    // next state after the rule is applied
    next_state: State,
}

// A program is a list of rules
pub struct Program(pub Vec<Rule>);

impl Program {
    pub fn iter(&self) -> &Vec<Rule> {
        &self.0
    }
}

impl Rule {
    // Create a new rule.
    //
    // The guard and action clauses are checked to involve disjoint sets of tape ids.
    pub fn new(
        guard: Clause,
        action: Clause,
        cur_state: State,
        next_state: State,
    ) -> Result<Self, ()> {
        let mut guard = guard;
        let mut action = action;
        Rule::normalize_clause(&mut guard);
        Rule::normalize_clause(&mut action);
        Rule::validate_disjoint_clauses(&guard, &action)?;
        Ok(Rule::unchecked_new(guard, action, cur_state, next_state))
    }

    // Remove atoms that entail 0 tape movement.
    fn normalize_clause(clause: &mut Clause) {
        clause.0.retain(|(_, amt)| *amt > 0);
    }

    // Check that no two atoms in the clause refer to the same tape id.
    fn validate_disjoint_ids(clause: &Clause) -> Result<(), ()> {
        let mut seen = HashSet::new();
        for (tid, _) in clause.iter() {
            if !seen.insert(tid) {
                return Err(());
            }
        }
        Ok(())
    }

    // Check that the two given clauses have disjoint tape id sets.
    fn validate_disjoint_clauses(clause1: &Clause, clause2: &Clause) -> Result<(), ()> {
        Rule::validate_disjoint_ids(clause1)?;
        Rule::validate_disjoint_ids(clause2)?;
        let mut seen1 = HashSet::new();
        let mut seen2 = HashSet::new();
        for (tid, _) in clause1.iter() {
            seen1.insert(tid);
        }
        for (tid, _) in clause2.iter() {
            seen2.insert(tid);
        }
        if seen1.is_disjoint(&seen2) {
            Ok(())
        } else {
            Err(())
        }
    }

    fn unchecked_new(guard: Clause, action: Clause, cur_state: State, next_state: State) -> Self {
        Rule {
            guard,
            action,
            cur_state,
            next_state,
        }
    }
}

impl TapeState {
    // Examine a tape state and determine whether a guard clause is satisfied, i.e. can the tapes be
    // moved backwards by the amounts specified in the guard?
    fn test_guard(&self, guard: &Clause) -> Result<bool, ErrorCode> {
        for (tid, amt) in guard.iter() {
            let tpos = self.0.get(&tid);
            if tpos.is_none() {
                return Err(ErrorCode::BadClause);
            } else {
                let tpos = tpos.unwrap();
                if *tpos < *amt {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    // Move the tape heads backwards an amount indicated by the given clause.
    //
    // This function does not check whether the move is valid or not.
    fn move_backwards(&mut self, clause: &Clause) {
        self.move_dir(clause, -1);
    }

    // Move the tape heads forwards an amount indicated by the given clause.
    fn move_forwards(&mut self, clause: &Clause) {
        self.move_dir(clause, 1);
    }

    // Helper function that moves tape heads each an amount in a particular direction
    //
    // This function does not check whether the move is valid or not.
    //
    // TODO: document parameters
    fn move_dir(&mut self, clause: &Clause, dir: i32) {
        for (tid, amt) in clause.iter() {
            match self.0.entry(*tid) {
                e @ Entry::Occupied(_) => {
                    e.and_modify(|v| *v += *amt * dir);
                }
                _ => panic!("bad tape id"),
            }
        }
    }
}

impl Machine {
    pub fn new(machine_state: i32, tape_state: HashMap<TapeId, i32>) -> Self {
        Machine {
            machine_state,
            tape_state: TapeState(tape_state),
        }
    }

    pub fn tape_pos(&self, tid: TapeId) -> i32 {
        self.tape_state.0[&tid]
    }

    // Try to apply the give rule to the machine.
    //
    // If the rule's guard is satisfied, move the tapes in the guard backwards and the tapes in the
    // action forward. Then update the machine's state. If successful, return Ok, else Err.
    pub fn apply_rule(&mut self, rule: &Rule) -> Result<(), ErrorCode> {
        if self.machine_state != rule.cur_state {
            return Err(ErrorCode::WrongState);
        }
        // propagate possible error while testing up the stack
        let make_it_so = self.tape_state.test_guard(&rule.guard)?;
        if make_it_so {
            self.tape_state.move_backwards(&rule.guard);
            self.tape_state.move_forwards(&rule.action);
            self.machine_state = rule.next_state;
            Ok(())
        } else {
            return Err(ErrorCode::GuardNotSAT);
        }
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
        let g = Clause(vec![(1, 1), (2, 2)]);
        let a = Clause(vec![(3, 3), (4, 4)]);
        let rule = Rule::new(g, a, 0, 0).unwrap();
        assert_eq!(rule.cur_state, 0);
        assert_eq!(rule.next_state, 0);
        assert_eq!(&rule.guard.0[0], &(1, 1));
        assert_eq!(&rule.action.0[0], &(3, 3));
    }

    #[test]
    fn test_test_guard() {
        let mut tape_state: TapeState = TapeState(HashMap::new());
        tape_state.0.insert(0, 2);
        tape_state.0.insert(1, 2);
        let big_guard = Clause(vec![(0, 5)]);
        let small_guard = Clause(vec![(0, 0), (1, 2)]);
        let wide_guard = Clause(vec![(0, 0), (1, 2), (2, 1)]);
        assert!(!tape_state.test_guard(&big_guard).unwrap());
        assert!(tape_state.test_guard(&small_guard).unwrap());
        assert!(tape_state.test_guard(&wide_guard).is_err());
    }

    #[test]
    fn test_move_tapes() {
        // Test move_backwards
        let mut tape_state: TapeState = TapeState(HashMap::new());
        tape_state.0.insert(0, 2);
        tape_state.0.insert(1, 2);
        let guard = Clause(vec![(0, 1), (1, 1)]);
        tape_state.move_backwards(&guard);
        assert_eq!(tape_state.0[&0], 1);
        assert_eq!(tape_state.0[&1], 1);

        // Test move_forwards
        let mut tape_state: TapeState = TapeState(HashMap::new());
        tape_state.0.insert(0, 0);
        tape_state.0.insert(1, 0);
        let action = Clause(vec![(0, 1), (1, 9)]);
        tape_state.move_forwards(&action);
        assert_eq!(tape_state.0[&0], 1);
        assert_eq!(tape_state.0[&1], 9);
    }

    #[test]
    fn test_apply_rule() {
        let mut tape_state: TapeState = TapeState(HashMap::new());
        tape_state.0.insert(0, 0); // initial tape positions both at 0
        tape_state.0.insert(1, 0);
        // Rule 0: no guard, no machine state transition
        let rule0 = Rule::new(Clause(vec![]), Clause(vec![(0, 1), (1, 1)]), 0, 0).unwrap();
        // Rule 1: move tape 0 back 1, tape 1 forward 2, transition to new machine state
        let rule1 = Rule::new(Clause(vec![(0, 1)]), Clause(vec![(1, 2)]), 0, 1).unwrap();
        // Rule 2: move backwards only, preserve machine state
        let rule2 = Rule::new(
            Clause(vec![(0, 2), (1, 2)]),
            Clause(vec![(0, 0), (1, 0)]),
            1,
            1,
        )
        .unwrap();

        // Initial machine state
        let mut machine = Machine {
            machine_state: 0,
            tape_state: tape_state,
        };
        // tape_state: (0, 0), (1, 0), machine_state: 0
        assert_eq!(machine.machine_state, 0);
        assert_eq!(machine.tape_state.0[&0], 0);
        assert_eq!(machine.tape_state.0[&1], 0);

        // Rule 1 & 2 don't apply b/c machine state is wrong
        assert!(machine.apply_rule(&rule1).is_err());
        assert!(machine.apply_rule(&rule2).is_err());
        // tape_state: (0, 0), (1, 0), machine_state: 0
        assert_eq!(machine.machine_state, 0);
        assert_eq!(machine.tape_state.0[&0], 0);
        assert_eq!(machine.tape_state.0[&1], 0);

        // Rule 0 always applies
        assert!(machine.apply_rule(&rule0).is_ok());
        // tape_state: (0, 1), (1, 1), machine_state: 0
        assert_eq!(machine.machine_state, 0);
        assert_eq!(machine.tape_state.0[&0], 1);
        assert_eq!(machine.tape_state.0[&1], 1);

        assert!(machine.apply_rule(&rule0).is_ok());
        // tape_state: (0, 2), (1, 2), machine_state: 0
        assert_eq!(machine.machine_state, 0);
        assert_eq!(machine.tape_state.0[&0], 2);
        assert_eq!(machine.tape_state.0[&1], 2);

        assert!(machine.apply_rule(&rule0).is_ok());
        assert!(machine.apply_rule(&rule0).is_ok());
        // tape_state: (0, 4), (1, 4), machine_state: 0

        assert!(machine.apply_rule(&rule1).is_ok());
        // tape_state: (0, 3), (1, 6), machine_state: 1
        assert_eq!(machine.machine_state, 1);
        assert_eq!(machine.tape_state.0[&0], 3);
        assert_eq!(machine.tape_state.0[&1], 6);

        // Rule 1 doesn't apply anymore b/c we're in state 1
        assert!(machine.apply_rule(&rule1).is_err());

        // Rule 2 applies:
        assert!(machine.apply_rule(&rule2).is_ok());
        // tape_state: (0, 1), (1, 4), machine_state: 1
        assert_eq!(machine.machine_state, 1);
        assert_eq!(machine.tape_state.0[&0], 1);
        assert_eq!(machine.tape_state.0[&1], 4);

        // Rule 2 doesn't apply anymore b/c tape 0 can't move back 2
        assert!(machine.apply_rule(&rule2).is_err());
    }

    #[test]
    fn test_interpret() {
        let rule0 = Rule::new(
            Clause(vec![(0, 0), (1, 0), (2, 1)]),
            Clause(vec![(0, 1), (1, 1), (2, 0)]),
            0,
            0,
        )
        .unwrap();
        let rule1 = Rule::new(
            Clause(vec![(0, 0), (1, 5), (2, 0)]),
            Clause(vec![(0, 0), (1, 0), (2, 0)]),
            0,
            1,
        )
        .unwrap();
        let rule2 = Rule::new(
            Clause(vec![(0, 1), (1, 0), (2, 0)]),
            Clause(vec![(0, 0), (1, 1), (2, 2)]),
            1,
            1,
        )
        .unwrap();
        let program = Program(vec![rule0, rule1, rule2]);

        let machine = Machine {
            machine_state: 0,
            tape_state: TapeState(vec![(0, 0), (1, 0), (2, 5)].into_iter().collect()),
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
        assert_eq!(end_machine.tape_state.0[&0], 0);
        assert_eq!(end_machine.tape_state.0[&1], 5);
        assert_eq!(end_machine.tape_state.0[&2], 10);
    }
}
