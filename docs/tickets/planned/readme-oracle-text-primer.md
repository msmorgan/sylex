---
needs: []
---

# Introduce Oracle text before the README relies on it

**Why.** The README's pitch, Design and Architecture sections treat "Oracle text" as known and as
an authoritative source language. A reader who does not play Magic gets no answer to the first
question: why would card text be uniform enough to compile at all?

**Done when.**
- A paragraph or two near the top, ahead of Design, says what Oracle text is: Wizards of the
  Coast's authoritative rules text for each card, maintained centrally rather than fixed at
  print time, so an old card carries current wording; and written to a house template, the same
  concept in the same words in the same order across the card pool, with the existing corpus
  reissued when the template changes.
- It calls Oracle text "templated", "regimented" or a "controlled sublanguage", never a "regular
  language": formally it is not one, and "regular" elsewhere in the repo means regular morphology.
- It describes the observed regularity, citing `docs/oracle-style-guide.md`, if at all, as the
  project's own reconstruction from the CR and the corpus, never as a published standard.
- It describes the input corpus, not the parser: no grammar-coverage or implementation claims.
- The GitHub repository's About blurb carries the same framing in one line.

**Not in scope.** Restructuring the Design section; the rest of the README rewrite
(`portfolio-polish`).

**Pointers.** `README.md`, `docs/oracle-style-guide.md`.
