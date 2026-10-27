/-- [CR#702.170d] "A plotted card's owner may cast it from exile without paying its mana cost
during their main phase while the stack is empty during any turn after the turn in which it became
plotted. … A plotted card may be cast this way even if it doesn't have the plot ability while in
exile." A rule for every plotted card, not part of plot's text [CR#702.170a]; "you" on a card in
exile is its owner [CR#109.5]. -/
conferral plottedCardCast := {
  body :=
    { scope := .and [.isCard, hasDesignation plotted, in_ exileZone],
      confer :=
        .ability (static
          (onlyUnless
            (can you (cast this_ (from_ := exileZone) (payment := withoutPaying))
              (timing := asSorcery))
            (did any (gainDesignation this_ plotted) soFarThisTurn))) }
}
