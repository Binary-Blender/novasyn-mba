// Agicore Generated TypeScript Types
// App: novasyn_mba

export interface Business {
  id: string;
  name: string;
  tagline: string | null;
  description: string | null;
  stage: string;
  category: string | null;
  marketNiche: string | null;
  businessModel: string | null;
  revenueTarget: number;
  launchDate: string | null;
  exitDate: string | null;
  notes: string | null;
  createdAt: string;
  updatedAt: string;
}

export interface CreateBusinessInput {
  name: string;
  tagline?: string;
  description?: string;
  stage?: string;
  category?: string;
  marketNiche?: string;
  businessModel?: string;
  revenueTarget?: number;
  launchDate?: string;
  exitDate?: string;
  notes?: string;
}

export interface UpdateBusinessInput {
  name?: string;
  tagline?: string;
  description?: string;
  stage?: string;
  category?: string;
  marketNiche?: string;
  businessModel?: string;
  revenueTarget?: number;
  launchDate?: string;
  exitDate?: string;
  notes?: string;
}

export interface StrategicPosition {
  id: string;
  businessId: number;
  targetMarket: string | null;
  valueProposition: string | null;
  primaryMoat: string;
  competitors: string | null;
  unfairAdvantages: string | null;
  moatScore: number;
  businessId: string;
  createdAt: string;
  updatedAt: string;
}

export interface CreateStrategicPositionInput {
  businessId: number;
  targetMarket?: string;
  valueProposition?: string;
  primaryMoat?: string;
  competitors?: string;
  unfairAdvantages?: string;
  moatScore?: number;
  businessId: string;
}

export interface UpdateStrategicPositionInput {
  businessId?: number;
  targetMarket?: string;
  valueProposition?: string;
  primaryMoat?: string;
  competitors?: string;
  unfairAdvantages?: string;
  moatScore?: number;
}

export interface OperationsAudit {
  id: string;
  businessId: number;
  knowledgeBottleneck: number;
  qualityBottleneck: number;
  coordinationBottleneck: number;
  communicationBottleneck: number;
  creativeBottleneck: number;
  decisionBottleneck: number;
  overallScore: number;
  auditDate: string | null;
  businessId: string;
  createdAt: string;
  updatedAt: string;
}

export interface CreateOperationsAuditInput {
  businessId: number;
  knowledgeBottleneck?: number;
  qualityBottleneck?: number;
  coordinationBottleneck?: number;
  communicationBottleneck?: number;
  creativeBottleneck?: number;
  decisionBottleneck?: number;
  overallScore?: number;
  auditDate?: string;
  businessId: string;
}

export interface UpdateOperationsAuditInput {
  businessId?: number;
  knowledgeBottleneck?: number;
  qualityBottleneck?: number;
  coordinationBottleneck?: number;
  communicationBottleneck?: number;
  creativeBottleneck?: number;
  decisionBottleneck?: number;
  overallScore?: number;
  auditDate?: string;
}

export interface DollarAudit {
  id: string;
  businessId: number;
  activityName: string;
  tier: string;
  hoursPerWeek: number;
  aiTransferable: number;
  transferStatus: string;
  notes: string | null;
  businessId: string;
  createdAt: string;
  updatedAt: string;
}

export interface CreateDollarAuditInput {
  businessId: number;
  activityName: string;
  tier?: string;
  hoursPerWeek?: number;
  aiTransferable?: number;
  transferStatus?: string;
  notes?: string;
  businessId: string;
}

export interface UpdateDollarAuditInput {
  businessId?: number;
  activityName?: string;
  tier?: string;
  hoursPerWeek?: number;
  aiTransferable?: number;
  transferStatus?: string;
  notes?: string;
}

export interface FinancialSnapshot {
  id: string;
  businessId: number;
  period: string;
  revenue: number;
  cac: number;
  ltv: number;
  ltvCacRatio: number;
  grossMarginPct: number;
  mrr: number;
  cashRunwayMonths: number;
  recordedAt: string | null;
  businessId: string;
  createdAt: string;
  updatedAt: string;
}

