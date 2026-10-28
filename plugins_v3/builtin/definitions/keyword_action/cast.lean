/-- "cast <subject> [from <from_>] [paying <payment>]" (`Definition`, [CR#701.5a]). -/
action cast (subject : NounPhrase) (from_ : Option Location := none)
    (payment : PlayPayment := itsOwnCost)
    (resulting : Option Predicate := none) : Action := {
  -- TODO(spelling): cast <subject> [from <from_>] [paying <payment>] [<resulting>]; string
  -- substitution cannot branch on optional fields or supply contextual agreement and morphology.
  lexical := { verbs := [{ lemma := "cast", past := "cast", frames := [Monotransitive] }] },
  body :=
    procedure Semantics.Procedure.cast subject payment resulting from_ none
}
