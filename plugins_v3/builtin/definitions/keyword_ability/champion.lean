keyword champion (subject : Predicate) : Ability := {
  lexical := {
    lemma := "champion",
    parameters := [[Quality (None)]],
    verbs := [{ lemma := "champion", frames := [Monotransitive] }]
  },
  -- TODO(spelling): Champion a creature / champion an Elf; the quality needs context-dependent
  -- article or number inflection, not literal substitution.
  body :=
    [ when (does thisPermanent enter)
        (mayOrElse
          (pay
            (.perform (exile (a (and [subject, youControl, isOther]))))
            .once)
          (sacrifice thisPermanent)),
      when (does thisPermanent leave)
        (returnToBattlefield (the (and [was (exile which), isCard]))
          [.under (ownerOf it)]) ]
}
