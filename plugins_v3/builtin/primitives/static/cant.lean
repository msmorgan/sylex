/-- "<subj> can't <action>" [CR#101.2]. `causedBy` restricts the prohibition to an event that
cause causes, as `causedBy` does a would-clause: "damage doesn't cause you to lose life" is
`cant you (loseLife anyAmount) (causedBy := byEvent (does (a isSource) (dealDamage anyAmount you)))`
[CR#120.3a]. -/
-- [CR#101.2]
-- [CR#120.3a]
primitive cant (subj : Subject) (action : Action)
    (timing : Option Timing := none)
    (asThough : Option AsThough := none)
    (except : Option DeonticExcept := none)
    (causedBy : Option Causing := none) : StaticSpec := {
  -- TODO(spelling): <subj> can't <action>; wording varies with asThough, causedBy, except, timing, requiring optional clauses, inflection or structure-dependent phrasing.
  body :=
    Semantics.Primitives.StaticSpec.cant subj action { timing := timing, asThough := asThough, except := except, cause := causedBy }
}
