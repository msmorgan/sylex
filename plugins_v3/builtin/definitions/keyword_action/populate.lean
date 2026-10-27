action populate : Action := {
  lexical := { verbs := [{ lemma := "populate", frames := [Intransitive] }] },
  spelling := "populate",
  body :=
    sequentially
        [ choose (a (and [.isToken, isCreature, isPermanent, actorControls])),
          create 1 (copyOf thatToken) ]
}
