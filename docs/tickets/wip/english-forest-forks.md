---
needs: [english-forest-counts-and-forks]
---

# Forks: where a unit's Derivations differ, read off the forest

## Why

The exact counts (`english-forest-counts-and-forks`) say how many Derivations use each node,
item and edge, but not where a unit's analyses part. A forest vertex with several live families
is where analyses re-merge, not where they differ: of 41,555 item vertices with several live
families in the pool's 94,335 units, 34,187 have options with the same previous item and a child
of the same Category and span, which differ only further down (*Crook of Condemnation*'s "Exile
target card from a graveyard." has its one difference, whether "from a graveyard" modifies "card"
or adjoins the Verb Phrase, five levels below its one such vertex). The survey and narrowing
commands, the pool database and the judge need the places where analyses differ, stated in the
grammar's terms, with exact counts, and a way to narrow a forest by them.

## Design

- A **fork** is a question about one word of a unit whose answer differs between the unit's
  Derivations; each option is one answer, with the exact number of Derivations that give it. A
  fork is not a forest vertex. Its context is the Derivations in which its word has an answer:
  the options are disjoint and sum to it. A fork whose context is not every Derivation is
  conditional, and says so.
- A word's **head line** runs from its leaf up through each mother of which it is the head-line
  child (`CompiledGrammar::head_line_child`): in a Linked Conjunct (a coordination schema's
  `link` Category), its linking connective; else the `head` child; else the child that carries
  a coordination schema's coordinator, so a coordination is identified by its coordinator leaf
  and a Linked Coordination by its connective; else the only child besides punctuation; else a
  closed-class marker, by child name (`coordinator`, then `marker`); else the first child that
  bears no dependent function (`not`'s `filler`, not the `modifier` *not*); else the first
  child. The `head` itself, which features and `head_word` follow, is unchanged. Each Production
  names at most one head-line child, so every Constituent is on exactly one word's head line.
  The top of a head line is the word's maximal projection.
- The rule's dependent-function step reads child names against a fixed list
  (`DEPENDENT_FUNCTIONS`, nine names); today it decides one Production (`NegatedConjunct`), and a
  dependent named outside the list would fall through to the first-child step unnoticed. The
  grammar declaring each headless Production's head-line child would remove the list. No
  load-time check refuses a headless Production whose head-line child has its own Category: by
  index it would refuse the 46 engine-built serial tails, whose private Category hosts no
  attachment, and by name also nine wrappers and the Linked Coordinations' binary Productions.
- Three kinds of fork:
  - lexical: which lexical alternative the leaf starting at the word is (its end, Lexeme, Word
    Form and features);
  - dependency: what the word's maximal projection depends on: the function it bears (its name
    in its mother's Production), its governor's Category and its governor's head leaf, or that
    it is a root. The Category is needed: in "that many plus one", "plus one" modifies a
    Determiner Phrase or a Noun Phrase, both headed by "many";
  - frame: the Productions on the word's head line that have one child besides punctuation or
    a Selected Frame (a frame's phrase, a word mention, a quoted ability and the like), in order
    from the leaf up.
- A frame candidate is a fork only where the lexical and dependency candidates do not already
  determine it: one whose partition of its context is a coarsening of the meet of the lexical
  and dependency candidates' partitions there is discarded. Candidates that induce the same
  partition are one fork, named by the first in display order. There is no stacking kind.
- A fork is named by its kind, the surface of the leaf it is about (the shortest leaf starting at
  its word) and that surface's 1-based occurrence among the unit's leaves, so "player's" is the
  leaves "player" and "'s". An option is named by grammatical content, never by an index or a
  count: a dependency option by its function, its governor's Category and the governor leaf's
  surface and occurrence; a lexical option by its Lexeme, Word Form and features; a frame option
  by its Productions. A name is the identity (`ForkWord::names`, `Answer::names`): no offset is
  part of it, so it stays valid when text elsewhere changes. A word's start serves display order
  only, and a lexical answer carries no end: two leaves starting at one word never share a
  lexical reading (the split asserts it). Forks are displayed in order of their word's position, then by kind;
  options in order of their governor's position, then the rest of their content; never by count
  or Ranking Key (`docs/decisions/english.md` §5).
- Forks are computed on a split copy of the forest: each node is split by its head leaf's
  position and its head line's frame Productions, and each item by those of its Production's
  head-line child, so every node and item has one head and every option of every fork is an
  exact set of edges.
- Narrowing: excluding an option disables its edges, sweeps what dies and recounts; picking an
  option of an unconditional fork excludes every other option; picking an option of a
  conditional fork keeps the Derivations that realize it, splitting each vertex above its edges
  on whether a Derivation through it uses one. Forks and counts after narrowing are computed by
  the same code on the narrowed forest, with the unit's names.
- The computation lives in `sylex_syntax` and names no English Category or Production.
- Gap Antecedents and the secondary links of a Constituent (`Constituent::also`: Right Node
  Raising, Complement Clusters, postposed modifiers) are not fork kinds.

## Done when

- Forks are computed for every unit of the pool without enumerating a Reading, and the landing
  reports the pool's forks by kind, its conditional forks, forks per ambiguous unit, how many
  units the lexical and dependency forks alone separate fully, how many need frame forks and
  which the three kinds do not separate, and the CPU time for the pool.
- On every unit of the pool (an opt-in test) and on named real units in the default suite, each
  fork's options are disjoint and sum to its context, the forks computed on the forest equal
  those computed from the unit's enumerated Readings, and for every option of every fork,
  picking it leaves exactly the Derivations enumeration says realize it and excluding it leaves
  the rest; the landing reports how many Derivations remain after picking one option of every
  fork.
- A documented public API in `sylex_syntax` for the survey command and the database's fork
  tables to build on.
- The term *fork* has an entry in `docs/contexts/oracle-english/CONTEXT.md`.
- `cargo test` and `cargo clippy --tests -- -D warnings` pass for each changed crate.

## Not in scope

- Printing forks (`english-forestwalk-survey`), the narrowing command
  (`english-forestwalk-narrowing`) and storing forks (`english-forest-database-forks`).
- Choosing among options (`english.md` §4, §5).

## Pointers

- `docs/decisions/english.md` §1, §4, §5, §8.
- `crates/syntax/src/counts.rs`, `alternatives.rs`, `chart.rs`, `reading.rs` (`locate`,
  `head_word`, `child_names`).
- The research and its review in the gitignored `docs/evidence/english-forest-forks/`.
