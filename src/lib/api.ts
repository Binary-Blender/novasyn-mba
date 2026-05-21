// Agicore Generated Invoke Wrappers
// App: novasyn_mba

import { invoke } from '@tauri-apps/api/core';
import type {
  Business, CreateBusinessInput, UpdateBusinessInput,
  StrategicPosition, CreateStrategicPositionInput, UpdateStrategicPositionInput,
  OperationsAudit, CreateOperationsAuditInput, UpdateOperationsAuditInput,
  DollarAudit, CreateDollarAuditInput, UpdateDollarAuditInput,
  FinancialSnapshot, CreateFinancialSnapshotInput, UpdateFinancialSnapshotInput,
  MarketingAsset, CreateMarketingAssetInput, UpdateMarketingAssetInput,
  SalesContact, CreateSalesContactInput, UpdateSalesContactInput,
  ContentPipelineItem, CreateContentPipelineItemInput, UpdateContentPipelineItemInput,
  AIStackDecision, CreateAIStackDecisionInput, UpdateAIStackDecisionInput,
  BoardMember, CreateBoardMemberInput, UpdateBoardMemberInput,
  PortfolioSynergy, CreatePortfolioSynergyInput, UpdatePortfolioSynergyInput,
  WeeklyReview, CreateWeeklyReviewInput, UpdateWeeklyReviewInput,
  StrategyAdvisorResult,
  OperationsAdvisorResult,
  FinanceAdvisorResult,
  MarketingAdvisorResult,
  SalesAdvisorResult,
  TechAdvisorResult,
  LeadershipAdvisorResult,
  PortfolioSynergyDetectorResult,
  AdvanceBusinessStageResult,
  CalculateLtvCacResult,
  ExportBusinessReportResult,
  GeneratePitchDeckOutlineResult,
  ImportCsvContactsResult,
  SynthesizeFirst30DaysResult,
} from './types';

// --- Business ---
export const listBusinesss = () =>
  invoke<Business[]>('list_businesses');

export const createBusiness = (input: CreateBusinessInput) =>
  invoke<Business>('create_business', { input });

export const getBusiness = (id: string) =>
  invoke<Business>('get_business', { id });

export const updateBusiness = (id: string, input: UpdateBusinessInput) =>
  invoke<Business>('update_business', { id, input });

export const deleteBusiness = (id: string) =>
  invoke<void>('delete_business', { id });

// --- StrategicPosition ---
export const listStrategicPositions = () =>
  invoke<StrategicPosition[]>('list_strategic_positions');

export const createStrategicPosition = (input: CreateStrategicPositionInput) =>
  invoke<StrategicPosition>('create_strategic_position', { input });

export const getStrategicPosition = (id: string) =>
  invoke<StrategicPosition>('get_strategic_position', { id });

export const updateStrategicPosition = (id: string, input: UpdateStrategicPositionInput) =>
  invoke<StrategicPosition>('update_strategic_position', { id, input });

export const deleteStrategicPosition = (id: string) =>
  invoke<void>('delete_strategic_position', { id });

// --- OperationsAudit ---
export const listOperationsAudits = () =>
  invoke<OperationsAudit[]>('list_operations_audits');

export const createOperationsAudit = (input: CreateOperationsAuditInput) =>
  invoke<OperationsAudit>('create_operations_audit', { input });

export const getOperationsAudit = (id: string) =>
  invoke<OperationsAudit>('get_operations_audit', { id });

export const updateOperationsAudit = (id: string, input: UpdateOperationsAuditInput) =>
  invoke<OperationsAudit>('update_operations_audit', { id, input });

export const deleteOperationsAudit = (id: string) =>
  invoke<void>('delete_operations_audit', { id });

// --- DollarAudit ---
export const listDollarAudits = () =>
  invoke<DollarAudit[]>('list_dollar_audits');

