/-- "any number of <group> with total <axis> <comparator> <bound>" [CR#701.59a]. -/
-- [CR#701.59a]
template withTotal (axis : ProjAxis) (comparator : Comparator) (bound : Amount)
    (group : NounPhrase) : NounPhrase := {
  spelling := "any number of <group> with total <axis> <comparator> <bound>",
  body :=
    nounPhraseSomeOf (totalling axis comparator bound) none group
}
