use crate::logic_expression::NotTerminal::*;
use crate::logic_expression::SyntaxTree::*;
use crate::logic_expression::Token_::*;
use crate::logic_expression::{ParseError, SyntaxTree};
use lazy_static::lazy_static;

fn parse(str: &str) -> Result<SyntaxTree, ParseError> {
    let s = crate::logic_expression::parse(str.to_string());
    s.map(|(_, tree)| tree)
}

#[test]
fn bracket_balance() {
    let parsed = parse("(a(");
    assert!(parsed.is_err())
}

#[test]
fn empty_input() {
    let parsed = parse("");
    assert!(parsed.is_err())
}

#[test]
fn few_operations() {
    let parsed = parse("xor and or");
    assert!(parsed.is_err())
}

#[test]
fn few_variables() {
    let parsed = parse("a b");
    assert!(parsed.is_err())
}

#[test]
fn single_not() {
    let parsed = parse("not");
    assert!(parsed.is_err())
}

#[test]
fn single_xor() {
    let parsed = parse("xor");
    assert!(parsed.is_err())
}

#[test]
fn single_or() {
    let parsed = parse("or");
    assert!(parsed.is_err())
}

#[test]
fn single_and() {
    let parsed = parse("and");
    assert!(parsed.is_err())
}

#[test]
fn right_xor() {
    let parsed = parse("a xor");
    assert!(parsed.is_err())
}

#[test]
fn right_or() {
    let parsed = parse("a or");
    assert!(parsed.is_err())
}

#[test]
fn right_and() {
    let parsed = parse("a and");
    assert!(parsed.is_err())
}

#[test]
fn left_xor() {
    let parsed = parse("xor a");
    assert!(parsed.is_err())
}

#[test]
fn left_or() {
    let parsed = parse("or a");
    assert!(parsed.is_err())
}

#[test]
fn left_and() {
    let parsed = parse("and a");
    assert!(parsed.is_err())
}

#[test]
fn double_not() {
    let parsed = parse("not not a");

    assert!(parsed.is_err());
}

lazy_static! {
    static ref VAR_TREE: SyntaxTree = Tree(
        X,
        vec!(
            Tree(
                O,
                vec!(
                    Tree(A, vec!(Tree(N, vec!(Tree(T, vec!(Leaf(Var))))), Tree(A_, vec!(Leaf(Eps))))),
                    Tree(O_, vec!(Leaf(Eps)))
                )
            ),
            Tree(X_, vec!(Leaf(Eps)))
        )
    );
}

#[test]
fn var() {
    let expected = VAR_TREE.clone();

    let parsed = parse("a");
    assert!(parsed.is_ok());
    assert_eq!(expected, parsed.unwrap());
}

#[test]
fn var_in_bracket() {
    let expected = Tree(
        X,
        vec![
            Tree(
                O,
                vec![
                    Tree(
                        A,
                        vec![
                            Tree(
                                N,
                                vec![Tree(T, vec![Leaf(LeftBracket), VAR_TREE.clone(), Leaf(RightBracket)])],
                            ),
                            Tree(A_, vec![Leaf(Eps)]),
                        ],
                    ),
                    Tree(O_, vec![Leaf(Eps)]),
                ],
            ),
            Tree(X_, vec![Leaf(Eps)]),
        ],
    );

    let parsed = parse("(a)");
    assert!(parsed.is_ok());
    assert_eq!(expected, parsed.unwrap());
}

#[test]
fn not_with_var() {
    let expected = Tree(
        X,
        vec![
            Tree(
                O,
                vec![
                    Tree(
                        A,
                        vec![
                            Tree(N, vec![Leaf(Not), Tree(T, vec![Leaf(Var)])]),
                            Tree(A_, vec![Leaf(Eps)]),
                        ],
                    ),
                    Tree(O_, vec![Leaf(Eps)]),
                ],
            ),
            Tree(X_, vec![Leaf(Eps)]),
        ],
    );

    let parsed = parse("not a");
    assert!(parsed.is_ok());
    assert_eq!(expected, parsed.unwrap());
}

