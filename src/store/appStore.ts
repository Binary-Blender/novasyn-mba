// Agicore Generated Zustand Store
// App: novasyn_mba

import { create } from 'zustand';
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
} from '../lib/types';
import {
  listBusinesss, createBusiness, updateBusiness, deleteBusiness,
  listStrategicPositions, createStrategicPosition, updateStrategicPosition, deleteStrategicPosition,
  listOperationsAudits, createOperationsAudit, updateOperationsAudit, deleteOperationsAudit,
  listDollarAudits, createDollarAudit, updateDollarAudit, deleteDollarAudit,
  listFinancialSnapshots, createFinancialSnapshot, updateFinancialSnapshot, deleteFinancialSnapshot,
  listMarketingAssets, createMarketingAsset, updateMarketingAsset, deleteMarketingAsset,
  listSalesContacts, createSalesContact, updateSalesContact, deleteSalesContact,
  listContentPipelineItems, createContentPipelineItem, updateContentPipelineItem, deleteContentPipelineItem,
  listAIStackDecisions, createAIStackDecision, updateAIStackDecision, deleteAIStackDecision,
  listBoardMembers, createBoardMember, updateBoardMember, deleteBoardMember,
  listPortfolioSynergys, createPortfolioSynergy, updatePortfolioSynergy, deletePortfolioSynergy,
  listWeeklyReviews, createWeeklyReview, updateWeeklyReview, deleteWeeklyReview,
} from '../lib/api';

interface AppState {
  currentView: string;
  setCurrentView: (view: string) => void;

  selectedModel: string;
  setSelectedModel: (model: string) => void;

  businesss: Business[];
  selectedBusinessId: string | null;
  loadBusinesss: () => Promise<void>;
  addBusiness: (input: CreateBusinessInput) => Promise<void>;
  editBusiness: (id: string, input: UpdateBusinessInput) => Promise<void>;
  removeBusiness: (id: string) => Promise<void>;
  selectBusiness: (id: string | null) => void;

  strategicPositions: StrategicPosition[];
  selectedStrategicPositionId: string | null;
  loadStrategicPositions: () => Promise<void>;
  addStrategicPosition: (input: CreateStrategicPositionInput) => Promise<void>;
  editStrategicPosition: (id: string, input: UpdateStrategicPositionInput) => Promise<void>;
  removeStrategicPosition: (id: string) => Promise<void>;
  selectStrategicPosition: (id: string | null) => void;

  operationsAudits: OperationsAudit[];
  selectedOperationsAuditId: string | null;
  loadOperationsAudits: () => Promise<void>;
  addOperationsAudit: (input: CreateOperationsAuditInput) => Promise<void>;
  editOperationsAudit: (id: string, input: UpdateOperationsAuditInput) => Promise<void>;
  removeOperationsAudit: (id: string) => Promise<void>;
  selectOperationsAudit: (id: string | null) => void;

  dollarAudits: DollarAudit[];
  selectedDollarAuditId: string | null;
  loadDollarAudits: () => Promise<void>;
  addDollarAudit: (input: CreateDollarAuditInput) => Promise<void>;
  editDollarAudit: (id: string, input: UpdateDollarAuditInput) => Promise<void>;
  removeDollarAudit: (id: string) => Promise<void>;
  selectDollarAudit: (id: string | null) => void;

  financialSnapshots: FinancialSnapshot[];
  selectedFinancialSnapshotId: string | null;
  loadFinancialSnapshots: () => Promise<void>;
  addFinancialSnapshot: (input: CreateFinancialSnapshotInput) => Promise<void>;
  editFinancialSnapshot: (id: string, input: UpdateFinancialSnapshotInput) => Promise<void>;
  removeFinancialSnapshot: (id: string) => Promise<void>;
  selectFinancialSnapshot: (id: string | null) => void;

  marketingAssets: MarketingAsset[];
  selectedMarketingAssetId: string | null;
  loadMarketingAssets: () => Promise<void>;
  addMarketingAsset: (input: CreateMarketingAssetInput) => Promise<void>;
  editMarketingAsset: (id: string, input: UpdateMarketingAssetInput) => Promise<void>;
  removeMarketingAsset: (id: string) => Promise<void>;
  selectMarketingAsset: (id: string | null) => void;

  salesContacts: SalesContact[];
  selectedSalesContactId: string | null;
  loadSalesContacts: () => Promise<void>;
  addSalesContact: (input: CreateSalesContactInput) => Promise<void>;
  editSalesContact: (id: string, input: UpdateSalesContactInput) => Promise<void>;
  removeSalesContact: (id: string) => Promise<void>;
  selectSalesContact: (id: string | null) => void;

  contentPipelineItems: ContentPipelineItem[];
  selectedContentPipelineItemId: string | null;
  loadContentPipelineItems: () => Promise<void>;
  addContentPipelineItem: (input: CreateContentPipelineItemInput) => Promise<void>;
  editContentPipelineItem: (id: string, input: UpdateContentPipelineItemInput) => Promise<void>;
  removeContentPipelineItem: (id: string) => Promise<void>;
  selectContentPipelineItem: (id: string | null) => void;

