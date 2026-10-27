/-- "<subj> can <action>": a permission [CR#101.2], with `rider` the later text that modifies the
act it permits [CR#608.2c] (`DeonticMods.rider`). -/
-- [CR#101.2]
-- [CR#608.2c]
template can (subj : Subject) (action : Action)
    (timing : Option Timing := none)
    (asThough : Option AsThough := none)
    (except : Option DeonticExcept := none)
    (rider : Option Action := none) : StaticSpec := {
  -- TODO(spelling): <subj> can <action>; wording varies with asThough, except, rider, timing, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Templates.StaticSpec.staticSpecCan subj action { timing := timing, asThough := asThough, except := except, rider := rider }
}
