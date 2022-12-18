use crate::parser_generator::prelude_generator::generate_prelude;
use crate::parser_generator::parser_description::parse_parser_description;
use crate::parser_generator::parser_generator::generate_parser;
use crate::parser_generator::tokenizer_generator::generate_tokenizer;

mod tokenizer_generator;
mod parser_generator;
mod prelude_generator;
mod parser_description;


pub fn generate_parser_from_string(grammar: String) -> String {
    let (_, parser_description) = parse_parser_description(&grammar).unwrap();
    let imports = generate_prelude(&parser_description);
    let tokenizer = generate_tokenizer(&parser_description);
    let parser = generate_parser(&parser_description);

    format!("{imports}\n{tokenizer}\n{parser}")
}
