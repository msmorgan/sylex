template thisAura : NounPhrase := {
  spelling := "this Aura",
  body :=
    Semantics.Templates.NounPhrase.headed (Semantics.NounWord.subtype Semantics.Definitions.aura) Semantics.Templates.NounPhrase.this_
}
