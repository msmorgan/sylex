---
needs: [plugins-v3-primitive-template-boundary]
---
# Remove the remaining sort-prefixed structural primitive names

Standard constraints apply.

Eliminate these 11 remaining structural primitive names:
`actionSearch`, `actionShuffle`, `actionVote`, `actionChooseModes`, `actionFlipCoins`,
`actionPay`, `actionRemoveCounters`, `actionShiftResult`, `nounPhraseSomeOf`,
`amountAggregate` and `durationBetween`.

Their shorter names collide with same-sort keyword actions or type-adapting templates;
return-type dispatch alone cannot distinguish them. Resolve ownership at the primitive/template
boundary before renaming, then update declarations and all consumers without changing their
expansions. Preserve the distinction between structural primitives, keyword Definitions and
worded Templates. Record explicit key renames in expansion comparisons and verify both Lean
and Rust bundle loading; retaining the prefixes is a defect to resolve.
