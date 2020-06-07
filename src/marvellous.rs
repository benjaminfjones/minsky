//! # Marvellous Transpiler
//!
//! This module implements a transpiler from magnificent Minsky machines, implemented in
//! [`magnificent`], to marvellous Minsky machines.
//!
//! A marvellous Minsky machine is a Minsky machine with the property that it uses only one machine
//! state. Any magnificent Minsky machine can be translated into a machine using one state. The
//! process is described below.
//!
//! Suppose we have a magnificent Minsky machine A with `n` tapes, `m` states, and an a program
//! consisting of an ordered list of rules. It is convenient to organize the rules by which current
//! state they fire in. For state `s`, let the list of rules that apply in state `s`, in the order
//! they appear in the program, be denoted by `r_{s,1}`, `r_{s,2}`, ..., `r_{s,nr(s)}` (where
//! `nr(s)` is a function giving the number of rules that can fire in state `s`).
//!
//! Define two functions on rules as follows: `state(r)` is the state `r` will fire in and
//! `state'(r)` is the state the machine will transition to after `r` fires. Thus `state(r_{s,i})
//! == s` and `state'(r_{s,i})` may or may not be `s`.
//!
//! The marvellous machine B that we will translate A into has a single machine state 0, `n` tapes
//! corresponding the tapes of A, and `2*m` additional tapes used to emulate the `m` states of A.
//!
//! Suppose rule `r_{s,i}` for the program of machine A has tape head adjustments `[a_1, ...,
//! a_n]`. To obtain a corresponding rule for machine B, we keep these adjustments and append `2*m`
//! entries as follows. Since `r_{s,i}` fires in original state `s` we guard the new rule by
//! putting `-1` in the `n+2*s+1` position. If `state'(r_{s,i}) == s`, we also add an action of
//! `+1` in position `n+2*s+2`. These two adjustments to the tapes beyond the `n`-th have the
//! effect of guarding this rule on original state `s` and signaling that the machine should
//! transition to a new emulated state whose only purpose is to reset the emulated state back to
//! `s`. To that effect, we insert a new rule after the translation of `r_{s,i}` whose tape
//! adjustment is zero everywhere except for 1 at tape `n+2*s+1` and -1 at tape `n+2*s+2`. On the
//! other hand, if `state'(r_{s,i}) == t` with `t != s` then we append entries such that `n+2*s+1`
//! is -1 and `n+2*t+1` is 1. This has the effect of transitioning B from emulated state `s` to
//! emulated state `t`.
//!
//! For example, if `n == 2`, `m == 2` and we have a rule:
//!
//! ```text
//! 0 [-1, 2] 0
//! ```
//!
//! It is translated to:
//!
//! ```text
//! 0 [-1,  2, -1,  1,  0,  0] 0
//! 0 [ 0,  0,  1, -1,  0,  0] 0
//! ```
//!
//! Whereas the rule:
//!
//! ```text
//! 0 [1, 1] 1 ```
//!
//! is translated to:
//!
//! ```text
//! 0 [1,  1, -1,  0,  1,  0] 0
//! ```
//!
//! Note that the translated rule fires in state 0 and the machine remains in state 0 after firing.
//!
//! We apply this translation to each rule from A that fires in state `s` obtaining a new list of
//! rules that operate over a single machine state 0 and have tape head adjustments corresponding
//! to `n + 2*m` tapes. Doing this for all original states gives us the translated program for
//! machine B.
//!
//! Initial tape positions for machine B are the same as for A in the first `n` tapes, and have a
//! single non-zero position of 1 at the emulated state corresponding to the initial machine state
//! for A. For example, if the initial machine state of A is 0 and the initial tape positions are
//! `[p_1, ..., p_n]`, then the intial tape positions for B are:
//!
//! ```text
//! [p_1, ..., p_n, 1, 0, 0, 0, ..., 0, 0]
//! ```
//!
//! [`magnificent`]: minsky::magnificent

use std::collections::HashMap;
use std::collections::HashSet;

use crate::magnificent::{Program, Rule, State};

/// The unique state of marvellous Minsky machines
const MARV_STATE: State = 0;

/// The action tape adjustment for emulated states
const ACTION_ADJ: i32 = 1;

/// The guard tape adjustment for emulated states
const GUARD_ADJ: i32 = -1;

/// Compute a mapping from original states to new states.
///
/// The original states are collected and sorted by value, then paired with a new state.
/// If there are `m` original states, then the new state values will range from 0 ... m-1.
fn compute_state_map(program: &Program) -> HashMap<State, State> {
    let mut orig_states: HashSet<usize> = HashSet::new();
    for rule in program.iter() {
        orig_states.insert(rule.cur_state());
        orig_states.insert(rule.next_state());
    }

    // sort and collect the original states into a map from original to new states. If
    // `num_orig_states == m`, then the new states are {0, 1, ..., m-1}.
    let mut orig_states: Vec<State> = orig_states.into_iter().collect();
    orig_states.sort();
    orig_states.into_iter().zip((0 as usize)..).collect()
}

