-- [CR#110.5]
template phasedOut : Status := {
  lexical := { prepositions := [{ lemma := "phased out", frames := [{ complement := Intransitive, hosts := NominalAndPredicate }] }] },
  spelling := "phased out",
  body :=
    Semantics.Status.phasedOut
}