export const createDollarAudit = (input: CreateDollarAuditInput) =>
  invoke<DollarAudit>('create_dollar_audit', { input });

export const getDollarAudit = (id: string) =>
  invoke<DollarAudit>('get_dollar_audit', { id });

export const updateDollarAudit = (id: string, input: UpdateDollarAuditInput) =>
  invoke<DollarAudit>('update_dollar_audit', { id, input });

export const deleteDollarAudit = (id: string) =>
  invoke<void>('delete_dollar_audit', { id });

// --- FinancialSnapshot ---
export const listFinancialSnapshots = () =>
  invoke<FinancialSnapshot[]>('list_financial_snapshots');

export const createFinancialSnapshot = (input: CreateFinancialSnapshotInput) =>
  invoke<FinancialSnapshot>('create_financial_snapshot', { input });

export const getFinancialSnapshot = (id: string) =>
  invoke<FinancialSnapshot>('get_financial_snapshot', { id });

export const updateFinancialSnapshot = (id: string, input: UpdateFinancialSnapshotInput) =>
  invoke<FinancialSnapshot>('update_financial_snapshot', { id, input });

export const deleteFinancialSnapshot = (id: string) =>
  invoke<void>('delete_financial_snapshot', { id });

// --- MarketingAsset ---
export const listMarketingAssets = () =>
  invoke<MarketingAsset[]>('list_marketing_assets');

export const createMarketingAsset = (input: CreateMarketingAssetInput) =>
  invoke<MarketingAsset>('create_marketing_asset', { input });

export const getMarketingAsset = (id: string) =>
  invoke<MarketingAsset>('get_marketing_asset', { id });

export const updateMarketingAsset = (id: string, input: UpdateMarketingAssetInput) =>
  invoke<MarketingAsset>('update_marketing_asset', { id, input });

export const deleteMarketingAsset = (id: string) =>
  invoke<void>('delete_marketing_asset', { id });

// --- SalesContact ---
export const listSalesContacts = () =>
  invoke<SalesContact[]>('list_sales_contacts');

export const createSalesContact = (input: CreateSalesContactInput) =>
  invoke<SalesContact>('create_sales_contact', { input });

export const getSalesContact = (id: string) =>
  invoke<SalesContact>('get_sales_contact', { id });

export const updateSalesContact = (id: string, input: UpdateSalesContactInput) =>
  invoke<SalesContact>('update_sales_contact', { id, input });

export const deleteSalesContact = (id: string) =>
  invoke<void>('delete_sales_contact', { id });

// --- ContentPipelineItem ---
export const listContentPipelineItems = () =>
  invoke<ContentPipelineItem[]>('list_content_pipeline_items');

export const createContentPipelineItem = (input: CreateContentPipelineItemInput) =>
  invoke<ContentPipelineItem>('create_content_pipeline_item', { input });

export const getContentPipelineItem = (id: string) =>
  invoke<ContentPipelineItem>('get_content_pipeline_item', { id });

export const updateContentPipelineItem = (id: string, input: UpdateContentPipelineItemInput) =>
  invoke<ContentPipelineItem>('update_content_pipeline_item', { id, input });

export const deleteContentPipelineItem = (id: string) =>
  invoke<void>('delete_content_pipeline_item', { id });

// --- AIStackDecision ---
export const listAIStackDecisions = () =>
  invoke<AIStackDecision[]>('list_a_i_stack_decisions');

export const createAIStackDecision = (input: CreateAIStackDecisionInput) =>
  invoke<AIStackDecision>('create_a_i_stack_decision', { input });

export const getAIStackDecision = (id: string) =>
  invoke<AIStackDecision>('get_a_i_stack_decision', { id });

export const updateAIStackDecision = (id: string, input: UpdateAIStackDecisionInput) =>
  invoke<AIStackDecision>('update_a_i_stack_decision', { id, input });

export const deleteAIStackDecision = (id: string) =>
  invoke<void>('delete_a_i_stack_decision', { id });

