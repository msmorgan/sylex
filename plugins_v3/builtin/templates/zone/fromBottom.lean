/-- "<n>th from the bottom of <loc>" -/
template fromBottom (n : Nat) (loc : Location) : Location := {
  -- TODO(spelling): <n>th from the bottom of <loc>; string substitution cannot inflect verbs, nouns, or ordinals.
  body := fromEnd .bottom (.nth n) loc
}
