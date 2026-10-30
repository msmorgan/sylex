---
needs: [plugins-v3-data-language]
---
# Insert first and later mentions in the checker's read order

Standard constraints apply. `docs/decisions/semantics-v3.md` §4.5 item 3 says a parameter's first
mention is the occurrence the checker reads first. `plugins-v3-data-language` keeps today's
insertion order instead (the expanded expression's order, each `or` alternative in its own
scope), so that the conversion changes syntax only.

The two orders differ for an explicit-agent passive (`Subject.by np`, §3.2): the checker reads the
promoted patient inside the action before `np`. Make the mention walk follow the checker's read
order, and review each changed record in the Lean dump (`lean-elaborated-value-dump`); hexproof,
shadow and flanking are among the definitions that use `by`.
