use std::{char, fs};

use crate::tokens::Token;

fn is_delimiter_token(c: char) -> Option<Token> {
    match c {
        // Seperator(Seperator),
        '{' => Some(Token::OpenCurlyBrace),
        '}' => Some(Token::CloseCurlyBrace),
        '(' => Some(Token::OpenBrace),
        ')' => Some(Token::CloseBrace),
        '[' => Some(Token::OpenSquareBrace),
        ']' => Some(Token::CloseSquareBrace),
        ',' => Some(Token::Comma),
        ';' => Some(Token::SemiColon),
        // NEWLINE
        '\n' => Some(Token::Newline),
        // Operator - OneCharOperator(OneCharOperator),
        '+' => Some(Token::Add),
        '-' => Some(Token::Substract),
        '*' => Some(Token::Multiply),
        '/' => Some(Token::Divide),
        '%' => Some(Token::Modulus),
        '^' => Some(Token::RaiseTo),
        ':' => Some(Token::Colon),
        '~' => Some(Token::Tilde),
        '$' => Some(Token::Dollar),
        '?' => Some(Token::Question),
        // whitespace
        ' ' => Some(Token::WhiteSpace),
        _ => None,
    }
}

fn deduce_partial(prev: Vec<char>) -> Option<Token> {
    let str = prev.iter().collect::<String>();

    let is_builtin = match str.as_str() {
        "BEGIN" => Some(Token::Begin),
        "END" => Some(Token::End),
        "break" => Some(Token::Break),
        "continue" => Some(Token::Continue),
        "delete" => Some(Token::Delete),
        "do" => Some(Token::Do),
        "else" => Some(Token::Else),
        "exit" => Some(Token::Exit),
        "for" => Some(Token::For),
        "function" => Some(Token::Function),
        "if" => Some(Token::If),
        "in" => Some(Token::In),
        "next" => Some(Token::Next),
        "print" => Some(Token::Print),
        "printf" => Some(Token::Printf),
        "return" => Some(Token::Return),
        "while" => Some(Token::While),
        "atan2" => Some(Token::Atan2),
        "cos" => Some(Token::Cos),
        "sin" => Some(Token::Sin),
        "exp" => Some(Token::Exp),
        "log" => Some(Token::Log),
        "sqrt" => Some(Token::Sqrt),
        "int" => Some(Token::Int),
        "rand" => Some(Token::Rand),
        "srand" => Some(Token::Srand),
        "gsub" => Some(Token::Gsub),
        "index" => Some(Token::Index),
        "length" => Some(Token::Length),
        "match" => Some(Token::Match),
        "split" => Some(Token::Split),
        "sprintf" => Some(Token::Sprintf),
        "sub" => Some(Token::Sub),
        "substr" => Some(Token::Substr),
        "tolower" => Some(Token::Tolower),
        "toupper" => Some(Token::Toupper),
        "close" => Some(Token::Close),
        "system" => Some(Token::System),
        "getline" => Some(Token::Getline),
        "||" => Some(Token::Or),
        "&&" => Some(Token::And),
        "!=" => Some(Token::NoMatch),
        "==" => Some(Token::Eq),
        "<=" => Some(Token::Le),
        ">=" => Some(Token::Ge),
        // Single pasangai
        "!" => Some(Token::Invert),
        "<" => Some(Token::LessThan),
        ">" => Some(Token::GreaterThan),
        "|" => Some(Token::Bar),
        "=" => Some(Token::Equal),
        _ => None,
    };

    match is_builtin {
        Some(token) => Some(token),
        None => {
            if let Ok(num_parsed) = str.parse::<i64>() {
                return Some(Token::Number(num_parsed));
            }

            let identifier = str.chars().all(|c| char::is_alphabetic(c) || c == '_');
            if identifier && !prev.is_empty() {
                return Some(Token::Name(prev));
            }
            Some(Token::Unknown)
        }
    }
}

pub struct LookupResult {
    token: Option<Token>,
    prev: Option<Token>,
    partial: Vec<char>,
}
pub fn lookup(current: char, partial: Vec<char>) -> LookupResult {
    let mut result = LookupResult {
        token: None,
        prev: None,
        partial: partial.clone(),
    };
    // handle string literal
    if partial.first() == Some(&'"') {
        if current == '\n' {
            result.token = Some(Token::Error);
            return result;
        }
        // we donot support multiline string - scopedout
        if current == '"' {
            let mut cleaned_partial = partial.clone();
            cleaned_partial.remove(0);
            result.token = Some(Token::Literal(cleaned_partial));
            return result;
        } else {
            result.partial.push(current);
            return result;
        }
    }

    // handle others
    let is_delimiter = is_delimiter_token(current);
    match is_delimiter {
        Some(token) => {
            result.token = Some(token);
            if !partial.is_empty() {
                result.prev = deduce_partial(partial);
            }
        }
        None => {
            result.partial.push(current);
        }
    }
    result
}

pub fn tokenize(input: String) -> Vec<Token> {
    let mut chars = input.chars();
    let mut tokens: Vec<Token> = vec![];

    let mut partial: Vec<char> = vec![];
    while let Some(current) = chars.next() {
        let lookup_result = lookup(current, partial);
        if let Some(token) = lookup_result.token {
            if let Some(prev_token) = lookup_result.prev {
                tokens.push(prev_token);
            }
            partial = vec![];
            // no need to add whitespace token if it is already there
            if tokens.last() == Some(&Token::WhiteSpace) && token == Token::WhiteSpace {
                continue;
            }
            tokens.push(token);
        } else {
            partial = lookup_result.partial;
        }
    }
    tokens
}


#[test]
fn test_sample_1() {
    let sample_1 = fs::read_to_string("./tests/mocks/sample_1.awk").expect("Unable to read sample_1.txt");
    let lexed_1 = tokenize(sample_1.clone());
    insta::assert_compact_json_snapshot!(lexed_1);
}

#[test]
fn test_sample_2() {
    let sample_2 = fs::read_to_string("./tests/mocks/sample_2.awk").expect("Unable to read sample_2.txt");
    let lexed_2 = tokenize(sample_2.clone());
    insta::assert_compact_json_snapshot!(lexed_2);
}

#[test]
fn test_sample_3() {
    let sample_3 = fs::read_to_string("./tests/mocks/sample_3.awk").expect("Unable to read sample_3.txt");
    let lexed_3 = tokenize(sample_3.clone());
    insta::assert_compact_json_snapshot!(lexed_3);
}

#[test]
fn test_sample_4() {
    let sample_4 = fs::read_to_string("./tests/mocks/sample_4.awk").expect("Unable to read sample_4.txt");
    let lexed_4 = tokenize(sample_4.clone());
    insta::assert_compact_json_snapshot!(lexed_4);
}

#[test]
fn test_sample_5() {
    let sample_5 = fs::read_to_string("./tests/mocks/sample_5.awk").expect("Unable to read sample_5.txt");
    let lexed_5 = tokenize(sample_5.clone());
    insta::assert_compact_json_snapshot!(lexed_5);
}
