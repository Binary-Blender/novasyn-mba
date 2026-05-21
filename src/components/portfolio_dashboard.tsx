import { useEffect } from 'react';
import { useAppStore } from '../store/appStore';

// Agicore Generated — Dashboard layout
export function portfolio_dashboard() {
  const businesss = useAppStore((s) => s.businesss);
  const loadBusinesss = useAppStore((s) => s.loadBusinesss);

  useEffect(() => {
    loadBusinesss();
  }, []);

  return (
    <div className="p-8">
      <h2 className="text-2xl font-semibold mb-6" style={{ fontFamily: 'var(--font-family)' }}>portfolio_dashboard</h2>
      <div className="grid grid-cols-1 gap-4 mb-8">
        <div className="bg-[var(--bg-panel)] border border-[var(--border)] p-6" style={{ borderRadius: 'var(--radius)' }}>
          <div className="text-sm text-gray-400 mb-1">Business</div>
          <div className="text-3xl font-bold" style={{ color: 'var(--color-primary)' }}>{businesss.length}</div>
        </div>
      </div>
      <section>
        <h3 className="text-lg font-semibold mb-3" style={{ fontFamily: 'var(--font-family)' }}>Recent Businesss</h3>
        <div className="bg-[var(--bg-panel)] border border-[var(--border)]" style={{ borderRadius: 'var(--radius)' }}>
          {businesss.slice(0, 10).map((item) => (
            <div key={item.id} className="px-4 py-3 border-b border-[var(--border)] last:border-b-0 flex items-center justify-between">
              <span className="text-sm">{String(item.name ?? '—')}</span>
              <span className="text-xs text-gray-500">#{String(item.id).slice(0, 8)}</span>
            </div>
          ))}
          {businesss.length === 0 && (
            <div className="px-4 py-6 text-sm text-gray-500 text-center">No data yet.</div>
          )}
        </div>
      </section>
    </div>
  );
}
