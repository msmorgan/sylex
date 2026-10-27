/-- A token's characteristics from the parts a creature token names, with the abilities it is
created with ("… with flying") [CR#111.3]. -/
template creatureTokenOf (power toughness : Amount) (colors : List Color)
    (subtypes : List Subtype)
    (abilities : List Ability := []) : CharacteristicBundle := {
  -- TODO(spelling): <power>/<toughness> <colors> <subtypes> creature token with <abilities>;
  -- optional colors/abilities, coordination, and token number require conditional realization.
  body :=
    { characteristics :=
        { colors, types := [.creature], subtypes, text := abilities, power := power,
          toughness := toughness } }
}
