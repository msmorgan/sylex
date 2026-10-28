keyword unleash : Ability := {
  lexical := { lemma := "unleash" },
  spelling := "unleash",
  body :=
    [static
        (replace
          (would (actor thisPermanent)
            enter)
          (alongside
            (may
              (putCounters 1 (printedKind p1p1Counter)
                thisPermanent)))),
      static
        (onlyWhile
          (cant (actor thisPermanent)
            (block Templates.NounPhrase.any))
          (is thisPermanent
            (hasCounter (some p1p1Counter))))]
}
