keyword indestructible : Ability := {
  lexical := { lemma := "indestructible" },
  spelling := "indestructible",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.cant Semantics.Templates.Subject.any (Semantics.Definitions.destroy Semantics.Templates.NounPhrase.thisPermanent))]
}
