/-- Creature: a permanent type [CR#110.4], whose objects alone have power and toughness
[CR#302.4].

 [CR#302.1] A creature card may be cast from its owner's hand as a sorcery; [CR#508.1a] a
creature is a Combatant, a quality and not an ability [CR#113.12]; it can attack and block
[CR#302.5], but neither attacks nor activates an ability with {T} or {Q} in its cost unless its
controller has controlled it continuously since their most recent turn began [CR#302.6]; damage
dealt to it is marked on it [CR#120.3e]; and the toughness and lethal-damage state-based actions
apply to it [CR#704.5f,704.5g]. -/
card_type creature := {
  lexical := { lemma := "creature" },
  spelling := "creature",
  name := "Creature",
  cardClass := some .permanentCard,
  stats := [.power, .toughness],
  confers :=
    [castableFrom yourHand asSorcery,
      .property (combatant this_),
      .property (can this_ attack),
      .property (can this_ block),
      .property (onlyUnless (cant this_ attack) (controlledSinceTheirTurnBegan it)),
      .property
        (onlyUnless
          (cant any (activate (allOf (and [abilityHead .anyActivated, abilityOf this_,
            tapOrUntapInCost]))))
          (controlledSinceTheirTurnBegan this_)),
      damageResult (markDamage this_ thatMuch),
      .stateBased
        (compareAmt (toughnessOf this_)
          .atMost 0)
        (putInto this_ graveyard),
      .stateBased
        (and
          [compareAmt (toughnessOf this_)
              .greater 0,
            compareAmt
              (statOf .markedDamage this_)
              .greater 0,
            compareAmt
              (statOf .markedDamage this_)
              .atLeast (toughnessOf this_)])
        (destroy this_)]
}
