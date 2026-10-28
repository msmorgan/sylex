/-- "Suspend N—[cost]" represents three abilities [CR#702.62a]: "If you could begin to cast this
card by putting it onto the stack from your hand, you may pay [cost] and exile it with N time
counters on it. This action doesn't use the stack," a special action [CR#116.2f]; "At the
beginning of your upkeep, if this card is suspended, remove a time counter from it"; and "When
the last time counter is removed from this card, if it's exiled, you may play it without paying
its mana cost if able. If you don't, it remains exiled. If you cast a creature spell this way, it
gains haste until you lose control of the spell or the permanent it becomes." A suspended card is
in exile, has suspend and has a time counter on it [CR#702.62b]; whether the card could be cast
is the engine's to evaluate, prohibitions included [CR#702.62c]. Haste lasts for as long as you
control the spell or the permanent it becomes, which ends when you lose control of either. -/
keyword suspend (amount : Amount) (cost : Cost) : Ability := {
  lexical := {
    lemma := "suspend",
    parameters := [[Amount, DashCost]],
    verbs := [{ lemma := "suspend", frames := [Monotransitive] }]
  },
  spelling := "suspend <amount>—<cost>",
  body :=
    [static
        (specialAction
          (whileTrue
            (couldBeginToCast you thisCard
              yourHand))
          cost
          (exileWithCounters thisCard amount timeCounter)),
      at_ (beginningOfYour .upkeep)
        (removeCounters (oneThrough 1)
          (some (printedKind timeCounter)) thisCard)
        [] [] none none none
        (some
          (is thisCard
            (and
              [in_ exileZone,
                hasKeyword (theKeyword suspend.label),
                hasCounter (some timeCounter)]))),
      when
        (does any
          (removeCounters theLast
            (some (printedKind timeCounter)) thisCard))
        (sequentially
          [may
              (play thisCard none withoutPaying),
            doIf
              (did (actor you)
                (cast
                  (a
                    (and [isCreature, isSpell])))
                thisWay)
              (gain (that .spell) haste
                (some
                  (forAsLongAs
                    (or
                      [is (that .spell)
                          youControl,
                        is
                          (becomes (that .spell))
                          youControl]))))])
        [] [] none none none
        (some
          (is thisCard (in_ exileZone)))]
}
