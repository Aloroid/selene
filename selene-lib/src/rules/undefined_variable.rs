use super::*;
use crate::ast_util::scopes::{Reference, ScopeManager};
use std::{collections::HashSet, convert::Infallible};

use full_moon::ast::Ast;

pub struct UndefinedVariableLint;

lazy_static::lazy_static! {
    static ref VARARG_STRING: String = "...".to_owned();
}

impl Rule for UndefinedVariableLint {
    type Config = ();
    type Error = Infallible;

    fn new(_: Self::Config) -> Result<Self, Self::Error> {
        Ok(UndefinedVariableLint)
    }

    fn pass(&self, ast: &Ast, context: &Context) -> Vec<Diagnostic> {
        // ScopeManager repeats references, and I just don't want to fix it right now
        let mut read = HashSet::new();

        let mut diagnostics = Vec::new();
        let scope_manager = ScopeManager::new(ast);

        for (_, reference) in &scope_manager.references {
            if reference.resolved.is_none()
                && reference.read
                && !read.contains(&reference.identifier)
                && !is_valid_vararg_reference(&scope_manager, reference)
                && !context
                    .standard_library
                    .globals
                    .contains_key(&reference.name)
            {
                read.insert(reference.identifier);

                diagnostics.push(Diagnostic::new(
                    "undefined_variable",
                    format!("`{}` is not defined", reference.name),
                    Label::new(reference.identifier),
                ));
            }
        }

        diagnostics
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn rule_type(&self) -> RuleType {
        RuleType::Correctness
    }
}

// `...` is valid in the opening scope, but everywhere else must be explicitly defined.
fn is_valid_vararg_reference(scope_manager: &ScopeManager, reference: &Reference) -> bool {
    Some(reference.scope_id) == scope_manager.initial_scope && reference.name == *VARARG_STRING
}

#[cfg(test)]
mod tests {
    use super::{super::test_util::*, *};

    #[test]
    fn test_basic() {
        test_lint(
            UndefinedVariableLint::new(()).unwrap(),
            "undefined_variable",
            "basic",
        );
    }

    #[test]
    #[cfg(feature = "roblox")]
    fn test_compound_assignments() {
        test_lint(
            UndefinedVariableLint::new(()).unwrap(),
            "undefined_variable",
            "compound_assignments",
        );
    }

    #[test]
    fn test_hoisting() {
        test_lint(
            UndefinedVariableLint::new(()).unwrap(),
            "undefined_variable",
            "hoisting",
        );
    }

    #[test]
    fn test_self() {
        test_lint(
            UndefinedVariableLint::new(()).unwrap(),
            "undefined_variable",
            "self",
        );
    }

    #[test]
    fn test_shadowing() {
        test_lint(
            UndefinedVariableLint::new(()).unwrap(),
            "undefined_variable",
            "shadowing",
        );
    }

    #[test]
    fn test_varargs() {
        test_lint(
            UndefinedVariableLint::new(()).unwrap(),
            "undefined_variable",
            "varargs",
        );
    }
}
