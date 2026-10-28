/-- "<subj> can't <action> unless <its controller> pays <cost>" -/
primitive gatedBy (cost : Cost) (subj : Subject) (action : Action)
    (timing : Option Timing := none)
    (asThough : Option AsThough := none)
    (except : Option DeonticExcept := none) : StaticSpec := {
  -- TODO(spelling): Context-dependent wording: <subj> can't <action> unless <its controller> pays <cost>; string substitution cannot express its omitted or inflected parts.
  body :=
    Semantics.Primitives.StaticSpec.gatedBy cost subj action { timing := timing, asThough := asThough, except := except }
}