  aIStackDecisions: AIStackDecision[];
  selectedAIStackDecisionId: string | null;
  loadAIStackDecisions: () => Promise<void>;
  addAIStackDecision: (input: CreateAIStackDecisionInput) => Promise<void>;
  editAIStackDecision: (id: string, input: UpdateAIStackDecisionInput) => Promise<void>;
  removeAIStackDecision: (id: string) => Promise<void>;
  selectAIStackDecision: (id: string | null) => void;

  boardMembers: BoardMember[];
  selectedBoardMemberId: string | null;
  loadBoardMembers: () => Promise<void>;
  addBoardMember: (input: CreateBoardMemberInput) => Promise<void>;
  editBoardMember: (id: string, input: UpdateBoardMemberInput) => Promise<void>;
  removeBoardMember: (id: string) => Promise<void>;
  selectBoardMember: (id: string | null) => void;

  portfolioSynergys: PortfolioSynergy[];
  selectedPortfolioSynergyId: string | null;
  loadPortfolioSynergys: () => Promise<void>;
  addPortfolioSynergy: (input: CreatePortfolioSynergyInput) => Promise<void>;
  editPortfolioSynergy: (id: string, input: UpdatePortfolioSynergyInput) => Promise<void>;
  removePortfolioSynergy: (id: string) => Promise<void>;
  selectPortfolioSynergy: (id: string | null) => void;

  weeklyReviews: WeeklyReview[];
  selectedWeeklyReviewId: string | null;
  loadWeeklyReviews: () => Promise<void>;
  addWeeklyReview: (input: CreateWeeklyReviewInput) => Promise<void>;
  editWeeklyReview: (id: string, input: UpdateWeeklyReviewInput) => Promise<void>;
  removeWeeklyReview: (id: string) => Promise<void>;
  selectWeeklyReview: (id: string | null) => void;

}

