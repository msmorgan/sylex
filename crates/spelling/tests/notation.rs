//! Syntax acceptance and refusal tests for `sylex_spelling` (`docs/design/spelling.md` §7).

use sylex_spelling::Affix;
use sylex_spelling::ErrorKind;
use sylex_spelling::Form;
use sylex_spelling::Hole;
use sylex_spelling::HoleKind;
use sylex_spelling::Join;
use sylex_spelling::Mark;
use sylex_spelling::Optional;
use sylex_spelling::OwnName;
use sylex_spelling::Part;
use sylex_spelling::Span;
use sylex_spelling::Spelling;
use sylex_spelling::Tail;
use sylex_spelling::TailPlacement;
use sylex_spelling::Text;
use sylex_spelling::parse;

fn kind_of(input: &str) -> ErrorKind {
    parse(input)
        .expect_err(&format!("expected `{input}` to fail"))
        .kind
}

#[test]
fn parses_gap() {
    assert_eq!(parse("^"), Ok(Spelling::Gap(Span::new(0, 1))));
}

#[test]
fn parses_own_name_tokens_and_refuses_stray_hashes() {
    assert_eq!(
        parse("#"),
        Ok(Spelling::OwnName(OwnName::Full, Span::new(0, 1)))
    );
    assert_eq!(
        parse("##"),
        Ok(Spelling::OwnName(OwnName::Shortened, Span::new(0, 2)))
    );
    for input in [
        "###",
        "# ",
        " #",
        "deals # damage",
        "<#>",
        "## ",
        " ##",
        "$a %Clause: #",
    ] {
        assert_eq!(kind_of(input), ErrorKind::UnexpectedMark('#'), "{input}");
    }
}

#[test]
fn parses_plain_text_and_argument_holes() {
    let parsed = parse("draw $amount cards").unwrap();
    assert_eq!(
        parsed,
        Spelling::Parts {
            parts: vec![
                Part::Text(Text {
                    text: "draw ".to_owned(),
                    span: Span::new(0, 5),
                }),
                Part::Hole(Hole {
                    name: "amount".to_owned(),
                    name_span: Span::new(6, 12),
                    kind: HoleKind::Argument {
                        prefix: None,
                        mark: None,
                        suffix: None,
                    },
                    span: Span::new(5, 12),
                }),
                Part::Text(Text {
                    text: " cards".to_owned(),
                    span: Span::new(12, 18),
                }),
            ],
            tail: None,
            span: Span::new(0, 18),
        }
    );
}

#[test]
fn parses_ties_on_both_sides_of_a_hole() {
    assert_eq!(
        parse("other non~$p"),
        Ok(Spelling::Parts {
            parts: vec![
                Part::Text(Text {
                    text: "other ".to_owned(),
                    span: Span::new(0, 6),
                }),
                Part::Hole(Hole {
                    name: "p".to_owned(),
                    name_span: Span::new(11, 12),
                    kind: HoleKind::Argument {
                        prefix: Some(Affix {
                            text: "non".to_owned(),
                            span: Span::new(6, 9),
                        }),
                        mark: None,
                        suffix: None,
                    },
                    span: Span::new(6, 12),
                }),
            ],
            tail: None,
            span: Span::new(0, 12),
        })
    );

    assert_eq!(
        parse("$quality~cycling $cost"),
        Ok(Spelling::Parts {
            parts: vec![
                Part::Hole(Hole {
                    name: "quality".to_owned(),
                    name_span: Span::new(1, 8),
                    kind: HoleKind::Argument {
                        prefix: None,
                        mark: None,
                        suffix: Some(Affix {
                            text: "cycling".to_owned(),
                            span: Span::new(9, 16),
                        }),
                    },
                    span: Span::new(0, 16),
                }),
                Part::Text(Text {
                    text: " ".to_owned(),
                    span: Span::new(16, 17),
                }),
                Part::Hole(Hole {
                    name: "cost".to_owned(),
                    name_span: Span::new(18, 22),
                    kind: HoleKind::Argument {
                        prefix: None,
                        mark: None,
                        suffix: None,
                    },
                    span: Span::new(17, 22),
                }),
            ],
            tail: None,
            span: Span::new(0, 22),
        })
    );

    // Non-ASCII Unicode letters are valid `letter+` affixes in §7.
    let unicode_affixes = parse("über~$x~land").unwrap();
    assert_eq!(
        unicode_affixes,
        Spelling::Parts {
            parts: vec![Part::Hole(Hole {
                name: "x".to_owned(),
                name_span: Span::new("über~$".len(), "über~$x".len()),
                kind: HoleKind::Argument {
                    prefix: Some(Affix {
                        text: "über".to_owned(),
                        span: Span::new(0, "über".len()),
                    }),
                    mark: None,
                    suffix: Some(Affix {
                        text: "land".to_owned(),
                        span: Span::new("über~$x~".len(), "über~$x~land".len()),
                    }),
                },
                span: Span::new(0, "über~$x~land".len()),
            })],
            tail: None,
            span: Span::new(0, "über~$x~land".len()),
        }
    );
}

