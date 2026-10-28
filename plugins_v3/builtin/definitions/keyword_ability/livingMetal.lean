keyword livingMetal : Ability := {
  lexical := { lemma := "living metal" },
  spelling := "living metal",
  body :=
    [static
        (onlyWhile
          (characteristicChange thisPermanent
            [typeLine .adds
                { types := some [Semantics.CardType.artifact, Semantics.CardType.creature] }])
          (duringPart .turn (some you)))]
}
