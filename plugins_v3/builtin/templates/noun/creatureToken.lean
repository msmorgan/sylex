template creatureToken (power toughness : Nat) (colors : List Color)
    (subtypes : List Subtype) (abilities : List Ability := []) : CharacteristicBundle := {

  -- TODO(spelling): Format and coordinate the colors, subtypes, abilities list entries, including
  -- the empty case.
  body :=
    creatureTokenOf (lit ↑power) (lit ↑toughness) colors subtypes
      abilities
}
