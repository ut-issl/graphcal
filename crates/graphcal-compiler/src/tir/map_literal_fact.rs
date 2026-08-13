//! Checked semantic axes for map literals whose surface keys are contextual.

use crate::registry::declared_type::IndexTypeRef;
use crate::syntax::decl_name::ResolvedDeclName;
use crate::syntax::span::Span;

/// Stable identity of one map literal within a checked declaration body.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapLiteralKey {
    owner: ResolvedDeclName,
    span: Span,
}

impl MapLiteralKey {
    #[must_use]
    pub const fn new(owner: ResolvedDeclName, span: Span) -> Self {
        Self { owner, span }
    }
}

/// Axes established by type checking, in source key order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedMapLiteralAxes(Vec<IndexTypeRef>);

impl CheckedMapLiteralAxes {
    #[must_use]
    pub const fn new(axes: Vec<IndexTypeRef>) -> Self {
        Self(axes)
    }

    #[must_use]
    pub fn as_slice(&self) -> &[IndexTypeRef] {
        &self.0
    }
}
