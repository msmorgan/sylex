keyword riot : Ability := {
  lexical := { lemma := "riot" },
  spelling := "riot",
  body :=
    [ static
        (replace (would thisPermanent enter)
          (alongside
            (mayOrElse (putCounters 1 p1p1Counter thisPermanent)
              (gainHaste thisPermanent)))) ]
}
