# ADR: Semantics v3 — DRT Reference Resolution, the Definition/Template Split, and the Action-Clause Spine

- **Status:** Promoted (accepted by owner).
- **Supersedes (archived in `docs/archive/decisions/`):** `semantics-v2.md`,
  `oracle-text-is-forward-anaphoric.md`, `idris-is-a-soundness-gate.md`. Their rulings that remain in
  force are restated here (§6), so this ADR is self-contained. Never copy v2 or v3 work back into
  v1, and add no new v1 registries.

## Overview

Semantics v3 is the project's model of what card text means. A card's meaning is a `.syl` term: a
first-order tree of words (primitives, Templates and Definitions) that stays close to the card's
English. Each phrase sits where the card prints it and is read in printed order. Semantics v3
checks that a term is a faithful reading of its English, and lowering compiles the term to the
engine's core.

It replaces three earlier designs with one each: an explicit Discourse Representation Theory (DRT)
model of reference replaces positional binding stacks; rules-defined vocabulary (Definitions) is
kept apart from phrasing helpers (Templates); and each verb is defined once, as an Action, instead
of once per clause mood.

### Principles

The detail is in §1.

1. **Surface-faithful.** A construction stands where the English puts it. There are no hoisted
   binders and no target slot lists; lowering does all rearrangement.
2. **Read forward.** Every argument is read in the discourse context of everything printed before
   it. The only exception is one clause-internal step of the Contrast Anchor (§2.5).
3. **The checker reads text, not game state.** It decides whether a term is a faithful reading of
   its English. Whether an object can be found at runtime, and in what order effects apply, is the
   engine's concern.
4. **Rules are data.** Every decomposable game rule is a declaration, not engine code.
5. **The CR decides structure.** The model distinguishes what the Comprehensive Rules distinguish,
   composes existing parts before it adds new ones, and adds nothing to work around a check of
   runtime state.
6. **One source of truth.** Each word is declared once, in one `.syl` file, with its spelling,
   signature, lexical frames and meaning together. Everything else is derived from it.

### Pipeline

1. **`.syl` files (§5.1).** Every declaration and card is a `.syl` file whose text is its own
   first-order syntax tree. `deckmaste_dsl` parses it.
2. **Name resolution and expansion (§5.2).** Each word resolves by its name and the sort expected
   where it stands. Templates expand inline. A Definition expands into its own Reference Scope,
   under its Deed. Binding Identities are assigned in the order the checker reads the expanded
   tree.
3. **Discourse (§2, §4).** The expanded term is read left to right against a Discourse
   Representation Structure (DRS). Each mention introduces a referent or resolves to an existing
   one. Boxes govern which referents stay accessible, and the anaphor rules pick antecedents.
4. **Lowering.** The checked term compiles to the engine's core. Hoisting, target slot lists and
   register indexing all happen here ([`core-explicit-regions.md`](./core-explicit-regions.md)).
   The Rust rewrite (`semantics-rust-rewrite`) owns this step.

Lean is a workbench and verifier for this model. It is never in the hot path of card compilation:
the Rust pipeline reads `.syl` natively.

### Main concepts

