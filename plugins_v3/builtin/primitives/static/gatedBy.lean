/-- "<subj> can't <action> unless <its controller> pays <cost>" -/
primitive gatedBy (cost : Cost) (subj : Subject) (action : Action)
    (timing : Option Timing := none)
    (asThough : Option AsThough := none)
    (except : Option DeonticExcept := none) : StaticSpec := {
  -- TODO(spelling): Include or omit the clauses supplied by timing, asThough, except.
  body :=
    Semantics.Primitives.StaticSpec.gatedBy cost subj action { timing := timing, asThough := asThough, except := except }
}
