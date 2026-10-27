keyword daybound : Ability := {
  lexical := { lemma := "daybound" },
  spelling := "daybound",
  body :=
    [ static
        (replace (would thisPermanent enter)
          (doIf (gameIs night) (withRiders [.entersTransformed]))),
      static
        (.asEvent (gameBecomes night)
          (doIf (is thisPermanent (.currentFace .front))
            (transform thisPermanent)
            none)),
      static
        (cant any (transform thisPermanent) (except := some (.byKeyword daybound))) ]
}
