use std::fs::File;
use std::io::{Read, stdin, Write};
use std::process::{Command, Stdio};
use graph_viz::{generate_dot_format, GraphVizNode};
use crate::logic_expression::{parse, SyntaxTree};
use crate::logic_expression::SyntaxTree::{Leaf, Tree};

mod logic_expression;
mod test;

fn graph_viz_from_syntax_tree(syntax_tree: &SyntaxTree) -> GraphVizNode {
   graph_viz_from_syntax_tree_with_counter(syntax_tree, 0).0
}

fn graph_viz_from_syntax_tree_with_counter(syntax_tree: &SyntaxTree, id: i64) -> (GraphVizNode, i64) {
    match syntax_tree {
        Leaf(term) => (
            GraphVizNode::new_leaf(
                format!("term_{id}"),
                format!("{term:?}"),
            ),
            id + 1,
        ),
        Tree(not_terminal, children) => {
            let (children, new_count) = children.iter().fold(
                (Vec::with_capacity(children.len()), id + 1),
                |(mut vector, prev_id), elem| {
                    let (tree, new_id) =
                        graph_viz_from_syntax_tree_with_counter(elem, prev_id);
                    vector.push(tree);
                    (vector, new_id)
                },
            );

            (
                GraphVizNode::new_node(
                    format!("not_term_{id}"),
                    format!("{not_terminal:?}"),
                    children,
                ),
                new_count,
            )
        }
    }
}

fn read_input() -> String {
    let mut buffer = String::new();
    let stdin = stdin();
    let mut lock = stdin.lock();
    lock.read_to_string(&mut buffer).unwrap();
    buffer
}

fn main() {
    let input = read_input();
    let syntax_tree = match parse(input).map(|(_, t)| t) {
        Ok(tree) => tree,
        Err(parseError) => {
            eprintln!("{}", parseError.message);
            return;
        }
    };

    println!("{syntax_tree:?}");

    let dot_string = generate_dot_format(&graph_viz_from_syntax_tree(&syntax_tree));
    let result_file = File::create("result.svg").expect("Cannot open file to write result");

    let graphviz = Command::new("dot")
        .stdin(Stdio::piped())
        .stdout(result_file)
        .arg("-Tsvg")
        .spawn()
        .expect("Cannot start dot command");

    graphviz
        .stdin
        .expect("Expected stdin exist")
        .write_all(&dot_string.into_bytes())
        .expect("Cannot write image to result.svg");
}
