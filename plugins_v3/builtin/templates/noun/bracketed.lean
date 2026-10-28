/-- Rules text printed in square brackets: a fact about the wording, so it means what `x` means
and nothing more. The call survives in the authoring tree, where lowering reads it to find the text
that cleave removes [CR#702.148a,702.148b]. -/
-- [CR#702.148a,702.148b]
template bracketed {α : Type} (x : α) : α := {
  body :=
    x
}
