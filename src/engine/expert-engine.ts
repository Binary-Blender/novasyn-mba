// Agicore Generated Expert System Runtime
// App: novasyn_mba
// This file is auto-generated. Do not edit manually.
// --- Helper Functions ---

function renderTemplate(template: string, vars: Record<string, unknown>): string {
  return template.replace(/\{\{(\w+)\}\}/g, (_, key) => String(vars[key] ?? ''));
}

function resolveField(path: string, context: Record<string, unknown>): unknown {
  const parts = path.split('.');
  let current: unknown = context;
  for (const part of parts) {
    if (current == null || typeof current !== 'object') return undefined;
    current = (current as Record<string, unknown>)[part];
  }
  return current;
}

function compareValues(a: unknown, op: string, b: unknown): boolean {
  switch (op) {
    case '==': return a == b;
    case '!=': return a != b;
    case '>': return Number(a) > Number(b);
    case '<': return Number(a) < Number(b);
    case '>=': return Number(a) >= Number(b);
    case '<=': return Number(a) <= Number(b);
    case 'CONTAINS': return String(a).toLowerCase().includes(String(b).toLowerCase());
    default: return false;
  }
}

function evaluateCondition(expr: string, context: Record<string, unknown>): boolean {
  // Simple expression parser: "field op value"
  const match = expr.match(/^(\S+)\s*(==|!=|>=|<=|>|<|CONTAINS)\s*(.+)$/);
  if (!match) return false;
  const [, field, op, rawValue] = match;
  const fieldValue = resolveField(field!, context);
  let value: unknown = rawValue!.trim();
  // Parse value type
  if (value === 'true') value = true;
  else if (value === 'false') value = false;
  else if (!isNaN(Number(value))) value = Number(value);
  else if (typeof value === 'string' && value.startsWith('"') && value.endsWith('"')) value = value.slice(1, -1);
  return compareValues(fieldValue, op!, value);
}


// --- Fact Store (Working Memory) ---

export type AnyFact = Record<string, unknown>;

export class FactStore {
  private facts: Map<string, AnyFact[]> = new Map();
  private listeners: Array<(event: "assert" | "retract", fact: AnyFact) => void> = [];

  assert(fact: AnyFact): void {
    const type = fact._type;
    if (!this.facts.has(type)) this.facts.set(type, []);
    this.facts.get(type)!.push(fact);
    this.listeners.forEach(fn => fn("assert", fact));
  }

  retract(factId: string): AnyFact | undefined {
    for (const [type, list] of this.facts) {
      const idx = list.findIndex(f => f._id === factId);
      if (idx !== -1) {
        const removed = list.splice(idx, 1)[0]!;
        this.listeners.forEach(fn => fn("retract", removed));
        return removed;
      }
    }
    return undefined;
  }

  query<T extends AnyFact>(type: string, filter?: Partial<T>): T[] {
    const list = (this.facts.get(type) || []) as T[];
    if (!filter) return list;
    return list.filter(f => {
      for (const [key, val] of Object.entries(filter)) {
        if ((f as Record<string, unknown>)[key] !== val) return false;
      }
      return true;
    });
  }

  latest<T extends AnyFact>(type: string): T | undefined {
    const list = this.facts.get(type) || [];
    return list[list.length - 1] as T | undefined;
  }

  onFactChange(fn: (event: "assert" | "retract", fact: AnyFact) => void): void {
    this.listeners.push(fn);
  }

  clear(type?: string): void {
    if (type) this.facts.delete(type);
    else this.facts.clear();
  }

  all(): AnyFact[] {
    return Array.from(this.facts.values()).flat();
  }
}

// --- Rule Engine (Forward Chaining) ---

export interface RuleDef {
  name: string;
  conditions: Array<{ field: string; op: string; value: unknown; connector?: string }>;
  action: string;
  priority: number;
}

export interface RuleResult {
  rule: RuleDef;
  fired: boolean;
  action: string;
  timestamp: number;
}

export class RuleEngine {
  private rules: RuleDef[] = [];
  private auditLog: RuleResult[] = [];

  constructor() {
    this.init();
  }

  private init(): void {
    this.rules.push({ name: 'ltv_cac_critical', conditions: [{ field: 'FinancialSnapshot.ltv_cac_ratio', op: '<', value: 2, connector: undefined }], action: 'flag:finance_risk', priority: 10 });
    this.rules.push({ name: 'cash_runway_danger', conditions: [{ field: 'FinancialSnapshot.cash_runway_months', op: '<', value: 3, connector: undefined }], action: 'flag:existential_risk', priority: 10 });
    this.rules.push({ name: 'operations_bottlenecked', conditions: [{ field: 'OperationsAudit.overall_score', op: '>', value: 24, connector: undefined }], action: 'flag:operations_critical', priority: 8 });
    this.rules.push({ name: 'low_energy_warning', conditions: [{ field: 'WeeklyReview.energy_level', op: '<', value: 3, connector: undefined }], action: 'flag:founder_burnout_risk', priority: 8 });
    this.rules.push({ name: 'missed_revenue_target', conditions: [{ field: 'FinancialSnapshot.gross_margin_pct', op: '<', value: 50, connector: undefined }], action: 'flag:margin_pressure', priority: 5 });
  }

  addRules(newRules: RuleDef[]): void {
    this.rules.push(...newRules);
    this.rules.sort((a, b) => b.priority - a.priority);
  }

  evaluate(context: Record<string, unknown>): string[] {
    const actions: string[] = [];
    for (const rule of this.rules) {
      let pass = true;
      for (const cond of rule.conditions) {
        const fieldValue = resolveField(cond.field, context);
        const condResult = compareValues(fieldValue, cond.op, cond.value);
        if (cond.connector === "UNLESS") {
          if (condResult) { pass = false; break; }
        } else {
          if (!condResult) { pass = false; break; }
        }
      }
      if (pass) {
        actions.push(rule.action);
        this.auditLog.push({ rule, fired: true, action: rule.action, timestamp: Date.now() });
      }
    }
    return actions;
  }

  getAuditLog(): RuleResult[] {
    return [...this.auditLog];
  }
}

// --- Expert Engine (Orchestrator) ---

export interface ProcessResult {
  response: string | null;
  matchedPattern: string | null;
  firedRules: string[];
  scoreChanges: Record<string, number>;
  stateTransitions: Array<{ machine: string; from: string; to: string }>;
  moduleChanges: string[];
  actions: string[];
}

export class ExpertEngine {
  readonly facts = new FactStore();
  readonly rules = new RuleEngine();
  private turnCount = 0;
  private auditLog: ProcessResult[] = [];


  process(input: string): ProcessResult {
    this.turnCount++;
    const result: ProcessResult = {
      response: null,
      matchedPattern: null,
      firedRules: [],
      scoreChanges: {},
      stateTransitions: [],
      moduleChanges: [],
      actions: [],
    };

    // Build context for condition evaluation
    const context: Record<string, unknown> = {
      input,
      turn_count: this.turnCount,
    };

    // Rule evaluation
    result.firedRules = this.rules.evaluate(context);
    result.actions.push(...result.firedRules);

    this.auditLog.push(result);
    return result;
  }

  getTurnCount(): number { return this.turnCount; }

  getAuditLog(): ProcessResult[] { return [...this.auditLog]; }
}
