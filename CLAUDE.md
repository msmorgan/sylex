# CLAUDE.md

## Scope

A card is in scope when mtgjson lists it as `Legal` or `Restricted` in Vintage (its
`cardLegalities.vintage` column in `data/mtgjson/AllPrintings.sqlite`), whatever product printed it.

- Permanently out of scope: the Conspiracy and Plane card types, Vanguard, Scheme cards and ante.
  Do not ask about them, model them, or reserve room for them.
- Deferred, not permanently out: Un-sets (some Un-cards are Vintage-legal).

## Ground rules

- Solo research project: no users or teammates. Add no backward compatibility, deprecation paths
  or shims; change or delete freely. The docs are AI-written guidance: the owner's stated intent
  wins, and you report a contradiction instead of resolving it by invention.
- Fix a stale, broken or wrong thing when you notice it, in its own commit, and never copy a
  legacy wart into new code. "Already failing on default" and "unrelated to my change" are never
  reasons to leave a failure. Fix the cause, not just the instance (add the missing `.gitignore`
  entry, not only delete the stray file); telling the owner to clean something up is leaving it.
- Crash early: fail with an error that names the defect. Never swallow unexpected state or
  missing data with silent fallbacks, dummy values, `unwrap_or_default()`, catch-all `_ =>` arms
  or an early `return`.
- Declare a schema once and derive the rest in the build (macro, derive, build step). Never check
  in repetitive expanded code that no in-repo source regenerates; one-off scripts that edit the
  source are fine. Don't couple distinct concepts because their code looks alike.
- Write a decision into the document or code it governs, as plain current fact. No "Decided:"
  notes, decision logs, dates or owner labels.
- Size, count and coverage numbers are evidence, never targets: add no enforced thresholds and
  don't optimize a number. When a change grows an already-large file, consider whether it splits
  along natural concept boundaries, and if it does, split it fearlessly.

## Where things are

- `crates/`: the current crates. `v1/`: the old line, deleted once the new one reaches parity;
  never extend it or copy newer work into it.
- `lean/`: the Lean prototype that runs the `.syl` corpus in `plugins/`; it is due for deletion.
- `docs/decisions/`: ADRs; `semantics-v3.md` is the semantics design.

## Writing

- American spelling everywhere (code, identifiers, docs, tickets, messages); verbatim quotations
  keep theirs.
- Before naming a public item or coining a term, check `docs/contexts/game-model/CONTEXT.md` or
  `docs/contexts/oracle-english/CONTEXT.md`; rename the identifier to match, never the entry.

## Version control

- This repo uses **jj**, never git; the jj-sensei skills govern it. Keep change descriptions
  brief.
- Run jj as a top-level command, never inside scripts, `python -c` or shell functions; piping its
  output is fine.
- Per-task evidence, progress files and plan-scoped verifier code go in the gitignored
  `docs/evidence/` or `.superpowers/`, never in `crates/`.

## Workflow

- The jj-kata `kata` and `kanban` skills own the lifecycle. **Claim or start before any
  exploration, design or subagents**, then work in `.workspaces/NAME`.
- **Never `kata refresh --all`** (owner only).
- Ticket form: `docs/tickets/README.md`.

## CR citations

- Format: `[CR#704.5g]`, `[CR#601.2g,106.4]`, `[CR#601.2a..601.2b]`; never a bare `704.5g`.
- Rule numbers come from the CR (`data/rules/`), never memory.
- After changes: `cargo xtask cite check --list-noncompliant` empty; `cargo xtask cite check`
  0 stale; `cargo xtask cite bless` for newly cited rules.
- Before committing, run `jj diff --git | cargo xtask cite audit --diff` (it reads stdin; run bare,
  it audits nothing) and read each cited rule's text against its claim.

## Tests

- Per changed crate (code or consumed data): `cargo test -p <crate>` and
  `cargo clippy -p <crate> --tests -- -D warnings`. `cargo xtask lean check` only when `lean/` or
  `plugins/` changed.
- If a test fails and what it asserts is still wanted, fix the code.
- A test that asserts something wrong or pins incidental behavior is rewritten to assert what is
  wanted, or retired. Never write a known defect into a test or ticket as accepted; fix it or
  ticket the fix.
- Never `#[ignore]` without naming the exact blocker.
- Report tests added, rewritten and retired, with a reason for each retirement.
