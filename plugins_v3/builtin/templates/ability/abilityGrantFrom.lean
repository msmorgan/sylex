/-- "<subject> has all <classes> abilities of <source> [except <except>]" -/
template abilityGrantFrom (subject source : NounPhrase)
    (classes : List AbilityClass := []) (except : Option Predicate := none) : StaticSpec := {
  -- TODO(spelling): Context-dependent wording: <subject> has all <classes> abilities of <source> [except <except>]; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Templates.StaticSpec.characteristicChange subject [Semantics.Templates.CharacteristicEdit.characteristicEditAddedAbilitiesOf source classes except]
}
