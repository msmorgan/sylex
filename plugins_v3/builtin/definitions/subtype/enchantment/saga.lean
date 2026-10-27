/-- [CR#714.3a,714.3c,714.4] A Saga enters with a lore counter unless it has read ahead, gets one
as its controller's precombat main phase begins if it has one or more chapter abilities (a
turn-based action, no effect [CR#703.1,609.1]), and is sacrificed once its lore counters reach its
final chapter number, unless it is the source of a chapter ability that has triggered but not yet
left the stack. -/
subtype saga := {
  lexical := { lemma := "Saga" },
  spelling := "Saga",
  kind := .enchantment,
  name := "Saga",
  confers :=
    [Semantics.Conferral.ability
        (Semantics.Templates.Ability.static
          (Semantics.Templates.StaticSpec.onlyUnless
            (Semantics.Templates.StaticSpec.entersWithCounters Semantics.Templates.NounPhrase.thisSaga 1
              (Semantics.CounterKind.named "loreCounter"))
            (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.thisSaga
              (Semantics.Templates.Predicate.hasKeyword (Semantics.KeywordTerm.the (Semantics.KeywordLabel.named "readAhead")))))),
      Semantics.Conferral.turnBased Semantics.TurnPart.firstMain
        (Semantics.Templates.Action.doIf Semantics.Templates.Condition.sagaHasChapterAbilities
          Semantics.Templates.Action.sagaLoreIncrement),
      Semantics.Conferral.stateBased
        (Semantics.Templates.Condition.and
          [Semantics.Templates.Condition.sagaHasChapterAbilities,
            Semantics.Templates.Condition.compareAmt
              (Semantics.Templates.Amount.countersOn (Semantics.CounterKind.named "loreCounter") Semantics.Templates.NounPhrase.thisSaga)
              Semantics.Comparator.atLeast (Semantics.Templates.Amount.finalChapterNumber Semantics.Templates.NounPhrase.thisSaga),
            Semantics.Templates.Condition.not Semantics.Templates.Condition.sagaHasTriggeredChapter])
        (Semantics.Definitions.sacrifice Semantics.Templates.NounPhrase.thisSaga)]
}
