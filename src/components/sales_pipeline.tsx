import React from 'react';
import { useAppStore } from '../store';

export function sales_pipeline() {
  const items = useAppStore(s => s.salesContacts ?? []);

  const columns = Array.from(new Set(items.map((i: any) => i.stage ?? 'unknown')));

  return (
    <div className="flex gap-4 overflow-x-auto p-4 h-full" style={{ minHeight: 0 }}>
      {columns.map(col => (
        <div key={col} className="flex-shrink-0 w-72">
          <div
            className="mb-3 px-3 py-1.5 rounded-full text-xs font-semibold uppercase tracking-wide w-fit"
            style={{ background: 'var(--color-primary)', color: '#fff' }}
          >
            {col}
          </div>
          <div className="flex flex-col gap-2">
            {items
              .filter((i: any) => i.stage === col)
              .map((item: any) => (
                <div
                  key={item.id}
                  className="rounded-lg p-3 text-sm cursor-pointer hover:opacity-80 transition-opacity"
                  style={{ background: 'var(--color-surface, #1e293b)', color: 'var(--color-text, #f1f5f9)' }}
                >
                  {item.name ?? item.idea ?? item.title ?? item.subject ?? JSON.stringify(item)}
                </div>
              ))}
          </div>
        </div>
      ))}
    </div>
  );
}
