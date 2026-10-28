import Semantics
import Semantics.CallerScope

/-!
# Semantics.Primitives

The authoring boundary's primitive layer: the characteristic inputs that quality wording
expands through, and the generated constructor wrappers (`declare_semantic_primitives`, under
`Semantics.Primitives.<Type>`). Worded phrasings over a primitive are templates
(`Semantics.Templates`).
-/

namespace Semantics

/-- Convenience input for established quality wording, `op` written once over it. Expansion
produces typed edits; this input is not stored in the checked static or copy syntax. -/
inductive QualityPayload where
  /-- `characteristics`, each written with `op`. A `retained` type ("It's still a land") keeps
  every prior type, supertype and subtype [CR#205.1b], so the type line is added, whatever `op`
  writes the rest; the type it names is wording only. -/
  | bundle (characteristics : CharacteristicBundle) (retained : Option CardType)
  | quality (quality : WrittenQuality)
  | colored (colors : ColorSpec)

attribute [semantic_expression] QualityPayload

private def nonempty {α : Type} (xs : List α) : Option (List α) :=
  if xs.isEmpty then none else some xs

def Characteristics.typeChanges (c : Characteristics) : TypeLineChanges :=
  ⟨nonempty c.supertypes, nonempty c.types, nonempty c.subtypes⟩

/-- The value characteristics `c` writes, each with `op`: an addition sets a written value,
as "becomes a 2/2 creature in addition to its other types" does, and a removal removes a name,
colors, abilities and the mana cost (`manaCost none`). A number is not removed: a removal writes
a power, toughness, loyalty or defense as lowered by it (`stat .loses`), as "gets −N" is. No
expansion is checked syntax, so nothing refuses that; no card writes a removed number. -/
def Characteristics.valueEdits (c : Characteristics) (op : QualityOp) : List CharacteristicEdit :=
  let valueOp := if op == .loses then .loses else .sets
  c.name.toList.map (.name valueOp) ++
    c.cost.toList.map (fun cost => .manaCost (if op == .loses then none else some cost)) ++
    (if c.colors.isEmpty then [] else [.colors valueOp (.some c.colors)]) ++
    c.power.toList.map (.stat valueOp .power) ++
    c.toughness.toList.map (.stat valueOp .toughness) ++
    c.loyalty.toList.map (.stat valueOp .loyalty) ++
    c.defense.toList.map (.stat valueOp .defense) ++
    (if c.text.isEmpty then []
     else if op == .loses then [.removedAbilities (.specified (c.text.map .written))]
     else [.addedAbilities c.text])

def QualityPayload.edits (op : QualityOp) : QualityPayload → List CharacteristicEdit
  | .bundle b retained =>
    .typeLine (if retained.isSome then .adds else op) b.characteristics.typeChanges ::
      b.characteristics.valueEdits op ++ b.qualities.map (.quality op)
  | .quality q => [.quality op q]
  | .colored colors => [.colors op colors]

/-- A ballot read in the caller's Reference Scope: its candidates, when it names them. -/
def Ballot.inCaller : Ballot → Ballot
  | .byCandidate candidates => .byCandidate (.inCaller candidates)
  | ballot => ballot


end Semantics

/-! Named wrappers for exposed primitives. Generated wrappers preserve constructor signatures;
the authoring boundary records their names. -/

declare_semantic_primitives



namespace Semantics.Templates

/-- A bare noun phrase in a subject's place is the active voice: `cant you …`. -/
@[«semantic_template»] private def coeNounPhraseSubject : NounPhrase → Subject :=
  fun np => Semantics.Primitives.Subject.actor np

instance [AuthoringContext] : Coe NounPhrase Subject := ⟨coeNounPhraseSubject⟩
instance [AuthoringContext] : AnyValue Subject := ⟨Semantics.Primitives.Subject.any⟩
instance [AuthoringContext] : AnyValue NounPhrase := ⟨Semantics.Primitives.NounPhrase.any⟩
instance [AuthoringContext] : AnyValue Ballot := ⟨Semantics.Primitives.Ballot.anyBallot⟩
instance [AuthoringContext] : AnyValue Cost := ⟨Semantics.Primitives.Cost.anyCost⟩
instance [AuthoringContext] : AnyValue Amount := ⟨Semantics.Primitives.Amount.anyAmount⟩
instance [AuthoringContext] : AnyValue CounterKindSource := ⟨Semantics.Primitives.CounterKindSource.anyKind⟩
instance [AuthoringContext] : AnyValue RollWatch := ⟨Semantics.Primitives.RollWatch.anyResult⟩
instance [AuthoringContext] : AnyValue Location := ⟨Semantics.Primitives.Location.anyLocation⟩
instance [AuthoringContext] : AnyValue Timing := ⟨Semantics.Primitives.Timing.anyTiming⟩

open scoped Semantics.Authoring

/-- A keyword ability reads as the keyword it is. -/
private def coeAbilityKeywordTerm : Ability → KeywordTerm :=
  fun ability => match ability with
  | Semantics.Ability.keyword label _ _ => Semantics.KeywordTerm.the label
  | _ => Semantics.KeywordTerm.the (Semantics.KeywordLabel.named "")

instance [AuthoringContext] : Coe Ability KeywordTerm := ⟨coeAbilityKeywordTerm⟩

open Lean Elab Term Meta in
/-- A keyword ability's Definition names its keyword wherever a `KeywordLabel` is expected:
`keywordCostPaid kicker it`, `abilityHead (.keyword cycling)`, `.anyIn ⟨protection, none⟩`. The
identifier elaborates as the authored label constant the Definition generates (`kicker.label`,
`Authoring.keywordLabelConstant`), so a card never spells the internal `KeywordLabel.named` and a
misspelled keyword is no identifier at all. Any other identifier, or an expected type other than `KeywordLabel`, is left to the
ordinary elaborator. -/
@[scoped term_elab ident]
def elabKeywordName : TermElab := fun stx expectedType? => do
  let some expected := expectedType? | throwUnsupportedSyntax
  let expected ← instantiateMVars expected
  unless expected.getAppFn.isMVar || (← whnfR expected).isConstOf ``Semantics.KeywordLabel do
    throwUnsupportedSyntax
  if (← getLCtx).findFromUserName? stx.getId |>.isSome then throwUnsupportedSyntax
  let candidates ← try resolveGlobalConst stx catch _ => throwUnsupportedSyntax
  let env ← getEnv
  let some definition := candidates.find? (Authoring.definitionAttribute.hasTag env)
    | throwUnsupportedSyntax
  let label := Authoring.keywordLabelConstant definition
  unless env.contains label do throwUnsupportedSyntax
  if expected.getAppFn.isMVar then
    tryPostpone
    throwUnsupportedSyntax
  return mkConst label

open Lean Elab Term Meta in
/-- An Action Definition names its keyword action wherever a `Deed` is expected:
`.deed exile`. The identifier elaborates as the authored deed constant the
Definition generates (`exile.deed`, `Authoring.deedConstant`), so a pin or a Definition never
spells the internal `Deed.action` and a misspelled action is no identifier at all. Any other
identifier, or an expected type other than `Deed`, is left to the ordinary elaborator. -/
@[scoped term_elab ident]
def elabDeedName : TermElab := fun stx expectedType? => do
  let some expected := expectedType? | throwUnsupportedSyntax
  let expected ← instantiateMVars expected
  unless expected.getAppFn.isMVar || (← whnfR expected).isConstOf ``Semantics.Deed do
    throwUnsupportedSyntax
  if (← getLCtx).findFromUserName? stx.getId |>.isSome then throwUnsupportedSyntax
  let candidates ← try resolveGlobalConst stx catch _ => throwUnsupportedSyntax
  let env ← getEnv
  let some definition := candidates.find? (Authoring.definitionAttribute.hasTag env)
    | throwUnsupportedSyntax
  let deed := Authoring.deedConstant definition
  unless env.contains deed do throwUnsupportedSyntax
  if expected.getAppFn.isMVar then
    tryPostpone
    throwUnsupportedSyntax
  return mkConst deed
/-- A subtype is a predicate on objects of it: `goblin` for `hasSubtype goblin`. -/
private def coeSubtypePredicate : Subtype → Predicate :=
  fun s => Semantics.Primitives.Predicate.hasSubtype s

instance [AuthoringContext] : Coe Subtype Predicate := ⟨coeSubtypePredicate⟩

/-- `cost` as what paying it pays. Paying a cost carries out its instructions [CR#118.1], so where
the cost is itself one plain payment ("Ward—Pay 5 life", "unless you pay 1 life"), paying it is that
payment, made once: `pay (payLife 5)` is `payLife 5`, never a payment of a payment. Its Reference
Scope (`inCaller`), its scaling ("for each age counter") and the other parts of a compound cost
are kept. -/
def _root_.Semantics.Cost.asPaid (cost : Cost) (fuel : Nat := 64) : Cost :=
  match fuel, cost with
  | _ + 1, .perform (.pay inner .once .paid none none) => inner
  | n + 1, .inCaller body => .inCaller (body.asPaid n)
  | n + 1, .scaled body amount => .scaled (body.asPaid n) amount
  | n + 1, .compound costs => .compound (costs.map (·.asPaid n))
  | _, cost => cost
termination_by structural fuel

/-- A keyword ability with no parameters and no written definition, by its label. This and the
`keyword…` helpers below are not Templates: they spell `Ability.keyword`, an internal expansion
carrier, so they serve Definitions and fixtures, and `spelled` refuses them in a card, which
writes the keyword's typed Definition instead. -/
def keyword (label : KeywordLabel) : Ability := .keyword label [] []
/-- "<keyword> <cost>" -/
def keywordCosting (label : KeywordLabel) (cost : Cost) : Ability :=
  .keyword label [.cost cost] []
/-- "<keyword> <quality>", e.g. "protection from red" -/
def keywordQuality (label : KeywordLabel) (quality : Predicate) : Ability :=
  .keyword label [.quality quality] []
/-- "<keyword> <subject>", e.g. "enchant creature" -/
def keywordSubject (label : KeywordLabel) (subject : Predicate) : Ability :=
  .keyword label [.subject subject] []
/-- "<keyword> N", e.g. "bushido 2" -/
def keywordNumber (label : KeywordLabel) (amount : Amount) : Ability :=
  .keyword label [.number amount] []
/-- "<keyword> <quality> <cost>", e.g. "plainscycling {2}" [CR#702.29e] -/
def keywordQualityCosting (label : KeywordLabel) (quality : Predicate) (cost : Cost) :
    Ability :=
  .keyword label [.quality quality, .cost cost] []
/-- "<keyword> N—<cost>", e.g. "suspend 4—{1}{U}" -/
def keywordNumberCosting (label : KeywordLabel) (amount : Amount) (cost : Cost) :
    Ability :=
  .keyword label [.number amount, .cost cost] []

/-- A written mana cost is a cost: `[generic 1, pip .black]` for `mana [generic 1, pip .black]`. -/
private def coeManaCostCost : ManaCost → Cost :=
  fun cost => Semantics.Primitives.Cost.mana cost

instance [AuthoringContext] : Coe ManaCost Cost := ⟨coeManaCostCost⟩
/-- An action is the cost of performing it: `sacrifice thisArtifact` for
`perform (sacrifice thisArtifact)`. -/
private def coeActionCost : Action → Cost :=
  fun action => Semantics.Primitives.Cost.perform action

instance [AuthoringContext] : Coe Action Cost := ⟨coeActionCost⟩
private def coeCharacteristicBundleTokenSpec : CharacteristicBundle → TokenSpec :=
  fun characteristics => Semantics.Primitives.TokenSpec.written characteristics

instance [AuthoringContext] : Coe CharacteristicBundle TokenSpec := ⟨coeCharacteristicBundleTokenSpec⟩

/-- A bare duration is the span over it: `did you attack soFarThisTurn`. -/
private def coeDurationLookbackSpan : Duration → LookbackSpan :=
  fun d => Semantics.Primitives.LookbackSpan.duration d none

instance [AuthoringContext] : Coe Duration LookbackSpan := ⟨coeDurationLookbackSpan⟩
/-- A bare duration is a written span over it, where the span is optional. -/
instance [AuthoringContext] : Coe Duration (Option LookbackSpan) := ⟨fun d => some (Semantics.Primitives.LookbackSpan.duration d none)⟩

/-- A printed counter kind reads as its counter source. -/
private def coeCounterKindCounterKindSource : CounterKind → CounterKindSource :=
  fun kind => Semantics.Primitives.CounterKindSource.printed kind

instance [AuthoringContext] : Coe CounterKind CounterKindSource := ⟨coeCounterKindCounterKindSource⟩

/-- A printed name reads as its name source: `named "Rune Snag"`. -/
private def coeStringNameSource : String → NameSource :=
  fun name => Semantics.Primitives.NameSource.printed name

instance [AuthoringContext] : Coe String NameSource := ⟨coeStringNameSource⟩

/-- A condition reads as the "as though" it states. -/
private def coeConditionAsThough : Condition → AsThough :=
  fun cond => Semantics.Primitives.AsThough.condition cond

instance [AuthoringContext] : Coe Condition AsThough := ⟨coeConditionAsThough⟩

private def coeColorManaSymbol : Color → ManaSymbol :=
  fun color => Semantics.ManaSymbol.simple (Semantics.SimpleManaSymbol.specific (Semantics.ColorOrColorless.of color))

instance [AuthoringContext] : Coe Color ManaSymbol := ⟨coeColorManaSymbol⟩
instance [AuthoringContext] (n : Nat) : OfNat ManaSymbol n := ⟨.simple (.generic n)⟩
instance [AuthoringContext] (n : Nat) : OfNat (Option Amount) n := ⟨some (.lit n)⟩
instance : Coe Color ColorOrColorless := ⟨.of⟩
instance : Coe Characteristics CharacteristicBundle := ⟨fun c => { characteristics := c }⟩
private def coeStaticSpecAbility : StaticSpec → Ability :=
  fun spec => Semantics.Primitives.Ability.static spec

instance [AuthoringContext] : Coe StaticSpec Ability := ⟨coeStaticSpecAbility⟩

end Semantics.Templates

namespace Semantics.Templates
open Lean Elab Term
@[scoped term_elab Lean.Parser.Term.app]
def elabExpectedTypeDispatch : TermElab := Authoring.dispatchOnExpectedType
@[scoped term_elab ident]
def elabBareIdentDispatch : TermElab := Authoring.elabBareIdentDispatch
@[scoped term_elab Lean.Parser.Term.app]
def elabDomainList : TermElab := Authoring.elabDomainList
@[scoped term_elab ident]
def elabEmptyDomainList : TermElab := Authoring.elabDomainList
@[scoped term_elab Lean.Parser.Term.structInst]
def elabCharacteristicBundleStruct : TermElab := Authoring.elabCharacteristicBundleStruct
@[scoped term_elab ident]
def elabClosedWordIdent : TermElab := Authoring.elabClosedWordIdent
@[scoped term_elab Lean.Parser.Term.app]
def elabClosedWordApp : TermElab := Authoring.elabClosedWordApp
@[scoped term_elab Lean.Parser.Term.app]
def elabCardRecord : TermElab := Authoring.elabCardRecord
end Semantics.Templates
