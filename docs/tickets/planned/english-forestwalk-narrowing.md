---
needs: [english-forest-counts-and-forks, english-forest-forks, english-forestwalk-survey]
---

# `forestwalk` narrowing: slice a forest by picked and excluded fork options

## Why

The survey (`english-forestwalk-survey`) shows a unit's forks, but working out which Reading is
the right one, or writing the judgment that pins it, still means scanning enumerated trees.
Picking and excluding fork options on the forest itself narrows a unit of any size to the
Readings wanted, and from there to the assertions a judgment needs.

## Design

- A second stateless call takes fork selectors, `--pick …` and `--exclude …`, each naming a
  fork as `ForkForest::forks` names it (kind, leaf surface and occurrence) and an option by its
  answer, and narrows the forest with `ForkForest::pick` and `ForkForest::exclude`
  (`english-forest-forks`), printing the remaining count and the narrowed forest's forks.
- When at most `--limit` (default 1) Readings remain, each is printed as the survey's folded
  constituent tree, with a small set of judgment assertions that isolates it among the unit's
  Readings, found by greedy set cover over the assertions that hold on it, and marked as
  diagnostic output, not as a judgment to copy unread.
- A selector spells a word, a fork's own or a governor's, as the survey prints it:
  `"surface"#N`, its surface and 1-based occurrence. The occurrence is always written; a bare
  `"surface"` is an error, for a surface that occurs once too. A name with its occurrence stays
  valid when an edit elsewhere in the text adds the surface again (`ForkWord`), and a word copied
  from the survey's output is already a valid selector word.
- Selectors stay separate from the `=>` assertion syntax of judgments.
- Slicing works on forests of any size and does not depend on `READING_LIMIT`.

## Open questions

- How a selector spells a fork's kind and an answer on the command line, around the word
  spelling above. A selector carries a name and no offset (`ForkWord::names`, `Answer::names`):
  the kind and the word; a dependency answer's function, governor Category and governor word; a
  lexical answer's Lexeme, Word Form and features; a frame answer's Productions.

- The isolating assertions must exclude every other Reading of the unit, which on a large forest
  cannot be enumerated. Options: count, for each candidate assertion, the Readings that satisfy
  it on the forest, which is the forest-level assertion check `english-judge-reading-limit`
  builds (this ticket then needs it); or compute the cover by enumeration only when the unit is
  small enough, and refuse otherwise.
- Whether the command also writes the sliced forest of one unit in `english-forest-database`'s
  schema, for grammar forensics. If so, this ticket needs that one.

## Done when

- On *Draugr Necromancer*'s second line, picks and exclusions narrow the forest step by step to
  one Reading, and the count printed after each step equals the number of enumerated Readings
  that agree with the selectors so far.
- On *Remove Enchantments*' only line (9,216 Derivations) narrowing runs without enumerating,
  and the counts after each step agree with the option counts the survey printed.
- For every unit narrowed to one Reading in the tests, the printed assertions, written as a
  judgment entry, hold under `Judge::evaluate` and are satisfied by that Reading only.
- A selector naming no fork, or an option the slice has already removed, is an error naming it.
- `cargo test -p xtask` and `cargo clippy -p xtask --tests -- -D warnings` pass.

## Not in scope

- Choosing a Reading for the user, or ranking options (`docs/decisions/english.md` §4, §5).
- Writing judgments to `crates/english/judgments/`.

## Pointers

- `crates/xtask/src/english.rs`; `crates/english/src/judgment.rs` (`Assertion`, `holds`,
  `parse_file`), `crates/english/judgments/README.md`.
- `crates/syntax/src/forks.rs` (`ForkForest::pick`, `ForkForest::exclude`).
