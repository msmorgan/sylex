keyword undying : Ability := {
  lexical := { lemma := "undying" },
  spelling := "undying",
  body :=
    [ when (does thisPermanent die)
        (if_ := some (not (is thisPermanent (.hasCounters (some p1p1Counter)))))
        (returnToBattlefieldWithCounters thisPermanent (ownerOf thisPermanent) 1 p1p1Counter) ]
}
