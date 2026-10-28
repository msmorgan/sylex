keyword haunt : Ability := {
  lexical := { lemma := "haunt", verbs := [{ lemma := "haunt", frames := [Monotransitive] }] },
  spelling := "haunt",
  body :=
    [byCardClass
        (when
          (does (actor thisPermanent) die)
          (sequentially
            [exile it,
              relate it haunting
                (target isCreature)]))
        (when
          (causes
            (byEvent
              (does any
                (resolve thisSpell)))
            (does any
              (putInto thisSpell graveyard
                (some stack))))
          (sequentially
            [exile it,
              relate it haunting
                (target isCreature)]))]
}
