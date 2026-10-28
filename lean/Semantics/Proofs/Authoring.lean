import Semantics.Cards
import Semantics.Check
import Semantics.Check.Card
import Semantics.Definitions
import Semantics.Proofs.CardsChoice
import Semantics.Proofs.CardsCost

open Semantics Semantics.Templates Semantics.Definitions

namespace Semantics.Proofs.Authoring

private def plain : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.creature], cost := some [generic 1],
      power := stat 1, toughness := stat 1 } }

private def withKeyword : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.creature], cost := some [generic 1],
      power := stat 1, toughness := stat 1, text := [reach] } }

private def withDraw : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [spell (draw 1)] } }

private def preeminentCaptainWithDefaults : Spelled := spelled <| singleFacedCard
  { name := "Preeminent Captain", cost := [generic 2, pip .white], types := [.creature],
    subtypes := [kithkin, soldier],
    text :=
      [ firstStrike,
        whenever (does thisCreature attack)
          (may (putOntoBattlefieldTappedAttacking
            (a (and [soldier, isCreature, in_ yourHand])))) ],
    power := stat 2, toughness := stat 2 }

theorem templateDefaultsAreAuthored :
    preeminentCaptainWithDefaults.authoring.onlyAuthored = true ∧
    (cardNamed "Helm of Possession").authoring.onlyAuthored = true :=
  ⟨preeminentCaptainWithDefaults.onlyAuthored, (cardNamed "Helm of Possession").onlyAuthored⟩

private def rawKeywordCard : Card := .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.creature], cost := some [generic 1],
      power := stat 1, toughness := stat 1, text := [.keyword (.named "reach") [] []] } }

theorem expandedValueIsDefinitionallyIdentical : withKeyword.card = rawKeywordCard := by rfl

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Ability.keyword] -/
#guard_msgs in
example : Spelled := spelled <| rawKeywordCard

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.NounPhrase.you] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [spell (instruct .you (draw 1))] } }

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Action.ownScope] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [spell (.ownScope (some (.action "destroy")) (.draw (.exactlyOf 1)))] } }

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Action.ownScope] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [spell
        (.ownScope (some (.action "destroy")) (.move (target isCreature) (.zone .battlefield none) (some (.zone .hand none)) .arrival []))] } }

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Action.ownScope] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [spell (sequentially
        [.ownScope (some (.action "destroy")) (.sequentially [.move (target isCreature) (.zone .battlefield none) (some (.zone .hand none)) .arrival []])])] } }

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Replacement.instead] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [spell (replace (draw 1) (.instead (draw 2)))] } }

/-- error: invalid {...} notation, constructor for `Spelled` is marked as private -/
#guard_msgs in
example : Spelled := { withKeyword with card := rawKeywordCard, ok := by decide }

/-- error: Invalid `⟨...⟩` notation: Constructor for `Semantics.Spelled` is marked as private -/
#guard_msgs in
example : Spelled := ⟨rawKeywordCard, .data [], by decide, by decide⟩

opaque hiddenCard : Card := rawKeywordCard

/-- error: Definition Semantics.Proofs.Authoring.hiddenCard has no authoring evidence -/
#guard_msgs in
example : Spelled := spelled <| hiddenCard

/-- error: Semantic parameter card has no authoring evidence -/
#guard_msgs in
example (card : Card) : Spelled := spelled <| card

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Ability.keyword] -/
#guard_msgs in
example : Spelled := spelled <|
  let _discarded : Ability := .keyword (.named "flying") [] []
  withKeyword.card

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Ability.keyword] -/
#guard_msgs in
example : Spelled := spelled <| (withKeyword.card, (Ability.keyword (.named "flying") [] [])).1

-- `Subtype` is a semantic expression and `internal_expansion`: a card names a catalog entry
-- (`Semantics.Definitions.Subtypes`), no primitive wrapper spells one from a string, and a raw
-- constructor in a card is refused. *Mishra's Factory* is an Assembly-Worker [CR#205.3m], not the
-- "AssemblyWorker" a raw string let it write. The raw subtypes below are deliberate: each pins a
-- refusal. The accepted direction is the corpus: *Mishra's Factory* (`Cards/Mana.lean`) and the
-- Adventure card in `Cards/Faces.lean` are `spelled` with catalog subtypes.

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Subtype.of] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.artifact, .creature],
      subtypes := [.of .creature "AssemblyWorker"], cost := some [generic 1], power := stat 2,
      toughness := stat 2 } }

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Subtype.spell] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.instant], subtypes := [.spell "Adventure"],
      cost := some [generic 1], text := [spell (draw 1)] } }

/-- error: Unknown identifier `Semantics.Primitives.Subtype.of` -/
#guard_msgs in
example : Subtype := Semantics.Primitives.Subtype.of .creature "Dragon"

/-- error: Unknown identifier `Semantics.Primitives.Subtype.spell` -/
#guard_msgs in
example : Subtype := Semantics.Primitives.Subtype.spell "Adventure"

-- `CardType` is a catalog item in the same way (`Semantics.Definitions.CardTypes`): a structure
-- carrying its own fields, tagged `semantic_expression` and `internal_expansion`. A card names a
-- declared card type (`.creature`); a raw structure in a card is refused, here one that would
-- make an "Artifact" that is no permanent type [CR#110.4].

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.CardType.mk] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [{ name := "Artifact", cardClass := none }],
      cost := some [generic 1] } }

/-- error: Unknown identifier `Semantics.Primitives.CardType.mk` -/
#guard_msgs in
example : CardType := Semantics.Primitives.CardType.mk "Artifact" (some CardClass.permanentCard) []

-- `CounterKind`, `DesignationLabel` and `RelationLabel` are catalog items in the same way
-- (`Semantics.Definitions.Counters`, `.Designations`, `.Relations`): each value carries its own
-- columns, a counter's holder and a designation's scope, and is tagged `semantic_expression` and
-- `internal_expansion`. A card names a declared item; a raw constructor in a card is refused, so
-- an undeclared counter, designation or relation never reaches the checker, and a misspelled
-- catalog name is no declaration at all. The raw items below are deliberate: each pins a
-- refusal. The accepted direction is the witnesses that follow and the corpus.

private def sorceryWith (action : Action) : Card := .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [spell action] } }

private def withCatalogItems : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.sorcery], cost := some [generic 1],
      text := [spell (putCounters 1 p1p1Counter (target isCreature)),
        spell (gainDesignation you monarch),
        spell (relate this_ haunting (target isCreature))] } }

theorem catalogItemsRecordAuthorship : withCatalogItems.authoring.onlyAuthored = true :=
  withCatalogItems.onlyAuthored

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.CounterKind.named] -/
#guard_msgs in
example : Spelled := spelled <|
  sorceryWith (putCounters 1 (CounterKind.named "typoCounter") (target isCreature))

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.DesignationLabel.named] -/
#guard_msgs in
example : Spelled := spelled <| sorceryWith (gainDesignation you (.named "dusk" .heldByGame))

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.RelationLabel.named] -/
#guard_msgs in
example : Spelled := spelled <|
  sorceryWith (relate this_ (.named "unrelated") (target isCreature))

/--
error: Unknown identifier `p1p1Countr`
---
error: Definition sorryAx has no authoring evidence
-/
#guard_msgs in
example : Spelled := spelled <| sorceryWith (putCounters 1 p1p1Countr (target isCreature))

/--
error: Unknown identifier `monarc`
---
error: Definition sorryAx has no authoring evidence
-/
#guard_msgs in
example : Spelled := spelled <| sorceryWith (gainDesignation you monarc)

