/-- An additional cost, optionally offered; `times` specifies how often it may be paid and
defaults to once. Squad permits payment any number of times. [CR#702.157a] -/
primitive addedCost (cost : Cost) (offered : Bool) (times : PayTimes := PayTimes.once) :
    StaticSpec := {
  body :=
    Semantics.Primitives.StaticSpec.addedCost cost offered times
}
