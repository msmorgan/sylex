/-- "Infect": damage this deals to a creature isn't marked on it, and damage it deals to a player
doesn't cause that player to lose life; rather, its controller puts that many −1/−1 counters on
that creature, or gives that player that many poison counters [CR#702.90b,702.90c]. Each is a
replacement of the result the damage would otherwise have [CR#120.3a,120.3e,614.6], so a source
with wither as well puts the counters once [CR#120.3d]. The player's row is caused by this
damage event, not by this source, so other life loss this source causes is not replaced; the
life that damage to a player would make anyone lose is that player's [CR#120.3a], so the loss
names no loser. Infect functions from any zone [CR#702.90e]. -/
keyword infect : Ability := {
  lexical := { lemma := "infect" },
  spelling := "infect",
  body :=
    [ static
        (replace (causedBy (bySource this_) (would any (markDamage (a isCreature))))
          (instead (putCounters thatMuch m1m1Counter (that creature)))),
      static
        (replace
          (causedBy (byEvent (does this_ (dealDamage anyAmount (a isPlayer))))
            (would any (loseLife anyAmount)))
          (instead (putCounters (theOutcome .damageDealt) poison (that player)))) ]
}
