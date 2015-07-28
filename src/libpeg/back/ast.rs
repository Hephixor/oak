// Copyright 2015 Pierre Talbot (IRCAM)

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use rust;
use back::ast::FunctionKind::*;

pub use std::collections::HashMap;
pub use rust::{ExtCtxt, Span, Spanned, SpannedIdent, Ident};
pub use middle::ast::{Grammar_, Rule_, Expression_, ExprTy, CharacterInterval, CharacterClassExpr};

pub type RTy = rust::P<rust::Ty>;
pub type RExpr = rust::P<rust::Expr>;
pub type RItem = rust::P<rust::Item>;

pub type Grammar = Grammar_<Expression>;
pub type Rule = Rule_<Expression>;

pub type ExpressionNode = Expression_<Expression>;

pub struct Expression
{
  pub span: Span,
  pub node: ExpressionNode,
  pub ty: ExprTy,
  pub kind: FunctionKind
}

impl Expression
{
  pub fn return_type(&self, cx: &ExtCtxt) -> RTy {
    self.kind.to_type(cx)
  }

  pub fn kind(&self) -> FunctionKind {
    self.kind.clone()
  }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FunctionKind
{
  /// Only the recognizer is generated.
  Recognizer,
  /// Only the parser is generated with the type specified.
  Parser(RTy),
  /// The parser is an alias to the recognizer. Both functions are generated.
  ParserAlias,
  /// Recognizer and parser are both generated.
  Both(RTy)
}

impl FunctionKind
{
  pub fn is_recognizer(&self) -> bool {
    match self {
      &Recognizer | &Both(_) | &ParserAlias => true,
      _ => false
    }
  }

  pub fn is_unit(&self) -> bool {
    match self {
      &Recognizer | &ParserAlias => true,
      _ => false
    }
  }

  pub fn to_type(&self, cx: &ExtCtxt) -> RTy {
    match self.clone() {
      Parser(ty) | Both(ty) => ty,
      _ => quote_ty!(cx, ())
    }
  }
}
