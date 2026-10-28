/-- "Unleash" means "You may have this permanent enter with an additional +1/+1 counter on it" and
"This permanent can't block as long as it has a +1/+1 counter on it." [CR#702.98a] -/
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
            (block any))
          (is thisPermanent
            (hasCounter (some p1p1Counter))))]
}
