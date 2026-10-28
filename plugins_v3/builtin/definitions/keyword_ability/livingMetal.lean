/-- "Living metal" means "During your turn, this permanent is an artifact creature in addition to
its other types." [CR#702.161a] -/
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
