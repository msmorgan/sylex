import Semantics
import Semantics.Registry

namespace Semantics.Templates

def keywordBare (label : KeywordLabel) : Ability := .keyword label [] []
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
def keywordNumbered (label : KeywordLabel) (amount : Amount) : Ability :=
  .keyword label [.number amount] []
/-- "<keyword> <quality> <cost>", e.g. "plainscycling {2}" [CR#702.29e] -/
def keywordQualityCosting (label : KeywordLabel) (quality : Predicate) (cost : Cost) :
    Ability :=
  .keyword label [.quality quality, .cost cost] []
/-- "<keyword> N—<cost>", e.g. "suspend 4—{1}{U}" -/
def keywordNumberCosting (label : KeywordLabel) (amount : Amount) (cost : Cost) :
    Ability :=
  .keyword label [.number amount, .cost cost] []

instance : AnyValue Subject := ⟨Semantics.Subject.any⟩
instance : AnyValue NounPhrase := ⟨Semantics.NounPhrase.any⟩
instance : AnyValue Ballot := ⟨Semantics.Ballot.anyBallot⟩
instance : AnyValue Cost := ⟨Semantics.Cost.anyCost⟩
instance : AnyValue Amount := ⟨Semantics.Amount.anyAmount⟩
instance : AnyValue CounterKindSource := ⟨Semantics.CounterKindSource.anyKind⟩
instance : AnyValue RollWatch := ⟨Semantics.RollWatch.anyResult⟩
instance : AnyValue Location := ⟨Semantics.Location.anyLocation⟩
instance : AnyValue Timing := ⟨Semantics.Timing.anyTiming⟩
instance (n : Nat) : OfNat ManaSymbol n := ⟨.simple (.generic n)⟩
instance (n : Nat) : OfNat (Option Amount) n := ⟨some (.lit n)⟩

end Semantics.Templates
