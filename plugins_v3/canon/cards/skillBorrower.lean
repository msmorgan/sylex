/-- Skill Borrower -/
card skillBorrower : Spelled :=
  { name := "Skill Borrower", cost := [generic 2, pip .blue], types := [.artifact, .creature],
    subtypes := [human, wizard],
    text :=
      [ static (visibility .reveal you topOfLibrary),
        static (onlyWhile
          (abilityGrantFrom thisCreature (topSlice 1) [.anyActivated])
          (is (topSlice 1) (or [isArtifact, isCreature]))) ],
    power := stat 1, toughness := stat 3 }
