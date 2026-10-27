/-- "Prototype": its player may cast this card prototyped [CR#702.160a,718.3], with the inset
frame's mana cost, power and toughness. The frame holds that set (`Card.prototype`
[CR#718.1]), so the keyword carries no parameters of its own. -/
keyword prototype : Ability := {
  lexical := { lemma := "prototype", parameters := [[Cost, DashPowerToughness]] },
  -- TODO(spelling): Prototype [cost] — [power]/[toughness]; these values live in the card frame
  -- and are not parameters of this declaration.
  body :=
    [static (can you (cast thisCard (payment := prototyped)))]
}
