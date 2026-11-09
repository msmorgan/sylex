---
needs: [lean-retirement, syl-opaque-templates]
---

# A shared frame over modal fillers, read forward

## Why

Four Vintage-legal cards (*Call Damage Control*, *Wild Shape*, *Outlaws' Merriment*,
*Genku, Future Shaper*) print one frame sentence over Modes that are not clauses, and the frame's
demonstratives refer forward to the chosen Mode. The canon encodings of *Call Damage Control* and
*Wild Shape* copy the frame into every Mode, which for *Call Damage Control* turns one return
into two, performed in order [CR#608.2c], so a "whenever one or more cards leave your graveyard"
trigger fires twice instead of once. A heuristic corpus scan found no other Vintage-legal card of
this shape.

## Design

- One construct, `chooseModesOver`, has a choice (a quantity plus mode-choice modifiers), a frame
  (an Action expression) and fillers.
- A reserved word stands at the filler's site in the frame, the way `theActor` stands for the
  performer, and resolves by expected sort.
- Fillers are of a closed set of sorts: a NounPhrase (*Call Damage Control*) or a characteristics
  bundle (the other three).
- Under a plural choice, NounPhrase fillers reach the frame as one argument holding the chosen
  targets, so the frame is one instruction.
- "At random" (*Outlaws' Merriment*) is a new mode-choice modifier [CR#700.2b]; "that hasn't been
  chosen this turn" (*Genku*) reuses `notChosenBefore`.
- A characteristics filler requires a single choice; the construct refuses it under any choice of
  more than one.

## Open questions

- Whether the frame may be written with any Template whose argument is the reserved word: that
  holds only while Templates stay transparent, expanding before reference resolution
  (`syl-opaque-templates`), so that ticket lands first.

## Done when

- `chooseModesOver`, its reserved filler word and the "at random" mode-choice modifier exist, with
  frame Templates spelled by the `.syl` spelling conventions.
- *Call Damage Control* and *Wild Shape* are re-encoded with it; *Outlaws' Merriment* and *Genku,
  Future Shaper* are added.
- *Call Damage Control* is one return over the chosen targets.
- A characteristics filler under "choose two" is refused, with a test.
- Not checkable until reference resolution exists: each frame demonstrative resolves to the bound
  filler and nothing else; the mode choice fixes what it denotes [CR#700.2a,700.2b] before targets are chosen
  [CR#601.2b,601.2c], and an unchosen Mode's targets are never chosen [CR#700.2c].
- The construct's doc comment records it.
- Declaration order stays the printed order (`syl.md` §5).
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Pointers

`plugins/canon/cards/{Call Damage Control,Wild Shape}.syl`,
`plugins/builtin/primitives/action/actionChooseModes.syl`,
`plugins/builtin/primitives/static/notChosenBefore.syl`, `docs/decisions/semantics.md` §1 item 2.
