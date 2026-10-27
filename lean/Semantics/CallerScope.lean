import Semantics.Abilities

/-!
# Semantics.CallerScope

Caller-scope wrapping (`inCaller`) for `Definition` arguments in Semantics v3
(`docs/decisions/semantics-v3.md` §1.4, §2). `withBindings`, `CaptureInput`, and
`Amount.parameter` are retired. Definition `NounPhrase` parameter identities (`BindingId`)
are generated as outermost `firstMention` / `laterMention` around `inCaller`; a written
`NounPhrase.inCaller p` opts that occurrence out. The `action` and `keyword` commands wrap
parameters through these instances; `template` preserves their caller scope.
-/

namespace Semantics

/-- A play payment read in the caller's Reference Scope: its cost, when it names one. -/
def PlayPayment.inCaller : PlayPayment → PlayPayment
  | .payingInstead cost => .payingInstead (.inCaller cost)
  | .plusAdditional cost => .plusAdditional (.inCaller cost)
  | .faceDown spec cost => .faceDown spec (.inCaller cost)
  | .transformed cost => .transformed (.inCaller cost)
  | payment => payment


class CallerScope (α : Type) where
  caller : α → α

instance : CallerScope Action where
  caller := Action.inCaller

instance : CallerScope Cost where
  caller := Cost.inCaller

instance : CallerScope StaticSpec where
  caller := StaticSpec.inCaller

instance : CallerScope Replacement where
  caller := Replacement.inCaller

instance : CallerScope Amount where
  caller := Amount.inCaller

instance : CallerScope Predicate where
  caller := Predicate.inCaller

instance : CallerScope Quantity where
  caller := Quantity.inCaller

instance : CallerScope Location where
  caller := Location.inCaller

instance : CallerScope Condition where
  caller := Condition.inCaller

instance : CallerScope NounPhrase where
  caller := NounPhrase.inCaller

instance : CallerScope GameEvent where
  caller := GameEvent.inCaller

instance : CallerScope Ability where
  caller := Ability.inCaller

instance : CallerScope TokenSpec where
  caller := TokenSpec.inCaller

instance : CallerScope MoveRider where
  caller := MoveRider.inCaller

instance {α : Type} [CallerScope α] : CallerScope (Option α) where
  caller := Option.map CallerScope.caller

instance {α : Type} [CallerScope α] : CallerScope (List α) where
  caller := List.map CallerScope.caller

instance : CallerScope PlayPayment where
  caller := PlayPayment.inCaller

instance : CallerScope Ballot where
  caller
    | .byCandidate candidates => .byCandidate (CallerScope.caller candidates)
    | ballot => ballot

class CallerSplice (α : Type) where
  close : α → α

instance (priority := 100) {α : Type} [CallerScope α] : CallerSplice α where
  close := CallerScope.caller

instance {α β : Type} [CallerSplice β] : CallerSplice (α → β) where
  close body := fun argument => CallerSplice.close (body argument)

end Semantics
