primitive procedure (kind : Procedure) (subject : NounPhrase) (payment : PlayPayment)
    (resulting : Option Predicate := none) (from_ : Option Location := none)
    (of_ : Option NounPhrase := none) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Primitives.Action.procedure kind subject payment resulting from_ of_
}
