/-- "Champion an [object]" means "When this permanent enters, sacrifice it unless you exile another
[object] you control" and "When this permanent leaves the battlefield, return the exiled card to
the battlefield under its owner's control" [CR#702.72a] -/
keyword champion (subject : Predicate) : Ability := {
  lexical := {
    lemma := "champion",
    parameters := [[Quality (None)]],
    verbs := [{ lemma := "champion", frames := [Monotransitive] }]
  },
  -- TODO(spelling): Champion a creature / champion an Elf; the quality needs context-dependent
  -- article or number inflection, not literal substitution.
  body :=
    [when
        (does (actor thisPermanent) enter)
        (mayOrElse
          (pay
            (perform
              (exile
                (a
                  (and
                    [subject, youControl, isOther]))))
            .once)
          (sacrifice thisPermanent)),
      when
        (does (actor thisPermanent) leave)
        (returnToBattlefield
          (the
            (and
              [was (exile which),
                isCard]))
          [under (ownerOf it)])]
}
