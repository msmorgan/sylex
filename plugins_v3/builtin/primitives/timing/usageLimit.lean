/-- Full-arity binding for `UsageLimit.mk`. -/
primitive usageLimit (count : Amount) (per : Period) : UsageLimit := {
  body := Semantics.Primitives.UsageLimit.mk count per
}
