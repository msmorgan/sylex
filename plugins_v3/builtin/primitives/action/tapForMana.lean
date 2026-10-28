/-- "<subject> taps <permanent> for mana [of <type>]" [CR#106.12a] -/
-- [CR#106.12a]
primitive tapForMana (permanent : NounPhrase) (type : Option ManaTypeTerm := none) :
    Action := {
  -- TODO(spelling): Include or omit the clauses supplied by type.
  body :=
    Semantics.Primitives.Action.tapForMana permanent type
}
