/-- "Airbend <subject>" (`Definition`, [CR#701.65a]): per-card `doForEach` loop over
`allOf(and([card, did(any, exile(which), thisWay)]))` where `it` resolves by `D′` to the
singular loop referent. -/
action airbend (subject : NounPhrase) : Action := {
  lexical := { verbs := [{ lemma := "airbend", past := "airbent", frames := [Monotransitive, Intransitive] }] },
  spelling := "airbend <subject>",
  body :=
    sequentially
        [ exile subject,
          .doForEach (allOf (and [.isCard, .did { subj := .any
                                                  action := exile .which
                                                  within := some .thisWay }]))
            (instruct (ownerOf it)
              (.establish
                (can (ownerOf it)
                  (cast it (from_ := exileZone)
                    (payment := .payingInstead (.mana [generic 2]))))
                (some (.forAsLongAs (is it (.in_ exileZone)))))) ]
}
