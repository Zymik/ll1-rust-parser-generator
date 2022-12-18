use crate::parser_generator::parser_description::{ParserDescription, Token};

pub const EOF_TOKEN: &'static str = "Eof";

const PARSE_ERROR: &'static str =
    "#[derive(Debug)]
pub struct ParseError {
    pub position: usize,
    pub message: String,
}";

const TOKENIZER_STRUCT: &'static str =
    "pub struct Tokenizer {
     input: String,
     pointer: usize,
}";

const TOKENIZER_BASE_IMPL: &'static str =
    "
impl Tokenizer {
        fn new(input: String) -> Tokenizer {
            Tokenizer{
                input,
                pointer: 0,
            }
        }

        pub fn tokenize(input: String) -> Result<Vec<Token>, ParseError> {
            Tokenizer::new(input).run_tokenizing()
        }

        fn skip(&mut self) -> bool {
            for regex in SKIP_REGEX.iter() {
                if let Some(m) = regex.find_at(&self.input, self.pointer) {
                     let s = m.start();
                     if s == self.pointer {
                         self.pointer = m.end();
                         return true;
                     }
                }
            }
            false
        }

        fn run_tokenizing(&mut self) -> Result<Vec<Token>, ParseError> {
            let mut tokens: Vec<Token> = Vec::new();
            while self.pointer < self.input.len() {
                if self.skip() {
                    continue;
                }
                if let Some(token) = self.match_token() {
                   tokens.push(token);
                   continue;
                }
                return Err(
                    ParseError {
                        position: self.pointer,
                        message: \"Expected token\".to_string(),
                    }
                )
            }
            tokens.push(Token::Eof);
            Ok(tokens)
        }
}
";

const TOKENS_ENUM: &'static str = "pub enum Token {";

pub fn generate_tokenizer(parser_description: &ParserDescription) -> String {
    let mut answer = String::new();
    let skip = &parser_description.skip;
    let tokens = &parser_description.tokens;


    answer.push_str(&get_tokens_enum(tokens));

    answer.push('\n');
    answer.push_str(&get_tokens_regex(tokens));

    answer.push('\n');

    answer.push_str(&get_skip_regex(skip));
    answer.push('\n');

    answer.push_str(TOKENIZER_STRUCT);
    answer.push('\n');

    answer.push_str(PARSE_ERROR);
    answer.push('\n');

    answer.push_str(&get_matching_regex_in_tokenizer(tokens));
    answer.push('\n');

    answer.push_str(TOKENIZER_BASE_IMPL);
    answer.push('\n');

    answer
}

const TOKENS_ENUM_DERIVES: &'static str = "#[derive(Debug)]";

fn get_tokens_enum(tokens: &Vec<Token>) -> String{
    let mut answer = String::new();
    answer.push_str(TOKENS_ENUM_DERIVES);
    answer.push_str("\n");
    answer.push_str(TOKENS_ENUM);
    answer.push('\n');
    for token in tokens {
        let name = &token.name;
        let enum_value = format!("{name}(String),\n");
        answer.push_str(&enum_value);
    }

    answer.push_str(&format!("{EOF_TOKEN},\n"));
    answer.push_str("}\n");
    answer
}

fn get_tokens_regex(tokens: &Vec<Token>) -> String {
    let mut answer = String::new();
    answer.push_str("lazy_static!(\n");

    for token in tokens {
        let name = format!("{}_regex", token.name);
        let regex = format!("Regex::new({})", token.regex);
        let assignment = format!("static ref {name}: Regex = {regex}.unwrap(); \n");

        answer.push_str(&assignment);
    }

    answer.push_str(");\n");
    answer
}

fn get_skip_regex(skip_regex: &Vec<String>) -> String {
    let mut answer = String::new();
    answer.push_str("lazy_static!(\n");
    answer.push_str("static ref SKIP_REGEX: Vec<Regex> = vec!(");
    for regex in skip_regex {
        let regex_name = format!("Regex::new({regex}).unwrap()");
        answer.push_str(&regex_name);
        answer.push(',');
    }

    answer.push_str(");\n");
    answer.push_str(");\n");
    answer
}

fn get_matching_regex_in_tokenizer(tokens: &Vec<Token>) -> String {
    let mut answer = String::new();

    answer.push_str("impl Tokenizer {\n");
    answer.push_str("fn match_token(&mut self) -> Option<Token> {\n");

    for token in tokens {

        let regex = format!("{}_regex", token.name);
        let find_validation = format!("if let Some(m) = {regex}.find_at(&self.input, self.pointer) {{");
        let start_validation = format!("if m.start() == self.pointer {{");
        let set_pointer = "self.pointer = m.end();";
        let return_token = format!("return Some(Token::{}(self.input[m.start() .. m.end()].to_string()))", token.name);

        let formatted = format!(
            "{find_validation}\n{start_validation}\n{set_pointer}\n{return_token}}}}}\n"
        );
        answer.push_str(&formatted);
    }
    answer.push_str("None");

    answer.push('\n');

    answer.push_str("}\n");
    answer.push_str("}\n");

    answer
}
