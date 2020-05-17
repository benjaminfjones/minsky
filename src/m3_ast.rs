#[derive(Debug)]
pub struct RawProgram {
    pub num_tapes: usize,
    pub rules: Vec<RawRule>,
}

#[derive(Debug)]
pub struct RawRule {
    pub cur_state: i32,
    pub next_state: i32,
    pub rule: Vec<i32>,
}
