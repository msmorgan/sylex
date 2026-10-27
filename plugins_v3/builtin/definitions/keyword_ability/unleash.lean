keyword unleash : Ability := {
  lexical := { lemma := "unleash" },
  spelling := "unleash",
  body :=
    [ static
        (replace (would thisPermanent enter)
          (alongside (may (putCounters 1 p1p1Counter thisPermanent)))),
      static
        (onlyWhile (cant thisPermanent (.block .any))
          (is thisPermanent (.hasCounters p1p1Counter))) ]
}
