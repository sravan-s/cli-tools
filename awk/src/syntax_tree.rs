// This would have been much easier in ocaml
use crate::tokens::Token;

enum BuiltinFuncName {
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
}

enum Program {
    ItemList,
    ActionlessItemList,
}

// NewlineOpt -> Option<Newline>

enum ItemList {
    NewlineOpt(Option<Token::Newline>),
    ActionlessItemListItemTerminator(ActionlessItemList, Item, Terminator),
    ItemListItemTerminator(ItemList, Item, Terminator),
    ItemListActionTerminator(ItemList, Action, Terminator),
}

enum ActionlessItemList {
    ItemListPatternTerminator(ItemList, Pattern, Terminator),
    ActionlessItemListPatternTerminator(ActionlessItemList, Pattern, Terminator),
}

enum Item {
    PatternAction(Pattern, Action),
    FunctionInvocation(Token::FuncName, Token::OpenBrace, Option<ParamList>, Token::CloseBrace, Option<Token::Newline>, Action),
    FunctionDeclaration(Token::Function, Token::WhiteSpace, Token::FuncName, Token::OpenBrace, Option<ParamList>, Token::CloseBrace, Option<Token::Newline>, Action),
}

// Option<ParamList>
// enum ParamListOpt {
//     Empty,
//     ParamList,
// }

enum ParamList {
    Name(Token::Name),
    ParamListCommaName(ParamList, Token::Comma, Token::Name),
}

enum Pattern {
    Begin(Token::Begin),
    End(Token::End),
    Expr(Expr),
    ExprCommaNewlineOptExpr(Expr, Token::Comma, Option<Token::Newline>, Expr),
}

enum Action {
    ActionBraceNewlineOpt(Token::OpenCurlyBrace, Option<Token::Newline>, Token::CloseCurlyBrace),
    ActionBraceTerminatedStatementList(Token::OpenCurlyBrace, Option<Token::Newline>, TerminatedStatementList, Token::CloseCurlyBrace),
    ActionBraceUnterminatedStatementList(Token::OpenCurlyBrace, Option<Token::Newline>, UnterminatedStatementList, Token::CloseCurlyBrace),
}

enum Terminator {
    TerminatorSemicolon(Terminator, Token::Semicolon),
    TerminatorNewline(Terminator, Token::Newline),
    SemiColon(Token::Semicolon),
    Newline(Token::Newline),
}

enum TerminatedStatementList {
    TerminatedStatement,
    TerminatedStatementListTerminatedStatement(TerminatedStatementList, TerminatedStatement),
}

enum UnterminatedStatementList {
    UnterminatedStatement,
    UnterminatedStatementListUnterminatedStatement(UnterminatedStatementList, UnterminatedStatement),
}

enum TerminatedStatement {
    ActionNewlineOpt(Action, Option<Token::Newline>),
    IfStmt(Token::If, Token::OpenBrace, Expr, Token::CloseBrace, Option<Token::Newline>, TerminatedStatement),
    IfElseStmt(Token::If, Token::OpenBrace, Expr, Token::CloseBrace, Option<Token::Newline>, TerminatedStatement, Token::Else, Option<Token::Newline>, TerminatedStatement),
    WhileStmt(Token::While, Token::OpenBrace, Expr, Token::CloseBrace, Option<Token::Newline>, TerminatedStatement),
    ForStmt(Token::For, Token::OpenBrace, Option<SimpleStatement>, Token::Semicolon, Option<Expr>, Token::Semicolon, Option<SimpleStatement>, Token::CloseBrace, Option<Token::Newline>, TerminatedStatement),
    ForInStmt(Token::For, Token::OpenBrace, Token::Name, Token::In, Token::Name, Token::CloseBrace, Option<Token::Newline>, TerminatedStatement),
    SemiColonNewlineOpt(Token::Semicolon, Option<Token::Newline>),
    TerminatableStatementNewlineOpt(TerminatableStatement, Token::Newline, Option<Token::Newline>),
    TerminatableStatementSemiColonNewlineOpt(TerminatableStatement, Token::Semicolon, Option<Token::Newline>),
}

