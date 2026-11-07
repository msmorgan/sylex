# Decisions

A decision is recorded here only if it shapes the whole design and an implementer would get it
wrong without being told. A decision local to one construct lives in a doc comment where that
construct is used; an accepted design with no code yet lives in [`../specs/`](../specs/README.md).
What an unusual card does is never recorded here; it goes in
[`../unusual-cards.md`](../unusual-cards.md).

- [Architecture](architecture.md): the layers of the system and how they connect.
- [Semantics](semantics.md): reference resolution, Definitions, Templates and the action-clause spine.
- [Syl](syl.md): the `.syl` data language.
- [Core](core.md): the compiled core that the engine executes.
- [Engine](engine.md): the engine's laws.
- [English](english.md): the English grammar and parser: crate split, Catalog constraint, admission, ranking and the round-trip contract.
- [Lexicon](lexicon.md): how words reach the grammar: the analyzed text, lexical analysis, Categories, Provenance, multiword terms and names.
