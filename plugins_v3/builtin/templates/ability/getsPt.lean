/-- "<subject> gets +P/+T [until …]": one change to both stats, the subject written once. -/
template getsPt (subject : NounPhrase) (power toughness : Delta Amount) : StaticSpec := {
  spelling := "<subject> gets <power>/<toughness>",
  body :=
    characteristicChange subject
      [stat power power,
        stat toughness toughness]
}
