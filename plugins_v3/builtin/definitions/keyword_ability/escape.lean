keyword escape (cost : Cost) : Ability := {
  lexical := {
    lemma := "escape",
    parameters := [[DashNonManaCost]],
    verbs := [{ lemma := "escape", frames := [Intransitive, Prepositional (["with"])] }]
  },
  spelling := "escape—<cost>.",
  body :=
    [static
        (can (actor you)
          (cast this_ (some yourGraveyard)
            (payingInstead cost)))]
}
