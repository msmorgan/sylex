/-- "cast <subject> [from <from_>] [paying <payment>]" (`Definition`, [CR#701.5a]). -/
action cast (subject : NounPhrase) (from_ : Option Location := none)
    (payment : PlayPayment := itsOwnCost)
    (resulting : Option Predicate := none) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by from_, resulting.
  lexical := { verbs := [{ lemma := "cast", past := "cast", frames := [Monotransitive] }] },
  body :=
    procedure .cast subject payment resulting from_ none
}
