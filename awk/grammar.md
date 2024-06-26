%token NAME NUMBER STRING ERE
%token FUNC_NAME   /* Name followed by '(' without white space. */


/* Keywords */
%token       Begin   End
/*          'BEGIN' 'END'                            */


%token       Break   Continue   Delete   Do   Else
/*          'break' 'continue' 'delete' 'do' 'else'  */


%token       Exit   For   Function   If   In
/*          'exit' 'for' 'function' 'if' 'in'        */


%token       Next   Print   Printf   Return   While
/*          'next' 'print' 'printf' 'return' 'while' */


/* Reserved function names */
%token BUILTIN_FUNC_NAME
            /* One token for the following:
             * atan2 cos sin exp log sqrt int rand srand
             * gsub index length match split sprintf sub
             * substr tolower toupper close system
             */
%token GETLINE
            /* Syntactically different from other built-ins. */


/* Two-character tokens. */
%token ADD_ASSIGN SUB_ASSIGN MUL_ASSIGN DIV_ASSIGN MOD_ASSIGN POW_ASSIGN
/*     '+='       '-='       '*='       '/='       '%='       '^=' */


%token OR   AND  NO_MATCH   EQ   LE   GE   NE   INCR  DECR  APPEND
/*     '||' '&&' '!˜' '==' '<=' '>=' '!=' '++'  '--'  '>>'   */


/* One-character tokens. */
%token '{' '}' '(' ')' '[' ']' ',' ';' NEWLINE
%token '+' '-' '*' '%' '^' '!' '>' '<' '|' '?' ':' '˜' '$' '='


%start program
%%


program          : item_list
                 | item_list item
                 ;


item_list        : /* empty */
                 | item_list item terminator
                 ;


item             : action
                 | pattern action
                 | normal_pattern
                 | Function NAME      '(' param_list_opt ')'
                       newline_opt action
                 | Function FUNC_NAME '(' param_list_opt ')'
                       newline_opt action
                 ;


param_list_opt   : /* empty */
                 | param_list
                 ;


param_list       : NAME
                 | param_list ',' NAME
                 ;


pattern          : normal_pattern
                 | special_pattern
                 ;


normal_pattern   : expr
                 | expr ',' newline_opt expr
                 ;


special_pattern  : Begin
                 | End
                 ;


action           : '{' newline_opt                             '}'
                 | '{' newline_opt terminated_statement_list   '}'
                 | '{' newline_opt unterminated_statement_list '}'
                 ;


terminator       : terminator NEWLINE
                 |            ';'
                 |            NEWLINE
                 ;


terminated_statement_list : terminated_statement
                 | terminated_statement_list terminated_statement
                 ;


unterminated_statement_list : unterminated_statement
                 | terminated_statement_list unterminated_statement
                 ;


terminated_statement : action newline_opt
                 | If '(' expr ')' newline_opt terminated_statement
                 | If '(' expr ')' newline_opt terminated_statement
                       Else newline_opt terminated_statement
                 | While '(' expr ')' newline_opt terminated_statement
                 | For '(' simple_statement_opt ';'
                      expr_opt ';' simple_statement_opt ')' newline_opt
                      terminated_statement
                 | For '(' NAME In NAME ')' newline_opt
                      terminated_statement
                 | ';' newline_opt
                 | terminatable_statement NEWLINE newline_opt
                 | terminatable_statement ';'     newline_opt
    ;


unterminated_statement : terminatable_statement
                 | If '(' expr ')' newline_opt unterminated_statement
                 | If '(' expr ')' newline_opt terminated_statement
                      Else newline_opt unterminated_statement
                 | While '(' expr ')' newline_opt unterminated_statement
                 | For '(' simple_statement_opt ';'
                  expr_opt ';' simple_statement_opt ')' newline_opt
                      unterminated_statement
                 | For '(' NAME In NAME ')' newline_opt
                      unterminated_statement
                 ;


terminatable_statement : simple_statement
                 | Break
                 | Continue
                 | Next
                 | Exit expr_opt
                 | Return expr_opt
                 | Do newline_opt terminated_statement While '(' expr ')'
                 ;


simple_statement_opt : /* empty */
                 | simple_statement
                 ;


simple_statement : Delete NAME '[' expr_list ']'
                 | expr
                 | print_statement
                 ;


print_statement  : simple_print_statement
                 | simple_print_statement output_redirection
                 ;


simple_print_statement : Print  print_expr_list_opt
                 | Print  '(' multiple_expr_list ')'
                 | Printf print_expr_list
                 | Printf '(' multiple_expr_list ')'
                 ;


output_redirection : '>'    expr
                 | APPEND expr
                 | '|'    expr
                 ;


expr_list_opt    : /* empty */
                 | expr_list
                 ;


expr_list        : expr
                 | multiple_expr_list
                 ;


multiple_expr_list : expr ',' newline_opt expr
                 | multiple_expr_list ',' newline_opt expr
                 ;


expr_opt         : /* empty */
                 | expr
                 ;


expr             : unary_expr
                 | non_unary_expr
                 ;


