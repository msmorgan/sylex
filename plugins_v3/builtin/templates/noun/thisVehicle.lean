template thisVehicle : NounPhrase := {
  spelling := "this Vehicle",
  body :=
    Semantics.Templates.NounPhrase.headed (Semantics.NounWord.subtype Semantics.Definitions.vehicle) Semantics.Templates.NounPhrase.this_
}
