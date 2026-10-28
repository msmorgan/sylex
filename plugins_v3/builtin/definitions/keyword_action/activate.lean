/-- "activate <subject> [from <from_>] [paying <payment>]" (`Definition`, [CR#701.2a]). -/
action activate (subject : NounPhrase) (from_ : Option Location := none)
    (payment : PlayPayment := itsOwnCost)
    (resulting : Option Predicate := none) : Action := {
  -- TODO(spelling): activate <subject> [from <from_>] [paying <payment>] [<resulting>]; string
  -- substitution cannot branch on optional fields or supply contextual agreement and morphology.
  lexical := { verbs := [{ lemma := "activate", frames := [Monotransitive, Intransitive] }] },
  body :=
    procedure .activate subject payment resulting from_ none
}
