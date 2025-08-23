import { writable, derived } from 'svelte/store';

// 各機能のローディング状態
export const loadingStates = writable<Record<string, boolean>>({});

// 特定のローディング状態を設定
export function setLoading(key: string, isLoading: boolean): void {
  loadingStates.update(states => ({
    ...states,
    [key]: isLoading
  }));
}

// 特定のローディング状態を取得
export function getLoading(key: string): boolean {
  let result = false;
  const unsubscribe = loadingStates.subscribe(states => {
    result = states[key] || false;
  });
  unsubscribe();
  return result;
}

// 全体のローディング状態（何かがローディング中かどうか）
export const isAnyLoading = derived(
  loadingStates,
  $states => Object.values($states).some(loading => loading)
);

// 便利関数: ローディング状態を管理する高階関数
export function withLoading<T>(
  key: string,
  asyncFn: () => Promise<T>
): Promise<T> {
  setLoading(key, true);

  return asyncFn().finally(() => {
    setLoading(key, false);
  });
}

// よく使用されるローディングキー
export const LOADING_KEYS = {
  DOCUMENTS_SEARCH: 'documents:search',
  DOCUMENTS_CREATE: 'documents:create',
  DOCUMENTS_UPDATE: 'documents:update',
  DOCUMENTS_DELETE: 'documents:delete',
  DOCUMENT_LOAD: 'document:load',
  SYSTEM_STATUS: 'system:status',
  NOTIFICATIONS: 'notifications',
  CIRCULATION: 'circulation'
} as const;
