primitive characteristicEditAddedAbilitiesOf (source : Semantics.NounPhrase) (classes : List Semantics.AbilityClass) (except : Option Semantics.Predicate) : Semantics.CharacteristicEdit := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CharacteristicEdit.addedAbilitiesOf source classes except
}
