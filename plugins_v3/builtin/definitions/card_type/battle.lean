/-- Battle: a permanent type [CR#110.4], whose objects have defense [CR#310.4].

 [CR#310.1] A battle card may be cast from its owner's hand as a sorcery; it enters with
defense [CR#310.4b], can be attacked [CR#310.5], loses defense to damage [CR#120.3h], the
non-Siege state-based action applies to it [CR#704.5w,310.8], and its controller chooses its
protector as it enters [CR#310.9a]. -/
card_type battle := {
  lexical := { lemma := "battle" },
  spelling := "battle",
  name := "Battle",
  cardClass := some .permanentCard,
  stats := [.defense],
  confers :=
    [castableFrom yourHand asSorcery,
      .ability
        (static
          (entersWithCounters this_
            (statOf (.stat .defense) this_)
            (.named "defenseCounter"))),
      .property (can any (attack this_)),
      damageResult
        (loseCounters
          (some (printedKind (.named "defenseCounter")))
          (some thatMuch) this_),
      .stateBased
        (and
          [not
              (is this_
                (hasSubtype siege)),
            compareAmt
              (countersOn (.named "defenseCounter") this_)
              .eq 0])
        (putInto this_ graveyard),
      .property
        (onlyUnless
          (entersChoosingPlayer this_
            (a
              (and
                [isPlayer,
                  not (otherThan you)])))
          (is this_
            (hasSubtype siege)))]
}