- **Definition vs Template (§3).** A Definition is vocabulary the rules define (a keyword action or
  ability [CR#701.1,702.1], a Counter, a Designation), read in its own Reference Scope under its
  Deed. A Template is a printed phrasing that expands transparently in its caller.
- **The action-clause spine (§4).** Each verb is one Action. Imperatives, Handoffs, costs, triggers,
  replacements, deontic rules and lookbacks all embed that same Action, under a Subject that states
  its voice.
- **The DRS and reference resolution (§2).** Mentions introduce referents with a stable identity, a
  clause and a grammatical role. Anaphors and bare "another" resolve against the referents
  accessible in the current box.
- **Read order (§1 item 2, §5.2 item 3).** Arguments are read in printed order. A parameter used
  several times is bound where the checker first reads it and read back elsewhere.

---

## 1. Principles

1. **In-situ surface constructions (no prenex hoisting).** Semantics v3 encodes Magic card grammar
   as a reduced, surface-faithful form of English, translated from the `deckmaste_english` syntax
   tree ([`semantics-spelling-lowering.md`](./semantics-spelling-lowering.md) §17;
   [`english-v4.md`](./english-v4.md)). A construction occupies its surface position. There are no
   prenex target slot lists, no hoisted binders and no positional slot reads. All rearrangement,
   hoisting and register indexing belong to lowering.
2. **Strictly forward-anaphoric reading order.** Every argument position is checked in the
   discourse context of everything textually earlier. Argument order is textual order, and the DRS
   is threaded left to right. There is no cataphora apart from the Contrast Anchor's clause-internal
   step (§2.5 step 3), and there is no out-of-band target or announcement channel.
   - A delayed trigger or continuous effect reads previously announced targets as settled
     particulars ([CR#603.7c]). They are not its own targets, which are chosen per ability
     ([CR#603.3d,601.2c]). They are earlier mentions in scope, though, so they can be the Contrast
     Anchor of an "other" in it (§2.5 step 2).
   - Across an activated ability's colon (`cost : effect`), the effect reads every cost referent its
     text can name, wherever the cost moved it, a hidden zone included. Whether the engine can find
     the object at runtime, or must use its last known information, is the engine's concern
     ([CR#400.7j,608.2h]; item 4).
   - **A moved referent is read back by anaphora ([CR#400.7,400.7j]).** When a Definition or
     Template body moves a parameter's referent to another zone, the object in the new zone is a
     new object. Later clauses read it back by anaphora ("it", "that card", "each card exiled this
     way"), never by the parameter's Binding Identity.
3. **Checker charter: a faithful English reading, not a runtime simulation.** The checker verifies
   that a written term is a faithful reading of the card's English. Its discourse referents,
   pronouns, demonstratives, outcomes, choices, deed stamps, Reference Scopes, plurality and
   Handoff Actors resolve. Its terms are well-kinded. Its card frame is well-formed, from its card
   lines to frames such as prototype. A Characteristic-Defining Ability meets the criteria of
   [CR#604.3a] (what it edits, its subject, what it grants, and its conditions). A retention
   qualifies one clause that removes Attachments, for Attachments of that clause's host
   ([CR#702.16n,702.16p,303.4c,704.5m]). The checker is not a runtime rules engine and does not
   judge impossible instructions ([CR#101.3,609.3]).
4. **Engine boundary: rules are data, runtime state is the engine's.** Every decomposable game rule
   is modeled as data in `.syl`: lifelink's damage result ([CR#702.15b]), the +1/+1 and −1/−1
   counter annihilation state-based action ([CR#704.5q]), turn-based actions ([CR#703.1]), and
   intrinsic type rules such as a planeswalker's or battle's entry with counters
   ([CR#306.5b,310.4b]). Rules Tables and Registry Definitions hold those rules. The checker does not
   check that a rule is defined correctly; its CR citations carry that.
   - The checker's job is the meaning of text: compositional denotation, discourse and reference
     resolution (anaphora, scope, binders), and sort and binder well-formedness. Rules-as-data checks
     are well-formedness checks.
   - It does not simulate runtime state. These belong to the engine: whether an object in a hidden
     zone is visible at runtime, whether last known information is available ([CR#608.2h]), timing,
     and the order in which state-based actions or replacement effects apply ([CR#704.3,616.1]).
   - A reference with a well-formed textual antecedent is admitted: "the creature returned this way"
     names the creature a cost returned to its owner's hand. So reference resolution applies no
     public-zone filter anywhere: not across an activated ability's colon (item 2), not in box 1 of
     §2.2, not to Last Known Information reads (§2.4), and not to what an ability's text publishes to
     the abilities after it (§6).
   - Information reads of a departed spell do not require it to be on the stack (§2.4). The shuffle
     discourse rules and action-slot reach (§2.4 J1 item 4) remain.
5. **The CR is authoritative on rules structure.** A model distinction exists where the CR or
   anaphora resolution distinguishes it. This has three parts:
   - (a) **Encode the structure the CR states.** Where the CR distinguishes two rules structures,
     the model distinguishes them: an intervening "if" from a plain condition ([CR#603.4]), a
     replacement from a trigger ([CR#614.1,603.1]), an additional from an alternative cost
     ([CR#118.8,118.9]), a turn-based action from an effect ([CR#703.1]). A structural distinction is
     never dropped on the theory that no card notices it. Gift's second ability is an intervening
     "if" ([CR#702.174b]), and the "gives a gift" triggers of [CR#702.174c], Jolly Gerbils among
     them, depend on it.
   - (b) **Compose before minting.** A CR definition is expressed by composing existing primitives,
     with Templates carrying the printed wording, rather than by a one-off constructor. A
     planeswalker's or battle's entry with counters reads `statOf(stat(loyalty|defense), this)`,
     because off the battlefield loyalty and defense already are the printed number
     ([CR#306.5a,310.4a]). There is no printed-number projection.
   - (c) **No workaround for a runtime-state check.** Where a checker rule simulates runtime state
     (item 4), the check is removed. The model gains no node, field or variant to satisfy it. Reading
     the entry conferrals' count as depending on when the entering object is read was such a case.

---

## 2. Discourse and Reference Resolution

### 2.1 The DRS and Its Referents

Every mention that introduces an entity, quantity, choice or outcome into discourse creates a
**referent** with a stable identity in the current Discourse Representation Structure (DRS). A
mention that resolves to an existing referent re-mentions it; it creates nothing.

A referent records:

- its identity, and the Binding Identities of the parameters bound to it (§2.3);
- its determiner (a partitive sub-selection, from `choose` or `someOf`, is marked as partitive) and
  its number (one or many);
- the clause it was introduced or last foregrounded in, and its **grammatical role** there:
  *subject* (the clause subject: the active Actor, or the promoted patient of a passive), *object*
  (the direct object or primary patient of an active clause), or *other* (an oblique argument such
  as a destination, source, instrument or passive `by` phrase);
- whether a bare pronoun ("it", "them", "they") resolved to it in the current clause;
- its sort and facts. The sorts are: **object** (Card Types, zone, deed stamps, origin, size);
  **player**; **quality** (a chosen color, type, creature type or name); **outcome** ("that much",
  "that many", the mana produced, a roll result, the damage marked [CR#120.3d], the shortfall of
  "draw up to N cards", and so on); **difference** (the difference a comparison names); **letter**
  (a defined variable `X`); **turn** ("that turn"); **ability** (an ability object and its
  remembered location [CR#113.1c]); **pile**; and **join** (§6, joined kinds).

The DRS also holds the current clause, the Actors of the enclosing Handoffs, the stack of active
Definition Reference Scopes (§2.3), and the direct-argument mentions of the current clause.

**Actors.** A Handoff (`instruct`, a clause's subject, or the Source performing a deed with no
Handoff) makes its Actor the innermost on entry and restores the outer one on exit. An Actor
records its kind, whether it is "you", whether a quality can be ascribed to it, the referents it
names (none for an Actor of a kind only, several for a group) and its number. Actors are not
referents: they have no identity, and no pronoun, word, group or count reads them, except that the
`actor` word reads the innermost.

**Coordinated noun phrases.** A conjunction (`and([target artifact, target enchantment])`) threads
introductions in order and introduces one referent per conjunct. A disjunction ("you or a white
creature you control" on *Mangara's Equity*) reads each branch independently from the incoming DRS.
It exports only the referents common to every branch, plus a joined head referent where one
applies, so a branch-local indefinite does not leak out as an unconditional referent. When a
coordination is bound to a parameter, the Binding Identity attaches to every exported member, so
later mentions and zone or status changes after the action update all of them alike
([CR#608.2c]).

**Clause-mates and clause advancement.**

- A referent introduced or re-mentioned in a **direct verb argument slot** (subject, object or
  other) is recorded as a pending role of the current clause.
- Possessor mentions ("its controller", "its power", "target creature's", "that creature's owner")
  and nested modifier mentions are not recorded as pending roles, so J2 does not exclude them as
  clause-mates of the verb. A possessor introduction or re-mention does move the referent to the
  current clause with role *other*, so a later bare pronoun in the same clause sees the
  foregrounded possessor (D′).
- J2 (clause-mate exclusion) and Principle A reflexives consult the pending roles as well as each
  referent's clause (§2.4).
- At the end of the clause, pending roles are committed to their referents (clause and role), the
  pending set is cleared, the clause advances, and every "pronoun resolved here" mark is reset.
- Until then, D′ already ranks a pending referent in the current clause by its best pending role
  (§2.4, D′ step 3).

**A zone change clears identity ([CR#400.7,400.7j]).** When an object referent changes zone, the
referent after the move gets a fresh identity with no Binding Identities, and the departed referent
loses its Binding Identities too. A later mention of the pre-move parameter by its Binding Identity
is therefore refused.

### 2.2 Boxes and Quantifier Force

In DRT (Kamp & Reyle 1993), an indefinite's logical force (`a(creature)`) and its forward
accessibility depend on the **box** it is introduced in, not on separate determiner words.

- A universal bare plural ("creatures you control get +1/+1") is written with the `bare` Template,
  which expands to `all`.
- **A measured domain is an antecedent only where the consequent reads it.**
  - "The number of …" (`countOf`) and "the greatest … among …" (`aggregate`) measure a plural
    universal (`all`) and close it like a relative-clause box (box 5), so no later pronoun reads it
    back (*Tears of Rage*). A target announced inside the measured domain survives the closure,
    because it is chosen as the spell or ability is put on the stack ([CR#601.2c,115.1]): *Tibalt,
    the Fiend-Blooded*'s "the number of cards in target player's hand … to that player".
  - "There is …" / "there are …" (`exists`) publishes its subject for the consequent: a plain,
    counted or coordinated indefinite ("a creature", "two or more creatures"). The consequent reads
    it as box 2 reads an antecedent's indefinite (*Purging Scythe*). A `doIf` with no other branch,
    or a replacement's condition, closes it after the consequent.
  - The domain a replacement's existential condition publishes ranks below ordinary earlier mentions
    for a plural pronoun, and for the group "the rest" and "the other" are taken from. The
    replacement does instead what the replaced action would have done, to what that action read: in
    *Flow State*'s self-replacement "instead put two of them into your hand and the rest …", "them"
    and "the rest" read the cards looked at. Any other existential's domain ranks as an ordinary
    mention, the nearest: in *Juxtapose*'s "If two or more permanents a player controls are tied
    for greatest, their controller chooses one of them", "them" reads the tied permanents.
  - There is no separate measured-domain determiner.
- **A self-replacement starts from the state the replaced action read**, without the part and
  remainder that action introduced, since a replaced event never happens ([CR#614.6,614.15]). The
  targets it announced stay, chosen as the spell or ability is put on the stack (*Galvanic
  Blast*'s "4 damage to that permanent or player instead").
- A property test's coordinated subject may hold an indefinite conjunct, read existentially, and
  publishes its group ("you both own and control Titania, Voice of Gaea and a land named Argoth,
  Sanctum of Nature, exile them").

| Box | Introduced by | Force of an indefinite | Accessibility out of the box |
|---|---|---|---|
| **1. Top-level imperative, and activated cost→effect** | Spell abilities; activated and triggered effects (`sacrifice(a(creature))`); a performed cost (`perform(action)`) across `cost : effect` in an activated ability or special action | Existential choice at action time ([CR#608.2c,602.2b]) | Accessible to every later clause in the paragraph. Across the colon, the cost's action is read after its event, with the activator as the innermost Actor. Every referent the cost introduced is kept, wherever the cost moved it (§1 item 4; runtime visibility [CR#400.2,608.2h] is the engine's). A clause ends at the colon, so in "Sacrifice a creature: Draw cards equal to its power", "its", "that creature" and "the sacrificed creature" resolve by D′, AC>R1 and the sacrifice's deed stamp. |
| **2. Duplex / implicational (`K₁ ⇒ K₂`)** | Trigger event → effect (`when`/`whenever`); replacement event → replacement (`replace(would(…), instead(…))`); positive conditional → consequent | Universal over matching events and participants ([CR#603.2,614.1]) | Referents introduced in the antecedent are accessible in the consequent ("Whenever a creature enters, it gets …", "If you sacrificed a creature this turn, return it …") and closed after it. In an if/then(/else) instruction (`doIf` / `doOnlyIf`), announced targets (chosen at announcement [CR#601.2c,115.1]) and changes to earlier outer referents survive. With an else branch, branch-local non-target introductions are closed on exit. With none, the branch's output continues: in "Create a token if you control a creature. Put a counter on it.", "it" is the token, and only an existential condition's domain is closed after it. |
| **3. Negated / modal prohibition (`¬K` / `¬◇K`)** | Deontic rules (`cant`, `must`, `can`, `gatedBy`); a negated condition (`not(…)`, which "unless" writes); the else branch of `doIf(cond, then, else)` with respect to `cond` | Universal prohibition ($\neg\Diamond\exists x = \Box\forall x\,\neg$, [CR#101.2]) | Referents introduced inside are closed on exit and inaccessible to later sentences (nor accessible in the else branch from a negated `cond`). A definite Source reference in a negated property test ("if this enchantment is not a creature") stays available; the negated predicate introduces no accessible participants or positive type refinements. In a deontic rule, announced targets, definite references (the Source, "you", an Attachment Host) and universal domains are established in the enclosing ability's scope and stay accessible under their identities: *Nowhere to Run*'s "Creatures your opponents control can be the targets … as though they didn't have hexproof. Ward abilities of those creatures don't trigger." A domain published with `each` is singular, read by "it" as after "Tap each creature. Untap it."; one with `all` is plural. Hypothetical indefinite, counted and partitive participants the rule's clause introduces are closed, so after "You may cast a creature card from your graveyard", "Exile it" finds nothing. The box keeps the events a permission or requirement writes (§4.8) and the resolution bookkeeping (resolved pronouns, Contrast Anchors). An explicit `by` phrase still denotes the passive's agent, never an Attachment Host. |
| **4. Distributive loop (`∀x ∈ S : K`)** | `doForEach(collection, body)` | Universal distribution over the collection. A singular loop referent (number one, role subject) is introduced at the header's clause C, and the body starts at clause C + 1. | Because the body starts a clause after the header, "it" in a direct argument slot of the body (`cast(it)` in `airbend`) is no clause-mate (J2) and selects the loop referent (clause C) over any outer antecedent (clause < C). Loop-local referents are closed on exit; changes to outer referents survive; plural abstraction exports plural aggregates (below). |
| **5. Relative clause / comparison** | A relative lookback (`did(…, which, …)`), a comparison over candidates (`compareOver`, `comparesOwnStat`), the domain `countOf` and `aggregate` measure | Binds `which` (or the comparison candidate) in a subordinate clause (current clause + 1) | Local to the relative or comparison clause. Inside `compareOver`, `ownMember` reads the outer candidate. Closed on exit, while definite presuppositional references (an Attachment Host: "enchanted", "equipped", "fortified", [CR#301.5,301.6,303.4b]) project to the enclosing ability's box. |
| **6. Definition Reference Scope** | Every Definition use ([CR#701.1,702.1]) | Arguments are read in the caller's box; caller referents are hidden while the body runs; changes and outcomes are exported on exit | §2.3. |

**Plural abstraction on leaving a loop.** Leaving a distributive box (`doForEach`) closes the
singular loop variable and fresh local indefinites, and abstracts plural referents (number many,
with no Binding Identity or exact member count) over two things the body did:

1. **Announced targets.** A resolving `choose` publishes a plural target referent per announcement,
   keeping its zone, types, deed stamps and origin after the iteration (so *Vaevictis Asmadi, the
   Dire* reads "those permanents" after choosing a target permanent for each player). A target
   determiner alone does not make this publication: "reveal target creature" is an ordinary local
   mention, and a target inside an optional branch stays closed.
2. **Created objects.** The body publishes a plural referent for the objects it created, keeping
   their actual zone, types and stamps after the iteration (*Hate Mirage* reads "those tokens"). A
   token origin alone would include mere mentions of tokens, and deriving the fact from a create
   stamp would lose a created token after a move replaces that stamp ([CR#400.7]).

Both are distinct discourse facts the referent records: what a loop body announced, and what it
created (owner ruling). They are not derived from the determiner, origin and stamps. Being an
Attachment Host is a separate presupposition fact, read only when a relative clause closes; it
does not classify loop results.

**Partitive sub-selections and ceiling shortfalls** are ordinary referents. A partitive choice (from
`choose` or `someOf`) is what `notChosen` reads for "not chosen this way"; the shortfall of "draw up
to N cards" is an outcome.

### 2.3 Definition Reference Scope

A Definition body is read in its **own Reference Scope**. The caller's referents are hidden from it,
except its arguments and the innermost Actor. Each argument is read in the caller's scope. On exit,
what the body did to its arguments, and the outcomes it produced, are exported to the caller.

The expansion (§5.2 item 3) carries the Definition's Deed on its body, so `does`, `would`, `did`
and `cant` can name it (§3.1), and wraps every parameter: bound to a Binding Identity at its first
mention and read back by it at later ones. The identity is the Definition's own and outermost,
named by the declaration and parameter (`devour.subject`); only the argument is read in the caller.

A Template gets generated Binding Identities for every NounPhrase parameter it mentions twice or
more (§3.2). `.syl` has no helpers that can inspect parameters. A call that escapes scope
explicitly, for an expression that is not a parameter, is an ordinary declaration. Nested
Definitions (`connive` calling `draw` and `discard`, `forage` calling `exile` and `sacrifice`) nest
their scopes.

1. **Entering the scope.** The checker snapshots the referents that exist before the call. It hides
   all of them except those of the innermost Actor, records the caller's clause, and advances the
   clause.
2. **Reading an argument in the caller.** An argument is read in the immediate caller's visibility
   only. Referents hidden by an enclosing Definition stay hidden.
   - The first mention binds the Binding Identity to every referent the argument introduced or
     resolved to, inside the body's scope. A later mention reads them back.
   - Those bound referents become visible inside the body; every other caller referent stays
     hidden. The scope records the arguments it owns on the caller's behalf, including those first
     introduced while reading them. A zone change carries that ownership to the successor referent.
   - A parameter mentioned only once, or moved and read back by anaphora (`airbend` [CR#701.65a]),
     is legal. There is no unused-parameter refusal.
3. **Leaving the scope.**
   - The outer scope is restored (the enclosing Definition's, when nested).
   - **The caller's roles are restored.** A mention in the body never changes a role the caller can
     see. The caller's pending roles become the ones it had on entry, and each argument read in the
     caller is pending as the call's object, whatever role its mentions in the body took. So
     "Target creature fights target creature. Tap it." reads the fight's source.
   - **Scratch referents are closed.** A referent the body introduced, one that neither existed
     before the call nor is caller-owned, is dropped if it is a partitive sub-selection or a
     definition-local player binding. The test uses the snapshot taken on entry, not the hidden
     set, so a player passed as a parameter is never mistaken for a disposable local player.
   - **Changes to arguments are kept.** Zone changes, status changes and deed stamps applied to the
     arguments, or to their successors after a zone change ([CR#400.7,400.7j]), remain.
   - **Outcomes are exported.** Every non-scratch referent the body produced remains: outcomes of
     every sort, piles, qualities, letters, turns, and primary object outcomes (tokens from
     `create`, `investigate`, `amass` or `incubate`; cards moved, revealed, searched for, milled,
     drawn or scried). They take the caller's clause, with role object, ready for later anaphora
     ("Investigate. Sacrifice that token …", "Destroy target creature. It can't be regenerated.").

**Token specifications and Entry Riders.** A token specification passed into a Definition is read
in the caller's Reference Scope, and the callee's scope is restored afterward. A whole Entry Rider
argument is read the same way, an attacking defender included. `create` wraps its token
specification and each Entry Rider this way, so chosen characteristics and copy-source references
resolve in the caller. A copy source is read once, and the selected source's types are carried into
the created token. Embedded or granted ability text receives only the announced choices: visible
chosen-quality and chosen-player referents. Ordinary participants and the enclosing Actor stay
outside that text's discourse context.

**Type views.** `headed` (a Head Noun over its subject) and `becomes` (the permanent a spell
becomes) refine the referent they select in place. Describing an object's type or rules role does
not create a second discourse object.

### 2.4 Anaphor Resolution (J1, J2, D′, AC>R1 and Reflexives)

Every anaphor ("it", "them", "they", "itself", "themselves", "yourself", "that <noun>", "those
<nouns>") resolves against the visible referents in two stages. First an accessibility filter (J1)
removes impossible candidates. Then a rule for the anaphor's form (J2, D′, AC>R1, Principle A)
picks the antecedent or refuses. A Participial Description (§4.8) instead requires the unique
referent its deed stamped. It is the only reference that reaches an object a linked ability
published.

#### Stage 1: accessibility (J1)

A referent is **accessible** to an anaphor in a consuming verb slot if and only if:

1. it is not hidden by any active Definition Reference Scope (§2.3);
2. its number agrees with the anaphor's (one for "it", "itself", "that <noun>"; many for "them",
   "they", "themselves", "those <nouns>"; unfixed for a number-neutral description);
3. its sort matches the anaphor's sort, carrier-word zone and `<noun>` filter
   ([CR#109.2,109.2a..109.2c,110.1]): a bare permanent type requires the battlefield, "spell"
   requires the stack, and "card" requires a zone other than the battlefield or stack, or a
   read-back across a zone change; and
4. **the consuming verb can apply to it ([CR#109.2,701.21a]).** The verb or prepositional slot must
   be able to apply to the referent in its zone and kind. A creature that died is in the graveyard
   and cannot have counters put on it as a battlefield permanent, or be sacrificed; this settles
   *Vampiric Embrace* by J1 before ranking runs.
   - Before resolution, a bare pronoun is narrowed to the patient slot's carrier. Battlefield
     actions, and performed moves whose written origin is the battlefield, require a permanent.
     Counter placement keeps other zones [CR#122.1] but excludes the stack.
   - A description demanding "card", or a performed move's written card-zone origin, narrows a bare
     pronoun to the card carrier [CR#109.2a].
   - Unknown zones stay accessible. Patterns and lookback moves do not narrow: their patient is read
     after the event, in its current zone.

**Characteristic reads and Last Known Information ([CR#608.2h]).** Item 3's carrier-word zone
restriction describes action slots. A characteristic read such as "its power" or "that creature's
toughness" may select a departed permanent by its last battlefield type information, whatever zone
it went to, a hidden one included (§1 item 4). Ninjutsu's "the creature that was returned to its
owner's hand" ([CR#702.49c]) is admitted; whether its last known information is available at
runtime is the engine's concern. Such a read does not change the referent's current zone, or admit
it as the patient of a battlefield action. Ordinary D′ and AC>R1 ranking still applies: in *Kalitas,
Bloodchief of Ghet*, "its" selects the token and "that creature's" the destroyed creature. *Vampiric
Embrace* still excludes the dead creature from its counter placement. This is ordinary LKI phrasing,
including "Destroy target creature. You gain life equal to its power."

- **Departed spells.** An information read of "spell" does not need the spell still on the stack. A
  stamped Zone Change whose origin is the stack admits "that spell" and Spell Type words in
  characteristic and possessor reads ([CR#608.2h]), as on *Mana Drain*. It does not admit "counter
  that spell" after countering it, or broaden Participial Descriptions.
- **Possessors.** After a permanent is destroyed, "that permanent's controller" may read its last
  known controller (*Destructive Revelry*). The enclosing damage instruction acts on that player;
  the lookup does not make the destroyed permanent available to a later battlefield action. After a
  spell is countered, "that spell's controller" is admitted in the same way; an action on that
  player does not act on the departed spell.
- **Resolution-time guards are not characteristic reads.** Fight's guard (both participants
  `matches … isCreature`) tests them as they are when the fight happens: if either is no longer on
  the battlefield or no longer a creature, neither fights ([CR#701.14b]). That is fight's rule, not a
  general one; the general rule is [CR#608.2h]. Elsewhere, a present-tense guard over a description
  that names no zone and no "card" ("if it's a creature") reads the permanent as it is, since Oracle
  text puts a departed object's information in the past tense ("if it was a creature"). A
  description that names a zone or "card" tests the object where it is, so *Bioplasm*'s "if it's a
  creature card" holds of the exiled creature card. Evaluating a guard at runtime is the engine's
  (§1 item 4).
- **Historical movement patterns.** "That creature dies this way" reads a previously introduced
  participant against the move's written origin zone when selecting the move's patient in a
  lookback, without a dedicated mark on the DRS. The read covers a definite noun phrase as well as a
  mention written inside the move. Where the clause's subject is the move's patient ("that creature
  dies", [CR#700.4]), that subject is read the same way, so "Destroy target creature. If that
  creature dies this way, …" reads the destroyed creature as "it" does. The noun still constrains
  the read: "that artifact" finds no destroyed creature. The selected referent keeps its current
  zone, identity and stamp.
- **A recorded origin.** Where the move writes no origin ("If that artifact is put into a graveyard
  this way"), a grounded "this way" lookback takes the origin from the Zone Change recorded by the
  event it reads back ([CR#608.2c]). After "Destroy target artifact" that is the battlefield, so
  "that artifact" reads the destroyed artifact as the permanent it was [CR#608.2h]. A recorded
  origin other than the battlefield reads no permanent. The rule is limited to grounded "this way"
  lookbacks, which read back an event their own text caused. A written origin decides alone, a
  negated one ("from anywhere other than the battlefield") included, and a recorded event never
  overrides it. A lookback over another range, or a trigger, takes no recorded origin: "from
  anywhere" is no leaves-the-battlefield pattern [CR#603.6c].
- **Counters do not survive a zone change.** The new object's identity may be available, but its old
  counters ceased to exist ([CR#400.7,122.2]). Removing or transferring counters from a moved
  referent is refused until an instruction or an entry or placement rider puts new counters on it.
  This imposes no battlefield-only restriction on counters.

#### Stage 2: the rule for each form

- **Clause-mates (J2; see Clause-mate in `docs/contexts/oracle-english/CONTEXT.md`).** A referent is
  a clause-mate of an anaphor if and only if the anaphor occupies a direct argument slot (subject,
  direct object or oblique complement) of the current verb and the referent has a pending role in
  the current clause, that is, it was introduced or re-mentioned as a direct co-argument of the same
  verb. Possessors ("its controller", "its owner", "its power", "target creature's controller") and
  objects of small clauses ("counters on it") are not direct co-arguments of the matrix verb.
  A referent introduced or re-mentioned as a possessor has no pending role (§2.1): in "Target
  creature's controller sacrifices it", "it" resolves to the creature, and in
  *Weeping Angel*'s "that creature's owner shuffles it into their library", "it" resolves to the
  damaged creature by D′, with no explicit Binding Identity.
- **Reflexives (Principle A: "itself", "themselves", "yourself").** The exact complement of J2. A
  reflexive requires an accessible clause-mate subject, one with a pending subject role, agreeing in
  number. This covers newly introduced subjects ("Target creature deals 2 damage to itself") and
  re-mentioned ones: in "It deals 2 damage to itself", "It" records a pending subject role, so
  "itself" succeeds, while "it" in the same slot would be excluded by J2. With no matching
  clause-mate subject, it is refused.
- **D′: bare non-reflexive pronouns ("it", "them", "they").**
  1. Exclude clause-mates (J2), when the pronoun is in a direct argument slot.
  2. Among the remaining accessible referents, keep those in the **nearest clause**. A referent
     re-mentioned in a direct argument slot of the current clause counts as in the current clause.
  3. Within that clause, rank by role: subject beats object, which beats other. A re-mentioned
     referent ranks by its best pending role. Steps 2 and 3 change nothing for a pronoun in a direct
     argument slot, since step 1 has already excluded every pending referent. They foreground a
     re-mentioned subject for possessor and amount pronouns: in *Karplusan Yeti*'s "That creature
     deals damage equal to its power to this creature", "its" reads the re-mentioned "that
     creature", not the earlier clause's subject.
  4. If one referent strictly wins, resolve to it, mark it "pronoun resolved here" in the current
     clause, and record its pending role if the pronoun is in a direct argument slot.
  5. Otherwise, with no accessible candidate or a true tie, refuse.
- **AC>R1: demonstratives ("that <noun>", "those <nouns>").**
  1. Filter the accessible referents by `<noun>`.
  2. **A**: exclude clause-mates (J2).
  3. **C**, a preference for anti-coreference with a same-clause pronoun: exclude any referent a
     pronoun resolved to in the current clause, but only when another candidate remains; otherwise
     keep it, and the demonstrative corefers with the pronoun. In *Kalitas, Bloodchief of Ghet*,
     "Its power is equal to that creature's power": "Its" is the newly created token and "that
     creature" takes the remaining older creature. In *Saw in Half*, "its controller creates two
     tokens that are copies of that creature": no other creature remains, and "that creature" reads
     the destroyed creature "its" took. The preference turns on whether a second candidate exists,
     not on syntax: a possessor "its" c-commands "that creature" in neither card, yet only *Kalitas*
     reads them as disjoint.
  4. **R1**, recency: among the remaining candidates, pick the most recently introduced referent.
     Refuse only if none remains.
- **Stamped action participles ("the exiled card", "the sacrificed creature").** Filter the
  accessible referents by `<noun>`, number and deed stamp, and require a unique match; refuse
  otherwise.

**The eleven measured AC>R1 sites.** These cards failed flat surface-token AC>R1 in the original
measurement. Each must resolve to its intended referent from its raw wording, with no explicit
selector: *Tahngarth, First Mate*; *Unpredictable Cyclone*; *Eriette, the Beguiler*; *Runesword*;
*Scythe of the Wretched*; *Bronze Bombshell*; *Gisela, Blade of Goldnight*; *Solphim, Mayhem
Dominus*; *Ram Through*; *Mangara's Equity*; *Noetic Scales*.

- *Mangara's Equity* resolves once a disjunctive noun phrase closes branch-local indefinites
  (§2.1): "a white creature you control", inside "you or a white creature you control", does not
  leak out as an unconditional creature referent.
- *Noetic Scales* resolves when read as its actual move, which reads the moved subject "each
  creature that player controls …" before the destination "to its owner's hand", not in the surface
  order produced by heavy-NP shift.

### 2.5 The Contrast Anchor of "another" and "other"

**A bare "another" or "other" contrasts with a Contrast Anchor, which need not be an earlier
target.** "Another target creature you control" in reconfigure ([CR#702.151a]) and Olivia
Voldaren's "deals 1 damage to another target creature" (the example under [CR#707.11]) have no
earlier target; they exclude the Source. A bare printed "another"/"other" is written only with
`isOther` ("any other target" is `isAnyOtherTarget`, any target that `isOther`). `otherThan <NP>`
is only for a printed "other than <NP>".

A mention of the Source coordinated with the "other" phrase is that phrase's anchor (*Slimefoot and
Squee*'s "Return this card and up to one other target creature card"). Otherwise the checker tries,
in order:

1. **The current member of a distributive group.** This is the group the subject of the phrase's
   clause names ("each …", "all …", a bare plural or another plural group), when it is compatible
   as in step 2. Run's (*Hit // Run*) "Attacking creatures you control get +1/+0 until end of turn
   for each other attacking creature" counts, for each affected creature, the attacking creatures
   other than that creature, not those other than the whole group. The affected set is fixed when
   the effect begins ([CR#611.2c]), but the amount is a function of each member. The checker records
   the anchor as the member of the group's mention, so the engine evaluates the amount per member.
   An "other" phrase in a different clause, or in a nested clause whose subject names no such
   group, skips this step.
2. **The nearest earlier mention in scope compatible with the head** in kind, Card Type, zone and
   controller, whether a target or not ("Choose a player. Each other player …"). Recency is the
   DRS's: on *Arc Trail* ("deals 2 damage to any target and 1 damage to any other target"), the
   first target beats the Source.
   - Mentions of the Source and of "you" are never candidates in steps 1 to 3; they are step 4's
     fallback only.
   - A referent a condition introduces (an intervening "if", a `doIf` or `doOnlyIf` condition,
     `exists`) is hypothetical and never a candidate. Targets the action branch announces are
     candidates ([CR#601.2c]): *Flames of the Raze-Boar*'s "each other creature that player controls
     if you control a creature with power 4 or greater" contrasts with the target.
   - The zone check keeps a cast spell (*Hulkling, Young Avenger*) or a revealed card from anchoring
     a phrase on the battlefield.
   - The controller check applies at every step. A candidate whose controller restriction
     contradicts the head's ("you control" against "an opponent controls") is never the anchor, so
     *Bartz and Boko*'s "each other Bird you control deals damage equal to its power to target
     creature an opponent controls" contrasts with the Source, not the target.
   - The subject of an `is` condition is excluded only for the predicate's own "another", not for a
     noun phrase nested in it: *Unexpected Allies*' "if it has the same name as another creature you
     control" contrasts with "it".
   - The thing "another" describes, including the subject of the `is` condition it is predicated
     of, is never its own anchor, so backup's "If that's another creature" ([CR#702.165a]) contrasts
     with "this creature", not with "that creature".
3. **A compatible particular of the same clause or event, read forward.** When "another <X>" is
   the agent of a clause (a `would`, event, deontic, Handoff, copy or characteristic-change clause),
   that clause's compatible target, definite description or Attachment Host is the anchor. A generic
   indefinite ("a permanent or player", "each creature") never is, so *Embermaw Hellion*'s "another
   red source you control would deal damage to a permanent or player" falls through to the Source.
   *Inquisitor's Flail*'s "If another creature would deal combat damage to equipped creature, it
   deals double that damage to equipped creature instead" contrasts with the equipped creature. A
   redirection effect ([CR#614.9]) that makes the equipped creature deal combat damage to itself is
   therefore doubled once by the Flail's first ability, not twice ([CR#614.1a,616.1f]).
4. **Otherwise the Source**, for an object head in a zone the Source may be in and not restricted to
   a controller other than "you", whatever the Source's Card Types (Vehicles are creatures when
   their "another" applies); or **"you"**, for a player head ([CR#109.5]). A head that may be an
   object or a player ("any other target") falls back to the Source, never to "you".

---

## 3. Definitions and Templates

**A declaration's kind says whether it is a Definition or a Template; its file path never does.** A
Definition is vocabulary the rules define, read in its own Reference Scope under its Deed. A Template
is a printed phrasing, expanded transparently in its caller.

| | Definition | Template |
|---|---|---|
| **Domain** | Rules-defined vocabulary: [CR#701] keyword actions, [CR#702] keyword abilities, and registry definitions (Counter kinds, Subtypes, Designations, Relations, Flavor Words, Card Types). An ability word has no rules meaning ([CR#207.2c]), so it is a Template over `abilityWord`, its label checked against the CR's list. A turn part is a closed set, declared by no one. | Reusable Oracle phrasing patterns, multi-clause shorthands, and aliases of primitives. |
| **Declaration kind** | `action`, `ability`, and the registry kinds (`counter`, `designation`, `subtype`, `card_type`, `relation`, `token`, …) | `template` |
| **Reference Scope** | Its own (§2.3). It hides the caller's referents, keeps the keyword's Deed, and reads its arguments in the caller's DRS. | The caller's. It expands inline in the caller's DRS and hides nothing. |
| **Binding Identity** | Generated for every NounPhrase parameter (supporting several referents for a coordination); there is no opt-out (§5.2 item 3). Actor identities are written by hand. | Generated for each NounPhrase parameter mentioned more than once in the body: bound at the first occurrence and read back at later ones before any zone change. Hand-written ones are refused. |
| **Checking** | Trusted, as every authored declaration is. An action Definition's body is checked at every card or Definition use that expands it. A Registry Definition's conferrals are checked by the registry check. | Checked at every card or Definition use that expands it. |

### 3.1 Keyword Actions Are Definitions

**Every Vintage-playable [CR#701] keyword action is a Definition, whether its body is one step or
many.** The exception is an action the rules define as another action under another verb. It is a
Template over that action and adds no Deed of its own, because printed wording lives on Templates,
not on model nodes or deed stamps (`convert`, below). Planechase and Archenemy actions are
permanently out of scope; Un-set Contraption and Attraction actions are deferred (`CLAUDE.md`,
"Scope").

- **Single-step:** `activate` [CR#701.2], `attach` [CR#701.3], `cast` [CR#701.5], `counter`
  [CR#701.6], `create` [CR#701.7], `destroy` [CR#701.8], `discard` [CR#701.9], `exchange`
  [CR#701.12], `exile` [CR#701.13], `fight` [CR#701.14], `goad` [CR#701.15], `regenerate`
  [CR#701.19], `reveal` [CR#701.20], `sacrifice` [CR#701.21], `search` [CR#701.23], `shuffle`
  [CR#701.24], `tap` [CR#701.26a], `untap` [CR#701.26b], `transform` [CR#701.27], `fateseal`
  [CR#701.29], `clash` [CR#701.30], `proliferate` [CR#701.34], `detain` [CR#701.35], `populate`
  [CR#701.36], `monstrosity` [CR#701.37], `vote` [CR#701.38], `meld` [CR#701.42], `exert`
  [CR#701.43], `cloak` [CR#701.58], `suspect` [CR#701.60]. (`plot` is a keyword ability, not a
  keyword action [CR#702.170a].) `convert` [CR#701.28] is a Template over `transform`: converting
  follows the transform rules, so a "transforms into" trigger sees it and "can't transform" forbids
  it [CR#701.27e,701.28a,701.28f].
- **Regeneration:** a regeneration shield's application (removing damage, tapping, removing from
  combat) is the one `regeneration` Definition ([CR#701.19a]). The deed "can't be regenerated"
  forbids it, and the shield `regenerate` installs applies it ([CR#701.19c]). It carries the
  keyword action's Deed, so it is no Template.
- **Multi-step:** `doublePower` / `doubleToughness` / `doublePT` / `doubleLifeTotal` /
  `doubleCountersOn` [CR#701.10], `triplePower` / `tripleToughness` / `triplePT` [CR#701.11],
  `investigate` [CR#701.16], `mill` [CR#701.17], `play` [CR#701.18], `scry` [CR#701.22], `surveil`
  [CR#701.25], `bolster` [CR#701.39], `manifest` [CR#701.40], `support` [CR#701.41], `explore`
  [CR#701.44], `amass` [CR#701.47], `learn` [CR#701.48], `connive` [CR#701.50], `incubate`
  [CR#701.53], `theRingTemptsYou` [CR#701.54], `faceVillainousChoice` [CR#701.55], `timeTravel`
  [CR#701.56], `discover` [CR#701.57], `collectEvidence` [CR#701.59], `forage` [CR#701.61],
  `manifestDread` [CR#701.62], `endure` [CR#701.63], `airbend` [CR#701.65], `earthbend` [CR#701.66],
  `waterbend` [CR#701.67]. (`firebending` is a keyword ability, not a keyword action
  [CR#702.189a].)

**The Deed.** Every keyword-action Definition's body carries its action's Deed ([CR#701.22b]), and
that is its one Deed: no Definition wraps its body in a second Deed-carrying node.

- The checker reads the Deed as the head gates and lookbacks do. With no Handoff to a Permanent, a
  deed a permanent performs is performed by the Source permanent ([CR#701.14a,701.44a]). After the
  event, the body's patients receive the Deed's stamp, and what the body moves carries it, so an
  exile's move is exiling ([CR#701.13a]).
- A keyword ability's verb that is done by doing another deed is a Definition the same way, over
  that deed (`devour` over `sacrifice` [CR#702.82b]).
- The checker never guesses a deed from a body's shape.
- In imperative position (`scry(2)`), a multi-step Definition is read as its body under its Deed,
  and lowering keeps it as one composite action named by the Deed ([CR#701.22b]). Inside `does`,
  `would`, `cant` or `did` (`does(you, scry(any))`, `does(you, forage)`), it is matched by its Deed
  and its caller-read arguments.

Monstrosity and adapt are stated over "this permanent" and say nothing of who performs them
([CR#701.37a,701.46a]). Their bodies name the Source (`thisPermanent`); the ability's controller
follows them as it follows any instruction ([CR#608.2c,701.37c]); and no Handoff renames "this
permanent". Their Definitions carry no Deed marked as performed by a permanent.

### 3.2 Templates That Mention a Parameter Twice

A Template is transparent, but a Template that mentions a NounPhrase parameter more than once before
a zone change binds it at its first occurrence and reads it back at the later ones (generated, never
hand-written), so targets and indefinites are not duplicated ([CR#115.1,608.2c]). Examples:
`duringNextTurnOf`, `discardThenForEachNonland` (or reworded through "discarded this way",
[CR#701.50a]), `lookAndSortSlice`, `controllerSacrifices`, `comparesOwnStat`.

---

## 4. The Action-Clause Spine

### 4.1 Every Verb Is Defined Once, as an Action

**Every game action or verb phrase is defined once, as an Action: the bare infinitive verb phrase.**
Examples: `sacrifice(a(creature))`, `destroy(this)`, `attack()`, `block(this)`, `draw(2)`,
`dealDamage(3, anyTarget)`, `putCounters(1, p1p1, this)`, `move(this, battlefield, none)`.

All seven clause contexts embed the Action directly, with no double wrapping
(`replace(would(subj, action), …)`, not `replace(does(would(…)))`). The examples below abbreviate:
they write an active subject as a bare noun phrase, where `.syl` writes `actor(np)` (§5.1, no
coercions).

| Clause context | Form | Active (`subj = np`) | Passive, unspecified agent (`subj = any`) | Passive, explicit agent (`subj = by(np)`) |
|---|---|---|---|---|
| **1. Second-person imperative** | `action` | "Sacrifice a creature." → `sacrifice(a(creature))` | — | — |
| **2. Third-person imperative (Handoff)** | `instruct(subj, action)` | "Target opponent sacrifices a creature." → `instruct(target(opponent), sacrifice(a(creature)))`; "That creature explores." → `instruct(that(creature), explore)` | — | — |
| **3. Action cost** | `perform(action)` | "Sacrifice a creature: Draw a card." → `activated(perform(sacrifice(a(creature))), draw(a(card)))` | — | — |
| **4. Trigger event** | `does(subj, action)` | "Whenever you sacrifice a creature" → `does(you, sacrifice(a(creature)))` | "Whenever this creature becomes blocked" → `does(any, becomeBlocked(this))`, the Blocked status event [CR#509.1h,509.3c] | "Whenever this creature becomes blocked by a creature" → `does(by(a(creature)), block(this))` |
| **5. Replacement** | `replace(would(subj, action), repl, …)`; `repl` is `instead(a)` (`a` in the event's place), `before(a, body)` (`a` done before the event, which still happens, modified by `body` if one is written: the choices a replacement of an entering requires [CR#614.12a]), `alongside(a)` (`a` done as part of the event, which still happens: "enters with" and "as it enters" [CR#603.6d]), or `withRiders(riders)` (the entering happens with those Entry Riders); `doIf(cond, repl)` builds `onlyIf(cond, repl)`, which applies only while `cond` holds | "If you would draw a card" → `replace(would(you, draw(a(card))), …)` | "If this creature would be destroyed" → `replace(would(any, destroy(this)), …)` | "If this creature would become blocked by a creature" → `replace(would(by(a(creature)), block(this)), …)` |
| **6. Deontic** | `cant(subj, action)`, `must(subj, action)`, `can(subj, action)`, `gatedBy(cost, subj, action)` | "This creature can't block." → `cant(this, block(any))` | "This creature can't be blocked." → `cant(any, block(this))` | "This creature can't be blocked by Goblins." → `cant(by(a(goblin)), block(this))` |
| **7. Lookback** | `did(subj, action, within)` | "if you sacrificed a creature this turn" → `did(you, sacrifice(a(creature)), thisTurn)` | "if this creature was blocked this turn" → `did(any, becomeBlocked(this), thisTurn)` | "if this creature was blocked by a creature this turn" → `did(by(a(creature)), block(this), thisTurn)` |

**Contexts 4 to 7 are read through one shared spine:** the head gate (§4.6, shared by events,
lookbacks and deontic rules, which match a Definition by its Deed); then the subject, before the
action; the Actor's kind as the action needs it (refused once, by the head itself where the head
checks its own Actor, as a draw does [CR#121.1]); the action, as a pattern in its phase (§4.5); and
then a passive's agent. A context adds only what is its own: a lookback's history, grounding and
range (§4.8); a deontic rule's modifiers, cost and box (§2.2 box 3); an `asThough` condition. Only an
event clause checks its voice and a procedure's number.

### 4.2 Voice and Grammatical Roles

**The Subject of `does`, `would`, `cant`, `must`, `can`, `gatedBy` and `did` has three forms**:
active (`actor(np)`, `np` performs the action), unspecified-agent passive (`any`), and
explicit-agent passive (`by(np)`, `np` is the oblique agent). They assign roles as follows:

1. **Active (`actor(np)`):** `np` is read first in the clause, with role subject, and is the
   innermost Actor while the action is read. The action's primary patient takes role object. Any
   other noun phrases in the action (destinations, instruments, secondary targets) take role other.
2. **Unspecified-agent passive (`any`):** introduces no agent referent. The action's primary patient
   is promoted to role subject, matching the English surface, where the patient is the grammatical
   subject ("Whenever a creature is destroyed, it …").
3. **Explicit-agent passive (`by(np)`):** the action's primary patient is promoted to role subject
   and read **first**, matching English word order ("Whenever this creature becomes blocked by a
   creature …"). `np` is read second, with role other. So a later bare "it" in the same or the next
   clause resolves by D′ (subject over object over other) to the passive subject ("this creature")
   when both match in kind and zone, while "that creature" resolves by AC>R1 (recency) to the agent
   ("a creature").

### 4.3 One Wildcard Word, `any`

**There is one wildcard word, `any`, at every sort that has a wildcard,** instead of a separate
wildcard word per sort. The sorts are: Subject (the passive's unspecified agent), NounPhrase (an
unspecified target, object or defender: `cant(this, block(any))` for "This creature can't block"),
Amount (`cant(any, putCounters(any, any, this))` for "can't have counters put on it"), the counter
kind, die sides and roll result, zone, and Timing.

### 4.4 Transitive and Intransitive Actions, and Relative Clauses

- **Intransitive or Actor-only actions** (`attack`, `loseTheGame`, `commitCrime`) take no patient
  noun phrase, or only an optional defender or companion. The Actor comes from the enclosing
  clause's Subject, or is implicitly "you" or the innermost Actor in an imperative.
- **Transitive actions** always keep their primary patient inside the Action. Examples:
  `exile(patient)`, `destroy(patient)`, `sacrifice(patient)`, `tap(patient)`, `untap(patient)`,
  `discard(patient)`, `block(attacker)`, `target(objectOrPlayer)`, `move(patient, from, to, obs)`,
  `dealDamage(amount, recipient, kind)`, `putCounters(count, kind, target)`,
  `removeCounters(quantity, kind, target)`. The Subject of `does`, `would`, `cant`, `did` or
  `instruct` supplies the performer (`actor(np)`), an unspecified passive agent (`any`), or an
  oblique agent (`by(np)`).
- **Relative clauses use `which`.** A relative clause that filters a noun by an event (a relative
  lookback, `did`) uses the relative pronoun `which` directly inside `did(subj, action, within)`,
  in whatever argument slot is relativized:

| English noun phrase | Encoding |
|---|---|
| "all creatures that attacked this turn" | `allOf(and([creature, did(which, attack, thisTurn)]))` |
| "each card exiled this way" | `allOf(and([card, did(any, exile(which), thisWay)]))` |
| "a card you discarded this turn" | `a(and([card, did(you, discard(which), thisTurn)]))` |
| "a creature dealt damage by this creature this turn" | `a(and([creature, did(by(this), dealDamage(any, which), thisTurn)]))` |

A relative lookback opens a local relative-clause box (§2.2 box 5). `which` is bound there to a
temporary referent seeded from the outer Type Evidence: a player when filtering a player
description, or an object with the Card Types and expected zone gathered so far when filtering an
object description. The lookback is read in that box. `which` must appear at least once and pass J1
for its verb slot. Local indefinite participants and the temporary `which` are discarded on exit.
Definite presuppositional references (an Attachment Host: "enchanted", "equipped", "fortified",
[CR#301.5,301.6,303.4b]) project to the enclosing ability's box with their identity and zone
preserved, without a per-referent Attachment Host mark.

**Same-Is-True-For Abilities.** "… The same is true for first strike, double strike, …" is written
once, as `sameIsTrueFor(frame, keywords)`. The frame contains the placeholder word `thatKeyword`,
analogous to `which`. It is not expanded per keyword, because `.syl` files carry no evaluation.

- The checker checks a static or triggered frame once per keyword, with `thatKeyword` bound to that
  keyword. It refuses any other frame, and an empty keyword list.
- `thatKeyword` outside a frame is refused.
- Lowering distributes the frame over the keywords.

### 4.5 Before the Event (`would`) and After It (`does`, `did`, Imperatives)

**`would` reads its Action before the event; imperatives, `instruct`, `does` and `did` read it
after.** All four wrap the same Action, so the checker reads it in one of two phases:

- **Before the event (`would`, [CR#614.1]):**
  - The action's subject and object referents are introduced in their zone and status before the
    action. In "If a creature would die, exile it instead", the dying creature is still on the
    battlefield, so `exile(it)` can exile it from there ([CR#614.6]).
  - Prospective magnitudes are introduced ("that much" damage; "that many" counters, tokens or
    cards), so a replacement can read them ("prevent that much damage", "put twice that many
    counters on it instead").
  - Products of completion are **not** introduced: `would(you, draw(a(card)))` introduces no card
    drawn into a hand.
- **After the event (imperatives, `instruct`, `does`, `did`):**
  - Patient referents move to their zone and status after the action, are stamped with the deed,
    and are replaced by new referents where [CR#400.7] applies.
  - Products of completion are introduced: cards drawn, tokens created, the mana produced, a roll
    result.

### 4.6 Events

**An event is `does(subj, action)` over an event-capable Action head, or one of a small set of
structural events.** The event-capable heads, and what each pattern watches:

| Event | Encoding | Ruling |
|---|---|---|
| A keyword action or keyword ability's deed (`tap`, `untap`, `destroy`, `sacrifice`, `exile`, `discard`, `mill`, `scry`, `surveil`, `explore`, `connive`, `forage`, `bolster`, `adapt`, `amass`, `populate`, `proliferate`, `regenerate`, `reveal`, `search`, `shuffle`, `vote`, …; `crew`, `saddle`, `mutate`, `devour`) | `does(subj, action)` | Each deed is named by its Definition's Deed, or by its own head (`crew`, `saddle`). "Transforms into" writes the result face on `turnOver`'s `into` [CR#701.27e]. A shuffle names its library itself. |
| Casting | `does(caster, cast(spell, from))` | `cast` is a Definition over the cast procedure; `from` is the written origin. |
| Activating | `does(actor, activate(ability, of))` | The `activate` Definition, over the activate procedure. |
| Tapping for mana | `does(by, tapForMana(permanent))` | `subj` is `any` when no tapper is written, `by(p)` when one is. |
| Paying a cost | `does(payer, pay(cost, outcome))` | `pay` carries whether the cost was paid (paid, the default, or unpaid). |
| Paying life | `does(player, payLife(amount))` | `payLife` is a Template, `pay(perform(loseLife(amount)), once)`: a cost payment that subtracts life [CR#119.4], so a player who can't lose life can't pay it [CR#119.8]. There is no life Cost. In a cost slot `payLife` stands for `perform(payLife(amount))`, and `pay` of a cost that is itself one such payment pays it once. |
| Attacking with creatures | `does(player, attackWith(attackers, defender))` | The player-subject combat action ("Whenever you attack with one or more creatures"). |
| Combat | `does(attacker, attack(defender))`, `does(blocker, block(attacker))`, `does(attacker, attackUnblocked(defender))` | Active (`does(b, block(a))`); status (`does(any, becomeBlocked(a))`, "becomes blocked": the Blocked status, which an effect can also give [CR#509.1h,509.3c]); or `by`-passive (`does(by(b), block(a))`, "becomes blocked by B" [CR#509.3d]). `attackUnblocked` [CR#509.1h]. |
| Becoming a target | `does(by, target(subject))` | `target(subject)` [CR#115.1] serves both "Whenever ~ becomes the target of …" (`does(by(src), target(this))`) and shroud and hexproof (`cant(by(src), target(this))`). |
| Other heads | `does(player, draw(cards))`; `does(searcher, search(libraryOf(owner), any))`; `does(creator, create(tokens))`; `does(by, setStatus(edge, subject, status))` ("becomes tapped", "is turned face up", "phases out"); `does(any, attach(attachment, object, dir))` (`attach`, `unattach`); `does(flipper, flipCoin(call, outcome))`; `does(actor, commitCrime)` [CR#700.13]; `does(player, loseGame)`; `does(player, changeLife(direction, amount))` | `loseGame` is a Template over the game-concluding action; `gainLife` and `loseLife` are Templates over `changeLife`. |
| A Zone Change | `does(any, move(subject, from, to, at))` | The origin is anywhere, listed zones, or anywhere but listed zones; the Observation Point is arrival for "enters" and departure for "dies" and "leaves" [CR#700.4,603.6a,603.6c,603.10a]. A move carries no cause of its own: a caused move is written inside `causes` or `causedBy`. `die(subject)` [CR#700.4], `enter(subject)` and `leave(subject)` are Templates over `move`, so triggers (`does(any, die(a(creature)))`) and lookbacks (`did(any, die(a(creature)), thisTurn)`) share the one move. |
| Damage | `does(source, dealDamage(amount, recipient, kind))` | `kind` is the explicit qualifier: combat damage [CR#120.2a], noncombat damage [CR#120.2b], or none written. Unqualified "damage" in an instruction is the effect's own (noncombat) damage [CR#120.2b], and in an event, replacement, deontic rule or lookback matches all damage. The prevention Templates carry the same slot after their required arguments. A prevention, redirection or scaling replacement writes it on the `dealDamage` of the event it replaces, and the `dealDamage` of a replacement's `instead` keeps that event's kind ([CR#614.6]). Active, passive ("is dealt damage") or `by`-passive. |
| Counters | `countersPut(kind, amount, subject)` and `countersRemoved(kind, quantity, subject)`, Templates over `does(any, putCounters(…))` and `does(any, removeCounters(…))` | No Delta: the direction is which action the event names. There is no batch field: the amount or quantity the action already carries is what the pattern watches. "A counter is put" is `1`, matched by each counter, since one event can contain several occurrences ([CR#603.2c]); "one or more counters are put" is `anyAmount`, matched by the event ([CR#714.2b]). A removal reads its Quantity, and "the last counter" is `theLast` ([CR#310.12b,702.62a]). "The Nth counter is put" is `theNth`, the put that crosses N ([CR#122.7]), not an Event Ordinal. Only a plural amount or quantity publishes the count a `would` clause's "that many" reads. `moveCounters` stays distinct, for moving counters between two objects ([CR#122.5]). |
| Gaining control | `does(player, gainControlOf(object, from))` | With the player control is taken from, when written ("from you", [CR#603.10d]). It is the head of event, lookback and deontic patterns only, as `anyCost` and `anyBallot` are (§4.7). The imperative "gain control of" is one tree, `establish(controlGrant(player, object), duration)`, the layer-2 effect ([CR#611.2a,613.1b]), and the pattern stamps its patient as that effect does. |
| Rolling dice | `does(roller, rollDice(count, sides, watch))` | As for counters, `count` is what the pattern watches: "roll a die" is `1`, matched by each die ([CR#603.2c]), and "roll one or more dice" is `anyAmount`, matched by the roll. |
| A triggered ability triggering | `does(ability, trigger)` | The automatic triggering occurrence [CR#603.2]. |
| Crewing or saddling | `does(creature, crew(vehicle))`, `does(creature, saddle(permanent))` | Taking part in the cost [CR#702.122b,702.122d,702.171c]; the ability's activation and resolution keep their own events. |
| Fully unlocking a Room | `does(player, fullyUnlock(room))` | The Designation transition of [CR#709.5i]. |

**The event head gate.** The engine's core compiles events, replacements and lookbacks to a closed
set of event filters. So `does`, `would` and `did` admit only the event-capable heads above, plus a
keyword-action Definition matched by its Deed. Control-flow and compound instructions
(`sequentially`, `doIf`, `doForEach`, `chooseModes`, `establish`, …) are refused.

**Structural events:**

- `beginningOf(part, whose, which)` and `endOf(part, whose)`;
- `now`, the immediate event for a self-replacement ("As this enters …");
- `stateHolds(condition)`, a state trigger ("When this has no counters on it …", "when its power
  becomes 20" as a comparison) [CR#603.8];
- `gameBecomes(designation)`, "Whenever it becomes day / night" [CR#731.1];
- `chapterMark(chapters)`, Saga chapter triggers [CR#714.2];
- `triggers(ability, of, by)`, "Whenever a triggered ability of … triggers";
- `nthOccurrence(ordinal, per, event)`, "Whenever you cast your second spell each turn";
- `causes(cause, event)`, "Whenever an ability you control causes you to …";
- `stateBasedActionsChecked`, the state-based check boundary for lookbacks [CR#704.3,704.5h].

### 4.7 Static Specs: Deontic Rules, Replacements, Retention and Damage

**A deontic rule is `cant`, `must`, `can` or `gatedBy` over a Subject, an Action and optional
modifiers.** Each Template takes each modifier as an optional argument:

- `timing`: when or how often it applies ("each turn", "during each of your turns");
- `asThough`: "as though it had flash", "as though it didn't have defender";
- `except`: a Deontic Exception, a keyword or the objects exempted
  [CR#702.13b,702.111b,702.145b];
- `cause` (`cant` only): the cause the prohibited event must have [CR#120.3a];
- `rider` (`can` only): a Permission Rider, the rest of the permission's ability [CR#113.2c].

Rulings:

- **Compound deontics.** "Can't attack or block" is a conjunction of prohibitions,
  `conjunction([cant(this, attack(any)), cant(this, block(any))])`. There is no list-of-deeds
  wrapper.
- **The deontic gate is the event gate (§4.6).** Deontic rules have no head gate of their own. A
  "can't" effect takes precedence over an effect that allows or directs the thing [CR#101.2], so a
  prohibition is gated like an event and admits every action the checker knows as an event:
  `gainDesignation` ("can't become the monarch" [CR#725.1]), `preventDamage` and `redirectDamage`
  ("can't be prevented", "can't be dealt instead" [CR#615.12,614.9]), any `setStatus`, and any
  `move` ("can't be returned to its owner's hand"; `leave`, `putInto` and the `returnTo…` words
  are Templates over the move). Compound and control-flow actions are refused here as there. An
  action Definition is admitted by its Deed, whatever the Deed.
- **Permissions and wildcards.** Copy and vote permissions are required by *Display of Power* and
  by additional-vote clauses. Payment permissions use `pay(anyCost)` with the spend-as-though
  modifier. `anyCost` is a wildcard only in event, lookback, prospective and deontic action
  patterns; a performed cost must be specified. Likewise `anyBallot` is an action-pattern wildcard
  for additional-vote permissions; a performed vote requires actual labels or a candidate, and an
  empty label list is invalid. `gainControlOf` is a pattern head only ("other players can't gain
  control of them"); a performed control change is `establish(controlGrant(…))` ([CR#613.1b]).
  After reading a pattern, the checker returns to the caller's reading mode for the effects that
  follow.
- **Distinct occurrences stay distinct:** triggering [CR#603.2], crewing and saddling
  [CR#702.122b,702.171c], the phase-in status [CR#702.26c], and fully unlocking [CR#709.5i].

| Printed | Encoding |
|---|---|
| "This creature can't block." | `cant(this, block(any))` |
| "This permanent can't be sacrificed." | `cant(any, sacrifice(this))` |
| "doesn't untap" | `cant(any, untap(this))`: the CR leaves its classification open, and `cant` is the engine's choice (UD-14 in `docs/engine-adrs.md`) |
| "This creature can't be blocked by Goblins." | `cant(by(a(goblin)), block(this))` |
| "can't have counters put on it" | `cant(any, putCounters(any, any, s))` |
| "can't be the target of spells or abilities your opponents control" (shroud, hexproof, protection) | `cant(by(src), target(s))` |
| "attacks each combat if able" | `must(this, attack(any))` |
| "All creatures able to block this creature do so." | `must(by(allOf(creature)), block(this))` |
| A permission with a rider | `can(you, play(it), rider = …)` |
| *Ghostly Prison* | `gatedBy(mana([2]), any, attack(you))` |
| A replacement | `replace(would(any, destroy(this)), instead(regeneration(this)))`, with `wouldAny([would(…), …])` for a disjunction of replaced events; `repl` is one of `instead(a)`, `before(a, body)`, `alongside(a)`, `withRiders(riders)`, `onlyIf(cond, repl)` |

**Causal replacements.** `causedBy(cause, clause)` wraps a prospective action clause with the
source, event or effect that causes it [CR#700.1,614.1a]. "If an effect would create … / put …
counters" is `causedBy(anEffect, …)` [CR#614.16]. Whether an imperative, a cost or a turn-based
action is an effect follows from the node that encloses it, so a Saga's turn-based lore counter is
no effect [CR#703.1,714.3c], and no Action carries an effect flag. A performer and a cause are
independent: *Wilt-Leaf Liege*'s "If a spell or ability an opponent controls causes you to discard
this card" has both. The cause is read before the nested clause. *Rain of Gore* keeps both its
spell-or-ability source and that source's controller as the prospective life-gain subject; its
official ruling excluding life gain from combat damage with lifelink shows why a bare life-gain
replacement would lose that restriction.

**Retention: "This effect doesn't remove …".** `retention(spec, subject)` exempts something from one
state-based consequence of `spec`, a clause that removes Attachments already on its host. That
clause is either a protection grant, whose Auras with the quality go to the graveyard and whose
Equipment and Fortifications become unattached ([CR#702.16c,702.16d,704.5m,704.5n]), or a
prohibition on attaching ("can't be enchanted"), which leaves an Aura already attached enchanting an
illegal object ([CR#303.4c,704.5m]).

- It has the same shape as the zero-life exemption: a carve-out limited to one State-Based Cause.
  Applying it is the engine's.
- It wraps the one clause it limits, never a conjunction: *Guardian Beast*'s exemption belongs to
  its "can't be enchanted" clause, not to "have indestructible" or "can't gain control of".
- `subject` is "this Aura" (or "this Equipment", "this Fortification") on the clause's host, or a
  description of Attachments whose "attached to …" reads that host back by an anaphor (*Benevolent
  Blessing*'s "attached to it"). The checker refuses any other clause or subject.
- For the engine: the exemption covers this effect only, so another protection from the same quality
  still removes the Aura ([CR#702.16n,702.16p]). "Already attached to" is read once, as the effect
  starts to apply. An Attachment that arrives later is not exempt, and the clause still forbids it to
  attach ([CR#702.16p]).
- The term carries no onset marker. *Black Ward*'s "this Aura" and *Benevolent Blessing*'s "already
  attached to it" are told apart by the `attachedTo` the latter's subject carries.

**Stats.**

- `Stat` includes `defense` ([CR#310.4,310.4a], carried in the `battle` card type's stats). `Stat`
  and the characteristic stat stay distinct ([CR#109.3]).
- [CR#310.4b] is a conferral on the `battle` card-type definition: it enters with defense counters
  equal to `statOf(stat(defense), this)`.
- `statOf(stat(loyalty|defense), …)` reads by zone: counters on the battlefield, the printed number
  elsewhere ([CR#306.5a,306.5c,310.4a,310.4c]). So the planeswalker and battle entry conferrals
  ([CR#306.5b,310.4b]) read `statOf(stat(loyalty|defense), this)` (§1 item 5(b)). There is no
  printed-number projection (§1 item 5(c)).
- The marked-damage projection reads the damage marked on an object ([CR#120.3e,120.6,704.5g]).
- `stateBasedActionsChecked` ([CR#704.3]) bounds the deathtouch state-based lookback
  ([CR#704.5h]): `did(by(a(withKeyword(deathtouch))), dealDamage(any, this),
  between(stateBasedActionsChecked, now))`.

**Damage is composed, not special.** There is no damage-only Static Spec.

- The results of damage are rules conferred as properties, each an `asEvent` over the damage dealt
  ([CR#120.3a,120.3c,120.3e,120.3h,120.4c]). Toxic is one such row, reading the total toxic value
  ([CR#702.164b,702.164c]). Lifelink's additional result is its keyword's body ([CR#702.15b]).
- Wither and infect replace the marking and the life-loss results, so a source with both puts
  counters once ([CR#120.3d,614.6,702.80a,702.90b,702.90c]).
- Prevention, redirection and scaling replace the damage event:
  `replace(would(src, dealDamage(…)), instead(preventDamage(cut) | redirectDamage(cut, to) |
  instruct(it, dealDamage(…))))` ([CR#615.1,614.9]; scaling [CR#614.1a,614.6]; doubling
  [CR#701.10g]).
- "Can't be prevented" is `cant(any, preventDamage(all, damage = …))` ([CR#615.12]).
- The damage-marked outcome ([CR#120.3d]) is what `markDamage` publishes.

### 4.8 Lookback (`did` and `which`)

**A lookback is `did(subj, action, within)`: the same Action as everywhere else, looked at in the
past.** A lookback clause has a Subject, an Action, an optional `within` (a Duration with an
optional Timing, `thisWay`, `triggering`, or `thisObject`), and an optional Event Ordinal ("the
first spell you've cast this game", the ordinal `nthOccurrence` carries on events and `would`
clauses). The ordinal is what makes a "the"-headed lookback unique.

- The Templates mirror `does` and `would`, subject first: `did(subj, action, within)` and its
  agentless passive `was(action, within)` (subject `any`). Each is dispatched on the expected sort
  to a Predicate or a Condition. A bare Duration coerces to its range.
- A Condition lookback and a tally always write `within`. Only a Predicate lookback may leave it
  unwritten, and the checker refuses that anywhere but a Participial Description (below).
- **`thisWay` and `triggering` must be grounded.** `thisWay` reads back an event that the earlier
  text of its own spell or ability caused, its costs included ([CR#608.2c]); `triggering` reads
  an event its ability triggers on ([CR#603.2]). The checker refuses either with no such event to
  read. The grounding is textual: an optional action a player may decline, a branch not taken, or an
  act a permission allows ("You may play those cards this turn. If you cast a spell this way, …")
  still grounds it. Whether the event happened is the engine's (§1 item 4).
- **`thisObject`** reads the described object's own casting or activation in its current existence
  ([CR#400.7]): "artifact spells you cast", "if you cast it from your hand" and "spend this mana
  only to cast a spell from your graveyard" are `did(…, cast(which …), thisObject)`, and "abilities
  you activate" is `did(…, activate(which), thisObject)`. A cast rank is the clause's ordinal. A
  history window ("a spell you cast this turn") keeps its Duration.

The three lookback sites:

1. **Condition:** "if you sacrificed a creature this turn" → `did(you, sacrifice(a(creature)),
   thisTurn)`; "if a creature died this turn" → `did(any, die(a(creature)), thisTurn)`, where the
   `die` Template [CR#700.4] is the move from the battlefield to a graveyard, observed at
   departure, the same move as `does(any, die(a(creature)))` in §4.6.
2. **Predicate,** with `which` in the relativized slot: "creatures that attacked this turn" →
   `and([creature, did(which, attack(any), thisTurn)])`; "each card exiled this way" →
   `and([card, was(exile(which), thisWay)])`.
3. **Amount (`tally`):** counting events ("the number of times …": `tally(did(you,
   cast(yourCommander), thisGame), events)`) or quantity ("the number of cards …", "the amount of
   life …": `tally(did(you, draw(a(card)), thisTurn), quantity)`).

**Participial Descriptions.** A description whose determiner is "the" (number one, or many for "the
exiled cards"), "all", "each" or the bare plural, and whose predicate has a relative lookback
conjunct with `thisWay` or nothing written, with `which` relativizing what the deed acted on,
introduces no referent. It resolves to the one referent in scope that deed stamped, filtered by the
other conjuncts' Head Noun as the object was when acted on ("card", a battlefield location as
"permanent", Card Types [CR#608.2h]). Otherwise it is refused.

- `thisWay` reads only referents the current ability stamped, since "this way" is no linking phrase
  [CR#607.1,607.2a]. That includes referents stamped inside an optional body or arm
  (`withContinuation`), which escape it to Participial Descriptions only, never to pronouns: "this
  way" ranges over the events that happened.
- The exception is a target the optional body announced. A target written in an optional body is
  chosen as the spell or ability is put on the stack, whether or not the player acts
  [CR#601.2c,603.3d,115.1]. So it is an ordinary referent after the body and in its "if you don't"
  arm, read as announced, while the body's stamp on it stays participial-only. Inside a per-player
  loop it is a loop-local the loop abstracts (§2.2).
- A target written in either arm escapes the same way, since the arms are part of the same ability.
  It is an ordinary referent after the whole `may`, whichever arm runs, and in the other arm
  ("Otherwise, that creature"), under the same identity, while the arm's stamp on it stays
  participial-only. An arm's non-target referents stay its own.
- A "when you do" reflexive trigger is a separate ability that chooses its own targets
  [CR#603.12,603.3d]; its targets do not escape.
- With nothing written, a Participial Description also reads what a linked ability published
  [CR#607.2a], where a singular "the" reads a published group [CR#607.3].
- "It", "them" and "that card" are ordinary pronouns.
- Linked and self-linked abilities share one authored syntax tree (one tree per printed wording).
  Lowering finds the link from the same resolution, and game-time zone filtering (a card leaving
  exile before champion's return resolves) belongs to the lowering and core rewrite [CR#607.2a].

---

## 5. The `.syl` Data Language

### 5.1 Files and Syntax

**Declarations and cards are standalone `.syl` files (`plugins_v3/**/*.syl`), and each file's text
is its own first-order syntax tree.** The Rust pipeline (`semantics-rust-rewrite`: `semantics`,
`discourse`, `lowering`) reads `.syl` natively: `deckmaste_dsl` parses it into a syntax tree that
`semantics` consumes. For Lean verification, a generator emits Lean modules from the same files;
no Lean elaborator hooks or authoring contexts remain.

- **Declaration kinds:** `primitive`, `action`, `ability`, `ability_word`, `counter`,
  `designation`, `subtype`, `card_type`, `relation`, `token`, `sba`, `conferral`, `template`,
  `card`.
- **No evaluation and no local bindings.** Every declaration body is a first-order tree of
  constructor and declaration calls. There are no higher-order parameters, functions, `match`, `if`,
  list or option helpers, list operators, local bindings, tuples, anonymous constructor brackets or
  polymorphic sort binders, and no bottom-up type inference.
- **Expression grammar (six productions):**
  1. **Literals:** string `"..."`, integer, `true`/`false`.
  2. **Lists (`[...]`):** valid only at an expected sort `List T` or `Option (List T)` (the latter
     unwraps like any `Option U`; `some` is never written). A list is never coerced into a domain
     wrapper: `sequentially`, `and`, `conjunction`, `compound` and `mana` are written explicitly.
  3. **Record blocks:** `{ field = val, ... }`.
  4. **Calls:** a head applied to positional and named arguments.
  5. **Bare words:** identifiers with no arguments, never with a leading `.` (the one leading dot in
     `.syl` names a primitive's constructor, §5.2 item 1). A bare word never names a parameter.
  6. **Parameter references:** `$name`, a read of the value of a parameter of the enclosing
     declaration (§5.2 item 2, step 1).
- **Concrete syntax:**
  - **Calls:** `head(arg, arg, name = arg)`, with explicit parentheses and commas. Nested calls need
    no extra parentheses. A bare word is a call with no arguments; `head()` is not written. A named
    argument may stand anywhere among the positional ones, for readability only, so a call can
    follow its text's printed order (`when(does(actor(thisAura), enter), if = is(it,
    in(battlefield)), sequentially([...]))`). Positional arguments bind in written order, each to
    the next parameter not yet bound; a parameter bound twice, by name or by position, is an error.
    Argument order carries no meaning: a Template's parameter order is fixed by its spelling, and
    arguments apply in parameter order whatever their written order.
  - **Parameter references:** `$` means "the value of", so parameters stand out and what a word
    refers to can be read from its spelling (`exileWithCounters(thisCard, $amount, timeCounter)`).
    Only reads carry `$`: parameter declarations in headers (`ability suspend(amount: Amount, cost:
    Cost)`), argument labels (`play(thisCard, payment = withoutPaying)`) and record field names stay
    plain. The part after `$` follows the word rule, with no space between them; `$name` takes no
    arguments, so `$name(...)`, a lone `$` and `$ name` are parse errors. A parameter default may
    read only the parameters before it. Holes in `spelling` templates (`"suspend
    <amount>—<cost>"`) are strings, not parameter references.
  - **One token per meaning:** `=` binds (record fields `{ body = x }`, named arguments `from = x`,
    parameter defaults `p: Sort = x`); `:` means only "has sort", and appears only in declaration
    headers. Lists are `[a, b]`; trailing commas are allowed; whitespace and newlines are otherwise
    insignificant.
  - **Headers:** `kind name(param: Sort, param: Sort = default) { field = val, ... }`. Kinds without
    parameters omit the parentheses (`counter aegisCounter { ... }`, `card { ... }`). A kind that
    fixes its result sort writes none: `action` is Action and `ability` is Ability. `action` and
    `ability` declare a Keyword Action [CR#701.1] and a Keyword Ability [CR#702.1]; ordinary Actions
    and Abilities are built by Templates. Only `primitive` and `template` write a result sort, after
    the parameters: `template duringNextTurnOf(player: NounPhrase): Duration { ... }`, `primitive
    doForEachKind(axis: KindAxis, ...): Action { ... }`, and `template giftACard: Ability { ... }`
    when parameterless. The pair `(name, sort)` is the dispatch key of §5.2 item 2. There is no
    `->`.
  - **Sections:** `lexical`, `spelling`, `body`, `confers` and `rules` are ordinary record fields of
    the declaration block.
  - **Sorts in headers** are written like calls: `Option(NounPhrase)`, `List(Ability)`,
    `Option(List(Cost))`.
  - **Kind words** (`primitive`, `template`, `action`, `ability`, `card`, `counter`, `token`, …) are
    keywords only as the first word of a declaration; elsewhere they are ordinary identifiers
    (`token` and `card` are also Template names).
  - **Literals:** negative integers are written `-1`; strings are double-quoted and escape only `\"`
    and `\\`.
  - **Plain words:** `in`, `from`, `by`, `or`, `this`, `exists`, `where` and `variable` are written
    as themselves. `none`, `true` and `false` are reserved: they resolve before enum words, and no
    enum word may use them.
  - **Comments:** `#` to end of line, with no block form. A doc comment is a leading `#` block. CR
    citations in comments keep the bracketed form (`[CR#702.9b]`), and `cargo xtask cite` scans
    `.syl` comments as it scans `.rs` ones.
- **No coercions.** The direct sort atoms of §5.2 item 2 (integers, color words, `ability` and
  `action` names) are the only implicit resolutions. There is no other implicit conversion between
  sorts: every wrapper call is written explicitly, a Subject's `actor(…)` included. Uglier but
  simpler is the point; coercions may be reintroduced deliberately once the language is simple
  enough to refactor.
- **One block per declaration, three layers:**
  1. the lexical catalog fields, inside `lexical` (`lemma`, `plural`, `parameters`, `verbs`,
     `nouns`, `adjectives`, `determinatives`, `bound`, `variants`, `bracketed_text_removable`,
     matching `crates/deckmaste_lexical_source/src/schema.rs`);
  2. the `spelling` string templates;
  3. the meaning (`body`, `confers`, `rules`).
  The meaning side ignores `lexical` and `spelling`: their nested structures and identifiers need no
  semantic declarations and create no dependencies.
- **No `rule` data field.** CR citations live in comments (`[CR#701.47a]`), checked by `cargo xtask
  cite`.
- **Keyword `parameters` stay in `lexical`.** They list a keyword's printed frames, which the meaning
  signature cannot express. `modular(count: Amount)` prints both `Modular 2` and
  `Modular—Sunburst`, so its `lexical.parameters` are `[[Amount], [DashKeyword]]`. A frame is not a
  spelling choice the parameter itself decides: `madness(cost: Cost)` prints `Madness {1}{R}` and
  `Madness—Pay six {C}.` from its one `[[Cost]]` frame, the cost selecting the spelling. The
  signature's sorts are the meaning; frames are spelling only.

### 5.2 Name Resolution, Expansion and Read Order

1. **Files, primitives and names.**
   - Each declaration and card lives in its own `.syl` file under `plugins_v3/builtin/` (organized
     into `primitives/`, `definitions/`, `templates/` and `rules/`, then by category) or
     `plugins_v3/canon/cards/`. **Folder and file names are for human organization only, never
     semantically significant.** The loader never infers primitive, Definition or Template, or a
     grammatical category, from a path.
   - **The primitive/template boundary.** `primitives/` holds one full-arity declaration without a
     body per exposed constructor. `primitive doForEachKind(axis: KindAxis, domain: NounPhrase =
     none, sort: QualitySort, body: Action): Action` *is* the binding of the constructor with the
     same name and sort. A primitive whose name differs from its constructor's names the
     constructor after its signature with a leading dot, `primitive actionSearch(scope: Location,
     for_: NounPhrase): Action = .search`; this is the only place `.syl` writes a leading dot. Its
     parameters are matched to the constructor's fields by name, so its signature may follow the
     printed phrase. Only a primitive names a constructor, and only by being its binding.
   - Defaults belong on parameters. Fixed literal, enum, `none` or list arguments, and helpers that
     build expressions, belong in Templates that call the primitive. `can`, `cant`, `must`,
     `gatedBy`, and the Condition and Predicate `did`, assemble a deontic modifier record or a
     lookback clause record, which a primitive without a body cannot do; each is a Template over a
     renamed primitive, keeping the records' field defaults. `when`, `whenever` and `at_` stay
     separate Templates over `triggered`, as distinct spelling attachment points.
   - **Names.** A primitive that collides with nothing uses its constructor's name. No two
     declarations share one `(name, sort)`: the eleven primitives that would collide with a
     declaration of the same name and sort keep their existing prefixes until they are renamed.
     (`timesPaid` is a primitive at Linked Facet and a Template at Amount: different sorts, no
     collision.)

2. **Expected-sort dispatch and `Option` wrapping.**
   - Several declarations may share a bare name when they build different sorts (`and`, `not`,
     `target`, `did`, `token`, `doIf`, `replace`, `crew`, `creature`, …). There are no qualified names.
   - At expected sort `T`, a head resolves in constant time, with no backtracking, in this order:
     1. **`$name` is a parameter of the enclosing declaration** (declared at sort `T`, or `U` when
        `T = Option U`). `$name` naming no parameter of the enclosing declaration, or one at any
        other sort, is a load error. Bare words never consult the parameters, so a parameter never
        hides a same-named word; the remaining steps resolve bare words and calls only.
     2. **`none`**, when `T = Option U`.
     3. **The declaration `(head, T)`**, then **`(head, U)`** when `T = Option U` (the exact sort
        wins). A primitive found here stands for its constructor.
     4. **A constructor or closed enum word of `T`** (or of `U` when `T = Option U`).
   - **Declarations shadow constructors.** A declaration `(head, T)` may share its name with a
     constructor or closed enum word of `T` (the keyword action `search` and the search
     constructor). Dispatch tries the declaration first (step 3 before step 4), so the constructor
     is then reached only through its primitive (`actionSearch`).
   - **`Option U` wrapping.** A `U` where an `Option U` is expected is wrapped as present. An omitted
     argument or record field declared with a default (including `none`) takes that default. `none`
     stays available for explicit defaults and arguments; `some` is never written.
   - **Direct sort atoms:**
     - integer literals `n`: valid at `Nat`, `Int`, `Amount` (the literal amount `n`) and
       `ManaSymbol` (generic mana `n`);
     - color words (`white`, `blue`, `black`, `red`, `green`): valid at `Color`, `ColorOrColorless`
       and `ManaSymbol`;
     - Definition names: an `ability` Definition registers its name at `Ability`, `KeywordLabel` and
       `KeywordTerm` (including self-reference, as `hasKeyword(flying)` inside `flying`
       [CR#702.9b]); an `action` Definition registers its name at `Action` and `Deed`.

3. **Expansion and read-order mentions.**
   - **Templates** expand to constructor trees by first-order parameter substitution.
   - **Definitions (`action`, `ability`)** wrap every parameter of a sort that can be read in the
     caller's scope (`Action`, `Cost`, `StaticSpec`, `Replacement`, `Amount`, `Predicate`,
     `Quantity`, `Location`, `Condition`, `NounPhrase`, `GameEvent`, `Ability`, `TokenSpec`,
     `MoveRider`, `PlayPayment`, `Ballot`, and options and lists of them) so it is read in the
     caller (§2.3), and wrap the expanded body in the Definition's own Reference Scope under its
     Deed, or as its keyword ability. There is no opt-out: a parameter is always wrapped, so
     `search` and `searchZonesOf` bind `for_` like any other parameter.
   - `.syl` cannot write a Reference Scope, a caller-scope read, a first or later mention, a raw
     Deed, or a keyword-ability wrapper. The one temporary exception, for scope and Deed
     constructors only, is `plugins_v3/builtin/definitions/keyword_action/theRingTemptsYou.syl`,
     pending its separate modeling fix; it does not permit a raw keyword-ability wrapper.
   - `delayedTrigger` is an ordinary NounPhrase word for the latest delayed triggered ability that
     `delay` created in the current resolution. It requires a preceding creation, and does not make
     that ability available to ordinary pronouns or word anaphors. `delayUnlessPaid` uses it for the
     special action that stops that trigger [CR#116.2c,603.7].
   - A parameterless NounPhrase Template may declare `identity = "actor"` (`template theActor:
     NounPhrase { identity = "actor", body = actor }`). The mark belongs to its signature,
     independent of any generated name or bundle root. Repeated uses in an action share one
     declaration-local identity and keep the original Actor across controller Handoffs.
   - **Mentions follow read order.** A first mention reads its argument once, where it stands, and
     records its referents under the Binding Identity. A later mention reads them back by that
     identity and never reads the argument again, so a parameter used several times in a body does
     not introduce several referents. Which occurrence is first is decided **after** expansion, in
     the order the checker reads the occurrences. It is never decided by the order the author wrote
     the arguments, or by the declaration's parameter order. That order is usually the
     constructor's field order, but not always: an explicit-agent passive (`by(np)`) reads the
     promoted patient inside its action before `np` (§4.2). Authors write arguments in printed-card
     order, and primitive and Template signatures may follow the printed phrase. Independent
     alternatives keep separate mention scopes. Example: `doForEachKind(axis, domain, sort, body)`
     reads `domain` before `body`, so the NounPhrase in `domain` is the first mention and the one in
     `body` the later mention, whatever order a wrapping Template lists them in.

---

## 6. Standing Domain Mechanisms

These rulings were settled under `semantics-v2` and remain in force on the v3 spine and discourse
model.

- **Joined kinds.** A binding at a joined kind holds both halves' records side by side, so its kind
  is the join of its halves' kinds, and a demonstrative echo copies the pair
  ([`kind-index-joins-union-marking-is-spelling.md`](./kind-index-joins-union-marking-is-spelling.md)).
- **Stat changes ([CR#613.4b,613.4c]).** A two-stat change ("<subject> gets +P/+T", "has base power
  and toughness P/T") is one characteristic change over both stats (Templates `getsPt`, `getsBase`),
  so the subject is written and read once. `getsPt` always carries both printed deltas: "+X/+0" is
  `getsPt(subject, up(X), up(0))`, because the printed "+0" is part of the encoding and spelling
  must render it back. The single-stat `modification` Template is only for a change whose text names
  one stat. A Characteristic-Defining Ability is its own Static Spec (`definesPt`,
  `definesQuality`), applied in layer 7a for power and toughness and first in its layer otherwise
  ([CR#604.3a,613.3,613.4a]).
- **Counter kinds, Designations and Deeds are declared names ([CR#122.1,122.1a,122.1b,731.1]).** A
  counter kind, a Designation label and a Deed carry their declaration's name verbatim. A counter
  declaration states what **one** counter confers (`boost(up(1), up(1))`, `grants(flying)`). The
  declared name means the defined term, while the registry holds its Registry Definition apart for
  facts generation and `cargo xtask lean definitions`.
- **No chooser on `choose` and no taker on added turn parts ([CR#500.7..500.9]).** `choose` and the
  turn-part additions (`extraTurn`, `additionalPhase`, `additionalStep`) are performed by the
  innermost Actor, or "you".
- **Mana Substitutes ([CR#702.51a,702.66a,702.126a,701.67a]).** A Mana Substitute (an action, and
  which pips it pays: generic, or of a color) models "For each [generic | colored] mana in …, you
  may <act> rather than pay that mana" (convoke, delve, improvise, waterbend). A spell carries it as
  a Static Spec; a named cost carries it on the cost.
- **Piles ([CR#700.3b,700.3c,401.2,404.2,406.4]).** A Pile is a library (face down), a graveyard
  (face up), "that pile", or a face-down exile pile. It supports `slice(end, amount, pile)`,
  `run(end, stop, pile)`, `place(end, pile)` and `shuffle(pile)`; a bare "shuffle" is
  `shuffle(library(actor))` [CR#701.24a].
- **Timing, Durations, Usage Limits and Turn References ([CR#307.5,117.1d,611.2b,602.5b]).**
  - A Turn Reference is the current turn, the previous one, a player's next, each of a player's, a
    later one, or "that" turn.
  - A Timing is a phase or step of a turn (part, whose, which), before or after an event, while an
    event is underway, or only if a condition holds, combined with and, or and not. "As a sorcery"
    is priority, a main phase, your turn and an empty stack [CR#307.5]; instant speed is
    `onlyIf(hasPriority(you))` [CR#117.1d].
  - A Duration is `between(start, end)`, both optional events (`beginningOf`, `endOf`, `now`,
    `stateBasedActionsChecked`); or `forAsLongAs(condition)` [CR#611.2b]; or
    `modifiesPrecedingAction`, later text that modifies the meaning of the action before it, in
    force only for that action's event and no separate continuous effect ("Destroy target creature.
    It can't be regenerated." [CR#608.2c,701.19c]).
  - A Usage Limit is a count per period, where a period is a turn, your turn, the game, a part of a
    turn, or this object's existence [CR#602.5b,702.177a]. An Event Ordinal (`nthOccurrence`, a
    lookback's ordinal) counts within the same periods, which subsume turn parts, but not within
    this object's existence; "each turn" is a turn, never the turn part "turn". The checker refuses
    any other period.
- **Face-down specifications, special actions and move riders
  ([CR#406.3a,708.2,702.37a,702.168a,116.1,614.1c]).**
  - A face-down specification has no characteristics for face-down exile, and has them for
    face-down spells and permanents (morph, disguise, manifest, cloak). Turning face down, Entry
    Riders, Pile Placements and play payments use it.
  - A special action ([CR#116.1]) carries an optional player, an optional Timing, a cost and the
    action. The player is written when an effect allows a player other than you one. A delayed
    triggered ability `delay` created is stopped from triggering by a special action whose
    instruction establishes `cant(actor(ability), trigger)` with no Duration (*Quenchable Fire*,
    [CR#116.2c]).
  - A move rider is either an Entry Rider (onto the battlefield) or a Pile Placement (elsewhere)
    ([CR#614.1c,401.4,406.3]).
- **Modal choices ([CR#700.2d,700.2h,700.2i,702.42a,702.120a,702.172a,702.183a]).** `chooseModes`
  carries the number of modes, per-mode costs (spree, tiered) and optional pawprint weights. A Mode
  Change modifies a spell's `chooseModes`: all modes instead (entwine), a cost for extra modes
  (escalate), a set number (spree, tiered), repeatable modes, and modes not chosen before.
- **Off-Stack Statics ([CR#113.9]) and Resolution Delayed Triggers ([CR#608.3g,603.7d]).**
  - A Resolution Delayed Trigger is a permanent spell's static ability that functions on the stack
    and creates a delayed trigger. The trigger is created as the permanent is put onto the
    battlefield as the spell resolves, with the spell as its source and the spell's controller as it
    resolved as its controller [CR#608.3g,603.7d]. Dash's, blitz's and warp's are each its own
    ability [CR#702.109a,702.152a,702.185a,113.2c], naming "the permanent this spell becomes" as
    `becomes(thisSpell)`.
  - `asEvent(event, does)` models a printed "As [event], do". "As [this] enters" text (entry
    choices, devour, riot, tribute, unleash, amplify) instead replaces the entering. `before` holds
    what is done before it (an entry choice, devour's sacrifice, amplify's reveal, tribute's choice,
    an Aura's choice of what it will enchant [CR#614.12a,303.4f]). `alongside` holds what is done as
    part of it (riot's, unleash's, tribute's, amplify's and devour's counters, an Aura's
    attachment): `replace(would(this, enter), before(choose(…), alongside(…)))`. Megamorph's counter
    is `alongside` its turning face up [CR#614.1e].
  - An Aura's staying where it is, when it has nothing to enchant, replaces the entering too
    ([CR#303.4g]), and "enters tapped / with / as / transformed" replaces it with `withRiders(…)`
    ([CR#603.6d,614.1c,614.12]).
  - `whileHolds(condition, does)` models state statics (ascend, storied, daybound and nightbound
    [CR#702.131b,702.145b,702.195a]).
- **Sharing and Agreement ([CR#201.2a,201.2c,105.2,205.3m]).** `shares(axis, agreement, with)` tests
  whether an object shares at least one value (share) or none (differ) on `axis` with `with`.
  `agree(axis, agreement, group)` constrains a group selection to mutually share or differ on
  `axis`.
- **Linked abilities and declared Relations ([CR#607.2a,607.2d,607.2i,702.33e,702.55b,702.95b,702.99b]).**
  - A Link Source names a linked cost (by keyword, by the Nth keyword, the alternative cost, the
    additional cost), and a Linked Facet reads back its payment: the colors spent, the mana value
    spent, the times paid, a readback, and what was taken. Named object-to-object links (haunt,
    soulbond, cipher) are Relation Registry Definitions, with `relate`, `related` and
    `counterpart`.
  - The objects a linked ability acted on, and the player its cost chose, are read as the ability
    would read its own: by a Participial Description (§4.8), and by `the(isChosenPlayer)` (exactly
    one player choice in scope).
  - An ability's text publishes to the abilities after it (a card's text, and a keyword's `body`):
    its choices; its additional and alternative cost results (emerge's alternative cost
    [CR#702.119a]; harmonize's permission to cast paying another cost instead [CR#702.180a];
    retrace's and jump-start's additional discard [CR#702.81a,702.133a]); the target `addsTarget`
    adds, stamped by targeting, through a condition on it too (mutate's, required once its cost is
    chosen [CR#601.2c,702.140a]); what a replacement does `before` or `alongside` its event (devour,
    whose "it devoured" is a Deed done by sacrificing [CR#702.82b]); and the objects a printed
    triggered or activated ability itself introduces, in whatever zone it put them (§1 item 4),
    along each arm it may take (champion [CR#702.72a,607.2k]). It never publishes what its trigger
    event names [CR#607.1]. Published objects are linked: no pronoun or demonstrative reaches them.
- **Spell copies, text changes and Qualified Abilities
  ([CR#707.10c,702.40a,702.56a,612.1,613.1c,113.10a,702.165c,702.169b]).**
  - Spell-copy keywords write `may(chooseNewTargets(…))` ([CR#707.10c]), guarded by
    `matches(thisSpell, targeting(any, someTarget))` where the keyword's CR definition says "If the
    spell has any targets" (`storm` [CR#702.40a], `replicate` [CR#702.56a]).
  - Text-changing effects (cleave, overload, splice, word replacements
    [CR#612.1,613.1c,612.2,612.2a,702.47a,702.96a,702.148a]) use a Text Change Static Spec over a
    subject and a change (removing bracketed text, target to each, appending the text of, replacing a
    word). Bracketed text has no model until cleave is redesigned across English and semantics. A
    word replacement takes the replaceable word sorts (color, land type, creature type)
    [CR#612.2,612.2a], so a card name or any other sort is unrepresentable. Splice grants its text
    change and additional cost through a reveal made as the spell is cast [CR#601.2b,702.47b], not a
    replacement of the cast.
  - A keyword's ability parameter, and a Qualified Ability (an ability with its timing, limit, guard
    and zone), model abilities printed with and restricted by a keyword (backup's printed-after,
    saddle, exhaust [CR#113.10a,702.165c,702.169b..702.169d,702.177a]).
- **Subtype rules, conferrals and counter annihilation
  ([CR#301.5,301.6,303.4a,303.4c,310.7,704.5m,704.5n,704.5q,704.5s,704.5v,714.2d,714.3a,714.3c,714.4]).**
  - `Enchant` owns its spell target restriction, its legal-host permission, and its entry, attached
    to a legal host or not at all [CR#702.5a,303.4a,303.4c,303.4f,303.4g].
  - Subtype and counter state-based actions are state-based Conferrals on their definitions: Aura
    [CR#704.5m]; Equipment and Fortification [CR#704.5n]; Saga [CR#704.5s,714.4], using the chapter
    projection [CR#714.2d], `chapterAbilityOf` with `in_(stack)`, and a turn-based Conferral
    [CR#714.3c]; Siege [CR#310.7,704.5v], using `abilityOf` with `in_(stack)`; and the +1/+1 and
    −1/−1 counter annihilation on `m1m1Counter` [CR#704.5q,122.3], using the minimum aggregate and
    `smallerOf`.
