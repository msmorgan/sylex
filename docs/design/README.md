# Design

A design doc covers what shapes the whole design and an implementer would get wrong without
being told. A decision local to one construct lives in a doc comment where that construct is
used; an accepted design with no code yet lives in [`../specs/`](../specs/README.md). What an
unusual card does is never recorded here; it goes in [`../unusual-cards.md`](../unusual-cards.md).

A doc marked DRAFT is directional: it describes where the design is heading, and that design is
not settled. The others describe the current design.

- [Architecture](architecture.md): the layers of the system and how they connect.
- [Semantics](semantics.md) (DRAFT): reference resolution, Definitions, Templates and the action-clause spine.
- [Syl](syl.md) (DRAFT): the `.syl` data language.
- [Spelling](spelling.md) (DRAFT): the spelling grammar: Constructions, substitution and its two closed rules, its three extensions and the round trip between Readings and resolved terms.
- [Pragmatics](pragmatics.md) (DRAFT): choosing among the meanings a card's text can have: defaults, preferences stated on declarations, and why the stored term decides.
- [Core](core.md): the compiled core that the engine executes.
- [Engine](engine.md): the engine's laws.
- [English](english.md): the English grammar and parser: crate split, Catalog constraint, admission, ranking and the round-trip contract.
- [Lexicon](lexicon.md): how words reach the grammar: the analyzed text, lexical analysis, Categories, Provenance, multiword terms and names.
