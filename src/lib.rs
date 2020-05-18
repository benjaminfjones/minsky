pub mod arith;
pub mod m3_parser;
pub mod magnificent;

#[macro_use]
extern crate lalrpop_util;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
