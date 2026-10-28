/-- Devour's definition over the permanents `quality` describes, counted by the variant's head
noun `head`: "As this object enters, you may sacrifice any number of [quality] permanents. This
permanent enters with N +1/+1 counters on it for each [head] sacrificed this way." Devour counts
each creature [CR#702.82a], devour [quality] each permanent [CR#702.82c]. -/
template devourEntry (quality head : Predicate) (amount : Amount) : Ability := {

  -- TODO(spelling): Devour entry wording inflects quality and head and repeats the counted action; plain substitution cannot select those forms.
  body :=
    static
      (replace
        (would (actor thisPermanent) enter)
        (before
          (may
            (Action.devour
              (counted anyNumber
                (and [quality, in_ battlefield]))))
          (some
            (alongside
              (putCounters
                (forEach amount
                  (and
                    [head,
                      was (sacrifice which)
                        (some thisWay)]))
                (printedKind p1p1Counter) thisPermanent)))))
}
