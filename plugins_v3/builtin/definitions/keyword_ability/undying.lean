/-- "Undying" means "When this permanent is put into a graveyard from the battlefield, if it had no
+1/+1 counters on it, return it to the battlefield under its owner's control with a +1/+1
counter on it." [CR#702.93a] -/
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
