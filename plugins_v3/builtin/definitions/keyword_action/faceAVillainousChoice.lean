action faceAVillainousChoice (first second : Action) : Action := {
  lexical := { verbs := [{ lemma := "face", frames := [Monotransitive, (TransitiveWithComplement SetOffClause)] }] },
  spelling := "face a villainous choice — <first> or <second>",
  body :=
    chooseOne first second
}
