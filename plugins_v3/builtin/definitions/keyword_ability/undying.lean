keyword undying : Ability := {
  lexical := { lemma := "undying" },
  spelling := "undying",
  body :=
    [when
        (does (actor thisPermanent) die)
        (returnToBattlefieldWithCounters thisPermanent
          (ownerOf thisPermanent) 1 p1p1Counter)
        [] [] none none none
        (some
          (not
            (is thisPermanent
              (hasCounter (some p1p1Counter)))))]
}
