use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Shape {
    Circle,
    Box,
}

impl Display for Shape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Shape::Circle => "circle",
            Shape::Box => "box",
        };

        f.write_str(string)
    }
}

pub struct GraphVizNode {
    id: String,
    label: String,
    shape: Shape,
    children: Vec<GraphVizNode>,
}

const SPACES: &'static str = "    ";

impl GraphVizNode {
    fn new(id: String,
           label: String,
           shape: Shape,
           children: Vec<GraphVizNode>, ) -> GraphVizNode {
        GraphVizNode { id, children, label, shape }
    }

    pub fn new_leaf(id: String, label: String) -> GraphVizNode {
        GraphVizNode::new(id, label, Shape::Box, Vec::new())
    }

    pub fn new_node(id: String,
                    label: String,
                    children: Vec<GraphVizNode>, ) -> GraphVizNode {
        GraphVizNode::new(id, label, Shape::Circle, children)
    }

    fn to_dot_language(&self) -> String {
        let mut buffer = String::new();
        buffer.push_str("digraph ParseTree {\n");
        self.generates_nodes(&mut buffer);
        buffer.push_str("\n");
        self.generate_paths(&mut buffer);
        buffer.push('}');
        buffer
    }

    fn generates_nodes(&self, buffer: &mut String) {
        match self {
            GraphVizNode {
                id,
                label,
                shape,
                children,
            } => {
                let node_details = format!("{SPACES}{id}[shape={shape}, label={label}]\n");
                buffer.push_str(node_details.as_str());
                children
                    .iter()
                    .for_each(|node| node.generates_nodes(buffer))
            }
        }
    }

    fn generate_paths(&self, buffer: &mut String) {
        match self {
            GraphVizNode { id, children, .. } => {
                if children.len() == 0 {
                    return;
                }

                buffer.push_str(format!("{SPACES}{id}->{{").as_str());

                children
                    .iter()
                    .for_each(|GraphVizNode { id, .. }| buffer.push_str(format!("{id},").as_str()));
                buffer.pop();
                buffer.push_str("}\n");
                children.iter().for_each(|node| node.generate_paths(buffer))
            }
        }
    }
}

pub fn generate_dot_format(syntax_tree: &GraphVizNode) -> String {
    syntax_tree.to_dot_language()
}