#[test]
fn not_with_expr_in_bracket() {
    let expected = Tree(
        X,
        vec![
            Tree(
                O,
                vec![
                    Tree(
                        A,
                        vec![
                            Tree(
                                N,
                                vec![
                                    Leaf(Not),
                                    Tree(
                                        T,
                                        vec![
                                            Leaf(LeftBracket),
                                            Tree(
                                                X,
                                                vec![
                                                    Tree(
                                                        O,
                                                        vec![
                                                            Tree(
                                                                A,
                                                                vec![
                                                                    Tree(N, vec![Tree(T, vec![Leaf(Var)])]),
                                                                    Tree(A_, vec![Leaf(Eps)]),
                                                                ],
                                                            ),
                                                            Tree(O_, vec![Leaf(Eps)]),
                                                        ],
                                                    ),
                                                    Tree(X_, vec![Leaf(Eps)]),
                                                ],
                                            ),
                                            Leaf(RightBracket),
                                        ],
                                    ),
                                ],
                            ),
                            Tree(A_, vec![Leaf(Eps)]),
                        ],
                    ),
                    Tree(O_, vec![Leaf(Eps)]),
                ],
            ),
            Tree(X_, vec![Leaf(Eps)]),
        ],
    );
    let parsed = parse("not (a)");

    assert!(parsed.is_ok());
    assert_eq!(expected, parsed.unwrap());
}



#[test]
fn xor() {
    let expected = Tree(
        X,
        vec![
            Tree(
                O,
                vec![
                    Tree(A, vec![Tree(N, vec![Tree(T, vec![Leaf(Var)])]), Tree(A_, vec![Leaf(Eps)])]),
                    Tree(O_, vec![Leaf(Eps)]),
                ],
            ),
            Tree(
                X_,
                vec![
                    Leaf(Xor),
                    Tree(
                        O,
                        vec![
                            Tree(A, vec![Tree(N, vec![Tree(T, vec![Leaf(Var)])]), Tree(A_, vec![Leaf(Eps)])]),
                            Tree(O_, vec![Leaf(Eps)]),
                        ],
                    ),
                    Tree(X_, vec![Leaf(Eps)]),
                ],
            ),
        ],
    );
    let parsed = parse("a xor b");

    assert!(parsed.is_ok());
    assert_eq!(expected, parsed.unwrap())
}

#[test]
fn few_xor() {
    let expected = Tree(
        X,
        vec![
            Tree(
                O,
                vec![
                    Tree(A, vec![Tree(N, vec![Tree(T, vec![Leaf(Var)])]), Tree(A_, vec![Leaf(Eps)])]),
                    Tree(O_, vec![Leaf(Eps)]),
                ],
            ),
            Tree(
                X_,
                vec![
                    Leaf(Xor),
                    Tree(
                        O,
                        vec![
                            Tree(A, vec![Tree(N, vec![Tree(T, vec![Leaf(Var)])]), Tree(A_, vec![Leaf(Eps)])]),
                            Tree(O_, vec![Leaf(Eps)]),
                        ],
                    ),
                    Tree(
                        X_,
                        vec![
                            Leaf(Xor),
                            Tree(
                                O,
                                vec![
                                    Tree(
                                        A,
                                        vec![Tree(N, vec![Tree(T, vec![Leaf(Var)])]), Tree(A_, vec![Leaf(Eps)])],
                                    ),
                                    Tree(O_, vec![Leaf(Eps)]),
                                ],
                            ),
                            Tree(X_, vec![Leaf(Eps)]),
                        ],
                    ),
                ],
            ),
        ],
    );
    let parsed = parse("a xor b xor c");

    assert!(parsed.is_ok());
    assert_eq!(expected, parsed.unwrap())
}

