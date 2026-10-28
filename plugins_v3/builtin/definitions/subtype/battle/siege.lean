/-- [CR#310.12a,310.12b,704.5v] A Siege's protector is an opponent of its controller, it is
exiled and may be cast transformed when its last defense counter is removed, and it is put into
its owner's graveyard at defense 0 once no ability of it waits to leave the stack. -/
subtype siege := {
  lexical := { lemma := "Siege" },
  spelling := "Siege",
  kind := .battle,
  name := "Siege",
  confers :=
    [.property (entersChoosingPlayer thisSiege anOpponent),
      .ability
        (when (does any (removeCounters theLast defenseCounter thisPermanent))
          (sequentially
            [ exile it,
              may (cast it (payment := withoutPaying) (resulting := some isTransformed)) ])),
      .stateBased
        (and
          [compareAmt
              (countersOn defenseCounter this_)
              .eq 0,
            (not (exists_ (and [abilityOf this_, triggeredNotLeftStack])))])
        (putInto this_ graveyard)]
}
