/-- [CR#714.3a,714.3c,714.4] A Saga enters with a lore counter unless it has read ahead, gets one
as its controller's precombat main phase begins if it has one or more chapter abilities (a
turn-based action, no effect [CR#703.1,609.1]), and is sacrificed once its lore counters reach its
final chapter number, unless it is the source of a chapter ability that has triggered but not yet
left the stack. -/
subtype saga := {
  lexical := { lemma := "Saga" },
  spelling := "Saga",
  kind := enchantment,
  name := "Saga",
  confers :=
    have hasChapterAbilities : Condition :=
      compareAmt (finalChapterNumber thisSaga) atLeast 1;
    [ability
        (static
          (onlyUnless
            (entersWithCounters thisSaga 1
              (.named "loreCounter"))
            (is thisSaga
              (hasKeyword (.the (.named "readAhead")))))),
      .turnBased firstMain
        (doIf hasChapterAbilities
          (putCounters 1 (printedKind (.named "loreCounter")) thisSaga)),
      .stateBased
        (and
          [hasChapterAbilities,
            compareAmt
              (countersOn (.named "loreCounter") thisSaga)
              atLeast (finalChapterNumber thisSaga),
            not (exists_ (and [chapterAbilityOf thisSaga, triggeredNotLeftStack]))])
        (sacrifice thisSaga)]
}