/--
error: Unknown identifier `hauntng`
---
error: Definition sorryAx has no authoring evidence
-/
#guard_msgs in
example : Spelled := spelled <| sorceryWith (relate this_ hauntng (target isCreature))

/-- error: Unknown identifier `Semantics.Primitives.CounterKind.named` -/
#guard_msgs in
example : CounterKind := Semantics.Primitives.CounterKind.named "typoCounter"

/-- error: Unknown identifier `Semantics.Primitives.DesignationLabel.named` -/
#guard_msgs in
example : DesignationLabel := Semantics.Primitives.DesignationLabel.named "dusk" .heldByGame

/-- error: Unknown identifier `Semantics.Primitives.RelationLabel.named` -/
#guard_msgs in
example : RelationLabel := Semantics.Primitives.RelationLabel.named "unrelated"

-- A `KeywordLabel` is a catalog item in the same way: each keyword ability Definition generates
-- its authored label constant (`kicker.label`), which a card writes by the Definition's name
-- (`keywordCostPaid kicker it`), and `KeywordLabel.named` is an internal expansion. So a card
-- names only a declared keyword: a raw label is refused and a misspelled one is no identifier.
-- `Ability.keyword` and the by-label helpers that spell it (`keywordCosting`, …) are internal
-- too, so a card writes a keyword ability through its typed Definition, whose signature fixes
-- its parameters. The accepted direction is `withKeyword` above and the corpus (*Ertai's
-- Trickery*, *Merfolk Falconer*).

private def instantWith (action : Action) : Card := .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.instant], cost := some [generic 1],
      text := [spell action] } }

theorem keywordLabelIsGenerated : kicker.label = .named "kicker" ∧
    multikicker.label = .named "kicker" := by decide

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.KeywordLabel.named] -/
#guard_msgs in
example : Spelled := spelled <|
  instantWith (doOnlyIf (counter (target isSpell)) (keywordCostPaid (.named "flyign") it))

/--
error: Unknown identifier `kickre`
---
error: Definition sorryAx has no authoring evidence
-/
#guard_msgs in
example : Spelled := spelled <|
  instantWith (doOnlyIf (counter (target isSpell)) (keywordCostPaid kickre it))

/-- error: Unknown identifier `Semantics.Primitives.KeywordLabel.named` -/
#guard_msgs in
example : KeywordLabel := Semantics.Primitives.KeywordLabel.named "flyign"

/-- error: Unknown identifier `Semantics.Primitives.Ability.keyword` -/
#guard_msgs in
example : Ability := Semantics.Primitives.Ability.keyword (.named "flying") [] []

/-- error: Card definitions must use semantic templates or definitions; raw constructors: [Semantics.Ability.keyword] -/
#guard_msgs in
example : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Authoring witness", types := [.creature], cost := some [generic 1],
      power := stat 1, toughness := stat 1, text := [keywordCosting ward [generic 2]] } }

-- A keyword action's `Deed` is a catalog item the same way: each Action Definition generates its
-- authored deed constant (`exile.deed`) from its own Reference Scope's header, the one deed the
-- action performs, and `Deed.action` is an internal expansion.

theorem deedIsGenerated : exile.deed = .action "exile" ∧ searchZonesOf.deed = search.deed ∧
    explore.deed = .action "explore" .performs := by decide

-- The checker cannot import Definitions; pin its raw literals against generated deeds.
theorem checkerDeedLiteralsMatchDefinitions :
    (.action "cast" : Deed) = cast.deed ∧
    (.action "play" : Deed) = play.deed ∧
    searchDeed = search.deed ∧
    (.action "tap" : Deed) = tap.deed := by decide

-- Shared labels are allowed when their permanence agrees, even across modules and aliases.
private semantic_definition tapAgain (subject : NounPhrase) : Action := tap subject
private semantic_definition exploreAgain : Action := explore
private semantic_definition adaptAgain : Action := adapt 1
private semantic_definition localDeed : Action :=
  .ownScope (some (.action "authoringWitness" .performs)) (.draw (.exactlyOf 1))
private semantic_definition localDeedAgain : Action := localDeed

theorem sharedDeedsAgree :
    tapAgain.deed = tap.deed ∧ exploreAgain.deed = explore.deed ∧
    adaptAgain.deed = adapt.deed ∧ localDeedAgain.deed = localDeed.deed := by decide

/-- error: Action Definitions for deed "tap" must agree on DeedPermanent -/
#guard_msgs in
private semantic_definition tapMismatch : Action :=
  .ownScope (some (.action "tap" .performs)) (.draw (.exactlyOf 1))

/-- error: Action Definitions for deed "explore" must agree on DeedPermanent -/
#guard_msgs in
private semantic_definition exploreMismatch : Action :=
  .ownScope (some (.action "explore" .isAbout)) (.draw (.exactlyOf 1))

/-- error: Action Definitions for deed "adapt" must agree on DeedPermanent -/
#guard_msgs in
private semantic_definition adaptMismatch : Action :=
  .ownScope (some (.action "adapt")) (.draw (.exactlyOf 1))

/-- error: Action Definitions for deed "authoringWitness" must agree on DeedPermanent -/
#guard_msgs in
private semantic_definition localDeedMismatch : Action :=
  .ownScope (some (.action "authoringWitness")) (.draw (.exactlyOf 1))

/-- A Definition's name stands for its deed wherever a `Deed` is expected (`elabDeedName`). -/
example : (exile : Deed) = exile.deed := rfl

-- A misspelled keyword action is no identifier at all.
/-- error: Unknown identifier `destory` -/
#guard_msgs in
example : Deed := destory

/-! ## Coercions and the card envelope

`spelled` accepts the authoring coercions: core's `α → Option α`, a written mana cost and an action
as a `Cost`, a subtype as a `Predicate`, and a numeral as an `Amount`. A coercion is expanded by
the elaborator before `Authoring.inspect` reads the term, so it reaches `inspect` as the Template
call it unfolds to. A numeral stays an `OfNat.ofNat` term that `inspect` reads through to its
`Amount.lit` leaf. `singleFacedCard` frames a single-faced card's characteristics. Each pin below
restates a corpus card as it was written before the sweep and compares the two. -/

mutual
  /-- The Template and Definition calls a Form names, in order. -/
  private def formCalls : Authoring.Form → List Lean.Name
    | .call name arguments => name :: formsCalls arguments
    | .data children | .constructor _ children => formsCalls children
    | .binder _ body => formCalls body
    | .literal _ | .parameter _ | .raw _ => []

  private def formsCalls : List Authoring.Form → List Lean.Name
    | [] => []
    | form :: forms => formCalls form ++ formsCalls forms
end

/-- Printed data alone is authored with no expression Template: its Form names only the mana
cost, the card type and the printed power and toughness. -/
theorem printedDataNeedsNoExpressionTemplate :
    formCalls plain.authoring =
      [``Templates.ManaSymbol.generic, ``CardType.creature,
        ``Templates.Amount.stat, ``Templates.Amount.stat] := by
  decide

/-- A keyword written by its Definition is recorded as that Definition's call. -/
theorem keywordRecordsDefinitionAuthorship :
    (formCalls withKeyword.authoring).contains ``Definitions.reach = true := by decide

/-- A Template whose agent is left to its default is still recorded as the Template's call. -/
theorem templateDefaultAgentRetainsAuthorship :
    (formCalls withDraw.authoring).contains ``Templates.draw = true := by decide

mutual
  /-- The numeric literals a Form names, in order. -/
  private def formNumerals : Authoring.Form → List Nat
    | .literal (.natVal value) => [value]
    | .call _ children | .data children | .constructor _ children => formsNumerals children
    | .binder _ body => formNumerals body
    | .literal (.strVal _) | .parameter _ | .raw _ => []

  private def formsNumerals : List Authoring.Form → List Nat
    | [] => []
    | form :: forms => formNumerals form ++ formsNumerals forms