enum UnterminatedStatement {
    TerminatableStatement(TerminatableStatement),
    IfStmt(Token::If, Token::OpenBrace, Expr, Token::CloseBrace, Option<Token::Newline>, UnterminatedStatement),
    IfElseStmt(Token::If, Token::OpenBrace, Expr, Token::CloseBrace, Option<Token::Newline>, TerminatedStatement, Token::Else, Option<Token::Newline>, UnterminatedStatement),
    WhileStmt(Token::While, Token::OpenBrace, Expr, Token::CloseBrace, Option<Token::Newline>, UnterminatedStatement),
    ForStmt(Token::For, Token::OpenBrace, Option<SimpleStatement>, Token::Semicolon, Option<Expr>, Token::Semicolon, Option<SimpleStatement>, Token::CloseBrace, Option<Token::Newline>, UnterminatedStatement),
    ForInStmt(Token::For, Token::OpenBrace, Token::Name, Token::In, Token::Name, Token::CloseBrace, Option<Token::Newline>, UnterminatedStatement),
}

enum TerminatableStatement {
    SimpleStatement(SimpleStatement),
    Break(Token::Break),
    Continue(Token::Continue),
    Next(Token::Next),
    Exit(Token::Exit, Option<Expr>),
    Return(Token::Return, Option<Expr>),
    DoNewlineOptTerminatedStatementWhile(Token::Do, Option<Token::Newline>, TerminatedStatement, Token::While, Token::OpenBrace, Expr, Token::CloseBrace),
}

enum SimpleStatement {
    Delete(Token::Delete, Token::Name, Token::OpenSquareBrace, ExprList, Token::CloseSquareBrace),
    Expr(Expr),
    PrintStatement(PrintStatement),
}

enum PrintStatement {
    SimplePrintStatement(SimplePrintStatement),
    SimplePrintStatementOutputRedirection(SimplePrintStatement, OutputRedirection),
}

enum SimplePrintStatement {
    Print(Token::Print, Option<PrintExprList>),
    PrintOpenBraceMultipleExprListCloseBrace(Token::Print, Token::OpenBrace, MultipleExprList, Token::CloseBrace),
    PrintfPrintExprList(Token::Printf, PrintExprList),
    PrintfOpenBraceMultipleExprListCloseBrace(Token::Printf, Token::OpenBrace, MultipleExprList, Token::CloseBrace),
}

enum OutputRedirection {
    GreaterThan(Token::GreaterThan, Expr),
    Pipe(Token::Pipe, Expr),
}

enum ExprList {
    Expr,
    MultipleExprList,
}

enum MultipleExprList {
    ExprCommaNewlineOptExpr(Expr, Token::Comma, Option<Token::Newline>, Expr),
    MultipleExprListCommaNewlineOptExpr(MultipleExprList, Token::Comma, Option<Token::Newline>, Expr),
}

enum Expr {
    UnaryExpr,
    NonUnaryExpr,
}

enum UnaryExpr {
    PlusExpr(Token::Add, Expr),
    MinusExpr(Token::Substract, Expr),
    UnaryExprRaiseToExpr(UnaryExpr, Token::RaiseTo, Expr),
    UnaryExprMultiplyExpr(UnaryExpr, Token::Multiply, Expr),
    UnaryExprDivideExpr(UnaryExpr, Token::Divide, Expr),
    UnaryExprModulusExpr(UnaryExpr, Token::Modulus, Expr),
    UnaryExprAddExpr(UnaryExpr, Token::Add, Expr),
    UnaryExprSubstractExpr(UnaryExpr, Token::Substract, Expr),
    UnaryExprNonUnaryExpr(UnaryExpr, NonUnaryExpr),
    UnaryExprLessThanExpr(UnaryExpr, Token::LessThan, Expr),
    UnaryExprLessEqualExpr(UnaryExpr, Token::Le, Expr),
    // double check with grammar
    UnaryExprNotEqualExpr(UnaryExpr, Token::Invert, Expr),
    UnaryExprEqualExpr(UnaryExpr, Token::Eq, Expr),
    UnaryExprGreaterThanExpr(UnaryExpr, Token::GreaterThan, Expr),
    UnaryExprGreaterEqualExpr(UnaryExpr, Token::Ge, Expr),
    UnaryExprTildeExpr(UnaryExpr, Token::Tilde, Expr),
    UnaryExprNoMatchExpr(UnaryExpr, Token::NoMatch, Expr),
    UnaryExprInName(UnaryExpr, Token::In, Token::Name),
    UnaryExprAndNewlineOptExpr(UnaryExpr, Token::And, Option<Token::Newline>, Expr),
    UnaryExprOrNewlineOptExpr(UnaryExpr, Token::Or, Option<Token::Newline>, Expr),
    UnaryExprQuestionExprColonExpr(UnaryExpr, Token::Question, Expr, Token::Colon, Expr),
    UnaryInputFunction,
}

