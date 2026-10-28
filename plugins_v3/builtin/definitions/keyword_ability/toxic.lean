/-- Combat damage dealt to a player by a creature with toxic causes its controller to give that
player poison counters equal to its total toxic value, in addition to the damage’s other
results. [CR#702.164c] -/
keyword toxic (amount : Amount) : Ability := {
  lexical := { lemma := "toxic", parameters := [[Amount]] },
  spelling := "toxic <amount>",
  body :=
    []
}
