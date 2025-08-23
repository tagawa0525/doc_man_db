/**
 * フィーチャーフラグによる機能制御システム
 * TASK-056: 未実装機能の対応方針設計
 * 
 * 段階的な機能ロールアウトと A/B テストを支援
 */

import { writable, derived, type Readable } from 'svelte/stores';

export interface FeatureFlag {
  key: string;
  enabled: boolean;
  description: string;
  environment: 'development' | 'staging' | 'production' | 'all';
  userPercentage: number; // 0-100
  dependsOn?: string[];
  metadata?: Record<string, any>;
}

/**
 * 機能フラグの初期定義
 */
const INITIAL_FLAGS: Record<string, FeatureFlag> = {
  // コア機能フラグ
  enable_circulation: {
    key: 'enable_circulation',
    enabled: false,
    description: '回覧・承認フロー機能',
    environment: 'development',
    userPercentage: 0,
    dependsOn: ['enable_organization'],
    metadata: { phase: 'Phase 10', priority: 'high' }
  },

  enable_organization: {
    key: 'enable_organization',
    enabled: false,
    description: '組織管理・Active Directory連携',
    environment: 'development',
    userPercentage: 0,
    metadata: { phase: 'Phase 10', priority: 'high' }
  },

  enable_notifications: {
    key: 'enable_notifications',
    enabled: false,
    description: 'リアルタイム通知・Email/Teams連携',
    environment: 'development',
    userPercentage: 0,
    metadata: { phase: 'Phase 10', priority: 'high' }
  },

  enable_reports: {
    key: 'enable_reports',
    enabled: false,
    description: 'レポート・分析機能',
    environment: 'staging',
    userPercentage: 10,
    metadata: { phase: 'Phase 11', priority: 'medium' }
  },

  enable_file_upload: {
    key: 'enable_file_upload',
    enabled: false,
    description: 'ファイルアップロード・管理',
    environment: 'development',
    userPercentage: 0,
    metadata: { phase: 'Phase 10', priority: 'high' }
  },

  // UI/UX改善フラグ
  enable_advanced_search: {
    key: 'enable_advanced_search',
    enabled: true,
    description: '高度検索機能',
    environment: 'all',
    userPercentage: 100,
    metadata: { phase: 'Phase 9', priority: 'medium' }
  },

  enable_mobile_optimization: {
    key: 'enable_mobile_optimization',
    enabled: true,
    description: 'モバイル最適化',
    environment: 'all',
    userPercentage: 100,
    metadata: { phase: 'Phase 9', priority: 'high' }
  },

  // 実験的機能
  enable_ai_categorization: {
    key: 'enable_ai_categorization',
    enabled: false,
    description: 'AI文書分類',
    environment: 'development',
    userPercentage: 5,
    metadata: { phase: 'Phase 12', priority: 'low' }
  },

  enable_real_time_collaboration: {
    key: 'enable_real_time_collaboration',
    enabled: false,
    description: 'リアルタイム共同編集',
    environment: 'development',
    userPercentage: 1,
    metadata: { phase: 'Future', priority: 'low' }
  },

  // 開発・デバッグ支援
  show_placeholder_details: {
    key: 'show_placeholder_details',
    enabled: import.meta.env.DEV,
    description: 'プレースホルダー詳細表示',
    environment: 'development',
    userPercentage: 100,
    metadata: { debug: true }
  },

  enable_performance_monitoring: {
    key: 'enable_performance_monitoring',
    enabled: import.meta.env.DEV,
    description: 'パフォーマンス監視',
    environment: 'all',
    userPercentage: 100,
    metadata: { debug: true }
  }
};

// フィーチャーフラグストア
export const featureFlags = writable<Record<string, FeatureFlag>>(INITIAL_FLAGS);

/**
 * 機能が有効かどうかを判定
 */
export function isFeatureEnabled(flagKey: string): Readable<boolean> {
  return derived(featureFlags, ($flags) => {
    const flag = $flags[flagKey];
    if (!flag) return false;

    // 環境チェック
    if (flag.environment !== 'all') {
      const currentEnv = import.meta.env.DEV ? 'development' : 'production';
      if (flag.environment !== currentEnv) return false;
    }

    // 依存関係チェック
    if (flag.dependsOn) {
      const allDepsEnabled = flag.dependsOn.every(dep =>
        $flags[dep]?.enabled || false
      );
      if (!allDepsEnabled) return false;
    }

    return flag.enabled;
  });
}

/**
 * フィーチャーフラグを動的に更新
 */
export function updateFeatureFlag(key: string, updates: Partial<FeatureFlag>): void {
  featureFlags.update(flags => ({
    ...flags,
    [key]: { ...flags[key], ...updates }
  }));
}

/**
 * 管理者用フィーチャーフラグ制御
 */
export class FeatureFlagManager {
  static enableFeature(key: string, userPercentage = 100): void {
    updateFeatureFlag(key, { enabled: true, userPercentage });
  }

  static disableFeature(key: string): void {
    updateFeatureFlag(key, { enabled: false, userPercentage: 0 });
  }

  static rolloutFeature(key: string, percentage: number): void {
    updateFeatureFlag(key, { userPercentage: Math.min(100, Math.max(0, percentage)) });
  }

  static getFeatureStatus(): Record<string, FeatureFlag> {
    let currentFlags: Record<string, FeatureFlag> = {};
    featureFlags.subscribe(flags => currentFlags = flags)();
    return currentFlags;
  }
}

/**
 * A/Bテスト支援
 */
export function shouldShowVariant(flagKey: string, userId?: string): boolean {
  let currentFlags: Record<string, FeatureFlag> = {};
  featureFlags.subscribe(flags => currentFlags = flags)();

  const flag = currentFlags[flagKey];
  if (!flag || !flag.enabled) return false;

  // ユーザーベースの一貫性のあるA/Bテスト
  if (userId) {
    const hash = userId.split('').reduce((a, b) => {
      a = ((a << 5) - a) + b.charCodeAt(0);
      return a & a;
    }, 0);
    return Math.abs(hash) % 100 < flag.userPercentage;
  }

  // ランダム配信
  return Math.random() * 100 < flag.userPercentage;
}

/**
 * 開発者用フィーチャーフラグ情報表示
 */
export const debugInfo = derived(featureFlags, ($flags) => {
  if (!import.meta.env.DEV) return null;

  return Object.entries($flags).map(([key, flag]) => ({
    key,
    enabled: flag.enabled,
    phase: flag.metadata?.phase || 'Unknown',
    priority: flag.metadata?.priority || 'Unknown'
  }));
});