end

/-- A numeral is retained more than once along the path `inspect` reads it through, so literals
are compared as runs: repeats of one literal in a row count once. -/
private def runs : List Nat → List Nat
  | a :: b :: rest => if a = b then runs (b :: rest) else a :: runs (b :: rest)
  | other => other

/-- The characteristics of a single-faced card's Form, under either envelope. -/
private def faceOf : Authoring.Form → Authoring.Form
  | .call `Semantics.Templates.singleFacedCard [characteristics] => characteristics
  | .constructor `Semantics.Card.singleFaced
      [.constructor `Semantics.CardFace.mk [characteristics]] => characteristics
  | form => form

/-- *Cold Storage* before the sweep: `some`, `mana [...]` and `perform (...)` written out. -/
private def coldStorageWritten : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Cold Storage", cost := some [4], types := [artifact],
      text :=
        [ activated (mana [3]) (exile (target (and [isCreature, youControl]))),
          activated (perform (sacrifice thisArtifact))
            (returnToBattlefield (each (and [isCreature, cardIn (exiledWith thisArtifact)]))
              [(under you)]) ] } }

theorem tokenBundleCoercionIsWritten (t : CharacteristicBundle) (n : Amount) :
    create n t = create n (written t) := rfl

theorem costCoercionsKeepTheCard : (cardNamed "Cold Storage").card = coldStorageWritten.card := rfl

theorem costCoercionsKeepTheAuthoring :
    faceOf (cardNamed "Cold Storage").authoring = faceOf coldStorageWritten.authoring := rfl

theorem envelopeIsSingleFacedCard :
    (cardNamed "Cold Storage").authoring = coldStorageWritten.authoring := rfl

/-- *Korlessa, Scale Singer* before the sweep: `hasSubtype dragon` written out. -/
private def korlessaWritten : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Korlessa, Scale Singer", cost := some [green, blue],
      supertypes := [.legendary], types := [creature],
      subtypes := [dragon, bard],
      text :=
        [ static (visibility .lookAt you topOfLibrary),
          static (can you (cast (bare (and [isSpell, hasSubtype dragon])) (from_ := onTop))) ],
      power := 1, toughness := 4 } }

theorem subtypeCoercionKeepsTheCard : (cardNamed "Korlessa, Scale Singer").card = korlessaWritten.card := rfl

theorem subtypeCoercionKeepsTheAuthoring :
    faceOf (cardNamed "Korlessa, Scale Singer").authoring = faceOf korlessaWritten.authoring := rfl

/-- *Char-Rumbler* before the sweep: `.lit 1`, `.lit 0` and `some untilEndOfTurn` written out. -/
private def charRumblerWritten : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Char-Rumbler", cost := some [2, red, red], types := [creature],
      subtypes := [elemental],
      text :=
        [ doubleStrike,
          activated (mana [red])
            (get thisCreature (up (.lit 1)) (up (.lit 0)) (some untilEndOfTurn)) ],
      power := some (lit (-1)), toughness := 3 } }

theorem numeralsKeepTheCard : (cardNamed "Char-Rumbler").card = charRumblerWritten.card := rfl

theorem numeralsKeepTheTemplates :
    formCalls (faceOf (cardNamed "Char-Rumbler").authoring) = formCalls (faceOf charRumblerWritten.authoring) := rfl

theorem numeralsKeepTheLiterals :
    runs (formNumerals (faceOf (cardNamed "Char-Rumbler").authoring)) =
      runs (formNumerals (faceOf charRumblerWritten.authoring)) := rfl

/-! ## A sentence shared by a modal header

`chooseModesOver` writes the sentence a modal header shares with its bullets once, as a supplied
body over the bullets. `inspect` reads the call before it reduces, so the authoring tree holds the
header's sentence once and each bullet once, while the card reduces to an ordinary `chooseModes`:
each mode the sentence completed by its bullet, read in the caller's Reference Scope as a supplied
body is. -/

mutual
  /-- The bullets and the supplied body of the first `chooseModesOver` call a Form names. -/
  private def modesOver? : Authoring.Form → Option (List Authoring.Form × Authoring.Form)
    | .call ``Templates.chooseModesOver [_, _, items, body] => some (formItems items, body)
    | .call _ children | .data children | .constructor _ children => modesOverIn? children
    | .binder _ body => modesOver? body
    | .literal _ | .parameter _ | .raw _ => none

  private def modesOverIn? : List Authoring.Form → Option (List Authoring.Form × Authoring.Form)
    | [] => none
    | form :: forms => (modesOver? form).orElse fun _ => modesOverIn? forms

  /-- The elements of a written list. -/
  private def formItems : Authoring.Form → List Authoring.Form
    | .constructor ``List.cons [_, head, tail] => head :: formItems tail
    | _ => []
end

/-- The supplied body's parameter and the calls it names. -/
private def suppliedBody : Authoring.Form → Option (Lean.Name × List Lean.Name)
  | .binder name body => some (name, formCalls body)
  | _ => none

/-- *Call Damage Control*: one header, "Return those cards from your graveyard to your hand.", and
one target per bullet. -/
theorem callDamageControlWritesEachPartOnce :
    let calls := formCalls (cardNamed "Call Damage Control").authoring
    calls.count ``Templates.chooseModesOver = 1 ∧ calls.count ``Templates.returnToHand = 1 ∧
      calls.count ``Semantics.Templates.NounPhrase.target = 4 ∧
      (modesOver? (cardNamed "Call Damage Control").authoring).map
          (fun (items, body) => (items.map fun item => (formCalls item).head?, suppliedBody body)) =
        some ([some ``Semantics.Templates.NounPhrase.target,
            some ``Semantics.Templates.NounPhrase.target,
            some ``Semantics.Templates.NounPhrase.target,
            some ``Semantics.Templates.NounPhrase.target],
          some (`cards, [``Templates.returnToHand, ``Templates.yourGraveyard])) := by
  decide

/-- *Call Damage Control* written out as its modes. -/
private def callDamageControlModes : Card := .singleFaced
  { characteristics :=
    { name := "Call Damage Control", cost := some [generic 1, pip .green], types := [.sorcery],
      text :=
        [ spell (.chooseModes (upTo 2)
            [ (none, none, .inCaller (returnToHand (target (and [isArtifact, isCard])) yourGraveyard)),
              (none, none, .inCaller (returnToHand (target (and [isCreature, isCard])) yourGraveyard)),
              (none, none,
                .inCaller (returnToHand (target (and [isEnchantment, isCard])) yourGraveyard)),
              (none, none,
                .inCaller (returnToHand (target (and [isLand, isCard])) yourGraveyard)) ]) ] } }

theorem callDamageControlIsAnOrdinaryChooseModes :
    (cardNamed "Call Damage Control").card = callDamageControlModes := rfl