#[test]
fn parses_every_form_label_and_sentence_run_mark() {
    let cases = [
        ("Cardinal", Form::Cardinal),
        ("Arabic", Form::Arabic),
        ("Ordinal", Form::Ordinal),
        ("Roman", Form::Roman),
        ("Counting", Form::Counting),
        ("Present", Form::Present),
        ("Preterite", Form::Preterite),
        ("PastParticiple", Form::PastParticiple),
        ("GerundParticiple", Form::GerundParticiple),
    ];
    for (label, expected) in cases {
        assert_eq!(expected.as_str(), label);
        let input = format!("$x@{label} rest");
        let Spelling::Parts { parts, .. } = parse(&input).unwrap() else {
            panic!("expected Parts");
        };
        assert_eq!(
            parts[0],
            Part::Hole(Hole {
                name: "x".to_owned(),
                name_span: Span::new(1, 2),
                kind: HoleKind::Argument {
                    prefix: None,
                    mark: Some(Mark::Form(expected)),
                    suffix: None,
                },
                span: Span::new(0, 3 + label.len()),
            })
        );
    }

    let Spelling::Parts { parts, .. } = parse("$cost: $action*. Activate only $timing.").unwrap()
    else {
        panic!("expected Parts");
    };
    assert_eq!(
        parts[2],
        Part::Hole(Hole {
            name: "action".to_owned(),
            name_span: Span::new(8, 14),
            kind: HoleKind::Argument {
                prefix: None,
                mark: Some(Mark::Sentences),
                suffix: None,
            },
            span: Span::new(7, 15),
        })
    );
}

#[test]
fn parses_every_list_join_including_period_and_line_break() {
    let cases = [
        ("", Join::Juxtaposed),
        (",", Join::Comma),
        (".", Join::Period),
        ("and", Join::And),
        ("or", Join::Or),
        ("and/or", Join::AndOr),
        ("then", Join::Then),
        ("\n", Join::LineBreak),
    ];
    for (raw, expected) in cases {
        assert_eq!(expected.as_str(), raw);
        let input = format!("$xs#<{raw}>");
        assert_eq!(
            parse(&input),
            Ok(Spelling::Parts {
                parts: vec![Part::Hole(Hole {
                    name: "xs".to_owned(),
                    name_span: Span::new(1, 3),
                    kind: HoleKind::List {
                        join: expected,
                        join_span: Span::new(3, 6 + raw.len()),
                    },
                    span: Span::new(0, 6 + raw.len()),
                })],
                tail: None,
                span: Span::new(0, 6 + raw.len()),
            })
        );
    }
}

