/-- "Echo [cost]": "At the beginning of your upkeep, if this permanent came under your control
since the beginning of your last upkeep, sacrifice it unless you pay [cost]." [CR#702.30a] It came
under your control since then unless you have controlled it continuously since then. The payment
is of this permanent's echo cost, which a replacement or a trigger on paying it reads. -/
keyword echo (cost : Cost) : Ability := {
  lexical := { lemma := "echo", parameters := [[Cost], [DashNonManaCost]] },
  spelling := "echo <cost>",
  body :=
    [at_ (beginningOfYour upkeep)
        (doUnlessKeywordCost (sacrifice thisPermanent) cost
          thisPermanent echo.label)
        [] [] none none none
        (some
          (not
            (heldSince
              (is thisPermanent youControl)
              (beginningOf upkeep (some you)
                (previous (some you))))))]
}