/-- *Wild Shape*: one header, "Until end of turn, target creature you control has that base power
and toughness, becomes that creature type, and gains that ability.", and one creature shape per
bullet. -/
theorem wildShapeWritesEachPartOnce :
    let calls := formCalls (cardNamed "Wild Shape").authoring
    calls.count ``Templates.chooseModesOver = 1 ∧ calls.count ``Templates.establish = 1 ∧
      calls.count ``Semantics.Templates.NounPhrase.target = 1 ∧ calls.count ``Templates.untilEndOfTurn = 1 ∧
      (modesOver? (cardNamed "Wild Shape").authoring).map
          (fun (items, body) => (items.map formCalls, suppliedBody body)) =
        some ([[``turtle, ``hexproof],
            [``spider, ``reach],
            [``elephant, ``trample]],
          some (`shape,
            [``Templates.establish, ``Templates.qualityChange, ``Semantics.Templates.NounPhrase.target,
              ``Templates.Predicate.and, ``isCreature, ``youControl, ``Templates.bundle,
              ``Templates.untilEndOfTurn])) := by
  decide

/-- *Wild Shape* written out as its modes. -/
private def wildShapeModes : Card :=
  let shape (subtype : Subtype) (ability : Ability) (power toughness : Amount) : Action :=
    .inCaller (establish
      (qualityChange (target (and [isCreature, youControl])) .sets
        (bundle { characteristics :=
          { subtypes := [subtype], text := [ability], power := some power,
            toughness := some toughness } }))
      untilEndOfTurn)
  .singleFaced
    { characteristics :=
      { name := "Wild Shape", cost := some [pip .green], types := [.instant],
        text :=
          [ spell (.chooseModes (exactly 1)
              [ (none, none, shape turtle hexproof 1 3),
                (none, none, shape spider reach 1 5),
                (none, none, shape elephant trample 3 3) ]) ] } }

theorem wildShapeIsAnOrdinaryChooseModes : (cardNamed "Wild Shape").card = wildShapeModes := rfl

/-- A supplied body that ignores its bullet gives identical modes: the bullets are written, but
nothing of them reaches the modes. -/
theorem ignoredBulletGivesIdenticalModes :
    chooseModesOver (upTo 2) [target (and [isArtifact, isCard]), target (and [isLand, isCard])]
        (fun _ => draw 1) =
      .chooseModes (upTo 2) [(none, none, .inCaller (draw 1)), (none, none, .inCaller (draw 1))] :=
  rfl

/-- Bullets that are abilities: a supplied body binds a parameter of any semantic type, not only a
noun phrase. -/
private def grantedChoice : Spelled := spelled <| .singleFaced
  { characteristics :=
    { name := "Granted choice witness", types := [.instant],
      text :=
        [ spell (chooseModesOver (exactly 1) [flying, reach]
            (fun ability => gain (target isCreature) ability untilEndOfTurn)) ] } }

theorem suppliedBodyBindsAnAbility : grantedChoice.authoring.onlyAuthored = true :=
  grantedChoice.onlyAuthored

theorem suppliedAbilityBodyChecks : grantedChoice.card.check = [] := by decide

-- A leading-dot name resolves against the expected type before any coercion applies, so an
-- optional kind keeps its `some` (lean-v3-deal-damage-kind) and a mana cost of leading-dot
-- symbols alone keeps its `mana`.

/--
error: Unknown constant `Option.combat`

Note: Inferred this name from the expected resulting type of `.combat`:
  Option DamageKind
-/
#guard_msgs in
example : Action := dealDamage (to := a isPlayer) (kind := .combat)

example : activated [.snow] (draw 1) = activated (mana [.snow]) (draw 1) := rfl

namespace BundleDsl

example (card token action keyword counter subtype template primitive designation relation
    ability_word card_type sba conferral : Nat) : List Nat :=
  [card, token, action, keyword, counter, subtype, template, primitive, designation, relation,
   ability_word, card_type, sba, conferral]

/-- Declaration modifiers and documentation pass through to the generated template. -/
@[inline] private template annotated : Predicate := { body := Templates.Predicate.isCard }
example : annotated = Predicate.isCard := rfl

/-- error: Unknown field speling in template declaration -/
#guard_msgs in
template misspelledMetadata : Predicate := { speling := "card", body := .isCard }

primitive token : NounWord := { spelling := "token", body := .token }
template token (spec : TokenSpec) : CreationSpec := { body := Templates.CreationSpec.token spec [] }
primitive atom : Predicate := { lexical := { lemma := "card" }, body := .isCard }
template shared : Predicate := { body := atom }
template shared : Condition := { body := exists_ atom }

example : (shared : Predicate) = atom := rfl
example : (shared : Condition) = exists_ atom := rfl
example : (creature : Predicate) = isCreature := rfl
example : (creature : NounWord) = Templates.NounWord.creature := rfl
example : (creature : CardType) = Semantics.CardType.creature := rfl
example : (and [atom] : Predicate) = Templates.Predicate.and [atom] := rfl
example : (and [exists_ atom] : Condition) = Templates.Condition.and [exists_ atom] := rfl
example : (devour 2 : Ability) = Definitions.devour 2 := rfl
example : (devour thisCreature : Action) = Definitions.Action.devour thisCreature := rfl

/-- A parameter's two occurrences share the generated Binding Identity. -/
template twice (subject : NounPhrase) : Action := {
  body := instruct (controllerOf subject) (sacrifice subject),
}
example (subject : NounPhrase) : twice subject =
    instruct (controllerOf (.firstMention "twice.subject" subject))
      (sacrifice (.laterMention "twice.subject" subject)) := rfl

action drawCards (amount : Amount) := { body := draw amount }
example (amount : Amount) : drawCards amount =
    .ownScope (some (.action "drawCards" .none)) (draw (.inCaller amount)) := rfl
example : drawCards.deed = Deed.action "drawCards" .none := rfl

action subjects (subject : NounPhrase) := {
  body := instruct subject (sacrifice subject),
}
example (subject : NounPhrase) : subjects subject =
    .ownScope (some (.action "subjects" .none))
      (instruct (.firstMention "subjects.subject" (.inCaller subject))
        (sacrifice (.laterMention "subjects.subject" (.inCaller subject)))) := rfl

action optOut (subject : NounPhrase) := { body := sacrifice (.inCaller subject) }
example (subject : NounPhrase) : optOut subject =
    .ownScope (some (.action "optOut" .none)) (sacrifice (.inCaller subject)) := rfl

primitive dynamicActor : NounPhrase := {
  -- TODO(spelling): The actor at the current handoff, without a generated Binding Identity.
  body := Semantics.Primitives.NounPhrase.actor
}

action actorAcrossControllerHandoff := {
  body := instruct (controllerOf theActor) (instruct dynamicActor (sacrifice theActor))
}

theorem generatedActorKeepsDynamicHandoffReads :
    actorAcrossControllerHandoff = .ownScope (some (.action "actorAcrossControllerHandoff"))
      (instruct (controllerOf (.firstMention "actorAcrossControllerHandoff.actor" theActor))
        (instruct .actor
          (sacrifice (.laterMention "actorAcrossControllerHandoff.actor" theActor)))) := rfl

action actorTwice := { body := instruct theActor (sacrifice theActor) }
example : actorTwice = .ownScope (some (.action "actorTwice" .none))
    (instruct (.firstMention "actorTwice.actor" theActor)
      (sacrifice (.laterMention "actorTwice.actor" theActor))) := rfl

action containers (values : List Action) (rider : Option Location) := {
  body := sequentially (values ++ [enter (from_ := rider)]),
}

example (values : List Action) (rider : Option Location) : containers values rider =
    .ownScope (some (.action "containers" .none))
      (sequentially (values.map Action.inCaller ++ [enter (from_ := rider.map Location.inCaller)])) :=
  rfl

action ballotArgument (ballot : Ballot) := { body := vote (ballot := ballot) }
example (candidate : NounPhrase) : ballotArgument (.byCandidate candidate) =
    .ownScope (some (.action "ballotArgument" .none)) (vote (ballot := .byCandidate (.inCaller candidate))) := rfl

keyword recursiveKeyword := { body := [static (cant any attack
  (except := byObjects (bare (hasKeyword recursiveKeyword))))] }
keyword parameterKeyword (amount : Amount) (cost : Cost) := {
  lexical := { lemma := "parameter keyword" }, spelling := "parameter <amount> <cost>",
  body := [activated cost (draw amount)],
}
example : (parameterKeyword : KeywordLabel) = parameterKeyword.label := rfl
example : (parameterKeyword : KeywordTerm) = theKeyword parameterKeyword.label := rfl
example : (parameterKeyword : Option KeywordLabel) = some parameterKeyword.label := rfl
example : (parameterKeyword : Option KeywordTerm) = some (theKeyword parameterKeyword.label) := rfl
example : (recursiveKeyword : KeywordTerm) = theKeyword recursiveKeyword.label := rfl

keyword nounKeyword (subject : NounPhrase) := {
  body := [spell (instruct subject (sacrifice subject))],
}
example (subject : NounPhrase) : nounKeyword subject =
    Ability.keyword nounKeyword.label []
      [spell (instruct (.firstMention "nounKeyword.subject" (.inCaller subject))
        (sacrifice (.laterMention "nounKeyword.subject" (.inCaller subject))))] := rfl

abbrev KeywordNumber := Amount
keyword aliasedKeyword (amount : KeywordNumber) := { body := [spell (draw amount)] }
example (amount : Amount) : aliasedKeyword amount =
    Ability.keyword aliasedKeyword.label [.number (.inCaller amount)]
      [spell (draw (.inCaller amount))] := rfl

keyword selfFlying := {
  body := [static (cant any (block thisCreature)
    (except := byObjects (bare [creature, andOr [hasKeyword selfFlying, hasKeyword reach]])))],
}
keyword selfSuspend (amount : Amount) (cost : Cost) := {
  body := [
    static (specialAction (whileTrue (couldBeginToCast you thisCard yourHand)) cost
      (exileWithCounters thisCard amount timeCounter)),
    at_ (beginningOfYour .upkeep)
      (if_ := is thisCard [in_ exileZone, hasKeyword selfSuspend, hasCounter timeCounter])
      (removeCounters (oneThrough 1) timeCounter thisCard),
    when (does any (removeCounters theLast timeCounter thisCard))
      (if_ := is thisCard (in_ exileZone))
      [may (play thisCard (payment := withoutPaying)),
       doIf (did you (cast (a [creature, spell])) thisWay)
        (gain (that spell) haste
          (forAsLongAs (or [is (that spell) youControl, is (becomes (that spell)) youControl])))],
  ],
}
theorem selfFlyingChecks : Ability.check (DRS.ofList []) selfFlying = [] := by decide
theorem selfSuspendChecks : Ability.check (DRS.ofList []) (selfSuspend 2 [blue]) = [] := by decide

ability_word testWord := { body := activated [blue] (draw 1) }
example : testWord = abilityWord (.named "testWord") (activated [blue] (draw 1)) := rfl

ability_word alreadyWord := { body := landfall (activated [blue] (draw 1)) }
example : alreadyWord = landfall (activated [blue] (draw 1)) := rfl

action shieldWithoutDeed (subject : NounPhrase) := {
  deed := none,
  body := clearDamage subject
}

example : (match shieldWithoutDeed this_ with
  | .ownScope none _ => true
  | _ => false) = true := rfl

counter testCounter := { lexical := { lemma := "test counter", plural := "test counters" }, confers := [] }
subtype testSubtype := { name := "Test", kind := creature }
subtype testSpellSubtype := { name := "Test spell", kind := .spell }
designation testDesignation := { lexical := { lemma := "test designation" }, scope := .heldBy .player, sole := true }
card_type testCardType := { name := "Test" }
relation testRelation := { lexical := { lemma := "test relation" } }
token testToken := { name := "Test", token := { types := [artifact] } }
sba testSba := { scope := creature, when := exists_ creature, then_ := draw 1 }
conferral testConferral := { scope := creature, confer := boost (up 1) (up 1) }
card := { name := "Test Card", types := [sorcery], text := [spell (draw 1)] }

card := {
  name := "Authored surface", cost := [blue, 2], types := [sorcery],
  text := [parameterKeyword 2 [blue], spell [drawCards 1, draw 1]],
}
card := {
  name := "Authored spree", cost := [white], types := [sorcery],
  text := [spell (chooseSpree [([generic 1], draw 1), ([generic 2], draw 2)])],
}
theorem postponedListsRetainAuthorship : (cardNamed "Authored spree").authoring.onlyAuthored = true :=
  (cardNamed "Authored spree").onlyAuthored

theorem surfaceRetainsAuthorship : (cardNamed "Authored surface").authoring.onlyAuthored = true :=
  (cardNamed "Authored surface").onlyAuthored

open Lean Elab Command in
run_cmd do
  let env ← getEnv
  unless Semantics.Authoring.primitiveAttribute.hasTag env
      ``Semantics.Proofs.Authoring.BundleDsl.atom do throwError "primitive tag missing"

example : testCounter = CounterKind.named "testCounter" .object := rfl
example : parameterKeyword.label = KeywordLabel.named "parameterKeyword" := rfl
example : testDesignation = DesignationLabel.named "testDesignation" (.heldBy .player) true := rfl
example : testRelation = RelationLabel.named "testRelation" := rfl
example : testSpellSubtype = Subtype.spell "Test spell" := rfl
example : testCardType = { name := "Test", cardClass := none } := rfl

example : RegistryDefinition.check testCounter_definition = [] := ok_testCounter
example : RegistryDefinition.check testSubtype_definition = [] := ok_testSubtype
example : RegistryDefinition.check testSpellSubtype_definition = [] := ok_testSpellSubtype
example : RegistryDefinition.check testDesignation_definition = [] := ok_testDesignation
example : RegistryDefinition.check testCardType_definition = [] := ok_testCardType
example : RegistryDefinition.check testRelation_definition = [] := ok_testRelation
example : PredefinedToken.check testToken = [] := ok_testToken
example : SbaRule.check testSba = [] := ok_testSba
example : ConferralRule.check testConferral = [] := ok_testConferral
example : Card.check (cardNamed "Test Card").card = [] := cardNamedOk "Test Card"
example : (cardNamed "Test Card").authoring.onlyAuthored = true := (cardNamed "Test Card").onlyAuthored

example : ([] : Action) = sequentially [] := rfl
example : ([] : Predicate) = Templates.Predicate.and [] := rfl
example : ([] : Condition) = Templates.Condition.and [] := rfl
example : ([] : StaticSpec) = conjunction [] := rfl
example : ([] : Cost) = mana [] := rfl
example : ([] : Option (List Color)) = some [] := rfl
example : (instant : NounWord) = NounWord.type .instant := rfl
example : (sorcery : NounWord) = NounWord.type .sorcery := rfl
example : (kindred : NounWord) = NounWord.type .kindred := rfl

example : ([draw 1, draw 2] : Action) = sequentially [draw 1, draw 2] := rfl
example : ([isCard] : Predicate) = Templates.Predicate.and [isCard] := rfl
example : ([exists_ isCard] : Condition) = Templates.Condition.and [exists_ isCard] := rfl
example : ([cant any attack] : StaticSpec) = conjunction [cant any attack] := rfl
example : ([blue, 2] : Cost) = mana [pip .blue, generic 2] := rfl
example : ([[blue], tapSymbol] : Cost) = compound [mana [pip .blue], tapSymbol] := rfl
example : ([blue] : Option (List Color)) = some [Color.blue] := rfl
example : (blue : ColorOrColorless) = ColorOrColorless.of .blue := rfl
example : (combat : Option DamageKind) = some DamageKind.combat := rfl
example : (this : NounPhrase) = this_ := rfl
example : ({ subtypes := [testSubtype] } : CharacteristicBundle) =
    { characteristics := { subtypes := [testSubtype] } } := rfl
example : (cant any attack : Ability) = static (cant any attack) := rfl

open Lean Authoring in
#guard match topoSortBundle [
  { key := ⟨`z, `Semantics.Action⟩, file := "z.lean", references := [`a] },
  { key := ⟨`a, `Semantics.Predicate⟩, file := "a.lean" },
  { key := ⟨`a, `Semantics.Condition⟩, file := "other.lean" },
  { key := ⟨`flying, `Semantics.Ability⟩, file := "kw.lean", references := [`flying] },
  { key := ⟨`devour, `Semantics.Ability⟩, file := "kw.lean", references := [`devour] },
  { key := ⟨`devour, `Semantics.Action⟩, file := "act.lean" }
] with
| .ok items => items.map (·.key) ==
    [⟨`a, `Semantics.Condition⟩, ⟨`a, `Semantics.Predicate⟩, ⟨`devour, `Semantics.Action⟩,
     ⟨`devour, `Semantics.Ability⟩, ⟨`flying, `Semantics.Ability⟩, ⟨`z, `Semantics.Action⟩]
