/-- "Bands with other <quality>", a special form of banding [CR#702.22b]. Unlike a variant, it
keeps its own label: the banding rules count a creature with it as one without banding
[CR#702.22c], and only losing banding takes it too [CR#702.22b]. -/
keyword bandsWithOther (quality : Predicate) : Ability := {
  lexical := { lemma := "bands with other", parameters := [[Quality (None)]] },
  -- TODO(spelling): Bands with other legendary creatures; the quality needs context-dependent
  -- article or number inflection, not literal substitution.
  body :=
    []
}