export const useAppStore = create<AppState>((set, get) => ({
  currentView: 'portfolio_dashboard',
  setCurrentView: (view) => set({ currentView: view }),

  selectedModel: 'claude-sonnet-4-6',
  setSelectedModel: (model) => set({ selectedModel: model }),

  businesss: [],
  selectedBusinessId: null,
  loadBusinesss: async () => {
    const businesss = await listBusinesss();
    set({ businesss });
  },
  addBusiness: async (input) => {
    await createBusiness(input);
    await get().loadBusinesss();
  },
  editBusiness: async (id, input) => {
    await updateBusiness(id, input);
    await get().loadBusinesss();
  },
  removeBusiness: async (id) => {
    await deleteBusiness(id);
    await get().loadBusinesss();
  },
  selectBusiness: (id) => set({ selectedBusinessId: id }),

  strategicPositions: [],
  selectedStrategicPositionId: null,
  loadStrategicPositions: async () => {
    const strategicPositions = await listStrategicPositions();
    set({ strategicPositions });
  },
  addStrategicPosition: async (input) => {
    await createStrategicPosition(input);
    await get().loadStrategicPositions();
  },
  editStrategicPosition: async (id, input) => {
    await updateStrategicPosition(id, input);
    await get().loadStrategicPositions();
  },
  removeStrategicPosition: async (id) => {
    await deleteStrategicPosition(id);
    await get().loadStrategicPositions();
  },
  selectStrategicPosition: (id) => set({ selectedStrategicPositionId: id }),

  operationsAudits: [],
  selectedOperationsAuditId: null,
  loadOperationsAudits: async () => {
    const operationsAudits = await listOperationsAudits();
    set({ operationsAudits });
  },
  addOperationsAudit: async (input) => {
    await createOperationsAudit(input);
    await get().loadOperationsAudits();
  },
  editOperationsAudit: async (id, input) => {
    await updateOperationsAudit(id, input);
    await get().loadOperationsAudits();
  },
  removeOperationsAudit: async (id) => {
    await deleteOperationsAudit(id);
    await get().loadOperationsAudits();
  },
  selectOperationsAudit: (id) => set({ selectedOperationsAuditId: id }),

  dollarAudits: [],
  selectedDollarAuditId: null,
  loadDollarAudits: async () => {
    const dollarAudits = await listDollarAudits();
    set({ dollarAudits });
  },
  addDollarAudit: async (input) => {
    await createDollarAudit(input);
    await get().loadDollarAudits();
  },
  editDollarAudit: async (id, input) => {
    await updateDollarAudit(id, input);
    await get().loadDollarAudits();
  },
  removeDollarAudit: async (id) => {
    await deleteDollarAudit(id);
    await get().loadDollarAudits();
  },
  selectDollarAudit: (id) => set({ selectedDollarAuditId: id }),

  financialSnapshots: [],
  selectedFinancialSnapshotId: null,
  loadFinancialSnapshots: async () => {
    const financialSnapshots = await listFinancialSnapshots();
    set({ financialSnapshots });
  },
  addFinancialSnapshot: async (input) => {
    await createFinancialSnapshot(input);
    await get().loadFinancialSnapshots();
  },
  editFinancialSnapshot: async (id, input) => {
    await updateFinancialSnapshot(id, input);
    await get().loadFinancialSnapshots();
  },
  removeFinancialSnapshot: async (id) => {
    await deleteFinancialSnapshot(id);
    await get().loadFinancialSnapshots();
  },
  selectFinancialSnapshot: (id) => set({ selectedFinancialSnapshotId: id }),

  marketingAssets: [],
  selectedMarketingAssetId: null,
  loadMarketingAssets: async () => {
    const marketingAssets = await listMarketingAssets();
    set({ marketingAssets });
  },
  addMarketingAsset: async (input) => {
    await createMarketingAsset(input);
    await get().loadMarketingAssets();
  },
  editMarketingAsset: async (id, input) => {
    await updateMarketingAsset(id, input);
    await get().loadMarketingAssets();
  },
  removeMarketingAsset: async (id) => {
    await deleteMarketingAsset(id);
    await get().loadMarketingAssets();
  },
  selectMarketingAsset: (id) => set({ selectedMarketingAssetId: id }),

  salesContacts: [],
  selectedSalesContactId: null,
  loadSalesContacts: async () => {
    const salesContacts = await listSalesContacts();
    set({ salesContacts });
  },
  addSalesContact: async (input) => {
    await createSalesContact(input);
    await get().loadSalesContacts();
  },
  editSalesContact: async (id, input) => {
    await updateSalesContact(id, input);
    await get().loadSalesContacts();
  },
  removeSalesContact: async (id) => {
    await deleteSalesContact(id);
    await get().loadSalesContacts();
  },
  selectSalesContact: (id) => set({ selectedSalesContactId: id }),

  contentPipelineItems: [],
  selectedContentPipelineItemId: null,
  loadContentPipelineItems: async () => {
    const contentPipelineItems = await listContentPipelineItems();
    set({ contentPipelineItems });
  },
  addContentPipelineItem: async (input) => {
    await createContentPipelineItem(input);
    await get().loadContentPipelineItems();
  },
  editContentPipelineItem: async (id, input) => {
    await updateContentPipelineItem(id, input);
    await get().loadContentPipelineItems();
  },
  removeContentPipelineItem: async (id) => {
    await deleteContentPipelineItem(id);
    await get().loadContentPipelineItems();
  },
  selectContentPipelineItem: (id) => set({ selectedContentPipelineItemId: id }),

  aIStackDecisions: [],
  selectedAIStackDecisionId: null,
  loadAIStackDecisions: async () => {
    const aIStackDecisions = await listAIStackDecisions();
    set({ aIStackDecisions });
  },
  addAIStackDecision: async (input) => {
    await createAIStackDecision(input);
    await get().loadAIStackDecisions();
  },
  editAIStackDecision: async (id, input) => {
    await updateAIStackDecision(id, input);
    await get().loadAIStackDecisions();
  },
  removeAIStackDecision: async (id) => {
    await deleteAIStackDecision(id);
    await get().loadAIStackDecisions();
  },
  selectAIStackDecision: (id) => set({ selectedAIStackDecisionId: id }),

  boardMembers: [],
  selectedBoardMemberId: null,
  loadBoardMembers: async () => {
    const boardMembers = await listBoardMembers();
    set({ boardMembers });
  },
  addBoardMember: async (input) => {
    await createBoardMember(input);
    await get().loadBoardMembers();
  },
  editBoardMember: async (id, input) => {
    await updateBoardMember(id, input);
    await get().loadBoardMembers();
  },
  removeBoardMember: async (id) => {
    await deleteBoardMember(id);
    await get().loadBoardMembers();
  },
  selectBoardMember: (id) => set({ selectedBoardMemberId: id }),

  portfolioSynergys: [],
  selectedPortfolioSynergyId: null,
  loadPortfolioSynergys: async () => {
    const portfolioSynergys = await listPortfolioSynergys();
    set({ portfolioSynergys });
  },
  addPortfolioSynergy: async (input) => {
    await createPortfolioSynergy(input);
    await get().loadPortfolioSynergys();
  },
  editPortfolioSynergy: async (id, input) => {
    await updatePortfolioSynergy(id, input);
    await get().loadPortfolioSynergys();
  },
  removePortfolioSynergy: async (id) => {
    await deletePortfolioSynergy(id);
    await get().loadPortfolioSynergys();
  },
  selectPortfolioSynergy: (id) => set({ selectedPortfolioSynergyId: id }),

  weeklyReviews: [],
  selectedWeeklyReviewId: null,
  loadWeeklyReviews: async () => {
    const weeklyReviews = await listWeeklyReviews();
    set({ weeklyReviews });
  },
  addWeeklyReview: async (input) => {
    await createWeeklyReview(input);
    await get().loadWeeklyReviews();
  },
  editWeeklyReview: async (id, input) => {
    await updateWeeklyReview(id, input);
    await get().loadWeeklyReviews();
  },
  removeWeeklyReview: async (id) => {
    await deleteWeeklyReview(id);
    await get().loadWeeklyReviews();
  },
  selectWeeklyReview: (id) => set({ selectedWeeklyReviewId: id }),

}));
