/-- Haste exempts its creature from the summoning-sickness rule [CR#302.6]: it can attack
[CR#702.10b], and its controller can activate its activated abilities with the tap symbol or the
untap symbol in their activation costs [CR#702.10c], even if it hasn't been under its
controller's control continuously since their most recent turn began. "Even if" is an "as
though" permission whose condition is that rule's: the prohibition stays, and the creature is
treated as meeting its exception [CR#609.4]. The rule's "that creature" names the creature with
haste, which the Definition writes "this creature": the Source is never read by a noun word
(`Determiner.self`). -/
keyword haste : Ability := {
  lexical := { lemma := "haste" },
  spelling := "haste",
  body :=
    [ static (can thisCreature attack
        (asThough := holds (controlledSinceTheirTurnBegan it))),
      static (can (controllerOf thisCreature)
        (activate (allOf (and [abilityHead anyActivated, abilityOf it, tapOrUntapInCost])))
        (asThough := holds (controlledSinceTheirTurnBegan thisCreature))) ]
}
