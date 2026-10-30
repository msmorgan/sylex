/-- "Blitz [cost]" [CR#702.152a]. "If this spell's blitz cost was paid, sacrifice the permanent
this spell becomes at the beginning of the next end step" is its own ability [CR#113.2c], a
Resolution Delayed Trigger: it creates its delayed trigger as the permanent is put onto the
battlefield as the spell resolves [CR#608.3g], controlled by the spell's controller as it resolved
[CR#603.7d], so a countered spell creates none, and a copy's token permanent [CR#608.3f] creates
its own. -/
keyword blitz (cost : Cost) : Ability := {
  lexical := { lemma := "blitz", parameters := [[Cost]] },
  spelling := "blitz <cost>",
  body :=
    [ static (altCost this_ (some cost)),
      static
        (resolutionDelayedTrigger
          (doIf (keywordCostPaid blitz thisSpell)
            (atNext endStep (sacrifice (becomes thisSpell))))),
      static
        (onlyWhile (abilityGrant thisPermanent (.keyword haste.label [] []))
          (keywordCostPaid blitz thisPermanent)),
      static
        (onlyWhile
          (abilityGrant thisPermanent
            (when (does thisPermanent die) (draw 1)))
          (keywordCostPaid blitz thisPermanent)) ]
}
