---
needs: [lean-v3-entry-rider-and-duration-adjuncts, semantics-rust-rewrite]
---
# A shared frame over modal fillers, read cataphorically

Standard constraints apply. Done in `.syl` and Rust (`semantics`, `discourse`) after
`semantics-rust-rewrite`; the Lean names below are the current AST, which that epic ports. This
reverses the "no cataphora in the checker" design of `lean-v3-modal-shared-frame`.

Four Vintage-legal cards print one frame sentence between the mode header and Modes that are not
clauses. The frame's demonstratives refer forward to the chosen Mode:

- *Call Damage Control*: "Choose up to two. Return those cards from your graveyard to your hand."
  • Target artifact card. • … creature … • … enchantment … • … land card.
- *Wild Shape*: "Choose one. Until end of turn, target creature you control has that base power and
  toughness, becomes that creature type, and gains that ability." • 1/3 Turtle with hexproof. • …
- *Outlaws' Merriment*: "… choose one at random. Create a red and white creature token with those
  characteristics." • 3/1 Human Warrior with trample and haste. • …
- *Genku, Future Shaper*: "… choose one that hasn't been chosen this turn. Create a creature token
  with those characteristics." • 2/2 white Fox with vigilance. • …

A heuristic corpus scan finds no others. *Jund 'Em Out* and *Who's That Praetor?* share the shape
but have no Vintage legality.

The current encodings write the frame inside each Mode. For *Call Damage Control* that is a
defect, not just a spelling problem. The printed card is one instruction over the chosen targets,
but the encoding is two returns, performed in order [CR#608.2c]. A "whenever one or more cards
leave your graveyard" trigger would fire once for the card and twice for the encoding.

## Decisions already made

- **One construct, `chooseModesOver`:**
  - the choice: a quantity plus mode-choice modifiers;
  - a frame, which is an Action expression. A reserved word stands at the filler's site in it, the
    way `theActor` stands for the performer, and resolves by expected sort (§4.5): a NounPhrase or
    a characteristics bundle. Templates expand before resolution, so the frame may be written
    with any Template whose argument is that word;
  - the fillers.
- **Fillers are of a closed sort:** a NounPhrase (*Call Damage Control*) or a characteristics
  bundle (the other three). The checker steps the bound argument by its sort, as it does any
  Template argument.
  - A characteristics filler requires a single choice. The construct refuses it under any choice
    of more than one.
  - A NounPhrase filler under a plural choice reaches the frame as one argument holding the chosen
    targets, so it is one instruction.
- **The cataphora is scoped to the construct.** The frame's demonstratives ("those cards", "those
  characteristics", Wild Shape's three "that …") resolve to the bound filler and nothing else. The
  checker gets no general cataphora. The old CR objections do not apply to this:
  - The mode choice fixes what the demonstrative denotes [CR#700.2a,700.2b], before targets are
    chosen [CR#601.2b,601.2c].
  - "Up to two" chosen as fewer gives fewer cards.
  - An unchosen Mode's targets are never chosen [CR#700.2c].
- **Mode-choice modifiers belong to the choice:**
  - "at random" is new (*Outlaws' Merriment*). The ruling says the Mode is chosen at random as the
    ability is put on the stack.
  - "that hasn't been chosen this turn" (*Genku*) is the existing `notChosenBefore`.
- **Wild Shape's frame** is a Template over set-characteristics. Its spelling carries the three
  "that …" mentions, and its fronted Duration uses the adjunct node from
  `lean-v3-entry-rider-and-duration-adjuncts`.
- **Outlaws' Merriment's frame** adds "red and white" to the filler's characteristics. Genku's
  fillers carry their own colours.
- **No lowering work.** semantics_v3 is disconnected from other layers.

## Change

- **AST (Rust `semantics`):** add the construct, the reserved word for the bound filler, and "at
  random".
- **`discourse`:** resolve the bound argument by its sort; refuse a characteristics filler under a
  plural choice.
- **Templates:** add the frame Templates with spellings following
  `plugins-v3-spelling-conventions`.
- **Canon:**
  - Re-encode *Call Damage Control* and *Wild Shape*.
  - Add *Outlaws' Merriment* and *Genku, Future Shaper*.
- **ADR and glossary:**
  - Record the construct in semantics-v3.
  - Amend §0.1 item 2, "There is no cataphora, apart from the clause-internal Contrast Anchor",
    to name this construct as the second scoped exception.
  - Note in the oracle-english glossary that this Antecedent follows its Anaphor (cataphora), via
    `domain-modeling`.

## Acceptance

- Each of the four cards checks clean, and its demonstratives resolve to the bound filler.
- *Call Damage Control* is one return over the chosen targets.
- A characteristics filler under "choose two" is refused (test).
- The `semantics` and `discourse` tests pass.