enum NonUnaryExpr {
    BracketExprBracket(Token::OpenBrace, Expr, Token::CloseBrace),
    InvertExpr(Token::Invert, Expr),
    NonUnaryExprRaiseToExpr(NonUnaryExpr, Token::RaiseTo, Expr),
    NonUnaryExprMultiplyExpr(NonUnaryExpr, Token::Multiply, Expr),
    NonUnaryExprDivideExpr(NonUnaryExpr, Token::Divide, Expr),
    NonUnaryExprModulusExpr(NonUnaryExpr, Token::Modulus, Expr),
    NonUnaryExprAddExpr(NonUnaryExpr, Token::Add, Expr),
    NonUnaryExprSubstractExpr(NonUnaryExpr, Token::Substract, Expr),
    NonUnaryExprNonUnaryExpr(NonUnaryExpr, NonUnaryExpr),
    NonUnaryExprLessThanExpr(NonUnaryExpr, Token::LessThan, Expr),
    NonUnaryExprLessEqualExpr(NonUnaryExpr, Token::Le, Expr),
    NonUnaryExprNotEqualExpr(NonUnaryExpr, Token::Invert, Expr),
    NonUnaryExprEqualExpr(NonUnaryExpr, Token::Eq, Expr),
    NonUnaryExprGreaterThanExpr(NonUnaryExpr, Token::GreaterThan, Expr),
    NonUnaryExprGreaterEqualExpr(NonUnaryExpr, Token::Ge, Expr),
    NonUnaryExprTildeExpr(NonUnaryExpr, Token::Tilde, Expr),
    NonUnaryExprNoMatchExpr(NonUnaryExpr, Token::NoMatch, Expr),
    NonUnaryExprInName(NonUnaryExpr, Token::In, Token::Name),
    BracketMultipleExprListBracket(Token::OpenBrace, MultipleExprList, Token::CloseBrace, Token::In, Token::Name),
    NonUnaryExprAndNewlineOptExpr(NonUnaryExpr, Token::And, Option<Token::Newline>, Expr),
    NonUnaryExprOrNewlineOptExpr(NonUnaryExpr, Token::Or, Option<Token::Newline>, Expr),
    NonUnaryExprQuestionExprColonExpr(NonUnaryExpr, Token::Question, Expr, Token::Colon, Expr),
    Number(Token::Number),
    String(Token::String),
    LValue,
    // below are scoped out, wont be implemented
    // ERE,
    // LValueIncr(LValue, Token::Incr),
    // LValueDecr(LValue, Token::Decr),
    // IncrLValue(Token::Incr, LValue),
    // DecrLValue(Token::Decr, LValue),
    // LValueRaiseToAssignExpr(LValue, Token::RaiseToAssign, Expr),
    // LValueModulusAssignExpr(LValue, Token::ModulusAssign, Expr),
    // LValueMultiplyAssignExpr(LValue, Token::MultiplyAssign, Expr),
    // LValueDivideAssignExpr(LValue, Token::DivideAssign, Expr),
    // LValueAddAssignExpr(LValue, Token::AddAssign, Expr),
    // LValueSubstractAssignExpr(LValue, Token::SubstractAssign, Expr),
    // LValueAssignExpr(LValue, Token::Assign, Expr),
    FuncNameOpenBraceExprListCloseBrace(Token::FuncName, Token::OpenBrace, Option<ExprList>, Token::CloseBrace),
    BuiltinFuncNameOpenBraceExprListCloseBrace(BuiltinFuncName, Token::OpenBrace, Option<ExprList>, Token::CloseBrace),
    BuiltinFuncName(BuiltinFuncName),
    NonUnaryInputFunction,
}


enum PrintExprList {
    PrintExpr,
    PrintExprListCommaNewlineOptPrintExpr(PrintExprList, Token::Comma, Option<Token::Newline>, PrintExpr),
}

enum PrintExpr {
    UnaryPrintExpr,
    NonUnaryPrintExpr,
}

