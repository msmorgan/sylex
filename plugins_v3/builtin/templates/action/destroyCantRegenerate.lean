/-- "Destroy <subject>. It can't be regenerated." / "… They can't be regenerated.": the
regeneration the prohibition forbids is of the object or objects just destroyed, and the
prohibition rides that destruction, so no regeneration shield replaces it
[CR#608.2c,701.19c]. -/
template destroyCantRegenerate (subject : NounPhrase) : Action := {
  -- TODO(spelling): Destroy <subject>. It/They can't be regenerated. String substitution cannot
  -- choose the pronoun from subject plurality.
  body :=
    doUnderProhibition (destroy subject) (regeneration (pro .bare (NounPhrase.inCaller subject).plur))
}