#[test]
fn or() {
    let expected = Tree(
        X,
        vec![
            Tree(
                O,
                vec![
                    Tree(A, vec![Tree(N, vec![Tree(T, vec![Leaf(Var)])]), Tree(A_, vec![Leaf(Eps)])]),
                    Tree(
                        O_,
                        vec![
                            Leaf(Or),
                            Tree(A, vec![Tree(N, vec![Tree(T, vec![Leaf(Var)])]), Tree(A_, vec![Leaf(Eps)])]),
                            Tree(O_, vec![Leaf(Eps)]),
                        ],
                    ),
                ],
            ),
            Tree(X_, vec![Leaf(Eps)]),
        ],
    );
    let parsed = parse("a or b");

    assert!(parsed.is_ok());
    assert_eq!(expected, parsed.unwrap())
}

#[test]
fn few_or() {
    let expected = Tree(
        X,
        vec![
            Tree(
                O,
                vec![
                    Tree(A, vec![Tree(N, vec![Tree(T, vec![Leaf(Var)])]), Tree(A_, vec![Leaf(Eps)])]),
                    Tree(
                        O_,
                        vec![
                            Leaf(Or),
                            Tree(A, vec![Tree(N, vec![Tree(T, vec![Leaf(Var)])]), Tree(A_, vec![Leaf(Eps)])]),
                            Tree(
                                O_,
                                vec![
                                    Leaf(Or),
                                    Tree(
                                        A,
                                        vec![Tree(N, vec![Tree(T, vec![Leaf(Var)])]), Tree(A_, vec![Leaf(Eps)])],
                                    ),
                                    Tree(O_, vec![Leaf(Eps)]),
                                ],
                            ),
                        ],
                    ),
                ],
            ),
            Tree(X_, vec![Leaf(Eps)]),
        ],
    );
    let parsed = parse("a or b or c");

    assert!(parsed.is_ok());
    assert_eq!(expected, parsed.unwrap())
}

#[test]
fn and() {
    let expected = Tree(
        X,
        vec![
            Tree(
                O,
                vec![
                    Tree(
                        A,
                        vec![
                            Tree(N, vec![Tree(T, vec![Leaf(Var)])]),
                            Tree(
                                A_,
                                vec![
                                    Leaf(And),
                                    Tree(N, vec![Tree(T, vec![Leaf(Var)])]),
                                    Tree(A_, vec![Leaf(Eps)]),
                                ],
                            ),
                        ],
                    ),
                    Tree(O_, vec![Leaf(Eps)]),
                ],
            ),
            Tree(X_, vec![Leaf(Eps)]),
        ],
    );
    let parsed = parse("a and b");

    assert!(parsed.is_ok());
    assert_eq!(expected, parsed.unwrap())
}

#[test]
fn few_and() {
    let expected = Tree(
        X,
        vec![
            Tree(
                O,
                vec![
                    Tree(
                        A,
                        vec![
                            Tree(N, vec![Tree(T, vec![Leaf(Var)])]),
                            Tree(
                                A_,
                                vec![
                                    Leaf(And),
                                    Tree(N, vec![Tree(T, vec![Leaf(Var)])]),
                                    Tree(
                                        A_,
                                        vec![
                                            Leaf(And),
                                            Tree(N, vec![Tree(T, vec![Leaf(Var)])]),
                                            Tree(A_, vec![Leaf(Eps)]),
                                        ],
                                    ),
                                ],
                            ),
                        ],
                    ),
                    Tree(O_, vec![Leaf(Eps)]),
                ],
            ),
            Tree(X_, vec![Leaf(Eps)]),
        ],
    );
    let parsed = parse("a and b and c");

    assert!(parsed.is_ok());
    assert_eq!(expected, parsed.unwrap())
}

