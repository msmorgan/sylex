/-- Authoring primitive for `CharacteristicEdit.addedAbilitiesOf`. -/
primitive addedAbilitiesOf (source : NounPhrase) (classes : List AbilityClass)
    (except : Option Predicate) : CharacteristicEdit := {
  body := Semantics.Primitives.CharacteristicEdit.addedAbilitiesOf source classes except
}
