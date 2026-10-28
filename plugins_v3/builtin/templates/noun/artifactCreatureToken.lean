/-- "a P/T <colors> <subtypes> artifact creature token [with <abilities>]":
`creatureToken` with the artifact card type beside creature. -/
template artifactCreatureToken (power toughness : Nat) (colors : List Color)
    (subtypes : List Subtype) (abilities : List Ability := []) : CharacteristicBundle := {

  -- TODO(spelling): Format and coordinate the colors, subtypes, abilities list entries, including
  -- the empty case.
  body :=
    {
      characteristics :=
        { colors := colors, types := [Semantics.CardType.artifact, Semantics.CardType.creature], subtypes := subtypes,
          text := abilities, power := some (lit ↑power),
          toughness := some (lit ↑toughness) } }
}
