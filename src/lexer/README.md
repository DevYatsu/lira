# 🧾 Lira Lexer — Powered by `logos`

This folder contains the **lexical analysis (tokenization)** part of the Lira language.  
It uses the [`logos`](https://github.com/maciejhirsz/logos) crate — a fast, efficient, declarative lexer for Rust.

## 📦 What Is a Lexer?

Lexing is the first stage in a compiler or interpreter. It converts **raw source code** (plain text) into a stream of **tokens** — the building blocks of syntax.

## ⚙️ How logos Works

logos uses **Rust enums** with attributes to define token patterns concisely.

### 🧠 Example: A Token Enum

```rust
#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("let")]
    Let,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Int(i64),

    #[token("=")]
    Equals,

    #[token("+")]
    Plus,
}
```

## What Happens?

- logos turns this enum into a state machine (DFA).
- It reads your source string character-by-character.
- Each regex or token rule matches in priority order (top to bottom).
- For regexes like numbers, you can attach custom logic to parse into a value (i64, f64, etc).

### But what is DFA ?

A **state machine** is a system that:

- Has a set of **states** (e.g., "start", "reading number", "done")
- Reads input **one character at a time**
- **Transitions** between states based on the current character
- **Recognizes** when it has matched a valid token

Think of it like a character-level flowchart for pattern recognition.

### ✅ Deterministic Finite Automaton (DFA)

`logos` builds a **DFA** for your token rules. That means:

- It **knows exactly** what state to move to for each input
- **No backtracking** — just go forward, one char at a time
- It’s built and optimized at compile-time (zero-cost abstraction)

### 🔄 Example: Matching an Identifier

Let’s say you define:

```rust
#[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
Identifier
```

The underlying DFA works like this:

```bash
START
  ├─ 'a'..'z' or 'A'..'Z' or '_' → IDENT
IDENT
  ├─ 'a'..'z' or 'A'..'Z' or '0'..'9' or '_' → stay in IDENT
  └─ other → END (emit Token::Identifier)
```

- It begins in START
- On a valid first char, transitions to IDENT
- Continues reading valid chars
- Once a non-matching char appears → it stops, returns the matched slice

### 🧬 Multiple Tokens = Priority Ordered DFA

logos takes all your token rules and merges their DFAs into a master automaton.

- In the case of logos, the longest matched patterns wins ([better explained here](https://logos.maciej.codes/token-disambiguation.html))
- Regex rules are resolved into state transitions

## 🚀 Why Use logos?

I chose logos for:

### Performance

- Compiles regexes into deterministic automata
- No backtracking = blazing fast

### Simplicity

- No need to write complex match logic or hand-rolled scanners
- Lexer definitions are clear and maintainable

### Flexible Matching

- Supports skipping whitespace
- Regex + callback makes number parsing clean
- Fine control with priorities and token ordering

### Ergonomic with LALRPOP

- logos plays nicely with lalrpop, our parser.
- Produces token streams directly usable by the grammar.

## HUGE Logos Downside

Logos is by far the fastest lexing library availible in rust, **but it comes at a price !!!**

Logos does not support (at least currently, mars 2025) regex look-ahead, look-behind, capturing groups and back-references, which can be very limiting for some use cases... For instance, properly lexing an interpolated string (e.g., "Hello #{name}") isn’t straightforward—you’d need a secondary inner lexer to handle the tokens within the interpolated expression, adding complexity to your workflow.

## 🔀 Alternatives to logos

My approach isn’t the only way to build a parser in Rust. Many libraries take a different tack, often parsing source code directly into an AST in a single step, skipping the explicit lexing phase. For a detailed benchmark of Rust parsing libraries, check out [this comparison](https://github.com/rosetta-rs/parse-rosetta-rs/).

Here are the main alternatives to consider when building a parser in Rust:

## 1. 🧩 regex + manual state machine

The simplest (and most hands-on) option is to write a parser from scratch using basic character matching or regex, paired with a manual state machine.

```rust
for ch in chars {
    match ch {
        '0'..='9' => ...
        '+' => ...
        _ => ...
    }
}
```

**Pros**
✅ Maximum control over every detail.
✅ No external dependencies.

**Cons**
❌ Verbose and boilerplate-heavy.
❌ Error-prone; easy to miss edge cases.
❌ Slow to develop and maintain.

Best for small, simple languages or learning exercises.

## 2. 🛠️ Parser Combinator Libraries (Nom, Winnow, Chumsky, Yap)

Parser combinators let you build parsers by composing small, reusable functions. They’re flexible and expressive, often parsing directly to an AST without a separate lexing step.

**Options**
Nom: Mature, battle-tested, and fast, with a focus on zero-copy parsing.
Winnow: A lightweight fork of Nom, emphasizing simplicity and performance.
Chumsky: Modern, ergonomic, and great for error reporting; ideal for prototyping.
Yap: Minimalist and flexible, though less feature-rich.

**Pros**
✅ Intuitive, functional programming style.
✅ Good error messages (especially Chumsky).
✅ Single-step parsing (no lexing phase).

**Cons**
❌ Can be slower than Logos + LALRPOP (not necessarily problematic).
❌ Steeper learning curve for complex grammars.

Best for medium-complexity languages or when you want rapid development with decent performance.

## 3. 📜 PEG Parser Libraries (Peg, Pest)

Parsing Expression Grammar (PEG) libraries offer a declarative way to define grammars, often with built-in support for direct AST generation. They’re more powerful than Logos’ regex but less rigid than LALRPOP’s LR(1).

**Options**
Peg: Simple, generates Rust code at compile time, good for small projects.
Pest: Feature-rich, with a custom grammar syntax and strong tooling (e.g., error reporting).

**Pros**
✅ Expressive grammar syntax (supports lookahead, unlike Logos).
✅ Single-pass parsing to AST.
✅ Easier to read and maintain than manual code.

**Cons**
❌ Slower than Logos (typically 5-20M chars/sec, which is usually good enough though).
❌ PEG can be ambiguous or memory-hungry for large inputs.

Best for complex grammars where readability trumps raw speed.

## Final Thoughts

There’s no single "best" way to build a language — only tradeoffs.

In Lira, I chose **Logos + LALRPOP** because they offer a clean separation of concerns:

- `logos` handles **tokenization** declaratively and efficiently.
- `lalrpop` builds **structured parsers** with predictable, grammar-based control.

This mirrors how real-world compilers are built: cleanly layered, fast, and easy to reason about.

But depending on your goals — rapid prototyping, performance, ergonomics, or educational clarity — other tools may suit you better:

- Want **maximum control**? Write your own lexer.
- Want **functional elegance**? Try `chumsky` or `nom`.
- Prefer **declarative syntax**? `pest` or `peg` will feel familiar.

The point isn’t just to pick a tool — it’s to **understand what each layer of a language does**, and how to assemble them into something that works.

Lira’s architecture is intentionally layered, hackable, and readable.

The choice of the tool is yours, have a look at the documentation of these different tools and select the one which inspires you the most ! (if it obviously answers your needs)
