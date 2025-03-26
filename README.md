# 📚 Lira — A Learnable Language for Learning Compilers

**Lira** is a mini programming language written in Rust, built to teach you **how programming languages are made** — from your code to the computer actually running it.

Lira isn’t just for reading about compilers. You’ll **see, run, and modify** the real code behind a working language.

It’s like a **workshop for language creation** — hands-on, hackable, and beginner-friendly.

## 🧠 Why Build a Language?

Ever wondered:

- How Python or Rust actually _work_?
- How `1 + 2 * 3` turns into a number?
- What "parsing" or "AST" really means?
- How code gets executed, step by step?

Lira helps you understand the core pieces behind every language — by showing you **how to build your own**.

## ✨ What’s Inside

- 🧾 Clean, readable syntax
- 🧠 Beginner-friendly design
- 🦀 Written in idiomatic Rust
- 🧱 Built in **layers**:  
  `code → tokens → tree → result`

You’ll learn how code is broken down, understood, and run — just like a real language does it.

## 🧱 The Layers of Lira

Lira is built in layers. Each layer turns your code into something more structured:

### 1. ✍️ **Lexing** — Breaking source code into words

When you type code like:

```lira
let x = 5 + 3
```

The lexer turns it into a sequence of tokens, like:

```rust
[Let, Identifier("x"), Equals, Int(5), Plus, Int(3)]
```

This is like turning a sentence into words and punctuation. We use the [`logos`](https://github.com/maciejhirsz/logos) crate in Rust to do this.

👉 **Why lexing?** Because computers don’t understand code the way humans do. Lexing breaks the raw text into simple, meaningful pieces — like words and symbols — that are easier for the next stage (the parser) to work with.

## 2. 🧾 Parsing — Understanding structure

Now that we have tokens, we need to figure out the structure of the code.

The **parser** figures out that 5 + 3 is a math expression, and that let x = ... is a variable declaration.

It builds a tree from the tokens, called an Abstract Syntax Tree, we can represent it like this:

```bash
Let
├── Name: "x"
└── Value:
    ├── Add
    │   ├── Number(5)
    │   └── Number(3)
```

We use [`lalrpop`](https://github.com/lalrpop/lalrpop) to describe how this structure works, with a grammar like:

```lalrpop
Expr: RustExpr = {
    <number: Int> => Expr::Int(int),
    "(" <number: Number> ")" => Expr::ParenthesizedInt(int),
}
```

This just serves as an example, this code is not present in the lira source code.

👉 **Why parsing?** Because a list of tokens doesn’t tell us how they relate to each other. Parsing organizes tokens into a tree-like structure, so the interpreter knows what to do — like which operation to run first, or which code belongs inside a function or loop.

## 3. 🌳 AST (Abstract Syntax Tree) — The heart of the language

The parser builds an **AST**, which is just a Rust data structure that represents the program.

Example:

```rust
Statement::Expr(
    Expr::FunctionCall(
        "add",
        Box::new(Expr::Int(5)),
        Box::new(Expr::Int(7))
    )
)
```

This is how your code lives in memory after it’s parsed.

👉 Why? Because it’s easier to evaluate, transform, or compile code when it’s structured like a tree.

## 4. 🧮 Evaluation — Running the code

Once we have the AST, we can **interpret** it — that means walking the tree and doing what it says.

For `5 + 3`, we:

- Evaluate the left (5)
- Evaluate the right (3)
- Apply the + operator
- Return 8

Lira will grow to support:

- Variables
- Functions
- Control flow (if, while, etc.)
- And even compilation (JIT, bytecode…)

👉 Why? This is the step where your language becomes alive.

## 📌 What You Can Learn From This Project

- How real languages tokenize and parse code
- How ASTs are structured and evaluated
- How interpreters are built in layers
- How to design your own syntax
- How languages evolve: from expressions to functions to full-blown programs

## 🛣️ Roadmap

This project is being built in clear, incremental steps — so you can learn how each layer of a language works.

### ✅ Completed

- **Lexical analysis** (tokenization) using [`logos`](https://github.com/maciejhirsz/logos)
- **Parser setup** using [`lalrpop`](https://github.com/lalrpop/lalrpop)
- **Parsing support for:**
  - `let` bindings
  - Function declarations (`fn`)
  - `break`, `return`
  - Type aliases (`type`)
  - Struct declarations
  - Enums (including tuple and struct variants)
  - `use` imports with optional aliasing
  - Control flow: `if`, `while`, `for`, `match`
  - Concurrency with `spawn`
  - Expression statements

### 🏗️ In Progress

- Finalizing parsing rules and AST representation
- Adding unit and integration tests for parser correctness
- Preparing evaluation logic for expressions and statements

### 🔜 Coming Next

- Expression evaluation (interpreter)
- Scoped variable environments (symbol table)
- Function calls and stack frames
- Control flow execution (`if`, `match`, `while`, etc.)
- Struct and enum value construction
- Modules and imports
- Optional: Bytecode compiler and virtual machine
- Optional: JIT backend using Cranelift

## 🧰 Project Structure

```bash
├── src/
│ ├── lexer.rs
| ├── lexer/
| ├──── tokens.rs # Token definitions (logos)
| ├──── str_litteral.rs # Separate lexing of string litterals
| ├── parser/
│ ├──── grammar.lalrpop # Grammar definitions (lalrpop)
│ ├── ast.rs # AST definitions
│ ├── eval.rs # The interpreter
│ └── main.rs # Entry point
├── Cargo.toml
├── README.md
├── examples/
└──── test_files...
```

## ❤️ Contributing

Spotted a bug? Got an idea? Want to write a tutorial? You’re welcome here.

Let’s make learning languages something fun and hands-on.
