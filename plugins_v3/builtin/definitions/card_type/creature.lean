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
  permanent := true,
  stats := [.power, .toughness],
  instantOrSorcery := false,
  confers := [ castableFrom yourHand asSorcery, combatant, creatureCanAttack, creatureCanBlock,
      summoningSicknessAttack, summoningSicknessActivation, creatureDamageMarking, toughnessZero,
      lethalDamage ]
}
