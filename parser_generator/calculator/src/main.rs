use std::io;
use graph_viz::GraphVizNode;
use crate::calculator::{parse, ParseError};

mod calculator;

fn main() -> io::Result<()> {
    let io = io::stdin();
    for s in io.lines() {
        let t = s?;
        let res = parse(t.trim().to_string());
        match res {
            Ok((_, res)) => {println!("Result: {res}")}
            Err(_) => {println!("Invalid expression")}
        }
    }
    Ok(())
}

#[cfg(test)]
mod calculator_test {
    use crate::calculator::parse;

    #[test]
    fn invalid_tokens() {
        let e = parse("7### asdas zxzx".to_string());
        assert!(e.is_err())
    }

    #[test]
    fn invalid_expr() {
        let e = parse("9 + + 4 - 5 * * 6".to_string());
        assert!(e.is_err())
    }

    #[test]
    fn num() {
        let (_, res) = parse("4".to_string()).unwrap();
        assert_eq!(4, res)
    }
    #[test]
    fn sum() {
        let (_, res) = parse("9 + 4".to_string()).unwrap();
        assert_eq!(9 + 4, res );
    }

    #[test]
    fn sub() {
        let (_, res) = parse("9 - 4".to_string()).unwrap();
        assert_eq!(9 - 4, res);
    }

    #[test]
    fn mul() {
        let (_, res) = parse("9 * 4".to_string()).unwrap();
        assert_eq!(9 * 4, res);
    }

    #[test]
    fn div() {
        let (_, res) = parse("9 / 4".to_string()).unwrap();
        assert_eq!(9 / 4, res );
    }

    #[test]
    fn neg() {
        let (_, res) = parse("- (9 + 4)".to_string()).unwrap();
        assert_eq!(-(9 + 4), res );
    }

    #[test]
    fn complex_expression() {
        let (_, res) = parse("4 * 3 - 5 / 3 * 6 - (10 - (-(10 - 1))) + 4".to_string()).unwrap();
        assert_eq!(4 * 3 - 5 / 3 * 6 - (10 - (-(10 - 1))) + 4, res)
    }
}