| .error _ => false

open Lean Authoring in
#guard match topoSortBundle [
  { key := ⟨`a, `Semantics.Action⟩, file := "a.lean", references := [`b] },
  { key := ⟨`b, `Semantics.Action⟩, file := "b.lean", references := [`a] }
] with
| .error message => message ==
    "Dependency cycle in bundle: a : Semantics.Action (a.lean), b : Semantics.Action (b.lean)"
| .ok _ => false

example (bundle : CharacteristicBundle) :
    ({ bundle with qualities := [] } : CharacteristicBundle) =
      { characteristics := bundle.characteristics, qualities := [] } := rfl

open Lean Authoring in
#guard match topoSortBundle [
  { key := ⟨`a, `Semantics.Action⟩, file := "a.lean", references := [`b] },
  { key := ⟨`b, `Semantics.Action⟩, file := "b.lean", references := [`a] },
  { key := ⟨`blocked, `Semantics.Action⟩, file := "blocked.lean", references := [`a] }
] with
| .error message => message ==
    "Dependency cycle in bundle: a : Semantics.Action (a.lean), b : Semantics.Action (b.lean)"
| .ok _ => false

open Lean Authoring in
#guard match topoSortBundle [
  { key := ⟨`a, `Semantics.Action⟩, file := "first.lean" },
  { key := ⟨`a, `Semantics.Action⟩, file := "second.lean" }
] with
| .error message => message == "Duplicate bundle declaration: a : Semantics.Action (second.lean)"
| .ok _ => false

primitive dslLexicalRich : Predicate := {
  lexical := {
    lemma := "kick",
    verbs := [{ lemma := "kick", frames := [monotransitive] }],
    bound := { surface := "walk", stems := [LandType] },
    variants := [unelaboratedVariant], bracketed_text_removable := true
  },
  spelling := "kick", body := isCard,
}
example : dslLexicalRich = isCard := rfl

/-- error: Unknown field lemma in template declaration -/
#guard_msgs in
template misplacedLexical : Predicate := { lemma := "card", body := isCard }

/-- error: parameter identity is generated: manualIdentity.subject -/
#guard_msgs (error) in
action manualIdentity (subject : NounPhrase) := {
  body := sacrifice (.firstMention "manualIdentity.subject" subject),
}

action Action.sortedIdentity (subject : NounPhrase) := { body := sacrifice subject }
example (subject : NounPhrase) : Action.sortedIdentity subject =
    .ownScope (some (.action "sortedIdentity" .none))
      (sacrifice (.firstMention "sortedIdentity.subject" (.inCaller subject))) := rfl

example : Predicate.shared = atom := rfl
example : Condition.shared = exists_ atom := rfl
counter sharedRegistry := {}
relation sharedRegistry := {}
example : CounterKind.sharedRegistry = CounterKind.named "sharedRegistry" .object := rfl
example : RelationLabel.sharedRegistry = RelationLabel.named "sharedRegistry" := rfl
example : (sharedRegistry : CounterKind) = CounterKind.sharedRegistry := rfl
example : (sharedRegistry : RelationLabel) = RelationLabel.sharedRegistry := rfl

keyword customLabel (subject quality : Predicate) (promise : Action) := {
  label := "custom label",
}
example (subject quality : Predicate) (promise : Action) : customLabel subject quality promise =
    Ability.keyword (.named "custom label")
      [.subject (.inCaller subject), .quality (.inCaller quality)] [] := rfl
example : customLabel.label = KeywordLabel.named "custom label" := rfl

example (cost : Cost) : CallerScope.caller (PlayPayment.payingInstead cost) =
    PlayPayment.payingInstead (.inCaller cost) := rfl

/-- error: parameter identity is generated: singleManual.subject -/
#guard_msgs (error) in
template singleManual (subject : NounPhrase) : Action := {
  body := sacrifice (.laterMention "singleManual.subject" subject),
}

/-- error: parameter identity is generated: manualActor.actor -/
#guard_msgs (error) in
action manualActor := { body := sacrifice (.firstMention "manualActor.actor" theActor) }

keyword Ability.reverseShared (amount : Amount) := { label := "reverse shared" }
action reverseShared := { body := draw 1 }
example : Action.reverseShared = reverseShared := rfl
example : Ability.reverseShared.label = KeywordLabel.named "reverse shared" := rfl

keyword keywordFirst := {}
action keywordFirst := { body := draw 1 }
example : Ability.keywordFirst = Ability.keyword (.named "keywordFirst") [] [] := rfl
example : Ability.keywordFirst.label = KeywordLabel.named "keywordFirst" := rfl
example : Action.keywordFirst = .ownScope (some (.action "keywordFirst" .none)) (draw 1) := rfl

action paymentArgument (payment : PlayPayment) := {
  body := Templates.Action.procedure .cast this_ payment none none none,
}
example (payment : PlayPayment) : paymentArgument payment =
    .ownScope (some (.action "paymentArgument" .none))
      (.procedure .cast this_ payment.inCaller none none none) := rfl

example : Definitions.Ability.devour 2 = Definitions.devour 2 := rfl
example : Templates.NounPhrase.target isCreature = Templates.target isCreature := rfl

card := { name := "Mr. Orfeo, the Boulder", types := [creature], power := 3, toughness := 4 }
theorem printedNameLookupChecks : Card.check (cardNamed "Mr. Orfeo, the Boulder") = [] :=
  cardNamedOk "Mr. Orfeo, the Boulder"
theorem printedNameLookupRetainsProof : (cardNamed "Mr. Orfeo, the Boulder" : Spelled).card.check = [] :=
  cardNamedOk "Mr. Orfeo, the Boulder"
open Lean Elab Command Term in
run_cmd do
  let tree ← liftTermElabM do
    let term ← elabTerm (← `(term| (3 : Option Amount))) none
    synthesizeSyntheticMVarsNoPostponing
    Authoring.inspect (← instantiateMVars term)
  let rec hasLiteral : Authoring.Form → Bool
    | .constructor name children => name == `Semantics.Amount.lit || children.any hasLiteral
    | .data children | .call _ children => children.any hasLiteral
    | .binder _ body => hasLiteral body
    | _ => false
  unless hasLiteral tree do throwError "Optional stat numeral lost its Amount.lit leaf"

theorem optionalPowerNumeral : (3 : Option Amount) = some (.lit 3) := rfl
theorem optionalToughnessNumeral : (4 : Option Amount) = some (.lit 4) := rfl
theorem loyaltyNumeral : ({ loyalty := 3, defense := 5 } : Characteristics).loyalty = some (.lit 3) := rfl
theorem defenseNumeral : ({ loyalty := 3, defense := 5 } : Characteristics).defense = some (.lit 5) := rfl
theorem manaNumeralsAndColors : ([2, blue] : ManaCost) = [ManaSymbol.simple (.generic 2), .simple (.specific (.of .blue))] := rfl

/-- error: Named card declarations are no longer supported; use card := ... -/
#guard_msgs in
card legacyNamed := { name := "Legacy Named", types := [sorcery] }
/-- error: Named card declarations are no longer supported; use card := ... -/
#guard_msgs in
card legacyTyped : Spelled := { name := "Legacy Typed", types := [sorcery] }

/-- error: Anonymous card requires a printed name -/
#guard_msgs in
card := { types := [sorcery] }
/-- error: Anonymous card requires a printed name -/
#guard_msgs in
card := { name := none, types := [sorcery] }
/-- error: Anonymous card does not accept body := -/
#guard_msgs in
card := { body := singleFacedCard { name := "Legacy body", types := [sorcery] } }
/-- error: Anonymous card requires a printed name -/
#guard_msgs in
card := { characteristics := { name := "Legacy wrapper", types := [sorcery] } }
/-- error: Anonymous card requires a printed name -/
#guard_msgs in
card := transforming { front := { name := "Front", types := [sorcery] }, back := {} }

card := transforming {
  front := { name := "Day", types := [artifact] },
  back := { name := "Night", types := [artifact] } }
card := modalDfc {
  front := { name := "Front", types := [sorcery] },
  back := { name := "Back", types := [land] } }
card := split {
  right := { name := "Right", types := [instant] },
  left := { name := "Left", types := [instant] } }
card := sharedLineSplit {
  shared := { types := [enchantment] },
  left := { name := "Door one" }, right := { name := "Door two" } }
card := adventurer {
  normal := { name := "Normal", types := [artifact] },
  adventure := { name := "Adventure", types := [sorcery] } }
card := flip {
  normal := { name := "Unflipped", types := [artifact] },
  alternative := { name := "Flipped", types := [artifact] } }
card := leveler {
  inner := { name := "Leveler", types := [creature], power := 1, toughness := 1 },
  bands := [{ range := between 1 2, power := 2, toughness := 3 }] }
card := prototype {
  inner := { name := "Prototype", types := [creature], power := 4, toughness := 4 },
  alternative := { cost := [1, blue, blue], power := 2, toughness := 1 } }
theorem transformingPrintedKey : Card.check (cardNamed "Day // Night") = [] := cardNamedOk "Day // Night"
theorem modalDfcPrintedKey : Card.check (cardNamed "Front // Back") = [] := cardNamedOk "Front // Back"
theorem splitPrintedKey : Card.check (cardNamed "Left // Right") = [] := cardNamedOk "Left // Right"
theorem sharedLineSplitPrintedKey : Card.check (cardNamed "Door one // Door two") = [] := cardNamedOk "Door one // Door two"
theorem adventurerPrintedKey : Card.check (cardNamed "Normal // Adventure") = [] := cardNamedOk "Normal // Adventure"
theorem flipPrintedKey : Card.check (cardNamed "Unflipped // Flipped") = [] := cardNamedOk "Unflipped // Flipped"
theorem levelerPrintedKey : Card.check (cardNamed "Leveler") = [] := cardNamedOk "Leveler"
theorem prototypePrintedKey : Card.check (cardNamed "Prototype") = [] := cardNamedOk "Prototype"
theorem copyCharacteristicsDefault : (copyCharacteristics { power := 3 }) = copyCharacteristics { power := 3 } false := rfl

/-- error: Missing card field back -/
#guard_msgs in
example : Card := transforming { front := { name := "Front" } }
/-- error: Duplicate card field front -/
#guard_msgs in
example : Card := transforming { front := {}, front := {}, back := {} }
/-- error: Unknown card field extra -/
#guard_msgs in
example : Card := transforming { front := {}, back := {}, extra := {} }

end BundleDsl

namespace BundleFixture
load_bundle "test-fixtures/authoring-bundle"
example : (cardNamed "Bundle fixture").card.check = [] := cardNamedOk "Bundle fixture"
example : (cardNamed "Bundle fixture").authoring.onlyAuthored = true := (cardNamed "Bundle fixture").onlyAuthored
example : (bundleQuality : Predicate) = bundleAtom := rfl
example : Action.bundleShared = Action.ownScope (some (.action "bundleShared" .none)) bundleDraw := rfl
example : Ability.bundleShared = .keyword (.named "bundleShared") [] [spell Action.bundleShared] := rfl
end BundleFixture

open BundleDsl in
example : ([reverseShared 2] : List Ability) = [BundleDsl.Ability.reverseShared 2] := rfl
open BundleDsl in
example : (reverseShared : KeywordLabel) = KeywordLabel.named "reverse shared" := rfl
open BundleDsl in
example : (reverseShared : KeywordTerm) = theKeyword (.named "reverse shared") := rfl
open BundleDsl in
example : (keywordFirst : KeywordLabel) = KeywordLabel.named "keywordFirst" := rfl

namespace BundleAliases
load_bundle "test-fixtures/authoring-aliases"
example : aQuality = Templates.Predicate.and [Semantics.Predicate.isToken, Semantics.Predicate.isCard] := rfl
example : aSubject = NounPhrase.you := rfl
end BundleAliases

/-- error: bundle file 'test-fixtures/authoring-invalid-multiple/two.lean' must contain exactly one declaration, found 2 -/
#guard_msgs (error) in
load_bundle "test-fixtures/authoring-invalid-multiple"

/-- error: bundle file 'test-fixtures/authoring-invalid-empty/empty.lean' must contain exactly one declaration, found 0 -/
#guard_msgs (error) in
load_bundle "test-fixtures/authoring-invalid-empty"

namespace AbilityDeedBundle
load_bundle "test-fixtures/authoring-ability-deed"

example (subject : NounPhrase) : Action.bundleDevour subject =
    Action.ownScope (some (.ofAbility (.named "bundle devour")))
      (sacrifice (.firstMention "bundleDevour.subject" (.inCaller subject))) := rfl

example : bundleDevour = Ability.keyword (.named "bundle devour") []
    [spell (Action.bundleDevour thisCreature)] := rfl

/-- error: ofAbility cannot be combined with deed or permanent -/
#guard_msgs in
action invalidAbilityDeed := {
  ofAbility := bundleDevour, deed := "other", body := draw 1,
}
end AbilityDeedBundle

example (ability : Ability) : fatefulHour ability =
    abilityWord (.named "fateful hour") ability := rfl
example (ability : Ability) : councilsDilemma ability =
    abilityWord (.named "council’s dilemma") ability := rfl
example : typecycling.label = cycling.label := rfl
example : equipQuality.label = equip.label := rfl
example : hexproofFrom.label = hexproof.label := rfl
example : devourQuality.label = devour.label := rfl
example (subject : NounPhrase) : Definitions.Action.devour subject =
    Action.ownScope (some (.ofAbility devour))
      (sacrifice (.firstMention "devour.subject" (.inCaller subject))) := rfl

open Lean Authoring in
#guard match topoSortBundle [
  { key := ⟨`devour, `Semantics.Ability⟩, file := "ability.lean", references := [`entry] },
  { key := ⟨`entry, `Semantics.Ability⟩, file := "helper.lean", references := [`Action.devour] },
  { key := ⟨`devour, `Semantics.Action⟩, file := "action.lean" }
] with
| .ok items => items.map (·.key) ==
    [⟨`devour, `Semantics.Action⟩, ⟨`entry, `Semantics.Ability⟩, ⟨`devour, `Semantics.Ability⟩]
| .error _ => false

namespace KeywordActorScope
keyword repeatedActor := {
  body := [static (can theActor (draw 1)), static (cant theActor (draw 2))],
}
example : repeatedActor = Ability.keyword (.named "repeatedActor") []
    [static (can theActor (draw 1)), static (cant theActor (draw 2))] := rfl
end KeywordActorScope

example (ability : Ability) : (infinity ability : Ability) =
    Ability.keyword Definitions.Ability.infinity.label [.ability (.inCaller ability)]
      [static (onlyWhile (abilityGrant thisPermanent (.inCaller ability))
        (is thisPermanent (hasDesignation harnessed)))] := rfl
example (ability : Ability) : maxSpeed ability =
    Ability.keyword maxSpeed.label [.ability (.inCaller ability)]
      [static (onlyWhile (abilityGrant this_ (.inCaller ability))
        (.compareAmt (speedOf .you) .eq 4))] := rfl

end Semantics.Proofs.Authoring

namespace Semantics.Proofs.Authoring.RegistryBundle

open Semantics Semantics.Templates Semantics.Definitions

load_bundle "test-fixtures/authoring-registry"

theorem registryConferralMayReferToItsCatalogValue :
    bundleBearer_definition = .subtype bundleBearer [bundleConferral] := rfl

theorem appliedReturnTypeInBundle : bundlePredicates = [hasSubtype bundleBearer] := rfl

end Semantics.Proofs.Authoring.RegistryBundle

namespace Semantics.Proofs.Authoring.PrimitiveBoundary

open Semantics Semantics.Templates Semantics.Definitions

/-- error: Only primitive declarations may reference raw semantic constructor Semantics.Primitives.Action.skipPart -/
#guard_msgs in
template rawPrimitiveWrapper : Action := {
  body := Semantics.Primitives.Action.skipPart .combat 1
}

/-- error: Only primitive declarations may reference raw semantic constructor Semantics.Action.skipPart -/
#guard_msgs in
template rawAnonymousConstructor : Action := { body := .skipPart .combat 1 }

/-- error: Only primitive declarations may reference raw semantic constructor Semantics.Action.skipPart -/
#guard_msgs in
action rawActionConstructor := { body := .skipPart .combat 1 }

/-- error: Only primitive declarations may reference raw semantic constructor Semantics.Primitives.Action.skipPart -/
#guard_msgs in
template rawDefaultConstructor
    (effect : Action := Primitives.Action.skipPart .combat 1) : Action := {
  body := effect
}

/-- error: Only primitive declarations may reference raw semantic constructor Semantics.Predicate.and -/
#guard_msgs in
template rawConstructorAlias : Predicate := { body := Semantics.Predicate.and [] }

primitive skipCombatPrimitive : Action := {
  -- TODO(spelling): Test fixture for the constructor boundary.
  body := Semantics.Primitives.Action.skipPart .combat 1
}

template skipCombatTemplate : Action := {
  spelling := "skip your next combat phase",
  body := skipCombatPrimitive
}

theorem primitiveAndTemplatePreserveTheConstructor :
    skipCombatTemplate = Action.skipPart .combat 1 := rfl

template separateAlternatives (subject : NounPhrase) : Predicate := {
  body := Templates.Predicate.or [controlledBy subject, ownedBy subject]
}

/-- Each alternative introduces its own mention; neither branch borrows the other's identity. -/
theorem alternativeMentionsStayInTheirBranches (subject : NounPhrase) :
    separateAlternatives subject = Predicate.or
      [controlledBy (.firstMention "separateAlternatives.subject" subject),
       ownedBy (.firstMention "separateAlternatives.subject" subject)] := rfl

end Semantics.Proofs.Authoring.PrimitiveBoundary

namespace Semantics.Proofs.Authoring

open Lean Authoring in
#guard match topoSortBundle [
  { key := ⟨`timesPaid, `Semantics.Amount⟩, file := "amount.lean",
    references := [`timesPaid] },
  { key := ⟨`timesPaid, `Semantics.LinkedFacet⟩, file := "facet.lean", primitive := true }
] with
| .ok items => items.map (·.key) ==
    [⟨`timesPaid, `Semantics.LinkedFacet⟩, ⟨`timesPaid, `Semantics.Amount⟩]
