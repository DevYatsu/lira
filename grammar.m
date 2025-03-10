<program> ::= <statement>*

<statement> ::= <fn_decl> | <let_decl> | <struct_decl> | <if_stmt> | <block_stmt> | <spawn_stmt> | <expr_stmt> | <match_stmt>

<fn_decl> ::= "fn" <ident> "(" <params>? ")" ("->" <type>)? "{" <statement>* "}"
<let_decl> ::= "let" ("atomic" | "lazy")? <ident> (":" <type>)? "=" <expr>  # Added "lazy"
<struct_decl> ::= "struct" <ident> "{" <field>* "}"
<field> ::= <ident> ":" <type>

<if_stmt> ::= "if" <expr> "{" <statement>* "}" ("elsif" <expr> "{" <statement>* "}")* ("else" "{" <statement>* "}")?
<block_stmt> ::= <expr> "." <ident> ("|" <ident> "|")? "{" <statement>* "}"
<spawn_stmt> ::= "spawn" ("move")? ("|" <ident> "|")? "{" <statement>* "}" ("with" <expr>)?
<match_stmt> ::= "match" <expr> "{" <match_arm>* "}"  # New
<match_arm> ::= <pattern> ("if" <expr>)? "=>" <statement>  # New
<pattern> ::= <number> | <ident> | "_"  # New

<expr> ::= <ident> | <number> | <expr> "+" <expr> | <expr> "." <ident> | <expr> "(" <expr_list>? ")"
        | "move" <expr> | "shared" <expr> | <expr> "." "par_each" ("|" <ident> "|")? "{" <statement>* "}"
        | <expr> "." "par_map" ("|" <ident> "|")? "{" <statement>* "}"
        | <expr> ".." <expr> | "inf"  # Added range syntax for lazy
<expr_list> ::= <expr> ("," <expr>)*

<type> ::= "Int" | <ident>
<ident> ::= [a-zA-Z_][a-zA-Z0-9_]*
<number> ::= <decimal> | <binary> | <octal> | <hex>
<decimal> ::= [0-9]+
<binary> ::= "0b" [0-1]+
<octal> ::= "0o" [0-7]+
<hex> ::= "0x" [0-9a-fA-F]+