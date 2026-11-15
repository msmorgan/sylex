---
needs: [cleanup-step6-spine, syl-misnamed-declarations]
specs: [voice-and-object-phrases]
---

# A Definition reads its call's performer through `$actor`

## Why

An `action` Definition's body reads whoever performs its call through the bare word `actor`,
which reads the innermost Actor and so rebinds under a Handoff inside the body. Three bodies hand
their deed to the permanent's controller and then write `actor` for the permanent, where the word
reads the controller:

- `explore`: "that player puts a +1/+1 counter on the exploring permanent" [CR#701.44a];
- `connive`: "that player puts a +1/+1 counter on the conniving permanent" [CR#701.50a];
- `endure`: "that permanent's controller creates an N/N white Spirit creature token unless they
  put N +1/+1 counters on that permanent" [CR#701.63a].

Each writes `putCounters(…, actor)` inside `instruct(controllerOf(actor), …)`, which puts the
counter on the controller. A body has no word for its call's performer that a clause inside it
leaves alone (spec §2, §4).

## Done when

- `$actor` is an implicit parameter of every `action` Definition, at `NounPhrase`, bound once at
  the call to the call's performer. `cargo xtask dsl check` resolves it in an `action`
  Definition's block and refuses it anywhere else: in a Template, in a declaration of another
  kind and in a card, with a negative test for each.
- Every `action` Definition body that reads its call's performer writes `$actor`: 24 files under
  `plugins/builtin/definitions/keyword_action/` write the bare word today (`grep`; re-count when
  claimed). Each use is read against its rule's text: where the bare word stands under a Handoff
  and means that Handoff's subject (`explore`'s `libraryOf(actor)`, "their library"), it is
  written as the rule prints it, not as `$actor`. The landing lists each use with the reading it
  took.
- `explore`, `connive` and `endure` put their counters on `$actor`, the permanent, each body
  checked against its rule, and an expansion test (`crates/xtask/src/dsl_expand/tests.rs`) shows
  the counter's holder is the call's performer and the one who puts it that permanent's
  controller.
- The doc comment of the `actor` word (`primitives/pronoun/actor.syl`) says it reads the
  performer of the verb phrase it stands in and that a Definition's body reads its call's
  performer through `$actor`. `docs/design/syl.md` §4 and `docs/design/semantics.md` §2.3 say the
  same, where §2.3 now says "the `actor` word still reads the innermost Actor".
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- Rewriting the bodies as clauses and removing `instruct`: `syl-clause-and-imperative`.
- The `ability` Definitions that write the bare word (`aftermath`, `partnerWith`): `$actor` is an
  `action` Definition's.
- The Templates that read a participant through the bare word (`die`, `enter`, `leave`,
  `becomeStatus`, `phaseIn`, `phaseOut`): `syl-misnamed-declarations`, which gives them `subject`.
- A `done` call's performer, its `by`: `syl-passive-participle-and-trigger-events`.

## Pointers

- `docs/specs/voice-and-object-phrases.md` §2, §4
- `plugins/builtin/definitions/keyword_action/{explore,connive,endure,mill,fight}.syl`,
  `plugins/builtin/primitives/pronoun/actor.syl`
- `crates/xtask/src/dsl_check.rs` (`Resolver::parameter`), `crates/xtask/src/dsl_expand.rs`
- `docs/design/syl.md` §4, `docs/design/semantics.md` §2.3
