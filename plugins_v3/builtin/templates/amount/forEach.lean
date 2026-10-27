/-- "N for each <p>" -/
template forEach (per : Amount) (p : Predicate) : Amount := {
  spelling := "<per> for each <p>",
  body :=
    Semantics.Templates.Amount.times per (Semantics.Templates.Amount.countOf p)
}
