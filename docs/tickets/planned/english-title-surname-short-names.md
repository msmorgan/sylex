---
needs: []
---

# English: a Shortened Name made of a title and a surname

## Why

Star Trek legends refer to themselves by a rank or title and a surname that is not a prefix of the
card name [CR#201.5c]: Captain James T. Kirk says "Captain Kirk", Dr. Leonard "Bones" McCoy says
"Dr. McCoy", Ensign Harry Kim, Operations says "Ensign Kim". `rule` in
`crates/vocabulary/src/shortening.rs` only proposes a prefix (before a comma, before a Roman
numeral, before *the*/*of*, or the first word), so none of these is attested and each unit that
uses one goes unread in the census.

## Done when

- `short_name` proposes a title-and-surname form, and the form is admitted only where the face's
  own Oracle text attests it, as every Shortened Name is.
- These units read in `cargo xtask english strata`: Dr. Leonard "Bones" McCoy, Captain James T.
  Kirk, Commander Raffi Musiker, Lieutenant Hikaru Sulu ("he explores" included), Dr. Beverly
  Crusher, Ensign Pavel Chekov, Captain Jonathan Archer, Captain Michael Burnham, Captain Kathryn
  Janeway and Ensign Harry Kim, Operations.
- A `shortening.rs` unit test covers a title-and-surname name and a name with a quoted nickname.

## Pointers

- `crates/vocabulary/src/shortening.rs` (`short_name`, `rule`, `attested_short_name`)
- `docs/decisions/lexicon.md` §3 and §6
- `docs/contexts/oracle-english/CONTEXT.md` (Shortened Name)
