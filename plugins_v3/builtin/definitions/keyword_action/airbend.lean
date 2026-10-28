/-- "Airbend <subject>" (`Definition`, [CR#701.65a]): per-card `doForEach` loop over
`allOf(and([card, did(any, exile(which), thisWay)]))` where `it` resolves by `D′` to the
singular loop referent. -/
action airbend (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "airbend", past := "airbent", frames := [Monotransitive, Intransitive] }] },
  spelling := "airbend <subject>",
  body :=
    sequentially
      [Semantics.Definitions.exile subject,
        doForEach
          (allOf
            (and
              [isCard,
                predicateDid
                  { subj := Templates.Subject.any, action := Semantics.Definitions.exile which,
                    within := some thisWay }]))
          (instruct (ownerOf it)
            (establish
              (can (actor (ownerOf it))
                (cast it (some exileZone)
                  (payingInstead (mana [generic 2]))))
              (some
                (forAsLongAs
                  (is it
                    (in_ exileZone))))))]
}