| .error _ => false

theorem optionalStatDispatchesToAmount :
    (stat 3 : Option Amount) = Templates.Amount.stat 3 := rfl

theorem nonCallableLocalAllowsPrimitiveDispatch (name : String) :
    (name .sets name : CharacteristicEdit) = CharacteristicEdit.name .sets name := rfl

theorem callableLocalKeepsApplication (name : String → CharacteristicEdit) (value : String) :
    (name value : CharacteristicEdit) = name value := rfl

theorem overloadedNounCoercesToSubject :
    (a isCreature : Subject) = Subject.actor (Templates.NounPhrase.a isCreature) := rfl

namespace OrdinaryFunction
private def card (value : Nat) : Nat := value + 1

theorem ordinaryNamespaceFunctionKeepsApplication : card 3 = 4 := rfl
end OrdinaryFunction

end Semantics.Proofs.Authoring

namespace Semantics.Proofs.Authoring.BareWords
open Semantics Semantics.Templates Semantics.Definitions

theorem supertypeWord : (legendary : Supertype) = .legendary := rfl
theorem comparatorWord : (atLeast : Comparator) = .atLeast := rfl
theorem qualityOpWord : (sets : QualityOp) = .sets := rfl
theorem optionalTurnPartWord : (upkeep : Option TurnPart) = some .upkeep := rfl
theorem levelRangeWords : (between 1 2 : LevelRange) = .between 1 2 := rfl
theorem openLevelRangeWord : (plus 3 : LevelRange) = .plus 3 := rfl
theorem choiceWord : (unmarked : ChoiceMode) = .unmarked := rfl
theorem statusWord : (tapped : Status) = .tapped := rfl
theorem possessorWord : (controller : PossessorAxis) = .controller := rfl
theorem ordinalWord : (nth 2 : Ordinal) = .nth 2 := rfl
theorem dieSidesWord : (sides 6 : DieSides) = .sides 6 := rfl
theorem costNamedWord : (ofKeyword cycling : CostNamed) = .ofKeyword cycling := rfl
theorem projectionWord : (stat power : ProjAxis) = .stat .power := rfl
theorem nounWord : (ability : NounWord) = .ability := rfl
theorem roundingUpAndDeltaUp :
    half up (powerOf thisCreature) = half RoundMode.up (powerOf thisCreature) := rfl