#[test]
fn parses_optional_parts_and_tails() {
    let parsed = parse("put $subject into $destination< from $from< or $alt>>").unwrap();
    let Spelling::Parts { parts, tail, .. } = parsed else {
        panic!("expected Parts");
    };
    assert!(tail.is_none());
    assert_eq!(parts.len(), 5);
    let Part::Optional(outer) = &parts[4] else {
        panic!("expected outer Optional");
    };
    assert_eq!(outer.parts.len(), 3);
    assert!(matches!(outer.parts[2], Part::Optional(_)));

    // Tail directly after a Hole (no space) and with `inner`.
    let outer_tail = parse("$per%VerbPhrase: for each $p").unwrap();
    assert_eq!(
        outer_tail,
        Spelling::Parts {
            parts: vec![Part::Hole(Hole {
                name: "per".to_owned(),
                name_span: Span::new(1, 4),
                kind: HoleKind::Argument {
                    prefix: None,
                    mark: None,
                    suffix: None,
                },
                span: Span::new(0, 4),
            })],
            tail: Some(Tail {
                placement: TailPlacement::Outer,
                category: "VerbPhrase".to_owned(),
                category_span: Span::new(5, 15),
                parts: vec![
                    Part::Text(Text {
                        text: "for each ".to_owned(),
                        span: Span::new(17, 26),
                    }),
                    Part::Hole(Hole {
                        name: "p".to_owned(),
                        name_span: Span::new(27, 28),
                        kind: HoleKind::Argument {
                            prefix: None,
                            mark: None,
                            suffix: None,
                        },
                        span: Span::new(26, 28),
                    }),
                ],
                span: Span::new(4, 28),
            }),
            span: Span::new(0, 28),
        }
    );

    let inner_tail = parse("$per %inner UnknownCategory: for each $p< in $zone>").unwrap();
    let Spelling::Parts {
        tail: Some(tail), ..
    } = inner_tail
    else {
        panic!("expected Tail");
    };
    assert_eq!(tail.placement, TailPlacement::Inner);
    assert_eq!(tail.category, "UnknownCategory");
    assert_eq!(tail.parts.len(), 3);

    // Optional part holding its own Tail, followed by an outer Tail.
    let both = parse("do $x< with $y %VerbPhrase: alone> %Clause: if $z").unwrap();
    let Spelling::Parts {
        parts,
        tail: Some(top_tail),
        ..
    } = both
    else {
        panic!("expected Parts with Tail");
    };
    assert_eq!(top_tail.category, "Clause");
    let Part::Optional(Optional {
        tail: Some(opt_tail),
        ..
    }) = &parts[2]
    else {
        panic!("expected Optional with Tail");
    };
    assert_eq!(opt_tail.category, "VerbPhrase");

    // Optional part consisting solely of a Tail (`part*` empty, `tail` present).
    let tail_only_opt = parse("create $token<%Clause: , $spec>").unwrap();
    let Spelling::Parts { parts, .. } = tail_only_opt else {
        panic!("expected Parts");
    };
    let Part::Optional(Optional {
        parts: opt_parts,
        tail: Some(opt_tail),
        ..
    }) = &parts[2]
    else {
        panic!("expected Optional with Tail");
    };
    assert_eq!(opt_parts.as_slice(), []);
    assert_eq!(opt_tail.category, "Clause");
}

#[test]
fn refuses_form_outside_section_7_set() {
    assert_eq!(
        kind_of("$n@Plural"),
        ErrorKind::UnknownForm("Plural".to_owned())
    );
    assert_eq!(
        kind_of("$n@ordinal"),
        ErrorKind::UnknownForm("ordinal".to_owned())
    );
    assert_eq!(
        kind_of("$n@Irrealis"),
        ErrorKind::UnknownForm("Irrealis".to_owned())
    );
    assert_eq!(kind_of("$n@"), ErrorKind::MissingForm);
}

