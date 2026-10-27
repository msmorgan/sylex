/-- "Vigilance": "Attacking doesn't cause creatures with vigilance to tap." [CR#702.20b] Declaring
a creature as an attacker taps it [CR#508.1f]; vigilance is a cause-qualified prohibition of
that tap alone, as "damage doesn't cause you to lose life" is of the loss [CR#120.3a], so another
effect may still tap the creature. -/
keyword vigilance : Ability := {
  lexical := { lemma := "vigilance" },
  spelling := "vigilance",
  body :=
    [ static (cant any (tap thisCreature)
        (causedBy := byEvent (does thisCreature (.attack none)))) ]
}
