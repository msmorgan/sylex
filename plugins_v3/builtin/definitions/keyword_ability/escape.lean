keyword escape (cost : Cost) : Ability := {
  lexical := {
    lemma := "escape",
    parameters := [[DashNonManaCost]],
    verbs := [{ lemma := "escape", frames := [Intransitive, Prepositional (["with"])] }]
  },
  spelling := "escape—<cost>.",
  body :=
    [ static
        (can you
          (cast this_ (from_ := yourGraveyard) (payment := .payingInstead cost))) ]
}
