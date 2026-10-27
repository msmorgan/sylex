/-- "Cumulative upkeep [cost]" with its reminder text [CR#702.24a]. -/
keyword cumulativeUpkeep (cost : Cost) : Ability := {
  lexical := {
    lemma := "cumulative upkeep",
    parameters := [[Cost], [DashNonManaCost]],
    nouns := [{ lemma := "cumulative upkeep", countability := [Mass] }]
  },
  -- TODO(spelling): cumulative upkeep [mana cost] / cumulative upkeep—[verbal cost]; separator
  -- and terminal period depend on the Cost structure.
  body :=
    [ at_ (.beginningOf .upkeep (some .you) (.each none))
        (if_ := is thisPermanent isPermanent)
        (sequentially
          [ putCounters 1 ageCounter thisPermanent,
            doUnlessKeywordCost (sacrifice thisPermanent)
              (.scaled cost (times 1 (countersOn ageCounter thisPermanent)))
              thisPermanent cumulativeUpkeep ]) ]
}
