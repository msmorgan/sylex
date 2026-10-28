primitive addedAbilitiesOf (source : NounPhrase) (classes : List AbilityClass)
    (except : Option Predicate) : CharacteristicEdit := {
  -- TODO(spelling): Structural constructor; surface wording belongs to its phrasing templates.
  body := Semantics.Primitives.CharacteristicEdit.addedAbilitiesOf source classes except
}
