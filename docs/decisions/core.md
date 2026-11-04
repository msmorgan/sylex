# Core

The contract of `sylex_core` and `sylex_card`, the core layer of
[Architecture](architecture.md#the-layers)'s layers, which lowering targets and
the [engine](engine.md) executes.

## Laws

Core's executable unit is the region. Every binding in a region is a declared
parameter or a numbered definition, and every reference is a register read;
`Param`, `Provenance`, `DefId`, `Reference::Reg`, `Expr` and `Block` in
`crates/core` document how. Lowering resolves each reference once, and the
engine reads by index.

1. **A region is a closed function**: it reads nothing it did not declare.
2. **A capture is snapshotted once, at creation, and never chased again.**
3. **Linked memory is per object**; a Linked Facet is the one linked read that
   crosses a zone change.
4. **Announcement is declared on the ability**: targets and costs are its
   fields.
5. **Validate at load** what the types cannot rule out; malformed core is a
   lowering bug.

The [Card](#card) section fixes how `sylex_card` packages faces.

## Detail

### 1. A region is a closed function

A `Region { params, body }` reads nothing it did not declare. An ability is a
region. Its child regions are each mode of a `Modal`, each delayed or reflexive
trigger body, each carried body such as a floating replacement, each `Each`,
`Distribute` or `RevealUntil` body, and each per-candidate predicate, such as a
target filter, a `SelectAll`, or an aggregate. A child region sees its
enclosing region's definitions only through declared captures (law 2).

Why: a region is entered later and elsewhere than where it is written: a
delayed trigger fires after its creator is gone, a per-candidate filter runs
once per candidate, and a granted ability runs on another object. It runs
correctly only if everything it reads is in its signature. An environment to
search would bring back the run-time resolution that lowering's single
resolution removes.

### 2. A capture is snapshotted once, at creation, and never chased again

A child region (law 1), such as a delayed or reflexive trigger
[CR#603.7,603.12], declares each register it captures as a parameter whose
capture provenance names the enclosing register. The parameter list is the
capture map; a region keeps no separate capture list that could drift from it.
Apart from linked memory (law 3), nothing else crosses a region boundary.

The snapshot is taken when the child region is created or entered, through the
creating region's own read, which follows an object its effect moved to a
public zone [CR#400.7j] (see `DefId`). So what crosses is the object as that
effect left it: the card in exile, not the permanent that left the
battlefield. When the body runs, the capture is not chased again. A delayed
ability won't affect an object that is no longer in the zone it was expected
to be in, and an object that left and returned is a new object [CR#603.7c].
Information about a captured object still falls back to last known information
[CR#608.2h]. Validation refuses a capture the enclosing region does not define,
and the engine supplies every declared capture at entry, so none is ever
missing.

A granted ability closes over its captures the same way: they are snapshotted
when the resolving spell or ability creates the granting effect [CR#611.2] and
carried through layer derivation [CR#613.1f]. Only those declared captures come
from the granting resolution. The granted ability's intrinsic parameters
(source, controller, event roles) bind to the object that has the ability when
it activates or triggers.

### 3. Linked memory is per object

A `Remember` instruction writes one of the card's memory cells [CR#607.1] and
is that cell's declaration: a card's cells are exactly the ones its
`Remember`s write. A reading ability takes the cell as a parameter with linked
provenance, supplied at entry from the memory of the object both abilities are
printed on. A read that no ability on the card writes is a lowering error. It
is not [CR#607.5a]'s undefined choice, which covers a gained ability whose
linked partner was not copied. A permanent that leaves and returns is a new
object [CR#400.7] and reads nothing. A published value is snapshotted like a
capture, so "cards in the exile zone that were put there" [CR#607.2a] stops
naming a card that has since left exile.

A Linked Facet is the one linked read that crosses a zone change. "If it was
kicked" is a Linked Facet [CR#607.2i,702.33e], and a kicked permanent's ability
reads it after the spell has left the stack. [CR#400.7d] allows that read, so
the engine's payment record moves from the spell onto the permanent it
becomes. A memory cell could not carry it, because the spell and the permanent
are different objects. Core reads a Linked Facet with an expression on the
source (`Condition::PaidCost`, `Count::TimesPaid`, `Predicate::WasPaidWith`),
not through a parameter.

### 4. Announcement is declared on the ability

Targets are fields of the spell, activated and triggered ability and of each
mode, never effect nodes [CR#601.2b,601.2c]. A cost is a field of the kinds
that have one: a spell and an activated ability announce and pay theirs, a mode
carries its own, and a triggered ability has none. The cost is a block that
runs at announcement in the ability's own activation, so a paid product is a
definition the body reads [CR#400.7j].

Why: targets are chosen and costs paid while the spell is cast or the ability
activated [CR#601.2c,601.2h], before anything resolves. On effect nodes the
engine would have to discover them by walking the effect.

### 5. Validate at load

Core is a compiled artifact
([Architecture](architecture.md#design-principles), principle 5), so malformed
core is a lowering bug, never an authoring mistake; authoring mistakes are
caught at the semantic level. A malformed shape that core's types can rule out
is a compile error, not a load-time check. Validation at load covers the rest:
`sylex_core::validate` and its siblings check definition order, dominance,
kinds, region closure, captures and the expression rule, and refuse malformed
core with an error that names the card. What a run-time read can still find is
[engine](engine.md) law 6's.

## Card

A Card Face is the characteristics printed on what the CR calls a face
[CR#709.1,712.1]. A flip card's upside-down half [CR#710.1] and an adventurer
card's Adventure [CR#715.2] are Alternative Characteristics, not faces, and the
types keep them apart. A meld card is double-faced [CR#712.1], but its back is
half of an oversized card face [CR#712.4], so it is not a `CardFace` pair and
`Card::DoubleFaced` never stands in for a meld card.

Why: Alternative Characteristics are not a face. They apply only under one
condition, a flipped permanent on the battlefield [CR#710.1b] or an Adventure
spell on the stack [CR#715.3b], and a flipped permanent still keeps its normal
color and mana cost [CR#710.1c]. Each half of a split card is a face whose
characteristics combine with the other's [CR#709.4b,709.4c].
