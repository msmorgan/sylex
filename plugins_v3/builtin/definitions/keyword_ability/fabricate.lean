keyword fabricate (amount : Amount) : Ability := {
  lexical := { lemma := "fabricate", parameters := [[Amount]] },
  spelling := "fabricate <amount>",
  body :=
    [ when (does thisPermanent enter)
        (mayOrElse (putCounters amount p1p1Counter thisPermanent)
          (create amount (artifactCreatureToken 1 1 [] [servo]))) ]
}
