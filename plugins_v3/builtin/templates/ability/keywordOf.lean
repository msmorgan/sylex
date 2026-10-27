/-- The keyword a keyword ability is, as "with flying" names it: `hasKeyword flying`, or a list
`[flying, firstStrike, trample]`. A keyword ability is written under its declaration's name
(`.keyword (.named "flying") …`), so that name is the keyword. An ability that is no keyword
names none: its term is the empty label, which names no keyword and is refused
`knownKeywordTerm` wherever a keyword term is checked. -/
template keywordOf (ability : Ability) : KeywordTerm := {
  spelling := "with <ability>",
  body :=
    match ability with
    | Semantics.Ability.keyword label params body => Semantics.KeywordTerm.the label
    | x => Semantics.KeywordTerm.the (Semantics.KeywordLabel.named "")
}
