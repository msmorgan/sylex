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
        (static
          (onlyUnless
            (entersWithCounters thisSaga 1
              (Semantics.CounterKind.named "loreCounter"))
            (is thisSaga
              (hasKeyword (Semantics.KeywordTerm.the (Semantics.KeywordLabel.named "readAhead")))))),
      Semantics.Conferral.turnBased Semantics.TurnPart.firstMain
        (doIf sagaHasChapterAbilities
          sagaLoreIncrement),
      Semantics.Conferral.stateBased
        (and
          [sagaHasChapterAbilities,
            compareAmt
              (countersOn (Semantics.CounterKind.named "loreCounter") thisSaga)
              Semantics.Comparator.atLeast (finalChapterNumber thisSaga),
            not sagaHasTriggeredChapter])
        (sacrifice thisSaga)]
}