unary_expr       : '+' expr
                 | '-' expr
                 | unary_expr '^'      expr
                 | unary_expr '*'      expr
                 | unary_expr '/'      expr
                 | unary_expr '%'      expr
                 | unary_expr '+'      expr
                 | unary_expr '-'      expr
                 | unary_expr          non_unary_expr
                 | unary_expr '<'      expr
                 | unary_expr LE       expr
                 | unary_expr NE       expr
                 | unary_expr EQ       expr
                 | unary_expr '>'      expr
                 | unary_expr GE       expr
                 | unary_expr '˜'      expr
                 | unary_expr NO_MATCH expr
                 | unary_expr In NAME
                 | unary_expr AND newline_opt expr
                 | unary_expr OR  newline_opt expr
                 | unary_expr '?' expr ':' expr
                 | unary_input_function
                 ;


non_unary_expr   : '(' expr ')'
                 | '!' expr
                 | non_unary_expr '^'      expr
                 | non_unary_expr '*'      expr
                 | non_unary_expr '/'      expr
                 | non_unary_expr '%'      expr
                 | non_unary_expr '+'      expr
                 | non_unary_expr '-'      expr
                 | non_unary_expr          non_unary_expr
                 | non_unary_expr '<'      expr
                 | non_unary_expr LE       expr
                 | non_unary_expr NE       expr
                 | non_unary_expr EQ       expr
                 | non_unary_expr '>'      expr
                 | non_unary_expr GE       expr
                 | non_unary_expr '˜'      expr
                 | non_unary_expr NO_MATCH expr
                 | non_unary_expr In NAME
                 | '(' multiple_expr_list ')' In NAME
                 | non_unary_expr AND newline_opt expr
                 | non_unary_expr OR  newline_opt expr
                 | non_unary_expr '?' expr ':' expr
                 | NUMBER
                 | STRING
                 | lvalue
                 | ERE
                 | lvalue INCR
                 | lvalue DECR
                 | INCR lvalue
                 | DECR lvalue
                 | lvalue POW_ASSIGN expr
                 | lvalue MOD_ASSIGN expr
                 | lvalue MUL_ASSIGN expr
                 | lvalue DIV_ASSIGN expr
                 | lvalue ADD_ASSIGN expr
                 | lvalue SUB_ASSIGN expr
                 | lvalue '=' expr
                 | FUNC_NAME '(' expr_list_opt ')'
                      /* no white space allowed before '(' */
                 | BUILTIN_FUNC_NAME '(' expr_list_opt ')'
                 | BUILTIN_FUNC_NAME
                 | non_unary_input_function
                 ;


print_expr_list_opt : /* empty */
                 | print_expr_list
                 ;


print_expr_list  : print_expr
                 | print_expr_list ',' newline_opt print_expr
                 ;


print_expr       : unary_print_expr
                 | non_unary_print_expr
                 ;


unary_print_expr : '+' print_expr
                 | '-' print_expr
                 | unary_print_expr '^'      print_expr
                 | unary_print_expr '*'      print_expr
                 | unary_print_expr '/'      print_expr
                 | unary_print_expr '%'      print_expr
                 | unary_print_expr '+'      print_expr
                 | unary_print_expr '-'      print_expr
                 | unary_print_expr          non_unary_print_expr
                 | unary_print_expr '˜'      print_expr
                 | unary_print_expr NO_MATCH print_expr
                 | unary_print_expr In NAME
                 | unary_print_expr AND newline_opt print_expr
                 | unary_print_expr OR  newline_opt print_expr
                 | unary_print_expr '?' print_expr ':' print_expr
                 ;


non_unary_print_expr : '(' expr ')'
                 | '!' print_expr
                 | non_unary_print_expr '^'      print_expr
                 | non_unary_print_expr '*'      print_expr
                 | non_unary_print_expr '/'      print_expr
                 | non_unary_print_expr '%'      print_expr
                 | non_unary_print_expr '+'      print_expr
                 | non_unary_print_expr '-'      print_expr
                 | non_unary_print_expr          non_unary_print_expr
                 | non_unary_print_expr '˜'      print_expr
                 | non_unary_print_expr NO_MATCH print_expr
                 | non_unary_print_expr In NAME
                 | '(' multiple_expr_list ')' In NAME
                 | non_unary_print_expr AND newline_opt print_expr
                 | non_unary_print_expr OR  newline_opt print_expr
                 | non_unary_print_expr '?' print_expr ':' print_expr
                 | NUMBER
                 | STRING
                 | lvalue
                 | ERE
                 | lvalue INCR
                 | lvalue DECR
                 | INCR lvalue
                 | DECR lvalue
                 | lvalue POW_ASSIGN print_expr
                 | lvalue MOD_ASSIGN print_expr
                 | lvalue MUL_ASSIGN print_expr
                 | lvalue DIV_ASSIGN print_expr
                 | lvalue ADD_ASSIGN print_expr
                 | lvalue SUB_ASSIGN print_expr
                 | lvalue '=' print_expr
                 | FUNC_NAME '(' expr_list_opt ')'
                     /* no white space allowed before '(' */
                 | BUILTIN_FUNC_NAME '(' expr_list_opt ')'
                 | BUILTIN_FUNC_NAME
                 ;


lvalue           : NAME
                 | NAME '[' expr_list ']'
                 | '$' expr
                 ;


non_unary_input_function : simple_get
                 | simple_get '<' expr
                 | non_unary_expr '|' simple_get
                 ;


unary_input_function : unary_expr '|' simple_get
                 ;


simple_get       : GETLINE
                 | GETLINE lvalue
                 ;


newline_opt      : /* empty */
                 | newline_opt NEWLINE
                 ;

