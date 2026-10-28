keyword livingMetal : Ability := {
  lexical := { lemma := "living metal" },
  spelling := "living metal",
  body :=
    [static
        (onlyWhile
          (characteristicChange thisPermanent
            [characteristicEditTypeLine .adds
                { types := some [Semantics.CardType.artifact, Semantics.CardType.creature] }])
          (duringPart .turn (some you)))]
}
