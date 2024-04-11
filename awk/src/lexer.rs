use std::{char};

pub enum Token {
    // Identifier(Identifier),
    Name(Vec<char>),
    Number(i64),
    // Ere(Vec<char>), scopedout~~
    FuncName(Vec<char>),
    // Keyword(Keyword),
    Begin,
    End,
    Break,
    Continue,
    Delete,
    Do,
    Else,
    Exit,
    For,
    Function,
    If,
    In,
    Next,
    Print,
    Printf,
    Return,
    While,
    // Builtin(Builtin),
    Atan2,
    Cos,
    Sin,
    Exp,
    Log,
    Sqrt,
    Int,
    Rand,
    Srand,
    Gsub,
    Index,
    Length,
    Match,
    Split,
    Sprintf,
    Sub,
    Substr,
    Tolower,
    Toupper,
    Close,
    System,
    Getline,
    // Operator - OneCharOperator(OneCharOperator),
    Add, // '+'
    Substract, // '-'
    Multiply, // '*'
    Divide, // '/'
    Modulus, // '%'
    RaiseTo, // ^
    Colon, // ':'
    Tilde, // '~'
    Dollar, // '$'
    Question, // '?'
    // these can be combined with other operator to form a two char operator
    Invert,  // !
    LessThan, // '<'
    GreaterThan, // '>'
    Bar, // '|'
    Equal, // '='
    // Operator - TwoCharOperator(TwoCharOperator),
    Or, // '||'
    And, // '&&'
    NoMatch, // '!='
    Eq, // '=='
    Le, // '<='
    Ge, // '>='
    // Seperator(Seperator),
    OpenCurlyBrace, // '{'
    CloseCurlyBrace, // '}'
    OpenBrace, // '('
    CloseBrace, // ')'
    OpenSquareBrace, // '['
    CloseSquareBrace, // ']'
    Comma, // ','
    SemiColon, // ';'
    Newline, // '\n'

    Unknown,
    WhiteSpace,
}

pub fn tokenizer(input: String) -> Vec<Token> {
    let mut chars = input.chars();
    let mut tokens: Vec<Token> = vec![];

    let mut partial: Vec<char> = vec![];
    while let Some(current) = chars.next() {
        let lookup_result = lookup(current, partial);
        if let Some(token) = lookup_result.token {
            if let Some(prev_token) = lookup_result.prev {
                tokens.push(prev_token);
            }
            tokens.push(token);
            partial = vec![];
        } else {
            partial = lookup_result.partial;
        }
    }
    tokens
}

fn is_delimiter_token(c: char) -> Option<Token> {
    match c {
        // Seperator(Seperator),
        '{' =>  Some(Token::OpenCurlyBrace),
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
        _ => None,
    };

    match is_builtin {
        Some(token) => return Some(token),
        None => {
            if let Ok(num_parsed) = str.parse::<i64>() {
                return Some(Token::Number(num_parsed));
            }

            let identifier = str.chars().all(|c| char::is_alphabetic(c) || c == '_');
            if identifier {
                return Some(Token::Name(prev));
            }

            return Some(Token::Unknown);
        },
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
    let is_delimiter = is_delimiter_token(current);
    match is_delimiter {
        Some(token) => {
            result.token = Some(token);
            result.prev = deduce_partial(partial);
        },
        None => {
            result.partial.push(current);
        }
    }
    result
}
