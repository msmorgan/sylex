/-- "Changeling" means "This object is every creature type": a Characteristic-Defining Ability
of the object, a Kindred instant or sorcery as much as a creature [CR#702.73a,604.3a]. -/
keyword changeling : Ability := {
  lexical := { lemma := "changeling" },
  spelling := "changeling",
  body :=
    [static (qualityDefinition this_ .adds (everyTypeOf .creature))]
}
