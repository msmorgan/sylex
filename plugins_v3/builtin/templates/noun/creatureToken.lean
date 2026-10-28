template creatureToken (power toughness : Nat) (colors : List Color)
    (subtypes : List Subtype) (abilities : List Ability := []) : CharacteristicBundle := {

  -- TODO(spelling): <power>/<toughness> <colors> <subtypes> creature token with <abilities>;
  -- optional colors/abilities, coordination, and token number require conditional realization.
  body :=
    creatureTokenOf (lit ↑power) (lit ↑toughness) colors subtypes
      abilities
}
