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
          (some (deonticExceptByKeyword nightbound.label)))]
}
