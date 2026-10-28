keyword indestructible : Ability := {
  lexical := { lemma := "indestructible" },
  spelling := "indestructible",
  body :=
    [static
        (cant Templates.Subject.any (destroy thisPermanent))]
}
