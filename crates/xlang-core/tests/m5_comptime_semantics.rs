//! Pure reference model for the accepted M5 deterministic compile-time design.
//!
//! This intentionally models the fixed literal evaluator and directive budget,
//! not Aether parsing, AETH encoding, seed emission, or virtual-machine code.
//! It keeps the staging invariants executable independently of the product
//! compiler implementation.

const MAX_COMPTIME_BINDINGS: usize = 1_024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operation {
    Sum,
    Difference,
    Product,
    Quotient,
    Remainder,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Expression {
    LiteralBinary {
        operation: Operation,
        left: i64,
        right: i64,
    },
    RuntimeRead,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Directive {
    mutable: bool,
    expression: Expression,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ModelError {
    MutableBinding,
    RuntimeRead,
    DivisionByZero,
    WholeOverflow,
    BudgetExceeded { directives: usize },
}

fn evaluate(expression: Expression) -> Result<i64, ModelError> {
    let Expression::LiteralBinary {
        operation,
        left,
        right,
    } = expression
    else {
        return Err(ModelError::RuntimeRead);
    };

    match operation {
        Operation::Sum => left.checked_add(right).ok_or(ModelError::WholeOverflow),
        Operation::Difference => left.checked_sub(right).ok_or(ModelError::WholeOverflow),
        Operation::Product => left.checked_mul(right).ok_or(ModelError::WholeOverflow),
        Operation::Quotient => {
            if right == 0 {
                return Err(ModelError::DivisionByZero);
            }
            left.checked_div(right).ok_or(ModelError::WholeOverflow)
        }
        Operation::Remainder => {
            if right == 0 {
                return Err(ModelError::DivisionByZero);
            }
            left.checked_rem(right).ok_or(ModelError::WholeOverflow)
        }
    }
}

fn materialize(directives: &[Directive]) -> Result<Vec<i64>, ModelError> {
    if directives.len() > MAX_COMPTIME_BINDINGS {
        return Err(ModelError::BudgetExceeded {
            directives: directives.len(),
        });
    }
    directives
        .iter()
        .map(|directive| {
            if directive.mutable {
                Err(ModelError::MutableBinding)
            } else {
                evaluate(directive.expression)
            }
        })
        .collect()
}

fn literal(operation: Operation, left: i64, right: i64) -> Directive {
    Directive {
        mutable: false,
        expression: Expression::LiteralBinary {
            operation,
            left,
            right,
        },
    }
}

#[test]
fn literal_whole_arithmetic_materializes_exact_values() {
    let values = materialize(&[
        literal(Operation::Sum, 12, 4),
        literal(Operation::Difference, 5, 13),
        literal(Operation::Product, 16, 8),
        literal(Operation::Quotient, 144, 12),
        literal(Operation::Remainder, 17, 5),
    ]);
    assert_eq!(values, Ok(vec![16, -8, 128, 12, 2]));
}

#[test]
fn compilation_never_reads_runtime_state_or_mutates_a_comptime_binding() {
    let runtime_read = Directive {
        mutable: false,
        expression: Expression::RuntimeRead,
    };
    assert_eq!(materialize(&[runtime_read]), Err(ModelError::RuntimeRead));

    let mutable = Directive {
        mutable: true,
        expression: Expression::LiteralBinary {
            operation: Operation::Sum,
            left: 1,
            right: 2,
        },
    };
    assert_eq!(materialize(&[mutable]), Err(ModelError::MutableBinding));
}

#[test]
fn arithmetic_failures_are_explicit_and_deterministic() {
    assert_eq!(
        materialize(&[literal(Operation::Quotient, 1, 0)]),
        Err(ModelError::DivisionByZero)
    );
    assert_eq!(
        materialize(&[literal(Operation::Sum, i64::MAX, 1)]),
        Err(ModelError::WholeOverflow)
    );
    assert_eq!(
        materialize(&[literal(Operation::Quotient, i64::MIN, -1)]),
        Err(ModelError::WholeOverflow)
    );
}

#[test]
fn directive_budget_is_fixed_and_not_caller_configurable() {
    let directives = vec![literal(Operation::Sum, 1, 1); MAX_COMPTIME_BINDINGS + 1];
    assert_eq!(
        materialize(&directives),
        Err(ModelError::BudgetExceeded {
            directives: MAX_COMPTIME_BINDINGS + 1,
        })
    );
}
