import { writable } from 'svelte/store';

export interface AppError {
  id: string;
  message: string;
  type: 'error' | 'warning' | 'info';
  timestamp: Date;
  details?: any;
}

// エラー一覧
export const errors = writable<AppError[]>([]);

// エラー追加関数
export function addError(message: string, type: AppError['type'] = 'error', details?: any): void {
  const error: AppError = {
    id: `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`,
    message,
    type,
    timestamp: new Date(),
    details
  };

  errors.update(currentErrors => [...currentErrors, error]);

  // 自動削除（10秒後）
  setTimeout(() => {
    removeError(error.id);
  }, 10000);
}

// エラー削除関数
export function removeError(id: string): void {
  errors.update(currentErrors => currentErrors.filter(error => error.id !== id));
}

// 全エラークリア関数
export function clearErrors(): void {
  errors.set([]);
}

// 便利関数
export const showError = (message: string, details?: any) => addError(message, 'error', details);
export const showWarning = (message: string, details?: any) => addError(message, 'warning', details);
export const showInfo = (message: string, details?: any) => addError(message, 'info', details);
