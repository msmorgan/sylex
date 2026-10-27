keyword livingMetal : Ability := {
  lexical := { lemma := "living metal" },
  spelling := "living metal",
  body :=
    [ static
        (onlyWhile
          (.characteristicChange thisPermanent
            [.typeLine .adds { types := some [.artifact, .creature] }])
          (.duringPart .turn (some .you))) ]
}
