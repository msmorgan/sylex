template creatureToken (power toughness : Nat) (colors : List Color)
    (subtypes : List Subtype) (abilities : List Ability := []) : CharacteristicBundle := {

  -- TODO(spelling): <power>/<toughness> <colors> <subtypes> creature token with <abilities>;
  -- optional colors/abilities, coordination, and token number require conditional realization.
  body :=
    Semantics.Templates.CharacteristicBundle.creatureTokenOf (Semantics.Templates.Amount.amountLit ↑power) (Semantics.Templates.Amount.amountLit ↑toughness) colors subtypes
      abilities
}
