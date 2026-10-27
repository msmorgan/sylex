/-- Battle: a permanent type [CR#110.4], whose objects have defense [CR#310.4].

 [CR#310.1] A battle card may be cast from its owner's hand as a sorcery; it enters with
defense [CR#310.4b], can be attacked [CR#310.5], loses defense to damage [CR#120.3h], the
non-Siege state-based action applies to it [CR#704.5w,310.8], and its controller chooses its
protector as it enters [CR#310.9a]. -/
card_type battle := {
  lexical := { lemma := "battle" },
  spelling := "battle",
  name := "Battle",
  permanent := true,
  stats := [.defense],
  instantOrSorcery := false,
  confers := [ castableFrom yourHand asSorcery, battleDefense, battleCanBeAttacked, battleDefenseDamage,
      battleDefenseZero, battleProtector ]
}
