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
    [static (addsTarget thisAura subject),
      static
        (mayBeAttachedTo thisAura (allOf subject)),
      static
        (replace
          (would (actor thisAura) enter)
          (doIf
            (not
              (exists_ (legalHostOf thisAura)))
            (instead
              (doIf
                (is thisAura (in_ stack))
                (putInto thisAura graveyard))))),
      static
        (replace
          (would any
            (move thisAura
              (not stack) (some battlefield)
              .arrival
              [not (attachedTo any)]))
          (doIf
            (exists_ (legalHostOf thisAura))
            (before
              (choose
                (a (legalHostOf thisAura)))
              (some
                (alongside
                  (attachTo thisAura it)))))),
      static
        (replace
          (would (actor thisAura)
            (enter none
              [attachedTo any,
                not
                  (attachedTo
                    (a (legalHostOf thisAura)))]))
          (instead
            (doIf
              (is thisAura (in_ stack))
              (putInto thisAura graveyard))))]
}
