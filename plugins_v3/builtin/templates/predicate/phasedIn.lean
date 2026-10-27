-- [CR#110.5]
template phasedIn : Status := {
  lexical := { prepositions := [{ lemma := "phased in", frames := [{ complement := Intransitive, hosts := NominalAndPredicate }] }] },
  spelling := "phased in",
  body :=
    Semantics.Status.phasedIn
}
