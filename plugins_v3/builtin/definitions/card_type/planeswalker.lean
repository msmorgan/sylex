/-- Planeswalker: a permanent type [CR#110.4], whose objects alone have loyalty [CR#306.5].

 [CR#306.1] A planeswalker card may be cast from its owner's hand as a sorcery; it enters with
loyalty [CR#306.5b], can be attacked [CR#306.6], loses loyalty to damage [CR#120.3c], and the
loyalty state-based action applies to it [CR#704.5i]. -/
card_type planeswalker := {
  lexical := { lemma := "planeswalker" },
  spelling := "planeswalker",
  name := "Planeswalker",
  cardClass := some .permanentCard,
  stats := [loyalty],
  confers :=
    [castableFrom yourHand asSorcery,
      ability
        (static
          (entersWithCounters this_
            (statOf (.stat loyalty) this_)
            (.named "loyaltyCounter"))),
      .property (can any (attack this_)),
      damageResult
        (loseCounters
          (some (printedKind (.named "loyaltyCounter")))
          thatMuch this_),
      .stateBased
        (compareAmt
          (countersOn (.named "loyaltyCounter") this_)
          eq 0)
        (putInto this_ graveyard)]
}
