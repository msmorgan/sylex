keyword persist : Ability := {
  lexical := { lemma := "persist" },
  spelling := "persist",
  body :=
    [ when (does thisPermanent die)
        (if_ := some (not (is thisPermanent (.hasCounters (some m1m1Counter)))))
        (returnToBattlefieldWithCounters thisPermanent (ownerOf thisPermanent) 1 m1m1Counter) ]
}