theorem deltasKeepTheirDeclaration :
    getsPt it (up 2) (up 2) = getsPt it (Delta.up 2) (Delta.up 2) := rfl
/--
warning: Local variable 'up' resembles constructor 'Semantics.RoundMode.up' - write '.up' (with a dot) or 'Semantics.RoundMode.up' to use the constructor.

Note: This linter can be disabled with `set_option linter.constructorNameAsVariable false`
-/
#guard_msgs in
theorem compatibleLocalKeepsItsValue (up : RoundMode) : (up : RoundMode) = up := rfl
theorem compatibleOptionalLocalKeepsItsValue (up : Option RoundMode) :
    (up : Option RoundMode) = up := rfl
theorem incompatibleLocalAllowsWord (power : Amount) :
    (stat sets power power : CharacteristicEdit) = .stat .sets .power power := rfl

template white : Color := { body := white }
theorem wordInItsOwnDeclaration : white = Color.white := rfl
theorem colorSpecKeepsLeadingDot : (.some [white] : ColorSpec) = ColorSpec.some [.white] := rfl

/--
error: Type mismatch
  some ?m.2
has type
  Option ?m.1
but is expected to have type
  ColorSpec
-/
#guard_msgs in
example : ColorSpec := some [white]
end Semantics.Proofs.Authoring.BareWords
