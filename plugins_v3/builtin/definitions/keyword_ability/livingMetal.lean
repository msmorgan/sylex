keyword livingMetal : Ability := {
  lexical := { lemma := "living metal" },
  spelling := "living metal",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.onlyWhile
          (Semantics.Templates.StaticSpec.characteristicChange Semantics.Templates.NounPhrase.thisPermanent
            [Semantics.Templates.CharacteristicEdit.characteristicEditTypeLine Semantics.QualityOp.adds
                { types := Option.some [Semantics.CardType.artifact, Semantics.CardType.creature] }])
          (Semantics.Templates.Condition.duringPart Semantics.TurnPart.turn (Option.some Semantics.Templates.NounPhrase.you)))]
}
