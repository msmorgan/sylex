keyword nightbound : Ability := {
  lexical := { lemma := "nightbound" },
  spelling := "nightbound",
  body :=
    [ static
        (.asEvent (gameBecomes day)
          (doIf (is thisPermanent isTransformed)
            (transform thisPermanent)
            none)),
      static
        (cant any (transform thisPermanent) (except := some (.byKeyword nightbound))) ]
}