export interface CreateFinancialSnapshotInput {
  businessId: number;
  period: string;
  revenue?: number;
  cac?: number;
  ltv?: number;
  ltvCacRatio?: number;
  grossMarginPct?: number;
  mrr?: number;
  cashRunwayMonths?: number;
  recordedAt?: string;
  businessId: string;
}

export interface UpdateFinancialSnapshotInput {
  businessId?: number;
  period?: string;
  revenue?: number;
  cac?: number;
  ltv?: number;
  ltvCacRatio?: number;
  grossMarginPct?: number;
  mrr?: number;
  cashRunwayMonths?: number;
  recordedAt?: string;
}

export interface MarketingAsset {
  id: string;
  businessId: number;
  assetType: string;
  title: string;
  url: string | null;
  platform: string | null;
  publishedAt: string | null;
  authorityStageImpact: string | null;
  businessId: string;
  createdAt: string;
  updatedAt: string;
}

export interface CreateMarketingAssetInput {
  businessId: number;
  assetType?: string;
  title: string;
  url?: string;
  platform?: string;
  publishedAt?: string;
  authorityStageImpact?: string;
  businessId: string;
}

export interface UpdateMarketingAssetInput {
  businessId?: number;
  assetType?: string;
  title?: string;
  url?: string;
  platform?: string;
  publishedAt?: string;
  authorityStageImpact?: string;
}

export interface SalesContact {
  id: string;
  businessId: number;
  name: string;
  company: string | null;
  stage: string;
  ltvPotential: number;
  source: string;
  nextAction: string | null;
  nextActionDate: string | null;
  notes: string | null;
  businessId: string;
  createdAt: string;
  updatedAt: string;
}

export interface CreateSalesContactInput {
  businessId: number;
  name: string;
  company?: string;
  stage?: string;
  ltvPotential?: number;
  source?: string;
  nextAction?: string;
  nextActionDate?: string;
  notes?: string;
  businessId: string;
}

export interface UpdateSalesContactInput {
  businessId?: number;
  name?: string;
  company?: string;
  stage?: string;
  ltvPotential?: number;
  source?: string;
  nextAction?: string;
  nextActionDate?: string;
  notes?: string;
}

export interface ContentPipelineItem {
  id: string;
  businessId: number;
  idea: string;
  status: string;
  platform: string | null;
  publishedAt: string | null;
  inboundLeadsGenerated: number;
  businessId: string;
  createdAt: string;
  updatedAt: string;
}

export interface CreateContentPipelineItemInput {
  businessId: number;
  idea: string;
  status?: string;
  platform?: string;
  publishedAt?: string;
  inboundLeadsGenerated?: number;
  businessId: string;
}

export interface UpdateContentPipelineItemInput {
  businessId?: number;
  idea?: string;
  status?: string;
  platform?: string;
  publishedAt?: string;
  inboundLeadsGenerated?: number;
}

export interface AIStackDecision {
  id: string;
  businessId: number;
  capability: string;
  decision: string;
  toolOrApproach: string | null;
  costPerMonth: number;
  isCoreCapability: number;
  reviewDate: string | null;
  businessId: string;
  createdAt: string;
  updatedAt: string;
}

export interface CreateAIStackDecisionInput {
  businessId: number;
  capability: string;
  decision?: string;
  toolOrApproach?: string;
  costPerMonth?: number;
  isCoreCapability?: number;
  reviewDate?: string;
  businessId: string;
}

export interface UpdateAIStackDecisionInput {
  businessId?: number;
  capability?: string;
  decision?: string;
  toolOrApproach?: string;
  costPerMonth?: number;
  isCoreCapability?: number;
  reviewDate?: string;
}

export interface BoardMember {
  id: string;
  name: string;
  expertiseDomain: string | null;
  relationshipType: string;
  lastConsulted: string | null;
  notes: string | null;
  createdAt: string;
  updatedAt: string;
}

