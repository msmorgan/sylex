template gain (subject : NounPhrase) (ability : Ability)
    (duration : Option Duration := none) : Action := {
  -- TODO(spelling): No fixed surface wording; this helper composes semantic structure.
  body :=
    Semantics.Templates.Action.establish
      (Semantics.Templates.StaticSpec.characteristicChange subject [Semantics.Templates.CharacteristicEdit.characteristicEditAddedAbilities [ability]]) duration
}
