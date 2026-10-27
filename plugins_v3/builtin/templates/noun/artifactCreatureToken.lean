template artifactCreatureToken (power toughness : Nat) (colors : List Color)
    (subtypes : List Subtype) (abilities : List Ability := []) : CharacteristicBundle := {
  -- TODO(spelling): <power>/<toughness> <colors> <subtypes> artifact creature token with
  -- <abilities>; optional colors/abilities, coordination, and token number require conditional
  -- realization.
  body :=
    { characteristics :=
        { colors, types := [.artifact, .creature], subtypes, text := abilities,
          power := some (.lit power), toughness := some (.lit toughness) } }
}
