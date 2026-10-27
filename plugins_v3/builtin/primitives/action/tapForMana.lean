/-- "<subject> taps <permanent> for mana [of <type>]" [CR#106.12a] -/
-- [CR#106.12a]
primitive tapForMana (permanent : NounPhrase) (type : Option ManaTypeTerm := none) :
    Action := {
  -- TODO(spelling): Context-dependent wording: <subject> taps <permanent> for mana [of <type>]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.Action.tapForMana permanent type
}
