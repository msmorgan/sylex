keyword haunt : Ability := {
  lexical := { lemma := "haunt", verbs := [{ lemma := "haunt", frames := [Monotransitive] }] },
  spelling := "haunt",
  body :=
    [ byCardClass
        (when (does thisPermanent die)
          (sequentially
            [ exile it,
              .relate it haunting (target isCreature) ]))
        (when
          (.causes (.event (does .any (resolve thisSpell)))
            (does .any (putInto thisSpell graveyard (from_ := stack))))
          (sequentially
            [ exile it,
              .relate it haunting (target isCreature) ])) ]
}
