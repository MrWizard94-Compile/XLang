//! Pure reference model for the accepted M7 structured nursery design.
//!
//! This is intentionally not a parser, AETH encoder, VM, or product compiler
//! path. It makes the accepted source-order nursery, first-failure cancel, and
//! clean effect boundary executable before seed emission work is asserted.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Effect {
    Total,
    ErrorWhole,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Exit {
    Return(i64),
    ErrorWhole(i64),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Spawn {
    name: &'static str,
    destination: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Weave {
    name: &'static str,
    effect: Effect,
    result: i64,
    raises: Option<i64>,
    spawns: Vec<Spawn>,
    resourceful: bool,
}

#[derive(Debug)]
struct Program {
    weaves: Vec<Weave>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ModelError {
    EmptyNursery,
    TooManySpawns,
    NestedNursery,
    ResourceBoundary,
    ErroringChildInTotalParent,
    MainMayRaise,
    MissingChild,
}

fn validate(program: &Program) -> Result<(), ModelError> {
    for weave in &program.weaves {
        // M19b Policy A: parent may be resourceful; spawn callees must not be.
        if weave.spawns.is_empty() {
            continue;
        }
        if weave.spawns.is_empty() {
            return Err(ModelError::EmptyNursery);
        }
        if weave.spawns.len() > 8 {
            return Err(ModelError::TooManySpawns);
        }
        let mut may_raise = false;
        for spawn in &weave.spawns {
            let child = program
                .weaves
                .iter()
                .find(|candidate| candidate.name == spawn.name)
                .ok_or(ModelError::MissingChild)?;
            if child.resourceful {
                return Err(ModelError::ResourceBoundary);
            }
            if !child.spawns.is_empty() {
                return Err(ModelError::NestedNursery);
            }
            if child.raises.is_some() {
                may_raise = true;
            }
        }
        if may_raise && weave.effect != Effect::ErrorWhole {
            return Err(ModelError::ErroringChildInTotalParent);
        }
        if may_raise && weave.name == "main" {
            return Err(ModelError::MainMayRaise);
        }
    }
    Ok(())
}

fn run_weave(program: &Program, name: &str, slots: &mut [i64]) -> Result<Exit, ModelError> {
    let weave = program
        .weaves
        .iter()
        .find(|candidate| candidate.name == name)
        .ok_or(ModelError::MissingChild)?;
    if weave.spawns.is_empty() {
        if let Some(code) = weave.raises {
            return Ok(Exit::ErrorWhole(code));
        }
        return Ok(Exit::Return(weave.result));
    }
    let mut cancelled = false;
    let mut code = 0;
    for spawn in &weave.spawns {
        if cancelled {
            continue;
        }
        match run_weave(program, spawn.name, slots)? {
            Exit::Return(value) => {
                slots[spawn.destination] = value;
            }
            Exit::ErrorWhole(error) => {
                cancelled = true;
                code = error;
            }
        }
    }
    if cancelled {
        Ok(Exit::ErrorWhole(code))
    } else {
        Ok(Exit::Return(slots.iter().sum()))
    }
}

#[test]
fn total_nursery_joins_destinations_in_source_order() {
    let program = Program {
        weaves: vec![
            Weave {
                name: "left",
                effect: Effect::Total,
                result: 3,
                raises: None,
                spawns: Vec::new(),
                resourceful: false,
            },
            Weave {
                name: "right",
                effect: Effect::Total,
                result: 4,
                raises: None,
                spawns: Vec::new(),
                resourceful: false,
            },
            Weave {
                name: "main",
                effect: Effect::Total,
                result: 0,
                raises: None,
                spawns: vec![
                    Spawn {
                        name: "left",
                        destination: 0,
                    },
                    Spawn {
                        name: "right",
                        destination: 1,
                    },
                ],
                resourceful: false,
            },
        ],
    };
    validate(&program).expect("total nursery must validate");
    let mut slots = [0, 0];
    assert_eq!(run_weave(&program, "main", &mut slots), Ok(Exit::Return(7)));
    assert_eq!(slots, [3, 4]);
}

#[test]
fn first_failure_cancels_remaining_unstarted_spawns() {
    let program = Program {
        weaves: vec![
            Weave {
                name: "ok",
                effect: Effect::Total,
                result: 1,
                raises: None,
                spawns: Vec::new(),
                resourceful: false,
            },
            Weave {
                name: "boom",
                effect: Effect::ErrorWhole,
                result: 0,
                raises: Some(9),
                spawns: Vec::new(),
                resourceful: false,
            },
            Weave {
                name: "later",
                effect: Effect::Total,
                result: 5,
                raises: None,
                spawns: Vec::new(),
                resourceful: false,
            },
            Weave {
                name: "work",
                effect: Effect::ErrorWhole,
                result: 0,
                raises: None,
                spawns: vec![
                    Spawn {
                        name: "ok",
                        destination: 0,
                    },
                    Spawn {
                        name: "boom",
                        destination: 1,
                    },
                    Spawn {
                        name: "later",
                        destination: 2,
                    },
                ],
                resourceful: false,
            },
        ],
    };
    validate(&program).expect("cancel nursery must validate");
    let mut slots = [0, 0, 0];
    assert_eq!(
        run_weave(&program, "work", &mut slots),
        Ok(Exit::ErrorWhole(9))
    );
    assert_eq!(slots[0], 1);
    assert_eq!(
        slots[2], 0,
        "cancelled later spawn must leave destination unchanged"
    );
}

#[test]
fn nursery_rejects_resource_boundary_and_total_parent_errors() {
    // Resourceful spawn callee (self-spawn of resourceful main) is rejected.
    let resourceful_callee = Program {
        weaves: vec![Weave {
            name: "main",
            effect: Effect::Total,
            result: 0,
            raises: None,
            spawns: vec![Spawn {
                name: "main",
                destination: 0,
            }],
            resourceful: true,
        }],
    };
    assert_eq!(
        validate(&resourceful_callee),
        Err(ModelError::ResourceBoundary)
    );

    // Parent resourceful with pure child is admitted (M19b Policy A).
    let parent_resource_pure_child = Program {
        weaves: vec![
            Weave {
                name: "leaf",
                effect: Effect::Total,
                result: 1,
                raises: None,
                spawns: Vec::new(),
                resourceful: false,
            },
            Weave {
                name: "main",
                effect: Effect::Total,
                result: 0,
                raises: None,
                spawns: vec![Spawn {
                    name: "leaf",
                    destination: 0,
                }],
                resourceful: true,
            },
        ],
    };
    assert_eq!(validate(&parent_resource_pure_child), Ok(()));

    let total_parent = Program {
        weaves: vec![
            Weave {
                name: "boom",
                effect: Effect::ErrorWhole,
                result: 0,
                raises: Some(1),
                spawns: Vec::new(),
                resourceful: false,
            },
            Weave {
                name: "main",
                effect: Effect::Total,
                result: 0,
                raises: None,
                spawns: vec![Spawn {
                    name: "boom",
                    destination: 0,
                }],
                resourceful: false,
            },
        ],
    };
    assert_eq!(
        validate(&total_parent),
        Err(ModelError::ErroringChildInTotalParent)
    );
}

#[test]
fn source_order_is_the_only_scheduling_model() {
    let left_first = Program {
        weaves: vec![
            Weave {
                name: "a",
                effect: Effect::Total,
                result: 2,
                raises: None,
                spawns: Vec::new(),
                resourceful: false,
            },
            Weave {
                name: "b",
                effect: Effect::Total,
                result: 5,
                raises: None,
                spawns: Vec::new(),
                resourceful: false,
            },
            Weave {
                name: "main",
                effect: Effect::Total,
                result: 0,
                raises: None,
                spawns: vec![
                    Spawn {
                        name: "a",
                        destination: 0,
                    },
                    Spawn {
                        name: "b",
                        destination: 1,
                    },
                ],
                resourceful: false,
            },
        ],
    };
    let right_first = Program {
        weaves: vec![
            Weave {
                name: "a",
                effect: Effect::Total,
                result: 2,
                raises: None,
                spawns: Vec::new(),
                resourceful: false,
            },
            Weave {
                name: "b",
                effect: Effect::Total,
                result: 5,
                raises: None,
                spawns: Vec::new(),
                resourceful: false,
            },
            Weave {
                name: "main",
                effect: Effect::Total,
                result: 0,
                raises: None,
                spawns: vec![
                    Spawn {
                        name: "b",
                        destination: 0,
                    },
                    Spawn {
                        name: "a",
                        destination: 1,
                    },
                ],
                resourceful: false,
            },
        ],
    };
    let mut left_slots = [0, 0];
    let mut right_slots = [0, 0];
    assert_eq!(
        run_weave(&left_first, "main", &mut left_slots),
        Ok(Exit::Return(7))
    );
    assert_eq!(
        run_weave(&right_first, "main", &mut right_slots),
        Ok(Exit::Return(7))
    );
    assert_eq!(left_slots, [2, 5]);
    assert_eq!(right_slots, [5, 2]);
    assert_ne!(left_slots, right_slots);
}