export interface CreateBoardMemberInput {
  name: string;
  expertiseDomain?: string;
  relationshipType?: string;
  lastConsulted?: string;
  notes?: string;
}

export interface UpdateBoardMemberInput {
  name?: string;
  expertiseDomain?: string;
  relationshipType?: string;
  lastConsulted?: string;
  notes?: string;
}

export interface PortfolioSynergy {
  id: string;
  businessAId: number;
  businessBId: number;
  synergyType: string;
  description: string | null;
  potentialValue: string | null;
  status: string;
  businessId: string;
  createdAt: string;
  updatedAt: string;
}

export interface CreatePortfolioSynergyInput {
  businessAId: number;
  businessBId: number;
  synergyType?: string;
  description?: string;
  potentialValue?: string;
  status?: string;
  businessId: string;
}

export interface UpdatePortfolioSynergyInput {
  businessAId?: number;
  businessBId?: number;
  synergyType?: string;
  description?: string;
  potentialValue?: string;
  status?: string;
}

export interface WeeklyReview {
  id: string;
  businessId: number | null;
  weekOf: string;
  accomplishments: string | null;
  learnings: string | null;
  nextWeekFocus: string | null;
  energyLevel: number;
  ceoTimeHours: number;
  businessId: string;
  createdAt: string;
  updatedAt: string;
}

export interface CreateWeeklyReviewInput {
  businessId?: number;
  weekOf: string;
  accomplishments?: string;
  learnings?: string;
  nextWeekFocus?: string;
  energyLevel?: number;
  ceoTimeHours?: number;
  businessId: string;
}

export interface UpdateWeeklyReviewInput {
  businessId?: number;
  weekOf?: string;
  accomplishments?: string;
  learnings?: string;
  nextWeekFocus?: string;
  energyLevel?: number;
  ceoTimeHours?: number;
}

export interface StrategyAdvisorResult {
  moatAssessment: string;
  marketRisks: string;
  recommendedNextMove: string;
  questionToAsk: string;
  fullBriefing: string;
}

export interface OperationsAdvisorResult {
  highestLeverageBottleneck: string;
  bottleneckFixRecommendation: string;
  dollarRatioAnalysis: string;
  topTransferCandidates: string;
  fullBriefing: string;
}

export interface FinanceAdvisorResult {
  unitEconomicsDiagnosis: string;
  redFlagMetrics: string;
  pricingRecommendation: string;
  runwayAssessment: string;
  fullBriefing: string;
}

export interface MarketingAdvisorResult {
  authorityLadderPosition: string;
  gapAnalysis: string;
  topThreeContentMoves: string;
  platformRiskAssessment: string;
  fullBriefing: string;
}

export interface SalesAdvisorResult {
  pipelineHealthScore: string;
  staleContactActions: string;
  topFollowUpActions: string;
  referralArchitecture: string;
  fullBriefing: string;
}

export interface TechAdvisorResult {
  spendAnalysis: string;
  coreCapabilityRisk: string;
  dependencyConcentration: string;
  topAutomationOpportunities: string;
  fullBriefing: string;
}

export interface LeadershipAdvisorResult {
  energyTrendAssessment: string;
  ceoTimeRatio: string;
  patternIdentification: string;
  accountabilityQuestion: string;
  fullBriefing: string;
}

export interface PortfolioSynergyDetectorResult {
  topSynergies: string;
  synergyActions: string;
  fullBriefing: string;
}

export interface AdvanceBusinessStageResult {
  success: number;
  message: string;
}

export interface CalculateLtvCacResult {
  ltvCacRatio: number;
  healthFlag: string;
}

export interface ExportBusinessReportResult {
  filePath: string;
  success: number;
}

export interface GeneratePitchDeckOutlineResult {
  filePath: string;
  success: number;
}

export interface ImportCsvContactsResult {
  importedCount: number;
  skippedCount: number;
  errorSummary: string;
}

export interface SynthesizeFirst30DaysResult {
  day_1To_10: string;
  day_11To_20: string;
  day_21To_30: string;
  topPriority: string;
  fullPlan: string;
}
