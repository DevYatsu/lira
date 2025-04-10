use std::cell::RefCell;
use std::rc::Rc;

use super::interner::Interner;
use super::table::SymbolTable;
use crate::ast::*;
use crate::ir::*;
use crate::parser::string::ParserStringPart;

#[derive(Debug, PartialEq, Clone)]
pub struct LoweringContext<'i> {
    pub symbols: SymbolTable<'i>,
}

impl<'i> LoweringContext<'i> {
    pub fn new(interner: Rc<RefCell<Interner>>) -> Self {
        Self {
            symbols: SymbolTable::new(interner),
        }
    }

    pub fn lower_program(&mut self, program: Program<'i>) -> IrProgram {
        IrProgram {
            statements: program
                .statements
                .into_iter()
                .map(|stmt| self.lower_stmt(stmt))
                .collect(),
        }
    }

    fn lower_stmt(&mut self, stmt: Statement<'i>) -> IrStatement {
        match stmt {
            Statement::FnDecl {
                name,
                _async,
                params,
                return_type,
                body,
            } => IrStatement::FnDecl {
                name: self.symbols.get_or_intern(name),
                _async,
                params: params
                    .into_iter()
                    .map(|(n, t)| {
                        (
                            self.symbols.get_or_intern(n),
                            t.map(|ty| self.lower_type(ty)),
                        )
                    })
                    .collect(),
                return_type: return_type.map(|ty| self.lower_type(ty)),
                body: body.into_iter().map(|s| self.lower_stmt(s)).collect(),
            },
            Statement::Let {
                atomic,
                lazy,
                mutable,
                name,
                ty,
                value,
            } => IrStatement::Let {
                atomic,
                lazy,
                mutable,
                name: self.lower_let_pattern(name),
                ty: ty.map(|t| self.lower_type(t)),
                value: self.lower_expr(value),
            },
            Statement::Expr(expr) => IrStatement::Expr(self.lower_expr(expr)),
            Statement::Return(expr) => IrStatement::Return(self.lower_expr(expr)),
            Statement::Break => IrStatement::Break,
            _ => todo!("Statement lowering not implemented yet: {:?}", stmt),
        }
    }

    fn lower_let_pattern(&mut self, name: LetName<'i>) -> IrLetPattern {
        match name {
            LetName::Ident(n) => IrLetPattern::Ident(self.symbols.get_or_intern(n)),
            LetName::ArrayDestructure(names) => IrLetPattern::Array(
                names
                    .into_iter()
                    .map(|n| self.lower_let_pattern(n))
                    .collect(),
            ),
            LetName::TupleDestructure(names) => IrLetPattern::Tuple(
                names
                    .into_iter()
                    .map(|n| self.lower_let_pattern(n))
                    .collect(),
            ),
            LetName::StructDestructure(name, fields) => IrLetPattern::Struct(
                self.symbols.get_or_intern(name),
                fields
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            self.symbols.get_or_intern(k),
                            v.map(|s| self.symbols.get_or_intern(s)),
                        )
                    })
                    .collect(),
            ),
        }
    }

    fn lower_expr(&mut self, expr: Expr<'i>) -> IrExpr {
        match expr {
            Expr::Literal(lit) => IrExpr::Literal(self.lower_literal(lit)),
            Expr::Ident(name) => IrExpr::Symbol(self.symbols.get_or_intern(name)),
            Expr::Array(items) => {
                IrExpr::Array(items.into_iter().map(|e| self.lower_expr(e)).collect())
            }
            Expr::Tuple(items) => {
                IrExpr::Tuple(items.into_iter().map(|e| self.lower_expr(e)).collect())
            }
            Expr::Call(fun, args) => IrExpr::Call(
                Box::new(self.lower_expr(*fun)),
                args.into_iter().map(|e| self.lower_expr(e)).collect(),
            ),
            Expr::Binary(lhs, op, rhs) => IrExpr::Binary(
                Box::new(self.lower_expr(*lhs)),
                op,
                Box::new(self.lower_expr(*rhs)),
            ),
            Expr::Unary(op, val) => IrExpr::Unary(op, Box::new(self.lower_expr(*val))),
            Expr::Index(a, b) => {
                IrExpr::Index(Box::new(self.lower_expr(*a)), Box::new(self.lower_expr(*b)))
            }
            Expr::FieldAccess(obj, field) => IrExpr::FieldAccess(
                Box::new(self.lower_expr(*obj)),
                self.symbols.get_or_intern(field),
            ),
            _ => todo!("Expression lowering not yet implemented: {:?}", expr),
        }
    }

    fn lower_literal(&mut self, lit: Literal<'i>) -> IrLiteral {
        match lit {
            Literal::Int(i) => IrLiteral::Int(i),
            Literal::Float(f) => IrLiteral::Float(f),
            Literal::String(s) => IrLiteral::String(
                s.into_iter()
                    .map(|x| match x {
                        ParserStringPart::Text(x) => IrStringPart::Text(x),
                        ParserStringPart::Expression(expr) => {
                            IrStringPart::Expression(self.lower_expr(expr))
                        }
                    })
                    .collect(),
            ),
            Literal::Inf => IrLiteral::Inf,
            Literal::Bool(b) => IrLiteral::Bool(b),
        }
    }

    fn lower_type(&mut self, ty: Type<'i>) -> IrType {
        match ty {
            Type::Ident(name) => IrType::Ident(self.symbols.get_or_intern(name)),
            Type::Tuple(types) => {
                IrType::Tuple(types.into_iter().map(|t| self.lower_type(t)).collect())
            }
            Type::Function(args, ret) => IrType::Function(
                args.into_iter().map(|t| self.lower_type(t)).collect(),
                Box::new(self.lower_type(*ret)),
            ),
            Type::AsyncFunction(args, ret) => IrType::AsyncFunction(
                args.into_iter().map(|t| self.lower_type(t)).collect(),
                Box::new(self.lower_type(*ret)),
            ),
            Type::Array(elems) => {
                IrType::Array(elems.into_iter().map(|t| self.lower_type(t)).collect())
            }
            Type::Awaitable(inner) => IrType::Awaitable(Box::new(self.lower_type(*inner))),
        }
    }
}
