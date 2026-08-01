//! Pure reference model for the accepted M4 error-effect design.
//!
//! This is intentionally not a parser, AETH encoder, VM, or product compiler
//! path. It makes the accepted two-exit semantics and its safety boundaries
//! executable before source syntax or seed emission work begins.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ValueType {
    Whole,
    Truth,
    Text,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Value {
    Whole(i64),
    Truth(bool),
}

impl Value {
    const fn value_type(self) -> ValueType {
        match self {
            Self::Whole(_) => ValueType::Whole,
            Self::Truth(_) => ValueType::Truth,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Effect {
    Total,
    ErrorWhole,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BoundaryState {
    Clean,
    LiveOwner,
    BorrowLoan,
    AccessLoan,
    Arena,
    Buffer,
    ResourceOutcome,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Terminal {
    Yield(Value),
    Raise(i64),
    Forward { callee: &'static str },
    Handle { callee: &'static str },
    Call { callee: &'static str },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Weave {
    name: &'static str,
    result: ValueType,
    effect: Effect,
    boundary: BoundaryState,
    terminal: Terminal,
}

#[derive(Debug)]
struct Program {
    main: &'static str,
    weaves: Vec<Weave>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Exit {
    Return(Value),
    ErrorWhole(i64),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ModelError {
    MissingWeave {
        caller: &'static str,
        callee: &'static str,
    },
    DuplicateWeave {
        name: &'static str,
    },
    MainCannotDeclareEffect,
    RaiseRequiresEffect {
        weave: &'static str,
    },
    ForwardRequiresDeclaredEffect {
        weave: &'static str,
    },
    ForwardRequiresErrorTarget {
        weave: &'static str,
        callee: &'static str,
    },
    ForwardResultMismatch {
        weave: &'static str,
        callee: &'static str,
    },
    UnhandledEffectCall {
        weave: &'static str,
        callee: &'static str,
    },
    CallResultMismatch {
        weave: &'static str,
        callee: &'static str,
    },
    HandleRequiresTotalWeave {
        weave: &'static str,
    },
    HandleRequiresErrorTarget {
        weave: &'static str,
        callee: &'static str,
    },
    ResultTypeMismatch {
        weave: &'static str,
    },
    EffectBoundaryNotClean {
        weave: &'static str,
        boundary: BoundaryState,
    },
    ErrorSignatureMustReturnWhole {
        weave: &'static str,
    },
    TerminalHandleRequiresWholeResults {
        weave: &'static str,
        callee: &'static str,
    },
    EffectCallDepthExceeded,
}

fn weave(
    name: &'static str,
    result: ValueType,
    effect: Effect,
    boundary: BoundaryState,
    terminal: Terminal,
) -> Weave {
    Weave {
        name,
        result,
        effect,
        boundary,
        terminal,
    }
}

fn lookup<'a>(
    program: &'a Program,
    caller: &'static str,
    callee: &'static str,
) -> Result<&'a Weave, ModelError> {
    program
        .weaves
        .iter()
        .find(|weave| weave.name == callee)
        .ok_or(ModelError::MissingWeave { caller, callee })
}

fn require_clean_boundary(weave: &Weave) -> Result<(), ModelError> {
    if weave.boundary == BoundaryState::Clean {
        Ok(())
    } else {
        Err(ModelError::EffectBoundaryNotClean {
            weave: weave.name,
            boundary: weave.boundary,
        })
    }
}

fn validate(program: &Program) -> Result<(), ModelError> {
    for (index, weave) in program.weaves.iter().enumerate() {
        if program.weaves[index + 1..]
            .iter()
            .any(|other| other.name == weave.name)
        {
            return Err(ModelError::DuplicateWeave { name: weave.name });
        }
        if weave.name == program.main && weave.effect != Effect::Total {
            return Err(ModelError::MainCannotDeclareEffect);
        }
        if weave.effect == Effect::ErrorWhole && weave.result != ValueType::Whole {
            return Err(ModelError::ErrorSignatureMustReturnWhole { weave: weave.name });
        }

        match weave.terminal {
            Terminal::Yield(value) => {
                if value.value_type() != weave.result {
                    return Err(ModelError::ResultTypeMismatch { weave: weave.name });
                }
            }
            Terminal::Raise(_) => {
                if weave.effect != Effect::ErrorWhole {
                    return Err(ModelError::RaiseRequiresEffect { weave: weave.name });
                }
                require_clean_boundary(weave)?;
            }
            Terminal::Forward { callee } => {
                if weave.effect != Effect::ErrorWhole {
                    return Err(ModelError::ForwardRequiresDeclaredEffect { weave: weave.name });
                }
                require_clean_boundary(weave)?;
                let callee = lookup(program, weave.name, callee)?;
                if callee.effect != Effect::ErrorWhole {
                    return Err(ModelError::ForwardRequiresErrorTarget {
                        weave: weave.name,
                        callee: callee.name,
                    });
                }
                if callee.result != weave.result {
                    return Err(ModelError::ForwardResultMismatch {
                        weave: weave.name,
                        callee: callee.name,
                    });
                }
            }
            Terminal::Handle { callee } => {
                if weave.effect != Effect::Total {
                    return Err(ModelError::HandleRequiresTotalWeave { weave: weave.name });
                }
                require_clean_boundary(weave)?;
                let callee = lookup(program, weave.name, callee)?;
                if callee.effect != Effect::ErrorWhole {
                    return Err(ModelError::HandleRequiresErrorTarget {
                        weave: weave.name,
                        callee: callee.name,
                    });
                }
                if weave.result != ValueType::Whole || callee.result != ValueType::Whole {
                    return Err(ModelError::TerminalHandleRequiresWholeResults {
                        weave: weave.name,
                        callee: callee.name,
                    });
                }
            }
            Terminal::Call { callee } => {
                let callee = lookup(program, weave.name, callee)?;
                if callee.effect == Effect::ErrorWhole {
                    return Err(ModelError::UnhandledEffectCall {
                        weave: weave.name,
                        callee: callee.name,
                    });
                }
                if callee.result != weave.result {
                    return Err(ModelError::CallResultMismatch {
                        weave: weave.name,
                        callee: callee.name,
                    });
                }
            }
        }
    }

    lookup(program, program.main, program.main)?;
    Ok(())
}

fn evaluate(program: &Program, weave_name: &'static str, depth: usize) -> Result<Exit, ModelError> {
    const MAX_EFFECT_CALL_DEPTH: usize = 16;
    if depth >= MAX_EFFECT_CALL_DEPTH {
        return Err(ModelError::EffectCallDepthExceeded);
    }
    let weave = lookup(program, weave_name, weave_name)?;
    match weave.terminal {
        Terminal::Yield(value) => Ok(Exit::Return(value)),
        Terminal::Raise(code) => Ok(Exit::ErrorWhole(code)),
        Terminal::Forward { callee } | Terminal::Call { callee } => {
            evaluate(program, callee, depth + 1)
        }
        Terminal::Handle { callee } => match evaluate(program, callee, depth + 1)? {
            Exit::Return(value) => Ok(Exit::Return(value)),
            Exit::ErrorWhole(code) => Ok(Exit::Return(Value::Whole(code))),
        },
    }
}

fn run(program: &Program) -> Result<Exit, ModelError> {
    validate(program)?;
    let exit = evaluate(program, program.main, 0)?;
    if matches!(exit, Exit::ErrorWhole(_)) {
        return Err(ModelError::MainCannotDeclareEffect);
    }
    Ok(exit)
}

fn program(main: Weave, others: &[Weave]) -> Program {
    let mut weaves = others.to_vec();
    weaves.push(main);
    Program {
        main: "main",
        weaves,
    }
}

fn error_leaf(code: i64) -> Weave {
    weave(
        "leaf",
        ValueType::Whole,
        Effect::ErrorWhole,
        BoundaryState::Clean,
        Terminal::Raise(code),
    )
}

#[test]
fn handled_forwarded_and_rejected_routes_are_distinct() {
    let forwarded = weave(
        "forwarded",
        ValueType::Whole,
        Effect::ErrorWhole,
        BoundaryState::Clean,
        Terminal::Forward { callee: "leaf" },
    );
    let main = weave(
        "main",
        ValueType::Whole,
        Effect::Total,
        BoundaryState::Clean,
        Terminal::Handle {
            callee: "forwarded",
        },
    );
    let handled = program(main, &[error_leaf(17), forwarded]);
    assert_eq!(run(&handled), Ok(Exit::Return(Value::Whole(17))));

    let rejected = program(
        weave(
            "main",
            ValueType::Whole,
            Effect::Total,
            BoundaryState::Clean,
            Terminal::Call { callee: "leaf" },
        ),
        &[error_leaf(17)],
    );
    assert_eq!(
        validate(&rejected),
        Err(ModelError::UnhandledEffectCall {
            weave: "main",
            callee: "leaf",
        })
    );
}

#[test]
fn raise_has_no_continuation_to_resume() {
    let leaf = error_leaf(29);
    let main = weave(
        "main",
        ValueType::Whole,
        Effect::Total,
        BoundaryState::Clean,
        Terminal::Handle { callee: "leaf" },
    );
    let model = program(main, &[leaf]);
    validate(&model).expect("the bounded handled error model should validate");
    assert_eq!(evaluate(&model, "leaf", 0), Ok(Exit::ErrorWhole(29)));
    assert_eq!(run(&model), Ok(Exit::Return(Value::Whole(29))));
}

#[test]
fn total_weave_rejects_unhandled_error_call() {
    let model = program(
        weave(
            "main",
            ValueType::Whole,
            Effect::Total,
            BoundaryState::Clean,
            Terminal::Call { callee: "leaf" },
        ),
        &[error_leaf(3)],
    );
    assert_eq!(
        validate(&model),
        Err(ModelError::UnhandledEffectCall {
            weave: "main",
            callee: "leaf",
        })
    );
}

#[test]
fn forward_requires_matching_declared_effect() {
    let model = program(
        weave(
            "main",
            ValueType::Whole,
            Effect::Total,
            BoundaryState::Clean,
            Terminal::Forward { callee: "leaf" },
        ),
        &[error_leaf(7)],
    );
    assert_eq!(
        validate(&model),
        Err(ModelError::ForwardRequiresDeclaredEffect { weave: "main" })
    );

    let wrong_result = weave(
        "forwarded",
        ValueType::Whole,
        Effect::ErrorWhole,
        BoundaryState::Clean,
        Terminal::Forward { callee: "identity" },
    );
    let main = weave(
        "main",
        ValueType::Whole,
        Effect::Total,
        BoundaryState::Clean,
        Terminal::Handle {
            callee: "forwarded",
        },
    );
    let model = program(
        main,
        &[
            error_leaf(7),
            wrong_result,
            weave(
                "identity",
                ValueType::Whole,
                Effect::Total,
                BoundaryState::Clean,
                Terminal::Yield(Value::Whole(7)),
            ),
        ],
    );
    assert_eq!(
        validate(&model),
        Err(ModelError::ForwardRequiresErrorTarget {
            weave: "forwarded",
            callee: "identity",
        })
    );
}

#[test]
fn effect_boundary_rejects_owner_loan_and_resource_state() {
    let forbidden = [
        BoundaryState::LiveOwner,
        BoundaryState::BorrowLoan,
        BoundaryState::AccessLoan,
        BoundaryState::Arena,
        BoundaryState::Buffer,
        BoundaryState::ResourceOutcome,
    ];
    for boundary in forbidden {
        let model = program(
            weave(
                "main",
                ValueType::Whole,
                Effect::Total,
                BoundaryState::Clean,
                Terminal::Handle { callee: "leaf" },
            ),
            &[weave(
                "leaf",
                ValueType::Whole,
                Effect::ErrorWhole,
                boundary,
                Terminal::Raise(5),
            )],
        );
        assert_eq!(
            validate(&model),
            Err(ModelError::EffectBoundaryNotClean {
                weave: "leaf",
                boundary,
            })
        );
    }
}

#[test]
fn terminal_handle_requires_whole_result_on_both_exits() {
    let model = program(
        weave(
            "main",
            ValueType::Whole,
            Effect::Total,
            BoundaryState::Clean,
            Terminal::Handle { callee: "leaf" },
        ),
        &[weave(
            "leaf",
            ValueType::Truth,
            Effect::ErrorWhole,
            BoundaryState::Clean,
            Terminal::Yield(Value::Truth(true)),
        )],
    );
    assert_eq!(
        validate(&model),
        Err(ModelError::ErrorSignatureMustReturnWhole { weave: "leaf" })
    );
}

#[test]
fn unhandled_error_cannot_be_main_exit() {
    let model = Program {
        main: "main",
        weaves: vec![weave(
            "main",
            ValueType::Whole,
            Effect::ErrorWhole,
            BoundaryState::Clean,
            Terminal::Raise(1),
        )],
    };
    assert_eq!(validate(&model), Err(ModelError::MainCannotDeclareEffect));
}

#[test]
fn error_signature_requires_a_whole_result() {
    let model = program(
        weave(
            "main",
            ValueType::Whole,
            Effect::Total,
            BoundaryState::Clean,
            Terminal::Handle { callee: "leaf" },
        ),
        &[weave(
            "leaf",
            ValueType::Text,
            Effect::ErrorWhole,
            BoundaryState::Clean,
            Terminal::Raise(1),
        )],
    );
    assert_eq!(
        validate(&model),
        Err(ModelError::ErrorSignatureMustReturnWhole { weave: "leaf" })
    );
}

#[test]
fn ordinary_total_calls_remain_available() {
    let model = program(
        weave(
            "main",
            ValueType::Truth,
            Effect::Total,
            BoundaryState::Clean,
            Terminal::Call { callee: "identity" },
        ),
        &[weave(
            "identity",
            ValueType::Truth,
            Effect::Total,
            BoundaryState::Clean,
            Terminal::Yield(Value::Truth(true)),
        )],
    );
    assert_eq!(run(&model), Ok(Exit::Return(Value::Truth(true))));
}

#[test]
fn handler_preserves_a_normal_erroring_weave_result() {
    let model = program(
        weave(
            "main",
            ValueType::Whole,
            Effect::Total,
            BoundaryState::Clean,
            Terminal::Handle {
                callee: "may_succeed",
            },
        ),
        &[weave(
            "may_succeed",
            ValueType::Whole,
            Effect::ErrorWhole,
            BoundaryState::Clean,
            Terminal::Yield(Value::Whole(23)),
        )],
    );
    assert_eq!(run(&model), Ok(Exit::Return(Value::Whole(23))));
}
