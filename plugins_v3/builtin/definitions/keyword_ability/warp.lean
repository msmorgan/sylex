/-- "Warp [cost]" [CR#702.185a]. Its first static functions on the stack and grants no cast: the
rules already allow a cast from the hand [CR#601.3], so "cast this card from your hand by paying
[cost]" is an alternative cost [CR#118.9] that applies only while this spell was cast from your
hand. "If this spell's warp cost was paid, exile the permanent this spell becomes at the
beginning of the next end step. Its owner may cast this card after the current turn has ended for
as long as it remains exiled" is a Resolution Delayed Trigger, as blitz's is [CR#608.3g,603.7d]. -/
keyword warp (cost : Cost) : Ability := {
  lexical := {
    lemma := "warp",
    parameters := [[Cost], [DashNonManaCost]],
    verbs := [{ lemma := "warp", frames := [Monotransitive] }]
  },
  spelling := "warp <cost>",
  body :=
    [ static (onlyWhile (altCost this_ (some cost))
        (is this_ (did any (cast which (from_ := yourHand)) thisObject))),
      static
        (resolutionDelayedTrigger
          (doIf (keywordCostPaid warp thisSpell)
            (atNext endStep
              (sequentially
                [ exile (becomes thisSpell),
                  establishThroughout
                    (can (ownerOf it)
                      (cast it (from_ := exileZone)))
                    onALaterTurn ])))) ]
}
