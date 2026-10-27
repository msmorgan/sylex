/-- [CR#120.3a] "Damage dealt to a player by a source without infect causes that player to lose
that much life." In a row scoped to players, "you" is the player. Infect replaces the loss
(`infect`) [CR#702.90b]. -/
conferral playerDamageLifeLoss := {
  body :=
    { scope := isPlayer,
      confer := .property (asEvent (does any (dealDamage anyAmount you)) (loseLife thatMuch)) }
}
