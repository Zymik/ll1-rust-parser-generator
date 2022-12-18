use std::collections::{HashMap, HashSet};
use std::iter::Map;
use std::ops::Add;
use std::slice::Iter;
use crate::parser_generator::parser_description::{ParserDescription, Rule, RuleMember, RuleToken, Typed};
use crate::parser_generator::tokenizer_generator::EOF_TOKEN;


const S: &'static str = "S";

const GET_TOKEN: &'static str = "let token = &self.tokens[self.pointer];";


struct Grammar<'a> {
    terminal: HashSet<&'a str>,
    not_terminal: HashSet<&'a str>,
    rules: HashMap<&'a str, Rules<'a>>,
}

const PARSER: &'static str =
    "
struct Parser {
    tokens: Vec<Token>,
    pointer: usize,
    counter: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser{tokens, pointer: 0, counter: 0}
    }
}

";

const EOF_TOKEN_PARSE: &'static str =
    "
    fn Eof(&mut self) -> Result<(String, GraphVizNode), ParseError> {
        let token = &self.tokens[self.pointer];
        let pos = self.pointer;
        let id = self.counter.to_string();
        self.pointer += 1;
        self.counter += 1;
        match token {
            Token::Eof => Ok((\"Eof\".to_string(), GraphVizNode::new_leaf(id, \"Eof\".to_string()))),
            _          => Err(ParseError{position: pos, message: \"Expected eof\".to_string()})
        }
    }
";

type Rules<'a> = Vec<Vec<&'a str>>;

pub fn generate_parser(parser_description: &ParserDescription) -> String {

    let not_terminals: HashSet<&str> = parser_description
        .not_terminal
        .iter()
        .map(|x| x.name.as_str())
        .collect();

    if !not_terminals.contains("S") {
        panic!("No start terminal with name S")
    }
    let terminals: HashSet<&str> = parser_description
        .tokens
        .iter()
        .map(|x| x.name.as_str())
        .collect();



    let rules: HashMap<&str, Rules> = parser_description
        .not_terminal
        .iter()
        .map(|not_term| (not_term.name.as_str(), get_rules(&not_term.rules)))
        .collect();

    let grammar = Grammar { terminal: terminals, not_terminal: not_terminals, rules };

    let first = generate_first(&grammar);
    let follow = generate_follow(&grammar, &first);


    if !validate_first_and_follow(&first, &follow, &grammar) {
        panic!("Not LL(1) grammar")
    }


    let mut answer = String::new();
    answer.push_str(PARSER);
    answer.push_str("impl Parser {\n");

    answer.push_str(&generate_tokens_parse(&grammar));

    let rules = generate_rules(
        &grammar,
        &parser_description,
        &first,
        &follow,
    );

    answer.push_str(&rules);
    answer.push_str("}");

    answer.push_str(&generate_parse_func(&parser_description));

    answer
}

fn generate_parse_func(parser_description: &ParserDescription) -> String {
    let mut answer = String::new();
    let start = parser_description.not_terminal.iter().find(|s| s.name == "S").unwrap();

    let return_type = get_return_type(&start.returns);
    let input = Typed{name: "input".to_string(), ty: "String".to_string()};
    let mut all_input_args = start.args.clone();
    all_input_args.insert(0, input);

    let input_types = get_input_args_without_self(&all_input_args);
    let signature = format!("pub fn parse{input_types} -> {return_type}");
    let parser_input = get_tuple("", &start.args);
    let parsing = format!("Parser::new(Tokenizer::tokenize(input)?).S{parser_input}");
    let func = format!("{signature} {{ {parsing} }}");
    answer.push_str(&func);
    answer

}

fn generate_tokens_parse(grammar: &Grammar) -> String {
    let mut answer = String::new();
    for terminal in &grammar.terminal {
        answer.push_str(&generate_token_parse(terminal))
    }
    answer.push_str(EOF_TOKEN_PARSE);
    answer
}

fn generate_token_parse(terminal: &str) -> String {
    format!("
    fn {}(&mut self) -> Result<(GraphVizNode, (String)), ParseError> {{
        let token = &self.tokens[self.pointer];
        let pos = self.pointer;
        let id = self.counter.to_string();
        self.pointer += 1;
        self.counter += 1;
        match token {{
            Token::{}(s) => Ok((GraphVizNode::new_leaf(id, \"{}\".to_string()), s.clone())),
            _ => Err(ParseError{{position: pos, message: \"Expected {}\".to_string()}}),
        }}
    }}
   ", terminal, terminal, terminal, terminal)
}

fn generate_rules(
    grammar: &Grammar,
    parser_description: &ParserDescription,
    first: &HashMap<&str, HashSet<&str>>,
    follow: &HashMap<&str, HashSet<&str>>,
) -> String {
    let mut answer = String::new();
    let not_terminals = &parser_description.not_terminal;

    let return_map = get_return_map(parser_description);

    for not_term in not_terminals {
        let name = not_term.name.clone();

        let input_types = get_input_args_with_self(&not_term.args);
        let return_type = get_return_type(&not_term.returns);
        let signature = format!("fn {name}{input_types} -> {return_type}");

        answer.push_str(&signature);
        answer.push_str(" {\n");

        answer.push_str(&get_return_init(&not_term.returns));
        answer.push('\n');

        answer.push_str("let id = self.counter.to_string();\n");
        answer.push_str("let mut children: Vec<GraphVizNode> = Vec::new();\n");
        answer.push_str("self.counter += 1;\n");
        answer.push_str("let token = &self.tokens[self.pointer];\n");
        answer.push_str("match token {\n");

        for rule in &not_term.rules {
            let str_rule = convert_rule_to_strings(rule);
            let mut rule_first = get_first(&str_rule, grammar, first);
            if rule_first.contains("") {
                rule_first.extend(&follow[&name.as_str()])
            }
            rule_first.remove("");

            if rule_first.is_empty() {
                continue;
            }

            let token_case = get_tokens_for_match(&rule_first);

            answer.push_str(&token_case);
            answer.push_str("=> {\n");

            answer.push_str(&get_parsing_rule(rule, &return_map));

            answer.push_str("}\n");
        }

        answer.push_str("_ => return Err(ParseError{position: self.pointer, message: \"Can't match rule\".to_string()})\n");
        answer.push_str("}\n");
        let graph_viz = format!("GraphVizNode::new_node(id, \"{name}\".to_string(), children)");
        let tuple = get_tuple("", &not_term.returns);
        let return_line = format!("Ok(({graph_viz},{tuple}))\n");
        answer.push_str(&return_line);
        answer.push_str("}\n\n");
    }

    answer
}

fn get_parsing_rule(rule: &Rule, return_map: &HashMap<&str, Vec<Typed>>) -> String{
    let mut answer: String = String::new();


    for i in 0 .. rule.members.len() {
        let rule_member = &rule.members[i];
        let s = match rule_member {
            RuleMember::RuleCall(RuleToken{name, args}) => {
                let ident_prefix = name.clone().add(&format!("{i}_"));
                let call = format!("self.{name}{args}?;");
                let tuple = get_tuple(&ident_prefix, &return_map[name.as_str()]);
                let assignment = format!("let (child, {tuple}) = {call}");
                let child_push = format!("children.push(child);");
                assignment
                    .add("\n")
                    .add(&child_push)
                    .add("\n")
            }
            RuleMember::Command(command) => command.to_string()
                .add("\n")
        };
        answer.push_str(&s);
    }
    answer
}

fn get_tuple(prefix: &str, idents: &Vec<Typed>) -> String {
    let tuple = idents
        .iter()
        .map(
            |Typed{name, ..}|
                format!("{prefix}{name}")
        ).collect::<Vec<String>>()
        .join(",");

    format!("({tuple})")
}

fn get_tokens_for_match(rule_first: &HashSet<&str>) -> String {
    rule_first
        .iter()
        .map(
            |s| {
                if s == &EOF_TOKEN {
                    return format!("Token::{EOF_TOKEN}");
                }
                return format!("Token::{s}(_)");
            }
        ).collect::<Vec<String>>().join("|")
}

fn get_return_map(parser_description: &ParserDescription) -> HashMap<&str, Vec<Typed>> {
    let mut map = HashMap::new();
    let term = &parser_description.tokens;
    let ident = vec!(Typed { name: "ident".to_string(), ty: "String".to_string() });
    for t in term {
        map.insert(t.name.as_str(), ident.clone());
    }

    let not_term = &parser_description.not_terminal;
    for t in not_term {
        map.insert(t.name.as_str(), t.returns.clone());
    }
    map
}

fn get_return_init(args: &Vec<Typed>) -> String {
    get_typed_strings(args)
        .fold(String::new(), |mut acc, s| {
            acc.push_str("let mut ");
            acc.push_str(&s);
            acc.push(';');
            acc
        },
        )
}

fn get_typed_strings(args: &Vec<Typed>) -> Map<Iter<Typed>, fn(&Typed) -> String> {
    args.iter()
        .map(|Typed { name, ty }| {
            let mut s = String::with_capacity(name.len() + ty.len() + 1);
            s.push_str(&name);
            s.push(':');
            s.push_str(&ty);
            s
        }
        )
}

fn get_input_args_with_prefix(args: &Vec<Typed>, prefix: &str) -> String {
    let mut input_types =
        get_typed_strings(args)
            .fold(prefix.to_string(), |mut acc, s| {
                acc.push_str(&s);
                acc.push(',');
                acc
            },
            );
    input_types.push(')');

    input_types
}

fn get_input_args_without_self(args: &Vec<Typed>) -> String {
    get_input_args_with_prefix(args, "(")
}

fn get_input_args_with_self(args: &Vec<Typed>) -> String {
    get_input_args_with_prefix(args, "(&mut self,")
}

fn get_return_type(tuple_args: &Vec<Typed>) -> String {
    let return_types = tuple_args
        .iter()
        .map(|t| t.ty.clone())
        .collect::<Vec<String>>()
        .join(",");


    let mut return_type = "Result<(GraphVizNode, ".to_string();

    return_type.push_str(&format!("({return_types})"));
    return_type.push(')');
    return_type.push_str(",ParseError>");
    return_type
}

fn validate_first_and_follow(
    first: &HashMap<&str, HashSet<&str>>,
    follow: &HashMap<&str, HashSet<&str>>,
    grammar: &Grammar,
) -> bool {
    for a in &grammar.not_terminal {
        let a_follow = &follow[a];
        let rules = &grammar.rules[a];
        for i in 0..rules.len() {
            for j in i + 1..rules.len() {
                let alpha = rules.get(i).unwrap();
                let beta = rules.get(j).unwrap();
                let first_alpha = get_first(
                    alpha,
                    grammar,
                    first,
                );

                let first_beta = get_first(
                    beta,
                    grammar,
                    first,
                );

                if !first_alpha.is_disjoint(&first_beta) {
                    return false;
                }

                if first_alpha.contains("") && !a_follow.is_disjoint(&first_beta) {
                    return false;
                }

                if first_beta.contains("") && !a_follow.is_disjoint(&first_alpha) {
                    return false;
                }
            }
        }
    }
    true
}


fn generate_first<'a>(grammar: &'a Grammar) -> HashMap<&'a str, HashSet<&'a str>> {
    let mut not_terminal: HashMap<&str, HashSet<&str>> = HashMap::with_capacity(grammar.not_terminal.len());

    for a in &grammar.not_terminal {
        not_terminal.insert(a, HashSet::new());
    }
    let mut changed = true;
    while changed {
        changed = false;
        for a in &grammar.not_terminal {
            let rules = &grammar.rules[a];

            let rules_first: Vec<HashSet<&str>> = rules
                .iter()
                .map(|rule| get_first(rule, grammar, &not_terminal))
                .collect();

            let first = not_terminal.get_mut(a).unwrap();

            for rule_first in rules_first {
                for s in rule_first {
                    if first.insert(s) {
                        changed = true;
                    }
                }
            }
        }
    }
    not_terminal
}

fn get_first<'a>(rule: &[&'a str], grammar: &Grammar, not_terminal_first: &HashMap<&'a str, HashSet<&'a str>>) -> HashSet<&'a str> {
    if rule.is_empty() {
        return HashSet::from([""]);
    }

    let first_token = rule[0];

    if grammar.terminal.contains(first_token) {
        return HashSet::from([first_token]);
    }

    let mut first = not_terminal_first[first_token].clone();

    if first.contains("") {
        first.extend(get_first(&rule[1..], grammar, not_terminal_first))
    }

    first
}

fn generate_follow<'a>(grammar: &Grammar<'a>, first: &HashMap<&'a str, HashSet<&'a str>>)
                       -> HashMap<&'a str, HashSet<&'a str>> {
    let mut follow: HashMap<&'a str, HashSet<&'a str>> =
        HashMap::with_capacity(grammar.not_terminal.len());

    for a in &grammar.not_terminal {
        follow.insert(a, HashSet::new());
    }

    follow.insert(S, HashSet::from([EOF_TOKEN]));

    let mut changed = true;


    while changed {
        changed = false;
        for a in &grammar.not_terminal {
            let rules = &grammar.rules[a];

            for rule in rules {
                for i in 0..rule.len() {
                    let b = &rule[i];
                    if !grammar.not_terminal.contains(b) {
                        continue;
                    }
                    let first_rest = get_first(&rule[i + 1..], grammar, first);

                    let mut follow_b = follow[b].clone();

                    for s in &first_rest {
                        if !s.is_empty() && follow_b.insert(s) {
                            changed = true;
                        }
                    }

                    if first_rest.contains("") {
                        for s in &follow[a] {
                            if follow_b.insert(s) {
                                changed = true;
                            }
                        }
                    }

                    follow.insert(b, follow_b);
                }
            }
        }
    }

    follow
}

fn get_rules(rules: &Vec<Rule>) -> Rules {
    rules.iter()
        .map(convert_rule_to_strings)
        .collect()
}

fn convert_rule_to_strings(rule: &Rule) -> Vec<&str> {
    match rule {
        Rule { members } =>
            members
                .iter()
                .filter_map(|x| match x {
                    RuleMember::RuleCall(RuleToken { name, .. }) => Some(name.as_str()),
                    RuleMember::Command(_) => None
                }
                )
                .collect()
    }
}