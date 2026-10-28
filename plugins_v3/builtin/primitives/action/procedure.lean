/-- Carry out a rules procedure on the subject: activate, cast, or play
[CR#701.2a,701.5a,701.18a], or name its resolution as an event [CR#608.2]. -/
primitive procedure (kind : Procedure) (subject : NounPhrase) (payment : PlayPayment)
    (resulting : Option Predicate := none) (from_ : Option Location := none)
    (of_ : Option NounPhrase := none) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.procedure kind subject payment resulting from_ of_
}
