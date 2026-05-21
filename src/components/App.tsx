import { useAppStore } from '../store/appStore';
import { Sidebar } from './Sidebar';
import { TitleBar } from './TitleBar';
import { portfolio_dashboard } from './portfolio_dashboard';
import { business_cards } from './business_cards';
import { business_detail } from './business_detail';
import { dollar_audit_tool } from './dollar_audit_tool';
import { unit_economics_dashboard } from './unit_economics_dashboard';
import { content_pipeline } from './content_pipeline';
import { sales_pipeline } from './sales_pipeline';
import { weekly_review_form } from './weekly_review_form';
import { synergy_board } from './synergy_board';
import { board_members } from './board_members';

export function App() {
  const currentView = useAppStore((s) => s.currentView);

  const renderView = () => {
    switch (currentView) {
      case 'portfolio_dashboard': return <portfolio_dashboard />;
      case 'business_cards': return <business_cards />;
      case 'business_detail': return <business_detail />;
      case 'dollar_audit_tool': return <dollar_audit_tool />;
      case 'unit_economics_dashboard': return <unit_economics_dashboard />;
      case 'content_pipeline': return <content_pipeline />;
      case 'sales_pipeline': return <sales_pipeline />;
      case 'weekly_review_form': return <weekly_review_form />;
      case 'synergy_board': return <synergy_board />;
      case 'board_members': return <board_members />;
      default: return <div className="p-6">Unknown view</div>;
    }
  };

  return (
    <div className="h-screen flex flex-col bg-[var(--bg-page)] text-[var(--text-primary)]">
      <TitleBar />
      <div className="flex flex-1 overflow-hidden">
        <Sidebar />
        <main className="flex-1 overflow-hidden flex flex-col">
          {renderView()}
        </main>
      </div>
    </div>
  );
}