/// Given an original rule, produce either one or two new rules for the Marvellous machine.
fn translate_rule(rule: &Rule, state_map: &HashMap<State, State>, num_tapes: usize) -> Vec<Rule> {
    let orig_tapes = rule.len();
    let mut ret_rules = Vec::new();
    let new_emulated_state = state_map
        .get(&rule.cur_state())
        .expect("state map is incomplete");

    // duplicate the original tape adjustments
    let mut new_rule = vec![0; num_tapes];
    for (i, a) in rule.iter().enumerate() {
        new_rule[i] = *a;
    }

    if rule.cur_state() == rule.next_state() {
        // produce two new rules for the original one
        new_rule[orig_tapes + 2 * new_emulated_state] = GUARD_ADJ;
        new_rule[orig_tapes + 2 * new_emulated_state + 1] = ACTION_ADJ;

        // aux_rule sends the machine back to rule.cur_state()
        let mut aux_rule = vec![0; num_tapes];
        aux_rule[orig_tapes + 2 * new_emulated_state] = ACTION_ADJ;
        aux_rule[orig_tapes + 2 * new_emulated_state + 1] = GUARD_ADJ;

        ret_rules.push(Rule::new(MARV_STATE, MARV_STATE, new_rule));
        ret_rules.push(Rule::new(MARV_STATE, MARV_STATE, aux_rule));
    } else {
        // produce a single new rule for the original one
        let new_next_emulated_state = state_map
            .get(&rule.next_state())
            .expect("state map is incomplete");
        new_rule[orig_tapes + 2 * new_emulated_state] = GUARD_ADJ;
        new_rule[orig_tapes + 2 * new_next_emulated_state] = ACTION_ADJ;
        ret_rules.push(Rule::new(MARV_STATE, MARV_STATE, new_rule));
    }
    ret_rules
}

pub fn transpile(program: &Program) -> Program {
    // collect and sort the original rules by state
    let state_map = compute_state_map(program);
    let num_orig_states = state_map.len();

    // iterate over rules for each orig state, produce new rule(s)
    let new_num_tapes = program.num_tapes() + 2 * num_orig_states;
    let mut new_rules = Vec::new();
    for rule in program.iter() {
        new_rules.extend(translate_rule(rule, &state_map, new_num_tapes));
    }
    Program::new(new_num_tapes, new_rules)
}

#[cfg(test)]
mod test {
    use super::{transpile, MARV_STATE};
    use crate::magnificent::{interpret, Machine, Program, Rule};

    // Test that a transpiled program has the expected number of tapes and rules
    #[test]
    fn simple_transpile() {
        let rule0 = Rule::new(0, 0, vec![1, 1, -1]);
        let rule1 = Rule::new(0, 1, vec![0, -5, 0]);
        let rule2 = Rule::new(1, 1, vec![-1, 1, 2]);
        let program = Program::new(3, vec![rule0, rule1, rule2]);
        let marv_program = transpile(&program);
        let new_num_tapes = 3 + 2 * 2; // 3 orig tapes + 2*2 orig states
        let new_num_rules = 2 + 1 + 2; // 2 for first orig rule, 1 for 2nd, 2 for 3rd
        assert_eq!(marv_program.num_tapes(), new_num_tapes);
        assert_eq!(marv_program.num_rules(), new_num_rules);
        for rule in marv_program.iter() {
            assert_eq!(rule.len(), new_num_tapes);
        }
    }

    // Test that the transpiled adder program works
    #[test]
    fn transpile_equivalent_adder() {
        let rule = Rule::new(0, 0, vec![1, -1]);
        let program = Program::new(2, vec![rule]);
        let x = 4;
        let y = 5;

        let marv_program = transpile(&program);
        let marv_machine = Machine::new(0, vec![x, y, 1, 0]);

        let end_machine = interpret(marv_machine, &marv_program, 100);
        assert!(end_machine.is_ok());
        let (_, end_machine) = end_machine.unwrap();
        assert_eq!(end_machine.tape_pos(0), x + y);
    }

    // Test that the transpiled multiplier program works
    #[test]
    fn transpile_equivalent_mult() {
        let x = 3;
        let y = 11;
        let rule0 = Rule::new(0, 0, vec![1, -1, 1, 0]);
        let rule1 = Rule::new(0, 1, vec![0, 0, 0, 0]);
        let rule2 = Rule::new(1, 1, vec![0, 1, -1, 0]);
        let rule3 = Rule::new(1, 0, vec![0, 0, 0, -1]);
        let magnificent_program = Program::new(4, vec![rule0, rule1, rule2, rule3]);
        let marvellous_program = transpile(&magnificent_program);
        let machine = Machine::new(MARV_STATE, vec![0, x, 0, y - 1, 1, 0, 0, 0]);

        let end_machine = interpret(machine, &marvellous_program, 1000);
        assert!(end_machine.is_ok());
        assert_eq!(end_machine.unwrap().1.tape_pos(0), x * y);
    }
}
