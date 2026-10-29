/-- "<the ability, for the first keyword>. The same is true for <the other keywords>.": one ability
[CR#113.2c] that does, for each keyword in Oracle order, what `frame` writes for `thatKeyword`
[CR#702.1c]. A triggered `frame` makes one triggered ability under the first keyword's trigger
that does each keyword's effect in turn; a static `frame` makes one static ability whose parts
are each keyword's. A keyword with variants grants each of them through `keywordGrant`
[CR#702.1c]. -/
-- [CR#113.2c]
-- [CR#702.1c]
template alsoForKeywords (frame : Ability) (keywords : List KeywordTerm := []) : Ability := {
  -- TODO(spelling): Format and coordinate the keywords list entries, including the empty case.
  body :=
    sameIsTrueFor frame keywords
}
