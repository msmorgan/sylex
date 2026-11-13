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

- A second stateless call takes fork selectors, `--pick …` and `--exclude …`, and narrows the
  forest with `ForkForest::pick` and `ForkForest::exclude` (`english-forest-forks`). Each step
  prints the Derivations left; then the survey prints the narrowed forest: its settled spine and
  its forks. Slicing works on a forest of any size: it enumerates no Readings.
- A selector is one argument, `KIND "surface"#N ANSWER`. `KIND "surface"#N` is a fork's name as
  the survey prints it, an alias's too (`lexical`, `dependency` or `frame`), and `ANSWER` is one
  option's answer as an option line prints it, without its count (an alias's answer for an
  alias's name). The kind and word end at the occurrence, so the answer is everything after it
  and is matched against the answers the survey prints for that question, not parsed: a dependency
  answer's function, governor Category and governor word, a lexical answer's Lexeme, Word Form and
  features, and a frame answer's Productions all hold characters (`:` `,` `=` `#` `"`) that a
  structured grammar would trip on. The two renderings are one function, and two options of a
  question that print alike are an error.
- A selector spells a word, a fork's own or a governor's, as the survey prints it:
  `"surface"#N`, its surface and 1-based occurrence. The occurrence is always written; a bare
  `"surface"` is an error, for a surface that occurs once too. A name with its occurrence stays
  valid when an edit elsewhere in the text adds the surface again (`ForkWord`), and a word copied
  from the survey's output is already a valid selector word.
- Selectors stay separate from the `=>` assertion syntax of judgments: a selector begins with a
  lowercase kind and names grammar child names, an assertion with a Category and CGEL functions,
  and a pasted assertion is an error saying so.
- Steps apply in the order written, `--pick` and `--exclude` interleaved. The counts printed
  after each step and the error for an option an earlier step removed depend on the order.
- Each of these is an error naming the selector: an unknown kind, a word without its occurrence
  (a governor's too), an unclosed quote, a word the unit lacks, a kind and word that name no fork
  or alias of the unit, an answer that is no option of that fork (the options are listed as
  printed), an option an earlier step removed, and a step that leaves no Derivation.
- With at most `--limit` (default 1) Derivations left, each is printed as the survey's folded
  constituent tree (`ForkForest::settled_readings`, which gives each Reading with its tree as a
  settled node); one Derivation's tree is the spine itself.
- `--assertions` needs between 1 and `--limit` Derivations and prints, for each remaining Reading,
  a small set of judgment assertions that isolates it among the unit's Readings, marked as
  diagnostic output, not a judgment to copy unread. The set is found by greedy set cover: the
  candidates are the assertions the Reading's own Constituents suggest (a Category and surface,
  with an occurrence, a head word, a function or a Gap Antecedent), each holds on the Reading
  (`Judge::holds`), each covers the unit's other Readings it fails on, and a final pass drops each
  assertion the rest make redundant. It enumerates the unit's Readings as the judge does
  (`Judge::evaluate`), so the set is sound by construction. Where no assertion of these forms
  tells some Readings from the Reading, as when they differ only in Productions, which no
  assertion names, it reports how many.
- The command does not write the sliced forest to `english-forest-database`'s schema.

## Done when

- On *Draugr Necromancer*'s second line, picks and exclusions narrow the forest step by step to
  one Reading, and the count printed after each step equals the number of enumerated Readings
  that agree with the selectors so far.
- On *Remove Enchantments*' only line (9,216 Derivations) narrowing runs without enumerating,
  and the counts after each step agree with the option counts the survey printed.
- For every unit narrowed to one Reading in the tests, the printed assertions, written as a
  judgment entry, hold under `Judge::evaluate` and are satisfied by that Reading only.
- A selector naming no fork, or an option the slice has already removed, is an error naming it.
- `ForkForest::settled_readings` gives the Readings of a narrowed forest, each with a tree that
  builds it, and for one Derivation the settled spine.
- `cargo test -p xtask` and `cargo clippy -p xtask --tests -- -D warnings` pass, as do the tests
  and clippy of `sylex_syntax` and `sylex_english`.

## Not in scope

- Choosing a Reading for the user, or ranking options (`docs/decisions/english.md` §4, §5).
- Writing judgments to `crates/english/judgments/`.

## Pointers

- `crates/xtask/src/english/forestwalk.rs` and `forestwalk/` (`selector.rs`, `isolate.rs`);
  `crates/english/src/judgment.rs` (`Assertion`, `holds`, `parse_file`),
  `crates/english/judgments/README.md`.
- `crates/syntax/src/forks.rs` (`ForkForest::pick`, `ForkForest::exclude`) and
  `settled_spine.rs` (`ForkForest::settled_readings`).
