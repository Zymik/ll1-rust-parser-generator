use std::ops::Add;
use crate::parser_generator::parser_description::ParserDescription;

const DEFUALT_PREFIX: &'static str = "
#![allow(warnings, unused, non_snake_case, non_camel_case_types)]
use regex::Regex;
use lazy_static::lazy_static;
use graph_viz::GraphVizNode;
";

pub fn generate_prelude(parser_description: &ParserDescription) -> String {
    let answer = DEFUALT_PREFIX.to_string();
    answer.add(&parser_description.prelude)
}