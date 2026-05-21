import {  } from 'lucide-react';
import { useAppStore } from '../store/appStore';

const NAV_ITEMS = [

];

export function Sidebar() {
  const currentView = useAppStore((s) => s.currentView);
  const setCurrentView = useAppStore((s) => s.setCurrentView);

  return (
    <nav className="w-14 bg-[var(--bg-sidebar)] border-r border-[var(--border)] flex flex-col items-center py-2 gap-1">
      {NAV_ITEMS.map(({ view, icon: Icon, title }) => (
        <button
          key={view}
          onClick={() => setCurrentView(view)}
          title={title}
          className={`w-10 h-10 flex items-center justify-center rounded-lg transition-colors ${
            currentView === view
              ? 'bg-[var(--bg-active)] text-[var(--text-primary)]'
              : 'text-[var(--text-secondary)] hover:bg-[var(--bg-hover)]'
          }`}
        >
          <Icon size={20} />
        </button>
      ))}
    </nav>
  );
}
