import { writable } from 'svelte/store';
import { executeQuery } from '../api/client.js';
import {
  DASHBOARD_STATS,
  SYSTEM_STATUS,
  RECENT_ACTIVITIES,
  PENDING_APPROVALS,
  type DashboardStats,
  type SystemStatus,
  type Activity,
  type PendingApproval
} from '../api/queries/dashboard.js';

// ダッシュボード統計の状態管理
export const dashboardStats = writable<DashboardStats | null>(null);
export const isLoadingStats = writable<boolean>(false);

// システム稼働状況の状態管理
export const systemStatus = writable<SystemStatus | null>(null);
export const isLoadingSystemStatus = writable<boolean>(false);

// アクティビティフィードの状態管理
export const recentActivities = writable<Activity[]>([]);
export const isLoadingActivities = writable<boolean>(false);

// 承認待ち文書の状態管理
export const pendingApprovals = writable<PendingApproval[]>([]);
export const isLoadingApprovals = writable<boolean>(false);

// ダッシュボード全体のエラー状態
export const dashboardError = writable<string | null>(null);

// ダッシュボード統計取得関数
export async function loadDashboardStats(): Promise<void> {
  isLoadingStats.set(true);
  dashboardError.set(null);

  try {
    const result = await executeQuery<{ dashboardStats: DashboardStats }>(
      DASHBOARD_STATS
    );

    dashboardStats.set(result.dashboardStats);
  } catch (error: any) {
    console.error('Failed to load dashboard stats:', error);
    dashboardError.set(error.message || 'Failed to load dashboard statistics');
  } finally {
    isLoadingStats.set(false);
  }
}

// システム稼働状況取得関数
export async function loadSystemStatus(): Promise<void> {
  isLoadingSystemStatus.set(true);
  dashboardError.set(null);

  try {
    const result = await executeQuery<{ systemStatus: SystemStatus }>(
      SYSTEM_STATUS
    );

    systemStatus.set(result.systemStatus);
  } catch (error: any) {
    console.error('Failed to load system status:', error);
    dashboardError.set(error.message || 'Failed to load system status');
  } finally {
    isLoadingSystemStatus.set(false);
  }
}

// アクティビティフィード取得関数
export async function loadRecentActivities(limit: number = 10): Promise<void> {
  isLoadingActivities.set(true);
  dashboardError.set(null);

  try {
    const result = await executeQuery<{ recentActivities: Activity[] }>(
      RECENT_ACTIVITIES,
      { limit }
    );

    recentActivities.set(result.recentActivities);
  } catch (error: any) {
    console.error('Failed to load recent activities:', error);
    dashboardError.set(error.message || 'Failed to load recent activities');
  } finally {
    isLoadingActivities.set(false);
  }
}

// 承認待ち文書取得関数
export async function loadPendingApprovals(limit: number = 5): Promise<void> {
  isLoadingApprovals.set(true);
  dashboardError.set(null);

  try {
    const result = await executeQuery<{ pendingApprovals: PendingApproval[] }>(
      PENDING_APPROVALS,
      { limit }
    );

    pendingApprovals.set(result.pendingApprovals);
  } catch (error: any) {
    console.error('Failed to load pending approvals:', error);
    dashboardError.set(error.message || 'Failed to load pending approvals');
  } finally {
    isLoadingApprovals.set(false);
  }
}

// ダッシュボード全体の初期化関数
export async function initializeDashboard(): Promise<void> {
  // 並行して全データを取得
  await Promise.allSettled([
    loadDashboardStats(),
    loadSystemStatus(),
    loadRecentActivities(),
    loadPendingApprovals()
  ]);
}

// データ更新関数（定期的な更新用）
export async function refreshDashboard(): Promise<void> {
  await initializeDashboard();
}

// リアルタイム統計データ用のヘルパー関数
export function formatStatValue(value: number): string {
  if (value >= 1000) {
    return (value / 1000).toFixed(1) + 'k';
  }
  return value.toString();
}

export function formatUptime(seconds: number): string {
  const days = Math.floor(seconds / 86400);
  const hours = Math.floor((seconds % 86400) / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);

  if (days > 0) {
    return `${days}d ${hours}h ${minutes}m`;
  } else if (hours > 0) {
    return `${hours}h ${minutes}m`;
  } else {
    return `${minutes}m`;
  }
}

export function getStatusColor(status: 'healthy' | 'warning' | 'error'): string {
  switch (status) {
    case 'healthy':
      return 'green';
    case 'warning':
      return 'yellow';
    case 'error':
      return 'red';
    default:
      return 'gray';
  }
}
