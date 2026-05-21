import { useEffect } from 'react';
import { useAppStore } from '../store/appStore';

export function business_cards() {
  const businesss = useAppStore((s) => s.businesss);
  const load = useAppStore((s) => s.loadBusinesss);

  useEffect(() => { load(); }, []);

  return (
    <div className="p-6">
      <h2 className="text-xl font-semibold mb-4">business_cards</h2>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
        {businesss.map((item) => (
          <div key={item.id} className="bg-[var(--bg-panel)] border border-[var(--border)] rounded-lg p-4">
            <h3 className="font-medium mb-2">{String(item.name)}</h3>
            <div className="text-sm text-[var(--text-secondary)]">tagline: {String(item.tagline ?? '')}</div>
            <div className="text-sm text-[var(--text-secondary)]">description: {String(item.description ?? '')}</div>
            <div className="text-sm text-[var(--text-secondary)]">stage: {String(item.stage ?? '')}</div>
            <div className="text-sm text-[var(--text-secondary)]">category: {String(item.category ?? '')}</div>
            <div className="text-sm text-[var(--text-secondary)]">marketNiche: {String(item.marketNiche ?? '')}</div>
            <div className="text-sm text-[var(--text-secondary)]">businessModel: {String(item.businessModel ?? '')}</div>
            <div className="text-sm text-[var(--text-secondary)]">revenueTarget: {String(item.revenueTarget ?? '')}</div>
            <div className="text-sm text-[var(--text-secondary)]">launchDate: {String(item.launchDate ?? '')}</div>
            <div className="text-sm text-[var(--text-secondary)]">exitDate: {String(item.exitDate ?? '')}</div>
            <div className="text-sm text-[var(--text-secondary)]">notes: {String(item.notes ?? '')}</div>
          </div>
        ))}
      </div>
    </div>
  );
}
