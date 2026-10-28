keyword persist : Ability := {
  lexical := { lemma := "persist" },
  spelling := "persist",
  body :=
    [when
        (does (actor thisPermanent) die)
        (returnToBattlefieldWithCounters thisPermanent
          (ownerOf thisPermanent) 1 m1m1Counter)
        [] [] none none none
        (some
          (not
            (is thisPermanent
              (hasCounter (some m1m1Counter)))))]
}
