import Semantics.Primitives
import Semantics.Check.Phrase
import Semantics.Check.Rules

/-!
# Semantics.Definitions

The builtin vocabulary and rules, loaded from individual declarations in `plugins_v3/builtin`.
Checker-level card-type values live in `Semantics.Words`.
-/

namespace Semantics.Definitions

open Semantics.Templates

load_bundle "../plugins_v3/builtin" templatesIn Semantics.Templates

end Semantics.Definitions

namespace Semantics.Templates
export Semantics.Templates.Action (target)
export Semantics.Templates.Status (become)
end Semantics.Templates