enum UnaryPrintExpr {
    AddPrintExpr(Token::Add, PrintExpr),
    SubstractPrintExpr(Token::Substract, PrintExpr),
    UnaryPrintExprRaiseToPrintExpr(UnaryPrintExpr, Token::RaiseTo, PrintExpr),
    UnaryPrintExprMultiplyPrintExpr(UnaryPrintExpr, Token::Multiply, PrintExpr),
    UnaryPrintExprDividePrintExpr(UnaryPrintExpr, Token::Divide, PrintExpr),
    UnaryPrintExprModulusPrintExpr(UnaryPrintExpr, Token::Modulus, PrintExpr),
    UnaryPrintExprAddPrintExpr(UnaryPrintExpr, Token::Add, PrintExpr),
    UnaryPrintExprSubstractPrintExpr(UnaryPrintExpr, Token::Substract, PrintExpr),
    UnaryPrintExprNonUnaryPrintExpr(UnaryPrintExpr, NonUnaryPrintExpr),
    UnaryPrintExprTildePrintExpr(UnaryPrintExpr, Token::Tilde, PrintExpr),
    UnaryPrintExprNoMatchPrintExpr(UnaryPrintExpr, Token::NoMatch, PrintExpr),
    UnaryPrintExprInName(UnaryPrintExpr, Token::In, Token::Name),
    UnaryPrintExprAndNewlineOptPrintExpr(UnaryPrintExpr, Token::And, Option<Token::Newline>, PrintExpr),
    UnaryPrintExprOrNewlineOptPrintExpr(UnaryPrintExpr, Token::Or, Option<Token::Newline>, PrintExpr),
    UnaryPrintExprQuestionPrintExprColonPrintExpr(UnaryPrintExpr, Token::Question, PrintExpr, Token::Colon, PrintExpr),
}

enum NonUnaryPrintExpr {
    BracketPrintExprBracket(Token::OpenBrace, PrintExpr, Token::CloseBrace),
    InvertPrintExpr(Token::Invert, PrintExpr),
    NonUnaryPrintExprRaiseToPrintExpr(NonUnaryPrintExpr, Token::RaiseTo, PrintExpr),
    NonUnaryPrintExprMultiplyPrintExpr(NonUnaryPrintExpr, Token::Multiply, PrintExpr),
    NonUnaryPrintExprDividePrintExpr(NonUnaryPrintExpr, Token::Divide, PrintExpr),
    NonUnaryPrintExprModulusPrintExpr(NonUnaryPrintExpr, Token::Modulus, PrintExpr),
    NonUnaryPrintExprAddPrintExpr(NonUnaryPrintExpr, Token::Add, PrintExpr),
    NonUnaryPrintExprSubstractPrintExpr(NonUnaryPrintExpr, Token::Substract, PrintExpr),
    NonUnaryPrintExprNonUnaryPrintExpr(NonUnaryPrintExpr, NonUnaryPrintExpr),
    NonUnaryPrintExprTildePrintExpr(NonUnaryPrintExpr, Token::Tilde, PrintExpr),
    NonUnaryPrintExprNoMatchPrintExpr(NonUnaryPrintExpr, Token::NoMatch, PrintExpr),
    NonUnaryPrintExprInName(NonUnaryPrintExpr, Token::In, Token::Name),
    BracketMultipleExprListBracket(Token::OpenBrace, MultipleExprList, Token::CloseBrace, Token::In, Token::Name),
    NonUnaryPrintExprAndNewlineOptPrintExpr(NonUnaryPrintExpr, Token::And, Option<Token::Newline>, PrintExpr),
    NonUnaryPrintExprOrNewlineOptPrintExpr(NonUnaryPrintExpr, Token::Or, Option<Token::Newline>, PrintExpr),
    NonUnaryPrintExprQuestionPrintExprColonPrintExpr(NonUnaryPrintExpr, Token::Question, PrintExpr, Token::Colon, PrintExpr),
    Number(Token::Number),
    String(Token::String),
    LValue,
    FuncNameOpenBraceExprListCloseBrace(Token::FuncName, Token::OpenBrace, Option<ExprList>, Token::CloseBrace),
    BuiltinFuncNameOpenBraceExprListCloseBrace(BuiltinFuncName, Token::OpenBrace, Option<ExprList>, Token::CloseBrace),
    BuiltinFuncName(BuiltinFuncName),
    NonUnaryInputFunction,
}

enum LValue {
    Name(Token::Name),
    NameOpenSquareBraceExprListCloseSquareBrace(Token::Name, Token::OpenSquareBrace, ExprList, Token::CloseSquareBrace),
    DollarExpr(Token::Dollar, Expr),
}

enum NonUnaryInputFunction {
    SimpleGet(Token::Getline),
    SimpleGetLessThanExpr(Token::Getline, Token::LessThan, Expr),
    NonUnaryExprBarSimpleGet(NonUnaryExpr, Token::Bar, SimpleGet),
}

enum UnaryInputFunction {
    UnaryExprBarSimpleGet(UnaryExpr, Token::Bar, SimpleGet),
}

enum SimpleGet {
    Getline(Token::Getline),
    GetlineLValue(Token::Getline, LValue),
}

enum Ast {
    ItemList,
    ActionlessItemList,
}

enum AstPartial {
    Token,
    Ast,
}

impl Ast {
    fn from(token_list: Vec<Token>) -> Ast {
        // if start with Token::Function -> see of the pattern exists for function declaration
        Ast::ItemList
    }
}

