---
needs: []
---

# A keyword's term mentions its source where and how the rule prints it

## Why

`syl-this-head-template` gave every self-mention the head its rule prints and wrote `it` where
the rule prints "it" after its antecedent. These builtin sites could not take the printed word,
because the term's structure differs from the sentence's.

The term reads the "it" site before the antecedent the rule prints first:

- "exile it instead of putting it anywhere else any time it would leave the stack"
  ([CR#702.127a,702.34a,702.180a,702.133a,702.196a]): `aftermath`, `flashback`, `harmonize`,
  `jumpStart` and `beamMeUp` read `would(actor(this(…)), leave(from = stack))` first.
- `buyback` [CR#702.27a] and `rebound` [CR#702.88a] ("… instead of … as it resolves"), `haunt`
  [CR#702.55a] ("during its resolution"), `impending` [CR#702.176a] ("If you chose to pay this
  permanent's impending cost, it enters …"), `daybound` [CR#702.145b] ("… it enters
  transformed", whose term also omits "represented by a double-faced card"), `miracle`
  [CR#702.94a] ("as you draw it if it's the first card …") and `madness` [CR#702.35a] ("When
  this card is exiled this way", written over an unprinted discard).

The rule's antecedent is not written, so its "it" is a `this(…)`:

- `bestow` [CR#702.103a] ("cast it bestowed … rather than its mana cost"), `plottedCardCast`
  [CR#702.170d] ("A plotted card's owner may cast it") and `haste` [CR#702.10b,702.10c] ("If a
  creature has haste, it can attack …", "its controller").

The term writes a source argument the rule does not print:

- `keywordCostPaid(this(spell), …)` for "If the buyback cost was paid" and its kin in `buyback`,
  `flashback`, `harmonize` and `beamMeUp`; `megamorph`'s `keywordCostPaid(it, morph)` for "its
  megamorph cost"; `choiceWasTaken(this(spell), …)` for "If you do" in `offering` and `mutate`;
  `doUnlessKeywordCost(…, this(permanent), …)` for "unless you pay" in `echo` and
  `cumulativeUpkeep`.
- `cipher`'s `related(encodedOn, counterpart = this(creature))` for "the encoded card",
  `skulk`'s `powerOf(this(creature))` for "creatures with greater power", `saga`'s
  `finalChapterNumber(this(saga))` for "with one or more chapter abilities", and
  `cumulativeUpkeep`'s `is(this(permanent), noun(a(permanent)))` for "if this permanent is on the
  battlefield".

The rule prints a relative or a pro-form:

- `sneak` [CR#702.190b] ("A permanent spell whose sneak cost was paid") and `soulbond`
  [CR#702.95a] ("this one").

## Done when

- Each site above is read against its rule's text (mtg-rules skill), and its term mentions the
  source where the rule does, with the word the rule prints, or its ticket is named where another
  planned ticket's construction is what writes it (`syl-would-operator-and-replacement-clauses`
  for the "would leave the stack" replacements).
- Every other `this(…)` in `plugins/builtin/` whose rule prints "it", "its", a relative or no
  mention at that spot is found and treated the same way; the landing lists them.
- The landing runs the `syl-number-snapshot` diff and explains every flagged line.
- `cargo xtask dsl check`, `cargo xtask dsl fmt --check` and `cargo test -p xtask` pass.

## Not in scope

- Canon cards: `syl-canon-wording-defects`.
- "He", "she" and "they" for a card's subject: `syl-personal-pronoun-self-references`.

## Pointers

`plugins/builtin/definitions/keyword_ability/`, `plugins/builtin/primitives/pronoun/{this,it}.syl`;
`docs/tickets/done/syl-this-head-template.md`, `docs/tickets/done/syl-printed-it-self-references.md`.
