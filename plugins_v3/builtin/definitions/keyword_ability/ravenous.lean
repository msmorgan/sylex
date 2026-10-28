/-- "Ravenous" means "This permanent enters with X +1/+1 counters on it" and "When this permanent
enters, if X is 5 or more, draw a card." [CR#702.156a] -/
keyword ravenous : Ability := {
  lexical := { lemma := "ravenous" },
  spelling := "ravenous",
  body :=
    [static
        (entersWithCounters thisPermanent
          (letter .x) p1p1Counter),
      when
        (does (actor thisPermanent) enter)
        (draw 1) [] [] none none none
        (some
          (compareAmt (letter .x) .atLeast 5))]
}