// --- BoardMember ---
export const listBoardMembers = () =>
  invoke<BoardMember[]>('list_board_members');

export const createBoardMember = (input: CreateBoardMemberInput) =>
  invoke<BoardMember>('create_board_member', { input });

export const getBoardMember = (id: string) =>
  invoke<BoardMember>('get_board_member', { id });

export const updateBoardMember = (id: string, input: UpdateBoardMemberInput) =>
  invoke<BoardMember>('update_board_member', { id, input });

export const deleteBoardMember = (id: string) =>
  invoke<void>('delete_board_member', { id });

// --- PortfolioSynergy ---
export const listPortfolioSynergys = () =>
  invoke<PortfolioSynergy[]>('list_portfolio_synergies');

export const createPortfolioSynergy = (input: CreatePortfolioSynergyInput) =>
  invoke<PortfolioSynergy>('create_portfolio_synergy', { input });

export const getPortfolioSynergy = (id: string) =>
  invoke<PortfolioSynergy>('get_portfolio_synergy', { id });

export const updatePortfolioSynergy = (id: string, input: UpdatePortfolioSynergyInput) =>
  invoke<PortfolioSynergy>('update_portfolio_synergy', { id, input });

export const deletePortfolioSynergy = (id: string) =>
  invoke<void>('delete_portfolio_synergy', { id });

// --- WeeklyReview ---
export const listWeeklyReviews = () =>
  invoke<WeeklyReview[]>('list_weekly_reviews');

export const createWeeklyReview = (input: CreateWeeklyReviewInput) =>
  invoke<WeeklyReview>('create_weekly_review', { input });

export const getWeeklyReview = (id: string) =>
  invoke<WeeklyReview>('get_weekly_review', { id });

export const updateWeeklyReview = (id: string, input: UpdateWeeklyReviewInput) =>
  invoke<WeeklyReview>('update_weekly_review', { id, input });

export const deleteWeeklyReview = (id: string) =>
  invoke<void>('delete_weekly_review', { id });

// --- strategy_advisor ---
export const strategyAdvisor = (businessId: number, businessName: string, targetMarket: string, valueProposition: string, primaryMoat: string, competitors: string, unfairAdvantages: string, moatScore: number) =>
  invoke<StrategyAdvisorResult>('strategy_advisor', { businessId, businessName, targetMarket, valueProposition, primaryMoat, competitors, unfairAdvantages, moatScore });

// --- operations_advisor ---
export const operationsAdvisor = (businessId: number, businessName: string, knowledgeBottleneck: number, qualityBottleneck: number, coordinationBottleneck: number, communicationBottleneck: number, creativeBottleneck: number, decisionBottleneck: number, overallScore: number, dollarAuditSummary: string) =>
  invoke<OperationsAdvisorResult>('operations_advisor', { businessId, businessName, knowledgeBottleneck, qualityBottleneck, coordinationBottleneck, communicationBottleneck, creativeBottleneck, decisionBottleneck, overallScore, dollarAuditSummary });

// --- finance_advisor ---
export const financeAdvisor = (businessId: number, businessName: string, period: string, revenue: number, cac: number, ltv: number, ltvCacRatio: number, grossMarginPct: number, mrr: number, cashRunwayMonths: number) =>
  invoke<FinanceAdvisorResult>('finance_advisor', { businessId, businessName, period, revenue, cac, ltv, ltvCacRatio, grossMarginPct, mrr, cashRunwayMonths });

// --- marketing_advisor ---
export const marketingAdvisor = (businessId: number, businessName: string, assetSummary: string, contentPipelineSummary: string, totalAssets: number, publishedContentCount: number, totalInboundLeads: number) =>
  invoke<MarketingAdvisorResult>('marketing_advisor', { businessId, businessName, assetSummary, contentPipelineSummary, totalAssets, publishedContentCount, totalInboundLeads });

