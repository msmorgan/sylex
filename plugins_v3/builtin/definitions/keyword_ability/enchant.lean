/-- "Enchant [quality]" [CR#702.5a]: the Aura spell's target [CR#303.4a], the hosts it may be
attached to, and its entry. An Aura entering with no legal object or player to enchant, under its
enchant ability and any other applicable effects, doesn't enter: it stays in its zone, or is put
into its owner's graveyard from the stack, and a token Aura isn't created [CR#303.4g]. An Aura
entering other than by resolving as an Aura spell, by an effect that doesn't specify what it will
enchant, enters attached to a legal host its controller chooses as it enters [CR#303.4f]. An
effect that names the host (`MoveRider.attachedTo`) puts it onto that host, and if it can't
legally enchant that host or the host is undefined, the Aura doesn't enter, as with no legal host
at all [CR#303.4i]. Which hosts are legal is the engine's to evaluate (`legalHostOf`). -/
keyword enchant (subject : Predicate) : Ability := {
  lexical := {
    lemma := "enchant",
    parameters := [[Quality (None)]],
    determinatives := [{ number := Singular, countability := Count, lemma := "enchanted" }],
    verbs := [{ lemma := "enchant", frames := [Monotransitive] }]
  },
  spelling := "enchant <subject>",
  body :=
    [Semantics.Templates.Ability.static (Semantics.Templates.StaticSpec.addsTarget Semantics.Templates.NounPhrase.thisAura subject),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.mayBeAttachedTo Semantics.Templates.NounPhrase.thisAura (Semantics.Templates.NounPhrase.allOf subject)),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.replace
          (Semantics.Templates.WouldClause.would (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisAura) Semantics.Templates.Action.enter)
          (Semantics.Templates.Replacement.doIf
            (Semantics.Templates.Condition.not
              (Semantics.Templates.Condition.exists_ (Semantics.Templates.Predicate.legalHostOf Semantics.Templates.NounPhrase.thisAura)))
            (Semantics.Templates.Replacement.instead
              (Semantics.Templates.Action.doIf
                (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.thisAura (Semantics.Templates.Predicate.in_ Semantics.Templates.Location.stack))
                (Semantics.Templates.Action.putInto Semantics.Templates.NounPhrase.thisAura Semantics.Templates.Location.graveyard))))),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.replace
          (Semantics.Templates.WouldClause.would Semantics.Templates.Subject.any
            (Semantics.Templates.Action.actionMove Semantics.Templates.NounPhrase.thisAura
              (Semantics.Templates.Location.not Semantics.Templates.Location.stack) (Option.some Semantics.Templates.Location.battlefield)
              Semantics.ObservationPoint.arrival
              [Semantics.Templates.MoveRider.moveRiderNot (Semantics.Templates.MoveRider.moveRiderAttachedTo Semantics.Templates.NounPhrase.any)]))
          (Semantics.Templates.Replacement.doIf
            (Semantics.Templates.Condition.exists_ (Semantics.Templates.Predicate.legalHostOf Semantics.Templates.NounPhrase.thisAura))
            (Semantics.Templates.Replacement.before
              (Semantics.Templates.Action.choose
                (Semantics.Templates.NounPhrase.a (Semantics.Templates.Predicate.legalHostOf Semantics.Templates.NounPhrase.thisAura)))
              (Option.some
                (Semantics.Templates.Replacement.alongside
                  (Semantics.Definitions.attachTo Semantics.Templates.NounPhrase.thisAura Semantics.Templates.NounPhrase.it)))))),
      Semantics.Templates.Ability.static
        (Semantics.Templates.StaticSpec.replace
          (Semantics.Templates.WouldClause.would (Semantics.Templates.Subject.actor Semantics.Templates.NounPhrase.thisAura)
            (Semantics.Templates.Action.enter Option.none
              [Semantics.Templates.MoveRider.moveRiderAttachedTo Semantics.Templates.NounPhrase.any,
                Semantics.Templates.MoveRider.moveRiderNot
                  (Semantics.Templates.MoveRider.moveRiderAttachedTo
                    (Semantics.Templates.NounPhrase.a (Semantics.Templates.Predicate.legalHostOf Semantics.Templates.NounPhrase.thisAura)))]))
          (Semantics.Templates.Replacement.instead
            (Semantics.Templates.Action.doIf
              (Semantics.Templates.Condition.is Semantics.Templates.NounPhrase.thisAura (Semantics.Templates.Predicate.in_ Semantics.Templates.Location.stack))
              (Semantics.Templates.Action.putInto Semantics.Templates.NounPhrase.thisAura Semantics.Templates.Location.graveyard))))]
}