#[test]
fn refuses_join_chosen_by_a_parameter_and_unknown_joins() {
    assert_eq!(
        kind_of("$xs#<$name>"),
        ErrorKind::ParameterJoin("$name".to_owned())
    );
    assert_eq!(kind_of("$xs#<;>"), ErrorKind::UnknownJoin(";".to_owned()));
    assert_eq!(
        kind_of("$xs#< and >"),
        ErrorKind::UnknownJoin(" and ".to_owned())
    );
    assert_eq!(kind_of("$xs#<and"), ErrorKind::UnclosedJoin);
    assert_eq!(kind_of("non~$xs#<and>"), ErrorKind::TieOnListHole);
    assert_eq!(kind_of("$xs#<and>~walk"), ErrorKind::TieOnListHole);
    assert_eq!(kind_of("$xs*#<and>"), ErrorKind::MarkedListHole);
    assert_eq!(kind_of("$xs#<and>*"), ErrorKind::MarkedListHole);
}

#[test]
fn refuses_second_or_nested_tail() {
    assert_eq!(
        kind_of("$a %VerbPhrase: for each $b %Clause: if $c"),
        ErrorKind::SecondTail
    );
    assert_eq!(
        kind_of("<$a %VerbPhrase: for each $b %Clause: if $c>"),
        ErrorKind::SecondTail
    );
    assert_eq!(
        kind_of("$a %VerbPhrase: for each $b< $c %Clause: if $d>"),
        ErrorKind::NestedTail
    );
    assert_eq!(
        kind_of("$a %verbPhrase: for each $b"),
        ErrorKind::InvalidTailCategory("verbPhrase".to_owned())
    );
    assert_eq!(
        kind_of("$a %inner: for each $b"),
        ErrorKind::InvalidTailCategory("inner".to_owned())
    );
    assert_eq!(
        kind_of("$a %VerbPhrase:for each $b"),
        ErrorKind::MissingTailColon
    );
    assert_eq!(kind_of("$a %VerbPhrase: "), ErrorKind::EmptyTail);
}

#[test]
fn refuses_gap_that_is_not_the_whole_spelling() {
    for input in [
        "^ ",
        " ^",
        "cast ^",
        "^^",
        "<^>",
        "$a %Clause: ^",
        "$xs#<^>",
    ] {
        assert_eq!(kind_of(input), ErrorKind::GapNotWholeSpelling, "{input}");
    }
}

#[test]
fn refuses_hole_abutting_letter_or_digit_without_tie() {
    for input in [
        "non$p",
        "über$p",
        "2$p",
        "$p2",
        "$action*then",
        "$action*über",
        "$action*2",
        "$n@Ordinal2",
        "$xs#<and>more",
        "$xs#<and>2",
        "2non~$p",
        "$quality~walk2",
        "$quality~walk$other",
    ] {
        assert_eq!(kind_of(input), ErrorKind::AbuttingHole, "{input}");
    }
}

#[test]
fn refuses_malformed_ties_delimiters_and_stray_marks() {
    assert_eq!(kind_of("<>"), ErrorKind::EmptyOptional);
    assert_eq!(kind_of("draw <from"), ErrorKind::UnclosedOptional);
    assert_eq!(kind_of("draw > from"), ErrorKind::UnmatchedCloseAngle);
    assert_eq!(kind_of("$"), ErrorKind::MissingHoleName);
    assert_eq!(kind_of("$2"), ErrorKind::MissingHoleName);
    assert_eq!(kind_of("~$p"), ErrorKind::MissingAffix);
    assert_eq!(kind_of(" ~$p"), ErrorKind::MissingAffix);
    assert_eq!(kind_of("$p~"), ErrorKind::MissingAffix);
    assert_eq!(kind_of("a~b"), ErrorKind::UnexpectedMark('~'));
    assert_eq!(kind_of("a*b"), ErrorKind::UnexpectedMark('*'));
    assert_eq!(kind_of("a@b"), ErrorKind::UnexpectedMark('@'));
    assert_eq!(kind_of("a#b"), ErrorKind::UnexpectedMark('#'));
}
