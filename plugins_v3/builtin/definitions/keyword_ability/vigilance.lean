/-- "Vigilance": "Attacking doesn't cause creatures with vigilance to tap." [CR#702.20b] Declaring
a creature as an attacker taps it [CR#508.1f]; vigilance is a cause-qualified prohibition of
that tap alone, as "damage doesn't cause you to lose life" is of the loss [CR#120.3a], so another
effect may still tap the creature. -/
keyword vigilance : Ability := {
  lexical := { lemma := "vigilance" },
  spelling := "vigilance",
  body :=
    [Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.cant Semantics.Templates.Subject.any
          (Semantics.Definitions.tap Semantics.Templates.NounPhrase.thisCreature) Option.none Option.none Option.none
          (Option.some
            (Semantics.Templates.Causing.byEvent
              (Semantics.Templates.GameEvent.does (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisCreature)
                (Semantics.Templates.Action.attack Option.none)))))]
}
