import { useEffect, useState } from 'react';
import { useAppStore } from '../store/appStore';

export function dollar_audit_tool() {
  const dollarAudits = useAppStore((s) => s.dollarAudits);
  const load = useAppStore((s) => s.loadDollarAudits);
  const [search, setSearch] = useState('');

  useEffect(() => { load(); }, []);

  const filtered = search
    ? dollarAudits.filter((i) => String(i.businessId ?? '').toLowerCase().includes(search.toLowerCase()))
    : dollarAudits;

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center gap-3 p-4 border-b border-[var(--border)] flex-shrink-0">
        <h2 className="text-lg font-semibold flex-shrink-0">dollar_audit_tool</h2>
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
            <th className="text-left py-2 px-3 text-xs font-medium text-[var(--text-secondary)]">activityName</th>
            <th className="text-left py-2 px-3 text-xs font-medium text-[var(--text-secondary)]">tier</th>
            <th className="text-left py-2 px-3 text-xs font-medium text-[var(--text-secondary)]">hoursPerWeek</th>
            <th className="text-left py-2 px-3 text-xs font-medium text-[var(--text-secondary)]">aiTransferable</th>
            <th className="text-left py-2 px-3 text-xs font-medium text-[var(--text-secondary)]">transferStatus</th>
            <th className="text-left py-2 px-3 text-xs font-medium text-[var(--text-secondary)]">notes</th>
            </tr>
          </thead>
          <tbody>
            {filtered.length === 0 && (
              <tr>
                <td colSpan={7} className="text-center py-8 text-[var(--text-secondary)] text-sm">
                  {search ? 'No results.' : 'No items yet.'}
                </td>
              </tr>
            )}
            {filtered.map((item) => (
              <tr key={item.id} className="group border-b border-[var(--border)] hover:bg-[var(--bg-hover)] transition">
              <td className="py-2.5 px-3 text-sm"><span>{item.businessId != null ? Number(item.businessId).toLocaleString() : '—'}</span></td>
              <td className="py-2.5 px-3 text-sm"><span className="break-words">{String(item.activityName ?? '')}</span></td>
              <td className="py-2.5 px-3 text-sm"><span className="break-words">{String(item.tier ?? '')}</span></td>
              <td className="py-2.5 px-3 text-sm"><span>{item.hoursPerWeek != null ? Number(item.hoursPerWeek).toLocaleString() : '—'}</span></td>
              <td className="py-2.5 px-3 text-sm"><span>{item.aiTransferable != null ? Number(item.aiTransferable).toLocaleString() : '—'}</span></td>
              <td className="py-2.5 px-3 text-sm"><span className="break-words">{String(item.transferStatus ?? '')}</span></td>
              <td className="py-2.5 px-3 text-sm"><span className="break-words">{String(item.notes ?? '')}</span></td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      

      
    </div>
  );
}
