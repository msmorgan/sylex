/-- "play <subject> [from <from_>] [paying <payment>]" (`Definition`, [CR#701.18a]). -/
action play (subject : NounPhrase) (from_ : Option Location := none)
    (payment : PlayPayment := itsOwnCost)
    (resulting : Option Predicate := none) : Action := {
  -- TODO(spelling): Include or omit the clauses supplied by from_, resulting.
  lexical := { verbs := [{ lemma := "play", frames := [Monotransitive, Intransitive] }] },
  body :=
    procedure .play subject payment resulting from_ none
}
