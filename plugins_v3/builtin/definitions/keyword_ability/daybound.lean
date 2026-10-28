keyword daybound : Ability := {
  lexical := { lemma := "daybound" },
  spelling := "daybound",
  body :=
    [static
        (replace
          (would (actor thisPermanent)
            enter)
          (doIf (gameIs night)
            (withRiders [moveRiderEntersTransformed]))),
      static
        (asEvent (gameBecomes night)
          (doIf
            (is thisPermanent
              (currentFace .front))
            (transform thisPermanent))),
      static
        (cant Templates.Subject.any
          (transform thisPermanent) none none
          (some (deonticExceptByKeyword daybound.label)))]
}
