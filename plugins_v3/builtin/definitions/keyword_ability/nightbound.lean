/-- "Nightbound" means "As it becomes day, if this permanent is back face up, transform it" and
"This permanent can't transform except due to its nightbound ability." [CR#702.145e] -/
keyword nightbound : Ability := {
  lexical := { lemma := "nightbound" },
  spelling := "nightbound",
  body :=
    [static
        (asEvent (gameBecomes day)
          (doIf
            (is thisPermanent isTransformed)
            (transform thisPermanent))),
      static
        (cant any
          (transform thisPermanent) none none
          (some (byKeyword nightbound.label)))]
}
