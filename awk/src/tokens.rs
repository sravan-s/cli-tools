use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Token {
    // Identifier(Identifier),
    Name(Vec<char>),
    Number(i64),
    // Ere(Vec<char>), scopedout~~
    FuncName(Vec<char>), // derived from Name later
    Literal(Vec<char>),
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
    Add,       // '+'
    Substract, // '-'
    Multiply,  // '*'
    Divide,    // '/'
    Modulus,   // '%'
    RaiseTo,   // ^
    Colon,     // ':'
    Tilde,     // '~'
    Dollar,    // '$'
    Question,  // '?'
    // these can be combined with other operator to form a two char operator
    Invert,      // !
    LessThan,    // '<'
    GreaterThan, // '>'
    Bar,         // '|'
    Equal,       // '='
    // Operator - TwoCharOperator(TwoCharOperator),
    Or,      // '||'
    And,     // '&&'
    NoMatch, // '!='
    Eq,      // '=='
    Le,      // '<='
    Ge,      // '>='
    // Seperator(Seperator),
    OpenCurlyBrace,   // '{'
    CloseCurlyBrace,  // '}'
    OpenBrace,        // '('
    CloseBrace,       // ')'
    OpenSquareBrace,  // '['
    CloseSquareBrace, // ']'
    Comma,            // ','
    SemiColon,        // ';'
    Newline,          // '\n'

    Unknown,
    WhiteSpace,
    Error,
}
