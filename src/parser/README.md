# 📜 Lira Parser — Powered by `lalrpop`

This folder contains the **parsing logic** for the Lira programming language, written using [`lalrpop`](https://github.com/lalrpop/lalrpop) — a powerful LR(1) parser generator for Rust.

## ⚙️ How lalrpop Works

lalrpop is a grammar-driven parser generator that uses LR(1) parsing under the hood — the same kind used in real-world compilers like Rust’s own.

You define grammar rules in .lalrpop files like this:

```lalrpop
Expr: i64 = {
    <l:Expr> "+" <r:Expr> => l + r,
    <l:Expr> "*" <r:Expr> => l * r,
    <n:Num> => n,
};

Num: i64 = {
    <n:r"[0-9]+"> => n.parse().unwrap(),
};
```

## What Happens?

- lalrpop compiles this into a deterministic state machine.
- It walks through tokens left-to-right, looking ahead 1 token (LR(1)).
- It uses operator precedence and associativity to resolve ambiguity.
- Rules can return Rust expressions, build structs/enums, or even call helper functions.

## 🚀 Why Use lalrpop?

### ✅ Explicit Grammar Definitions

- You control the entire grammar
- See exactly what syntax is valid in your language

### ✅ Built-in Precedence + Associativity

`#[precedence(level = "5")] #[assoc(side = "left")]`

Allows clean handling of binary operations, grouping, and operator chaining.

### ✅ Rust-native Integration

- Return Rust types (like your AST enums) directly from grammar
- Use any Rust logic inside rule bodies

### ✅ Type-safe Parsing

- Compile-time errors for malformed rules
- Strong typing guarantees from Rust’s compiler

## 🧱 What Our Parser Handles

In Lira, the parser turns tokens into structured ASTs, including:

- let bindings
- Expressions: arithmetic, logical, function calls
- Structs, enums, type aliases
- Blocks, control flow (if, match, while)
- Lambdas, closures, index expressions, field access
- Precedence-aware infix parsing (+, \*, ==, etc.)
- Chaining, pipes, composition (|>, .foo(), etc.)

We define each layer of the language explicitly, so you can trace how 1 + 2 \* 3 becomes a nested tree.

## 🔬 Anatomy of a .lalrpop Grammar

```lalrpop
// Operator precedence
#[precedence(level = "6")] #[assoc(side = "left")]
<lhs:Expr> "+" <rhs:Expr> => Expr::Binary(Box::new(lhs), BinOp::Plus, Box::new(rhs))

// Function calls and postfix
<base:Expr> "(" <args:Args> ")" => Expr::Call(Box::new(base), args)
```

You can mix simple token matches with advanced parsing logic. You can even call helper functions from your Rust codebase.

## 🔀 Alternatives to LALRPOP

[See](../lexer/README.md#-alternatives-to-logos)

## 💥 Downsides of LALRPOP

- No left-recursion allowed (though this can be restructured)
- Debugging LALR(1) grammar errors can be cryptic
- Slightly steeper learning curve than parser combinators
- Grammar files are less “Rusty” than hand-rolled code (but easier to reason about at scale)

## 🧠 Final Thoughts

lalrpop is an amazing tool for building well-structured, layered, production-grade parsers.
It fits perfectly with Lira’s design goals: transparency, structure, and learning by doing.

Yes, there are other tools like chumsky that offer exciting functional programming styles. And yes, PEG parsers like pest are fun to read and easy to write. But for precision, performance, and grammar clarity, lalrpop stands strong.
