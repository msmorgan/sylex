keyword daybound : Ability := {
  lexical := { lemma := "daybound" },
  spelling := "daybound",
  body :=
    [static
        (replace
          (would (actor thisPermanent)
            enter)
          (doIf (gameIs night)
            (withRiders [entersTransformed]))),
      static
        (asEvent (gameBecomes night)
          (doIf
            (is thisPermanent
              (currentFace .front))
            (transform thisPermanent))),
      static
        (cant any
          (transform thisPermanent) none none
          (some (byKeyword daybound.label)))]
}
