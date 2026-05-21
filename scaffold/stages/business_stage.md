# Business.stage Stage Machine

Auto-generated from Agicore DSL.

## Transitions

### "Idea" → "Validated"

Match mode: **all** (all conditions must pass)

Requirements:
- StrategicPosition.value_proposition is present (not null)
- StrategicPosition.target_market is present (not null)
- SalesContact record count >= 1

### "Validated" → "Building"

Match mode: **all** (all conditions must pass)

Requirements:
- SalesContact.notes is present (not null)

### "Building" → "Active"

Match mode: **any** (any one condition must pass)

Requirements:
- FinancialSnapshot.revenue > 0
- SalesContact count >= 3 where stage = "closed_won"

### "Active" → "Scaling"

Match mode: **all** (all conditions must pass)

Requirements:
- FinancialSnapshot record count >= 2
- OperationsAudit.overall_score < 20
