/-- "<subject> has base power and toughness P/T" -/
template getsBase (subject : NounPhrase) (power toughness : Amount) : StaticSpec := {
  spelling := "<subject> has base power and toughness <power>/<toughness>",
  body :=
    getsPt subject (setTo power) (setTo toughness)
}