// --- sales_advisor ---
export const salesAdvisor = (businessId: number, businessName: string, pipelineSummary: string, staleContactsSummary: string, referralPct: number, totalPipelineValue: number) =>
  invoke<SalesAdvisorResult>('sales_advisor', { businessId, businessName, pipelineSummary, staleContactsSummary, referralPct, totalPipelineValue });

// --- tech_advisor ---
export const techAdvisor = (businessId: number, businessName: string, stackSummary: string, totalMonthlyToolSpend: number, coreCapabilitySpend: number) =>
  invoke<TechAdvisorResult>('tech_advisor', { businessId, businessName, stackSummary, totalMonthlyToolSpend, coreCapabilitySpend });

// --- leadership_advisor ---
export const leadershipAdvisor = (businessId: number, founderName: string, recentReviews: string, avgEnergyLevel: number, avgCeoTimeHours: number, reviewCount: number) =>
  invoke<LeadershipAdvisorResult>('leadership_advisor', { businessId, founderName, recentReviews, avgEnergyLevel, avgCeoTimeHours, reviewCount });

// --- portfolio_synergy_detector ---
export const portfolioSynergyDetector = (founderName: string, portfolioSummary: string, existingSynergies: string) =>
  invoke<PortfolioSynergyDetectorResult>('portfolio_synergy_detector', { founderName, portfolioSummary, existingSynergies });

// --- advance_business_stage ---
export const advanceBusinessStage = (businessId: number, newStage: string) =>
  invoke<AdvanceBusinessStageResult>('advance_business_stage', { businessId, newStage });

// --- calculate_ltv_cac ---
export const calculateLtvCac = (snapshotId: number, ltv: number, cac: number) =>
  invoke<CalculateLtvCacResult>('calculate_ltv_cac', { snapshotId, ltv, cac });

// --- export_business_report ---
export const exportBusinessReport = (businessId: number) =>
  invoke<ExportBusinessReportResult>('export_business_report', { businessId });

// --- generate_pitch_deck_outline ---
export const generatePitchDeckOutline = (businessId: number) =>
  invoke<GeneratePitchDeckOutlineResult>('generate_pitch_deck_outline', { businessId });

// --- import_csv_contacts ---
export const importCsvContacts = (businessId: number, csvPath: string) =>
  invoke<ImportCsvContactsResult>('import_csv_contacts', { businessId, csvPath });

// --- synthesize_first_30_days ---
export const synthesizeFirst_30Days = (businessId: number) =>
  invoke<SynthesizeFirst30DaysResult>('synthesize_first_30_days', { businessId });

// --- AI key management ---
export const getApiKeys = () =>
  invoke<Record<string, string>>('get_api_keys');

export const setApiKey = (provider: string, key: string) =>
  invoke<void>('set_api_key', { provider, key });

// --- Document file I/O ---
export interface ScannedDocument {
  path: string; title: string; size: number; modifiedAt: string;
}
export const readDocumentContent = (filePath: string) =>
  invoke<string>('read_document_content', { filePath });
export const writeDocumentContent = (filePath: string, content: string) =>
  invoke<void>('write_document_content', { filePath, content });
export const scanDocumentsDir = (dir: string) =>
  invoke<ScannedDocument[]>('scan_documents_dir', { dir });

// --- Send To (Semantic Compilers) ---
export const reviewsToActionPlan = (messageIds: string[]) =>
  invoke<unknown>('reviews_to_action_plan', { messageIds });

export const advisorOutputsToPriorities = (messageIds: string[]) =>
  invoke<unknown>('advisor_outputs_to_priorities', { messageIds });

export const financialDataToReport = (messageIds: string[]) =>
  invoke<unknown>('financial_data_to_report', { messageIds });

export const businessDataToPitch = (messageIds: string[]) =>
  invoke<unknown>('business_data_to_pitch', { messageIds });
