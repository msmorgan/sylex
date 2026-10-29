/-- "Destroy <subject>. They can't be regenerated.": `destroyCantRegenerate` over the objects just
destroyed [CR#608.2c,701.19c]. -/
template destroyPluralCantRegenerate (subject : NounPhrase) : Action := {
  spelling := "Destroy <subject>. They can't be regenerated.",
  body :=
    doUnderProhibition (destroy subject) (regeneration (pro .bare .many))
}
