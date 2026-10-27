template thisEquipment : NounPhrase := {
  spelling := "this Equipment",
  body :=
    Semantics.Templates.NounPhrase.headed (Semantics.NounWord.subtype Semantics.Definitions.equipment) Semantics.Templates.NounPhrase.this_
}
