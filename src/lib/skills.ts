// Agicore Generated — DO NOT EDIT BY HAND
// Re-run `agicore generate` to regenerate.
// Skill registry: keyword-based domain expertise injection.

export interface SkillDef {
  name: string;
  description: string;
  keywords: string[];
  domain: string;
  priority: number;
}

export const SKILL_REGISTRY: SkillDef[] = [
  {
    name: 'finance_frameworks',
    description: "Core financial frameworks for evaluating business unit economics and capital efficiency",
    keywords: ['finance', 'ltv', 'cac', 'mrr', 'runway', 'gross_margin', 'burn_rate'],
    domain: 'finance',
    priority: 8,
  },
  {
    name: 'strategy_frameworks',
    description: "Strategic positioning frameworks for competitive moat analysis and market differentiation",
    keywords: ['strategy', 'positioning', 'moat', 'value_proposition', 'competitive_advantage'],
    domain: 'strategy',
    priority: 8,
  },
  {
    name: 'operations_frameworks',
    description: "Operational bottleneck analysis and delegation frameworks for solo founders",
    keywords: ['operations', 'bottleneck', 'delegation', 'systemization', 'dollar_audit'],
    domain: 'operations',
    priority: 8,
  },
  {
    name: 'marketing_frameworks',
    description: "B2B authority marketing and content strategy frameworks for professional services",
    keywords: ['marketing', 'authority', 'content', 'positioning', 'distribution', 'b2b'],
    domain: 'marketing',
    priority: 7,
  },
  {
    name: 'sales_frameworks',
    description: "Consultative B2B sales and pipeline management frameworks",
    keywords: ['sales', 'pipeline', 'ltv', 'cac', 'qualification', 'referral'],
    domain: 'sales',
    priority: 7,
  },
  {
    name: 'leadership_frameworks',
    description: "Founder energy management and leadership development frameworks",
    keywords: ['leadership', 'energy', 'ceo_time', 'focus', 'weekly_review'],
    domain: 'leadership',
    priority: 7,
  },
  {
    name: 'tech_decision_frameworks',
    description: "Build/buy/automate decision frameworks and AI stack evaluation criteria for solo founders",
    keywords: ['technology', 'build_buy_automate', 'stack', 'tools', 'automation', 'vendor_risk'],
    domain: 'technology',
    priority: 7,
  },
  {
    name: 'portfolio_intelligence_frameworks',
    description: "Cross-portfolio synergy detection and compounding operator principles",
    keywords: ['portfolio', 'synergy', 'compounding', 'audience', 'distribution', 'content_leverage'],
    domain: 'portfolio',
    priority: 6,
  },
];

/**
 * Return skills whose keywords appear in the user message, ranked by priority.
 */
export function matchSkills(userMessage: string): SkillDef[] {
  const lower = userMessage.toLowerCase();
  return SKILL_REGISTRY
    .filter(skill => skill.keywords.some(kw => lower.includes(kw)))
    .sort((a, b) => b.priority - a.priority);
}

/**
 * Build a context prefix to prepend to the system prompt based on matched skills.
 * Returns an empty string when no skills match.
 */
export function buildSkillContext(userMessage: string): string {
  const matched = matchSkills(userMessage);
  if (matched.length === 0) return '';
  const lines = matched.map(
    s => `[${s.domain.toUpperCase()} CONTEXT: ${s.description}]`,
  );
  return lines.join('\n') + '\n\n';
}

/**
 * Return all distinct domains covered by declared skills.
 */
export function skillDomains(): string[] {
  return [...new Set(SKILL_REGISTRY.map(s => s.domain))];
}
