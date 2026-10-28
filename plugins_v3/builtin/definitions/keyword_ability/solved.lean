/-- "Solved — [Ability text]" is a static, triggered or activated ability [CR#702.169a]. The
restriction is "as long as this Case is solved" for a static ability, "this ability triggers
only if this Case is solved" for a triggered ability, and "activate only if this Case is solved"
for an activated ability. [CR#702.169b..702.169d] -/
keyword solved (ability : Ability) : Ability := {
  lexical := { lemma := "solved", parameters := [[DashAbility]] },
  spelling := "solved — <ability>",
  body :=
    [ qualified ability none none
        (is thisCase (hasDesignation DesignationLabel.solved))
        none ]
}
