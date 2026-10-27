keyword storied : Ability := {
  lexical := { lemma := "storied" },
  spelling := "storied",
  body :=
    [ static
        (.whileHolds
          (and
            [ .compareAmt
                (countOf
                  (and
                    [ isPermanent,
                      youControl,
                      andOr
                        [ isArtifact,
                          .hasSubtype saga,
                          .hasSupertype .legendary ] ]))
                .atLeast 3,
              not (is .you (hasDesignation enduringStory)) ])
          (gainDesignation .you enduringStory restOfGame)) ]
}
