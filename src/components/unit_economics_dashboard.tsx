import { useEffect, useState } from 'react';
import { useAppStore } from '../store/appStore';

export function unit_economics_dashboard() {
  const financialSnapshots = useAppStore((s) => s.financialSnapshots);
  const load = useAppStore((s) => s.loadFinancialSnapshots);
  const [search, setSearch] = useState('');

  useEffect(() => { load(); }, []);

  const filtered = search
    ? financialSnapshots.filter((i) => String(i.businessId ?? '').toLowerCase().includes(search.toLowerCase()))
    : financialSnapshots;

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center gap-3 p-4 border-b border-[var(--border)] flex-shrink-0">
        <h2 className="text-lg font-semibold flex-shrink-0">unit_economics_dashboard</h2>
        <input
          type="text"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          placeholder="Search..."
          className="flex-1 bg-[var(--bg-hover)] border border-[var(--border)] rounded px-3 py-1.5 text-sm text-[var(--text-primary)] focus:outline-none focus:border-blue-500"
        />
        
      </div>
      <div className="flex-1 overflow-y-auto">
        <table className="w-full text-sm">
          <thead className="sticky top-0 bg-[var(--bg-page)] z-10">
            <tr className="border-b border-[var(--border)]">
            <th className="text-left py-2 px-3 text-xs font-medium text-[var(--text-secondary)]">businessId</th>
            <th className="text-left py-2 px-3 text-xs font-medium text-[var(--text-secondary)]">period</th>
            <th className="text-left py-2 px-3 text-xs font-medium text-[var(--text-secondary)]">revenue</th>
            <th className="text-left py-2 px-3 text-xs font-medium text-[var(--text-secondary)]">cac</th>
            <th className="text-left py-2 px-3 text-xs font-medium text-[var(--text-secondary)]">ltv</th>
            <th className="text-left py-2 px-3 text-xs font-medium text-[var(--text-secondary)]">ltvCacRatio</th>
            <th className="text-left py-2 px-3 text-xs font-medium text-[var(--text-secondary)]">grossMarginPct</th>
            <th className="text-left py-2 px-3 text-xs font-medium text-[var(--text-secondary)]">mrr</th>
            <th className="text-left py-2 px-3 text-xs font-medium text-[var(--text-secondary)]">cashRunwayMonths</th>
            <th className="text-left py-2 px-3 text-xs font-medium text-[var(--text-secondary)]">recordedAt</th>
            </tr>
          </thead>
          <tbody>
            {filtered.length === 0 && (
              <tr>
                <td colSpan={10} className="text-center py-8 text-[var(--text-secondary)] text-sm">
                  {search ? 'No results.' : 'No items yet.'}
                </td>
              </tr>
            )}
            {filtered.map((item) => (
              <tr key={item.id} className="group border-b border-[var(--border)] hover:bg-[var(--bg-hover)] transition">
              <td className="py-2.5 px-3 text-sm"><span>{item.businessId != null ? Number(item.businessId).toLocaleString() : '—'}</span></td>
              <td className="py-2.5 px-3 text-sm"><span className="break-words">{String(item.period ?? '')}</span></td>
              <td className="py-2.5 px-3 text-sm"><span>{item.revenue != null ? Number(item.revenue).toLocaleString() : '—'}</span></td>
              <td className="py-2.5 px-3 text-sm"><span>{item.cac != null ? Number(item.cac).toLocaleString() : '—'}</span></td>
              <td className="py-2.5 px-3 text-sm"><span>{item.ltv != null ? Number(item.ltv).toLocaleString() : '—'}</span></td>
              <td className="py-2.5 px-3 text-sm"><span>{item.ltvCacRatio != null ? Number(item.ltvCacRatio).toLocaleString() : '—'}</span></td>
              <td className="py-2.5 px-3 text-sm"><span>{item.grossMarginPct != null ? Number(item.grossMarginPct).toLocaleString() : '—'}</span></td>
              <td className="py-2.5 px-3 text-sm"><span>{item.mrr != null ? Number(item.mrr).toLocaleString() : '—'}</span></td>
              <td className="py-2.5 px-3 text-sm"><span>{item.cashRunwayMonths != null ? Number(item.cashRunwayMonths).toLocaleString() : '—'}</span></td>
              <td className="py-2.5 px-3 text-sm"><span>{item.recordedAt ? new Date(String(item.recordedAt)).toLocaleString() : '—'}</span></td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      

      
    </div>
  );
}
