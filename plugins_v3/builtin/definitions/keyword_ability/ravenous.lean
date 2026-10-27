keyword ravenous : Ability := {
  lexical := { lemma := "ravenous" },
  spelling := "ravenous",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.entersWithCounters Semantics.Templates.NounPhrase.thisPermanent
          (Semantics.Templates.Amount.letter Semantics.Letter.x) Semantics.Definitions.p1p1Counter),
      Semantics.Templates.Ability.when
        (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisPermanent) Semantics.Templates.Action.enter)
        (Semantics.Templates.Action.draw 1) [] [] Option.none Option.none Option.none
        (Option.some
          (Semantics.Templates.Condition.compareAmt (Semantics.Templates.Amount.letter Semantics.Letter.x) Semantics.Comparator.atLeast 5))]
}
