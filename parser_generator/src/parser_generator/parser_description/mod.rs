use std::ops::Add;

use nom::IResult;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_till1, take_while1};
use nom::character::complete::{char, multispace0};

use nom::combinator::{opt};
use nom::Err::Error;
use nom::error::{ErrorKind, ParseError};
use nom::multi::{fold_many0, many0, separated_list0, separated_list1};
use nom::sequence::{delimited, preceded, terminated, tuple};


use self::RuleMember::{Command, RuleCall};

#[derive(Debug)]
pub struct ParserDescription {
    pub prelude: String,
    pub skip: Vec<String>,
    pub tokens: Vec<Token>,
    pub not_terminal: Vec<NotTerminal>,
}

#[derive(Debug)]
pub struct Token {
    pub name: String,
    pub regex: String,
}

#[derive(Debug)]
pub struct NotTerminal {
    pub name: String,
    pub args: Vec<Typed>,
    pub returns: Vec<Typed>,
    pub rules: Vec<Rule>,
}

#[derive(Debug, Clone)]
pub struct Rule {
    pub members: Vec<RuleMember>,
}

#[derive(Debug, Clone)]
pub enum RuleMember {
    RuleCall(RuleToken),
    Command(String),
}

#[derive(Debug, Clone)]
pub struct RuleToken {
    pub name: String,
    pub args: String,
}

#[derive(Debug, Clone)]
pub struct Typed {
    pub name: String,
    pub ty: String,
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
    where
        F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(
        multispace0,
        inner,
        multispace0,
    )
}

fn close_tag(str: &str) -> IResult<&str, (Option<&str>, &str)> {
    delimited(
        multispace0,
        tuple((opt(tag(";")), ws(tag("}")))),
        multispace0,
    )(str)
}

pub fn parse_parser_description(str: &str) -> IResult<&str, ParserDescription> {
    let (s, (prelude, skip, tokens, not_terminal)) =
        tuple(
            (
                delimited(
                    tuple((ws(tag("Prelude")), tag("{"))),
                    |s| parse_balanced(s, '{', '}'),
                    tag("}"),
                )
                ,
                delimited(
                    tuple((ws(tag("Skip")), tag("{"))),
                    parse_regexes,
                    close_tag,
                ),
                delimited(
                    tuple((ws(tag("Tokens")), tag("{"))),
                    parse_tokens,
                    close_tag,
                ),
                delimited(
                    tuple((ws(tag("NotTerminals")), tag("{"))),
                    parse_not_terminals,
                    close_tag,
                ),
            ),
        )(str)?;
    Ok((s, ParserDescription { prelude, skip, tokens, not_terminal }))
}


fn parse_tokens(str: &str) -> IResult<&str, Vec<Token>> {
    separated_list0(tag(";"), parse_token)(str)
}

fn parse_regexes(str: &str) -> IResult<&str, Vec<String>> {
    separated_list0(tag(";"), ws(parse_regex))(str)
}

fn parse_token(str: &str) -> IResult<&str, Token> {
    let (s, (name, regex)) = tuple((
        terminated(ws(take_while1(|c: char| c.is_alphabetic())), tag("->")),
        ws(parse_regex)
    )
    )(str)?;

    Ok((s, Token { name: name.to_string(), regex }))
}


fn parse_regex(str: &str) -> IResult<&str, String> {
    let (s1, s2) = delimited(
        tag("\""),
        take_till1(|c| c == '\"'),
        tag("\""),
    )(str)?;

    Ok((s1, format!("\"{s2}\"")))
}

fn parse_typed(str: &str) -> IResult<&str, Typed> {
    let (s, typed) = tuple(
        (
            delimited(multispace0, take_till1(|c: char| c == '#' || c == '}'), char('#')),
            ws(take_till1(|c| c == '}' || c == ';'))
        )
    )(str)
        .map(
            |(a, (s1, s2))|
                (a, Typed { name: s1.trim().to_string(), ty: s2.trim().to_string() })
        )?;
    if typed.name.is_empty() || typed.ty.is_empty() {
        let err: nom::error::Error<&str> = nom::error::Error { input: str, code: ErrorKind::Verify };
        return Err(Error(err));
    }
    Ok((s, typed))
}

fn parse_args(str: &str) -> IResult<&str, Vec<Typed>> {
    delimited(
        char('{'),
        separated_list0(char(';'), parse_typed),
        char('}'),
    )(str)
}

fn parse_args_in_bracket(str: &str) -> IResult<&str, String> {
    let (s, (l, args, r)) = tuple((
        char('('),
        |s| parse_balanced(s, '(', ')'),
        char(')')
    )
    )(str)?;

    Ok((s, format!("{l}{args}{r}")))
}

fn parse_not_terminals(str: &str) -> IResult<&str, Vec<NotTerminal>> {
    separated_list0(tag(";"), parse_not_terminal)(str)
}

fn parse_not_terminal(str: &str) -> IResult<&str, NotTerminal> {
    let (a, (name, args, returns, rules)) =
        tuple((
                  ws(take_while1(|c: char| c.is_alphabetic())),
                  ws(parse_args),
                  ws(parse_args),
                  delimited(
                      tag("->"),
                      separated_list1(tag("|"), parse_rule),
                      multispace0,
                  ),
              ),
        )(str)?;
    let not_terminal = NotTerminal { name: name.to_string(), args, returns, rules };
    Ok((a, not_terminal))
}


fn parse_rule(str: &str) -> IResult<&str, Rule> {
    let (s1, members) = delimited(
        multispace0,
        many0(parse_rule_member),
        multispace0,
    )(str)?;
    Ok((s1, Rule { members }))
}

fn parse_rule_member(str: &str) -> IResult<&str, RuleMember> {
    delimited(
        multispace0,
        alt((parse_rule_token, parse_command)),
        multispace0,
    )(str)
}

fn parse_rule_token(str: &str) -> IResult<&str, RuleMember> {
    let (a, (s1, s2)) =
        tuple(
            (take_while1(|c: char| c.is_alphabetic()),
             preceded(multispace0, opt(parse_args_in_bracket))
            )
        )(str)?;
    Ok((a, RuleCall(RuleToken { name: s1.to_string(), args: s2.unwrap_or("()".to_string()) })))
}


fn parse_command(str: &str) -> IResult<&str, RuleMember> {
    let (a, b) = delimited(
        tag("{"),
        |s| parse_balanced(s, '{', '}'),
        tag("}"),
    )(str)?;

    Ok((a, Command(b)))
}

fn parse_inner_balanced<'a>(str: &'a str, left_bracket: char, right_bracket: char) -> IResult<&'a str, String> {
    let (s, (s1, s2, s3)) = tuple(
        (char(left_bracket),
         (|s: &'a str| parse_balanced(s, left_bracket, right_bracket)),
         char(right_bracket))
    )
        (str)?;

    Ok((s, format!("{s1}{s2}{s3}")))
}

fn parse_balanced<'a>(str: &'a str, left_bracket: char, right_bracket: char) -> IResult<&'a str, String> {
    let (s, str) = fold_many0(
        alt((
                |x: &'a str|
                    (take_till1(|c: char| c == left_bracket || c == right_bracket)(x)
                        .map(|(a, b)| (a, b.to_string()))),
                |s| parse_inner_balanced(s, left_bracket, right_bracket)
            ),
        ),
        || String::new(),
        |acc: String, item: String| acc.add(&item),
    )(str)?;
    Ok((s, str))
}

