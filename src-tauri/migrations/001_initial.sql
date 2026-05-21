-- Agicore Generated Migration
-- App: novasyn_mba
-- Generated: 2026-05-21

PRAGMA journal_mode = WAL;
PRAGMA foreign_keys = ON;
CREATE TABLE IF NOT EXISTS businesses (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  tagline TEXT,
  description TEXT,
  stage TEXT DEFAULT 'Idea',
  category TEXT,
  market_niche TEXT,
  business_model TEXT,
  revenue_target REAL DEFAULT 0,
  launch_date TEXT,
  exit_date TEXT,
  notes TEXT,
  created_at TEXT DEFAULT (datetime('now')),
  updated_at TEXT DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS strategic_positions (
  id TEXT PRIMARY KEY,
  business_id INTEGER NOT NULL,
  target_market TEXT,
  value_proposition TEXT,
  primary_moat TEXT DEFAULT 'reputation',
  competitors TEXT,
  unfair_advantages TEXT,
  moat_score INTEGER DEFAULT 5,
  business_id TEXT NOT NULL,
  created_at TEXT DEFAULT (datetime('now')),
  updated_at TEXT DEFAULT (datetime('now')),
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_strategic_positions_business_id ON strategic_positions(business_id);

CREATE TABLE IF NOT EXISTS operations_audits (
  id TEXT PRIMARY KEY,
  business_id INTEGER NOT NULL,
  knowledge_bottleneck INTEGER DEFAULT 3,
  quality_bottleneck INTEGER DEFAULT 3,
  coordination_bottleneck INTEGER DEFAULT 3,
  communication_bottleneck INTEGER DEFAULT 3,
  creative_bottleneck INTEGER DEFAULT 3,
  decision_bottleneck INTEGER DEFAULT 3,
  overall_score INTEGER DEFAULT 18,
  audit_date TEXT,
  business_id TEXT NOT NULL,
  created_at TEXT DEFAULT (datetime('now')),
  updated_at TEXT DEFAULT (datetime('now')),
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_operations_audits_business_id ON operations_audits(business_id);

CREATE TABLE IF NOT EXISTS dollar_audits (
  id TEXT PRIMARY KEY,
  business_id INTEGER NOT NULL,
  activity_name TEXT NOT NULL,
  tier TEXT DEFAULT 'twenty',
  hours_per_week REAL DEFAULT 0,
  ai_transferable INTEGER DEFAULT 0,
  transfer_status TEXT DEFAULT 'not_started',
  notes TEXT,
  business_id TEXT NOT NULL,
  created_at TEXT DEFAULT (datetime('now')),
  updated_at TEXT DEFAULT (datetime('now')),
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_dollar_audits_business_id ON dollar_audits(business_id);

CREATE TABLE IF NOT EXISTS financial_snapshots (
  id TEXT PRIMARY KEY,
  business_id INTEGER NOT NULL,
  period TEXT NOT NULL,
  revenue REAL DEFAULT 0,
  cac REAL DEFAULT 0,
  ltv REAL DEFAULT 0,
  ltv_cac_ratio REAL DEFAULT 0,
  gross_margin_pct REAL DEFAULT 0,
  mrr REAL DEFAULT 0,
  cash_runway_months REAL DEFAULT 0,
  recorded_at TEXT,
  business_id TEXT NOT NULL,
  created_at TEXT DEFAULT (datetime('now')),
  updated_at TEXT DEFAULT (datetime('now')),
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_financial_snapshots_business_id ON financial_snapshots(business_id);

CREATE TABLE IF NOT EXISTS marketing_assets (
  id TEXT PRIMARY KEY,
  business_id INTEGER NOT NULL,
  asset_type TEXT DEFAULT 'content',
  title TEXT NOT NULL,
  url TEXT,
  platform TEXT,
  published_at TEXT,
  authority_stage_impact TEXT,
  business_id TEXT NOT NULL,
  created_at TEXT DEFAULT (datetime('now')),
  updated_at TEXT DEFAULT (datetime('now')),
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_marketing_assets_business_id ON marketing_assets(business_id);

CREATE TABLE IF NOT EXISTS sales_contacts (
  id TEXT PRIMARY KEY,
  business_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  company TEXT,
  stage TEXT DEFAULT 'prospect',
  ltv_potential REAL DEFAULT 0,
  source TEXT DEFAULT 'inbound',
  next_action TEXT,
  next_action_date TEXT,
  notes TEXT,
  business_id TEXT NOT NULL,
  created_at TEXT DEFAULT (datetime('now')),
  updated_at TEXT DEFAULT (datetime('now')),
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_sales_contacts_business_id ON sales_contacts(business_id);

CREATE TABLE IF NOT EXISTS content_pipeline_items (
  id TEXT PRIMARY KEY,
  business_id INTEGER NOT NULL,
  idea TEXT NOT NULL,
  status TEXT DEFAULT 'idea',
  platform TEXT,
  published_at TEXT,
  inbound_leads_generated INTEGER DEFAULT 0,
  business_id TEXT NOT NULL,
  created_at TEXT DEFAULT (datetime('now')),
  updated_at TEXT DEFAULT (datetime('now')),
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_content_pipeline_items_business_id ON content_pipeline_items(business_id);

CREATE TABLE IF NOT EXISTS a_i_stack_decisions (
  id TEXT PRIMARY KEY,
  business_id INTEGER NOT NULL,
  capability TEXT NOT NULL,
  decision TEXT DEFAULT 'buy',
  tool_or_approach TEXT,
  cost_per_month REAL DEFAULT 0,
  is_core_capability INTEGER DEFAULT 0,
  review_date TEXT,
  business_id TEXT NOT NULL,
  created_at TEXT DEFAULT (datetime('now')),
  updated_at TEXT DEFAULT (datetime('now')),
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_a_i_stack_decisions_business_id ON a_i_stack_decisions(business_id);

CREATE TABLE IF NOT EXISTS board_members (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  expertise_domain TEXT,
  relationship_type TEXT DEFAULT 'mentor',
  last_consulted TEXT,
  notes TEXT,
  created_at TEXT DEFAULT (datetime('now')),
  updated_at TEXT DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS portfolio_synergies (
  id TEXT PRIMARY KEY,
  business_a_id INTEGER NOT NULL,
  business_b_id INTEGER NOT NULL,
  synergy_type TEXT DEFAULT 'audience',
  description TEXT,
  potential_value TEXT,
  status TEXT DEFAULT 'identified',
  business_id TEXT NOT NULL,
  created_at TEXT DEFAULT (datetime('now')),
  updated_at TEXT DEFAULT (datetime('now')),
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_portfolio_synergies_business_id ON portfolio_synergies(business_id);

CREATE TABLE IF NOT EXISTS weekly_reviews (
  id TEXT PRIMARY KEY,
  business_id INTEGER,
  week_of TEXT NOT NULL,
  accomplishments TEXT,
  learnings TEXT,
  next_week_focus TEXT,
  energy_level INTEGER DEFAULT 3,
  ceo_time_hours REAL DEFAULT 0,
  business_id TEXT NOT NULL,
  created_at TEXT DEFAULT (datetime('now')),
  updated_at TEXT DEFAULT (datetime('now')),
  FOREIGN KEY (business_id) REFERENCES businesses(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_weekly_reviews_business_id ON weekly_reviews(business_id);

-- SEED: idempotent insert rows from ENTITY SEED blocks
INSERT OR IGNORE INTO businesses (name, tagline, description, stage, category, market_niche, business_model, revenue_target) VALUES ('Clarity Consulting', 'We help mid-size professional services firms stop losing revenue to broken internal processes', 'A boutique operations consulting practice specializing in workflow audit and system design for law firms and accounting practices. Retainer-based, 5 active clients, referral-only intake.', 'Active', 'Consulting', 'Operations consulting for professional services firms (law, accounting)', 'retainer', 240000);
INSERT OR IGNORE INTO strategic_positions (business_id, target_market, value_proposition, primary_moat, competitors, unfair_advantages, moat_score) VALUES (1, 'Managing partners at law firms and accounting practices with 10-50 employees experiencing growth-related operational chaos', 'We audit and rebuild internal workflows so professional services firms can scale without the managing partner becoming the bottleneck for every decision', 'system', '["Large consulting firms (too expensive, not specialized)", "Generic business coaches (no domain expertise)", "In-house ops hires (not available until the firm can afford a COO)"]', '15 years of domain expertise in professional services operations. Proprietary 6-bottleneck audit methodology. Network of referral partners across the legal and accounting industries.', 7);
INSERT OR IGNORE INTO operations_audits (business_id, knowledge_bottleneck, quality_bottleneck, coordination_bottleneck, communication_bottleneck, creative_bottleneck, decision_bottleneck, overall_score, audit_date) VALUES (1, 4, 3, 2, 3, 2, 3, 17, '2026-05-01');
INSERT OR IGNORE INTO dollar_audits (business_id, activity_name, tier, hours_per_week, ai_transferable, transfer_status, notes) VALUES (1, 'Client report formatting and delivery', 'twenty', 3, 1, 'in_progress', 'Using Claude to generate first draft from audit notes. Review takes 30 min instead of 3 hours.');
INSERT OR IGNORE INTO dollar_audits (business_id, activity_name, tier, hours_per_week, ai_transferable, transfer_status, notes) VALUES (1, 'Prospect qualification calls', 'two_hundred', 2, 0, 'not_started', 'Must remain with founder — requires judgment about client fit and relationship quality.');
INSERT OR IGNORE INTO dollar_audits (business_id, activity_name, tier, hours_per_week, ai_transferable, transfer_status, notes) VALUES (1, 'Scheduling and calendar management', 'twenty', 2, 1, 'not_started', 'Could use Calendly + AI email responder. Not set up yet.');
INSERT OR IGNORE INTO dollar_audits (business_id, activity_name, tier, hours_per_week, ai_transferable, transfer_status, notes) VALUES (1, 'Strategic roadmap design for each client', 'two_hundred', 6, 0, 'not_started', 'Core $200 work. Should protect and expand, not delegate.');
INSERT OR IGNORE INTO financial_snapshots (business_id, period, revenue, cac, ltv, ltv_cac_ratio, gross_margin_pct, mrr, cash_runway_months, recorded_at) VALUES (1, '2026-Q1', 52000, 800, 24000, 30, 82, 0, 9, '2026-04-01');
INSERT OR IGNORE INTO marketing_assets (business_id, asset_type, title, url, platform, published_at, authority_stage_impact) VALUES (1, 'case_study', 'How Meridian Law Group Cut Partner Review Time by 60% in 90 Days', 'https://clarityconsulting.co/case-studies/meridian-law', 'website', '2026-03-15', 'credible->authoritative');
INSERT OR IGNORE INTO marketing_assets (business_id, asset_type, title, url, platform, published_at, authority_stage_impact) VALUES (1, 'content', 'The 6 Operational Bottlenecks Killing Professional Services Firms', 'https://clarityconsulting.co/articles/6-bottlenecks', 'website', '2026-02-01', 'aware->credible');
INSERT OR IGNORE INTO sales_contacts (business_id, name, company, stage, ltv_potential, source, next_action, next_action_date, notes) VALUES (1, 'Rachel Torres', 'Torres & Webb LLP', 'proposal', 18000, 'referral', 'Send revised proposal with phased payment option', '2026-05-19', 'Referred by Marcus Chen (Meridian Law). Managing partner. 22-person firm. Main objection is timing — wants to start after tax season ends. Proposal sent 2026-05-10, awaiting response.');
INSERT OR IGNORE INTO sales_contacts (business_id, name, company, stage, ltv_potential, source, next_action, next_action_date, notes) VALUES (1, 'David Park', 'Park Holloway Accounting', 'qualified', 22000, 'inbound', 'Schedule discovery call', '2026-05-20', 'Found us via the 6-bottlenecks article. 35-person accounting firm. Said they have a ''coordination nightmare'' — every workflow runs through the senior partner. Perfect fit.');
INSERT OR IGNORE INTO content_pipeline_items (business_id, idea, status, platform, inbound_leads_generated) VALUES (1, 'Why Professional Services Firms Plateau at 20 Employees (And How to Break Through)', 'drafted', 'LinkedIn + website', 0);
INSERT OR IGNORE INTO content_pipeline_items (business_id, idea, status, platform, inbound_leads_generated) VALUES (1, 'The Delegation Ladder: A Framework for Getting Work Off the Managing Partner''s Plate', 'idea', 'website', 0);
INSERT OR IGNORE INTO a_i_stack_decisions (business_id, capability, decision, tool_or_approach, cost_per_month, is_core_capability, review_date) VALUES (1, 'AI-assisted report drafting', 'automate', 'Claude API via custom prompt template', 40, 0, '2026-11-01');
INSERT OR IGNORE INTO a_i_stack_decisions (business_id, capability, decision, tool_or_approach, cost_per_month, is_core_capability, review_date) VALUES (1, '6-bottleneck audit methodology', 'build', 'NovaSyn MBA (this app) + proprietary scoring rubric', 0, 1, '2027-01-01');
INSERT OR IGNORE INTO a_i_stack_decisions (business_id, capability, decision, tool_or_approach, cost_per_month, is_core_capability, review_date) VALUES (1, 'Client communication and scheduling', 'buy', 'Calendly + Gmail', 16, 0, '2026-12-01');
INSERT OR IGNORE INTO board_members (name, expertise_domain, relationship_type, notes) VALUES ('Charlie Munger', 'strategy', 'archetype', 'Mental model: invert. Ask ''what would make this business fail?'' and eliminate those things. Think in base rates. Avoid being foolish rather than trying to be brilliant.');
INSERT OR IGNORE INTO board_members (name, expertise_domain, relationship_type, notes) VALUES ('Patrick McKenzie', 'marketing', 'author', 'Key insight: positioning and pricing are the two highest-leverage decisions in a small business. Most founders under-charge and over-explain. Write for people who are already searching for your solution.');
INSERT OR IGNORE INTO weekly_reviews (business_id, week_of, accomplishments, learnings, next_week_focus, energy_level, ceo_time_hours) VALUES (1, '2026-05-12', 'Sent Torres proposal. Published bottleneck article (47 LinkedIn shares). Completed Park Holloway qualification call.', 'The phased payment option in the Torres proposal was my idea at the last minute — should be a standard offering from the start. Prospects respond much better to phased options.', 'Follow up Torres. Schedule Park discovery call. Outline delegation ladder article.', 4, 22);