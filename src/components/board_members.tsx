import { useEffect, useState } from 'react';
import { useAppStore } from '../store/appStore';

export function board_members() {
  const boardMembers = useAppStore((s) => s.boardMembers);
  const load = useAppStore((s) => s.loadBoardMembers);
  const [search, setSearch] = useState('');

  useEffect(() => { load(); }, []);

  const filtered = search
    ? boardMembers.filter((i) => String(i.name ?? '').toLowerCase().includes(search.toLowerCase()))
    : boardMembers;

  return (
    <div className="flex flex-col h-full">
      <div className="flex items-center gap-3 p-4 border-b border-[var(--border)] flex-shrink-0">
        <h2 className="text-lg font-semibold flex-shrink-0">board_members</h2>
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
            <th className="text-left py-2 px-3 text-xs font-medium text-[var(--text-secondary)]">name</th>
            <th className="text-left py-2 px-3 text-xs font-medium text-[var(--text-secondary)]">expertiseDomain</th>
            <th className="text-left py-2 px-3 text-xs font-medium text-[var(--text-secondary)]">relationshipType</th>
            <th className="text-left py-2 px-3 text-xs font-medium text-[var(--text-secondary)]">lastConsulted</th>
            <th className="text-left py-2 px-3 text-xs font-medium text-[var(--text-secondary)]">notes</th>
            </tr>
          </thead>
          <tbody>
            {filtered.length === 0 && (
              <tr>
                <td colSpan={5} className="text-center py-8 text-[var(--text-secondary)] text-sm">
                  {search ? 'No results.' : 'No items yet.'}
                </td>
              </tr>
            )}
            {filtered.map((item) => (
              <tr key={item.id} className="group border-b border-[var(--border)] hover:bg-[var(--bg-hover)] transition">
              <td className="py-2.5 px-3 text-sm"><span className="break-words">{String(item.name ?? '')}</span></td>
              <td className="py-2.5 px-3 text-sm"><span className="break-words">{String(item.expertiseDomain ?? '')}</span></td>
              <td className="py-2.5 px-3 text-sm"><span className="break-words">{String(item.relationshipType ?? '')}</span></td>
              <td className="py-2.5 px-3 text-sm"><span>{item.lastConsulted ? new Date(String(item.lastConsulted)).toLocaleDateString() : '—'}</span></td>
              <td className="py-2.5 px-3 text-sm"><span className="break-words">{String(item.notes ?? '')}</span></td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      

      
    </div>
  );
}
