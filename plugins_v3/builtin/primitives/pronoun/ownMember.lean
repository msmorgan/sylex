/-- "itself": the member a comparison over a domain is measuring (`semantics-v3.md` §1.3 Box 5). -/
primitive ownMember : NounPhrase := {
  spelling := "itself",
  body :=
    Semantics.Primitives.NounPhrase.pro Semantics.Reach.bare Semantics.Plurality.one
}