#[test]
fn complex_expression() {
    let expected = Tree(X, vec![Tree(O, vec![Tree(A, vec![Tree(N, vec![Tree(T, vec![Leaf(LeftBracket), Tree(X, vec![Tree(O, vec![Tree(A, vec![Tree(N, vec![Tree(T, vec![Leaf(Var)])]), Tree(A_, vec![Leaf(Eps)])]), Tree(O_, vec![Leaf(Eps)])]), Tree(X_, vec![Leaf(Xor), Tree(O, vec![Tree(A, vec![Tree(N, vec![Tree(T, vec![Leaf(Var)])]), Tree(A_, vec![Leaf(Eps)])]), Tree(O_, vec![Leaf(Eps)])]), Tree(X_, vec![Leaf(Eps)])])]), Leaf(RightBracket)])]), Tree(A_, vec![Leaf(And), Tree(N, vec![Leaf(Not), Tree(T, vec![Leaf(LeftBracket), Tree(X, vec![Tree(O, vec![Tree(A, vec![Tree(N, vec![Tree(T, vec![Leaf(Var)])]), Tree(A_, vec![Leaf(Eps)])]), Tree(O_, vec![Leaf(Or), Tree(A, vec![Tree(N, vec![Tree(T, vec![Leaf(Var)])]), Tree(A_, vec![Leaf(Eps)])]), Tree(O_, vec![Leaf(Eps)])])]), Tree(X_, vec![Leaf(Eps)])]), Leaf(RightBracket)])]), Tree(A_, vec![Leaf(Eps)])])]), Tree(O_, vec![Leaf(Eps)])]), Tree(X_, vec![Leaf(Xor), Tree(O, vec![Tree(A, vec![Tree(N, vec![Tree(T, vec![Leaf(Var)])]), Tree(A_, vec![Leaf(Eps)])]), Tree(O_, vec![Leaf(Or), Tree(A, vec![Tree(N, vec![Tree(T, vec![Leaf(Var)])]), Tree(A_, vec![Leaf(And), Tree(N, vec![Leaf(Not), Tree(T, vec![Leaf(LeftBracket), Tree(X, vec![Tree(O, vec![Tree(A, vec![Tree(N, vec![Tree(T, vec![Leaf(Var)])]), Tree(A_, vec![Leaf(Eps)])]), Tree(O_, vec![Leaf(Eps)])]), Tree(X_, vec![Leaf(Xor), Tree(O, vec![Tree(A, vec![Tree(N, vec![Tree(T, vec![Leaf(Var)])]), Tree(A_, vec![Leaf(Eps)])]), Tree(O_, vec![Leaf(Eps)])]), Tree(X_, vec![Leaf(Eps)])])]), Leaf(RightBracket)])]), Tree(A_, vec![Leaf(Eps)])])]), Tree(O_, vec![Leaf(Eps)])])]), Tree(X_, vec![Leaf(Eps)])])]);
    let parsed = parse("(a xor b) and not (y or b) xor c or d and not (b xor o)");

    assert!(parsed.is_ok());
    assert_eq!(expected, parsed.unwrap())
}

#[test]
fn priority() {
    let expected = Tree(
        X,
        vec![
            Tree(
                O,
                vec![
                    Tree(
                        A,
                        vec![
                            Tree(N, vec![Leaf(Not), Tree(T, vec![Leaf(Var)])]),
                            Tree(
                                A_,
                                vec![
                                    Leaf(And),
                                    Tree(N, vec![Tree(T, vec![Leaf(Var)])]),
                                    Tree(A_, vec![Leaf(Eps)]),
                                ],
                            ),
                        ],
                    ),
                    Tree(
                        O_,
                        vec![
                            Leaf(Or),
                            Tree(A, vec![Tree(N, vec![Tree(T, vec![Leaf(Var)])]), Tree(A_, vec![Leaf(Eps)])]),
                            Tree(O_, vec![Leaf(Eps)]),
                        ],
                    ),
                ],
            ),
            Tree(
                X_,
                vec![
                    Leaf(Xor),
                    Tree(
                        O,
                        vec![
                            Tree(A, vec![Tree(N, vec![Tree(T, vec![Leaf(Var)])]), Tree(A_, vec![Leaf(Eps)])]),
                            Tree(O_, vec![Leaf(Eps)]),
                        ],
                    ),
                    Tree(X_, vec![Leaf(Eps)]),
                ],
            ),
        ],
    );
    let parsed = parse("not a and b or c xor d ");

    assert!(parsed.is_ok());
    assert_eq!(expected, parsed.unwrap())
}
