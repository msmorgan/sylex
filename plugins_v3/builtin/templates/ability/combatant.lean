/-- What the creature type confers [CR#508.1a]: its object is a Combatant. -/
-- [CR#508.1a]
template combatant : Conferral := {
  -- TODO(spelling): What the creature type confers : its object is a Combatant.
  body :=
    Semantics.Conferral.property (Semantics.Templates.StaticSpec.staticSpecCombatant Semantics.Templates.NounPhrase.this_)
}
