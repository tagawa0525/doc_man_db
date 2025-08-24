/**
 * データ移行・互換性管理システム
 * TASK-056: 未実装機能の対応方針設計
 * 
 * モックデータから実際のAPIへの段階的移行をサポート
 */

/**
 * データソースの種類
 */
export type DataSource = 'mock' | 'api' | 'hybrid';

/**
 * 移行設定
 */
interface MigrationConfig {
  feature: string;
  currentSource: DataSource;
  targetSource: DataSource;
  migrationStarted: boolean;
  rollbackAvailable: boolean;
  compatibility: 'full' | 'partial' | 'none';
  lastUpdated: string;
}

/**
 * データアダプターインターフェース
 */
export interface DataAdapter<T> {
  source: DataSource;
  isAvailable(): Promise<boolean>;
  fetch(params?: any): Promise<T>;
  validate(data: T): boolean;
  transform?(data: any): T;
}

/**
 * 段階的データ移行マネージャー
 */
export class DataMigrationManager {
  private migrations = new Map<string, MigrationConfig>();
  private adapters = new Map<string, DataAdapter<any>[]>();

  constructor() {
    this.loadMigrationState();
  }

  /**
   * データアダプターを登録
   */
  registerAdapter<T>(feature: string, adapter: DataAdapter<T>): void {
    if (!this.adapters.has(feature)) {
      this.adapters.set(feature, []);
    }
    this.adapters.get(feature)!.push(adapter);
  }

  /**
   * 移行設定を更新
   */
  updateMigrationConfig(feature: string, config: Partial<MigrationConfig>): void {
    const current = this.migrations.get(feature) || {
      feature,
      currentSource: 'mock',
      targetSource: 'api',
      migrationStarted: false,
      rollbackAvailable: false,
      compatibility: 'none',
      lastUpdated: new Date().toISOString()
    } as MigrationConfig;

    this.migrations.set(feature, {
      ...current,
      ...config,
      lastUpdated: new Date().toISOString()
    });

    this.saveMigrationState();
  }

  /**
   * データを取得（フォールバック付き）
   */
  async fetchData<T>(feature: string, params?: any): Promise<T> {
    const adapters = this.adapters.get(feature) || [];
    const config = this.migrations.get(feature);

    // 優先順位に基づいてアダプターをソート
    const sortedAdapters = this.sortAdaptersByPriority(adapters, config);

    let lastError: Error | null = null;

    for (const adapter of sortedAdapters) {
      try {
        // アダプターが利用可能かチェック
        if (!(await adapter.isAvailable())) {
          console.warn(`[Migration] Adapter ${adapter.source} not available for ${feature}`);
          continue;
        }

        // データを取得
        const data = await adapter.fetch(params);

        // データを検証
        if (!adapter.validate(data)) {
          console.warn(`[Migration] Data validation failed for ${feature} from ${adapter.source}`);
          continue;
        }

        // 成功ログ
        console.log(`[Migration] Successfully fetched ${feature} data from ${adapter.source}`);

        // 使用状況を記録
        this.recordUsage(feature, adapter.source, 'success');

        return data;
      } catch (error) {
        lastError = error as Error;
        console.error(`[Migration] Failed to fetch ${feature} from ${adapter.source}:`, error);
        this.recordUsage(feature, adapter.source, 'error');
      }
    }

    // すべてのアダプターが失敗した場合
    throw new Error(`All data sources failed for feature: ${feature}. Last error: ${lastError?.message}`);
  }

  /**
   * 段階的移行を開始
   */
  async startMigration(feature: string, targetPercentage = 10): Promise<void> {
    const config = this.migrations.get(feature);
    if (!config) {
      throw new Error(`Migration config not found for feature: ${feature}`);
    }

    // A/Bテスト的な段階的移行
    const userId = this.getCurrentUserId();
    const shouldMigrate = this.shouldMigrateUser(userId, targetPercentage);

    this.updateMigrationConfig(feature, {
      migrationStarted: true,
      currentSource: shouldMigrate ? config.targetSource : config.currentSource
    });

    console.log(`[Migration] Started migration for ${feature}, target: ${targetPercentage}%, user migrated: ${shouldMigrate}`);
  }

  /**
   * 移行をロールバック
   */
  rollbackMigration(feature: string): void {
    const config = this.migrations.get(feature);
    if (!config || !config.rollbackAvailable) {
      throw new Error(`Rollback not available for feature: ${feature}`);
    }

    this.updateMigrationConfig(feature, {
      currentSource: 'mock', // 安全なデフォルトに戻す
      migrationStarted: false
    });

    console.log(`[Migration] Rolled back ${feature} to mock data`);
  }

  /**
   * 移行状況レポート
   */
  getMigrationReport(): Record<string, any> {
    const report: Record<string, any> = {};

    for (const [feature, config] of this.migrations) {
      const usage = this.getUsageStats(feature);
      report[feature] = {
        config,
        usage,
        recommendation: this.getRecommendation(config, usage)
      };
    }

    return report;
  }

  /**
   * プライベートメソッド
   */
  private sortAdaptersByPriority(adapters: DataAdapter<any>[], config?: MigrationConfig): DataAdapter<any>[] {
    const priority: Record<DataSource, number> = { api: 3, hybrid: 2, mock: 1 };

    return [...adapters].sort((a, b) => {
      // 移行設定に基づく優先度調整
      if (config?.migrationStarted) {
        if (a.source === config.targetSource) return -1;
        if (b.source === config.targetSource) return 1;
      }

      return priority[b.source] - priority[a.source];
    });
  }

  private shouldMigrateUser(userId: string, percentage: number): boolean {
    if (!userId) return false;

    const hash = userId.split('').reduce((a, b) => {
      a = ((a << 5) - a) + b.charCodeAt(0);
      return a & a;
    }, 0);

    return Math.abs(hash) % 100 < percentage;
  }

  private getCurrentUserId(): string {
    // 実際の実装では認証システムから取得
    return localStorage.getItem('userId') || 'anonymous';
  }

  private recordUsage(feature: string, source: DataSource, result: 'success' | 'error'): void {
    const key = `migration_usage_${feature}`;
    const existing = JSON.parse(localStorage.getItem(key) || '{}');

    if (!existing[source]) {
      existing[source] = { success: 0, error: 0 };
    }

    existing[source][result]++;
    existing.lastUpdated = new Date().toISOString();

    localStorage.setItem(key, JSON.stringify(existing));
  }

  private getUsageStats(feature: string): Record<string, any> {
    const key = `migration_usage_${feature}`;
    return JSON.parse(localStorage.getItem(key) || '{}');
  }

  private getRecommendation(config: MigrationConfig, usage: Record<string, any>): string {
    if (!config.migrationStarted) {
      return 'Migration not started. Consider starting with 10% of users.';
    }

    const apiSuccess = usage.api?.success || 0;
    const apiError = usage.api?.error || 0;
    const successRate = apiSuccess / (apiSuccess + apiError || 1);

    if (successRate > 0.95) {
      return 'API performing well. Consider increasing migration percentage.';
    } else if (successRate < 0.8) {
      return 'API issues detected. Consider rollback or investigation.';
    } else {
      return 'API performance acceptable. Monitor and proceed gradually.';
    }
  }

  private loadMigrationState(): void {
    try {
      const saved = localStorage.getItem('migration_state');
      if (saved) {
        const data = JSON.parse(saved);
        this.migrations = new Map(Object.entries(data));
      }
    } catch (error) {
      console.error('Failed to load migration state:', error);
    }
  }

  private saveMigrationState(): void {
    try {
      const data = Object.fromEntries(this.migrations);
      localStorage.setItem('migration_state', JSON.stringify(data));
    } catch (error) {
      console.error('Failed to save migration state:', error);
    }
  }
}

// グローバル移行マネージャー
export const migrationManager = new DataMigrationManager();

/**
 * 便利な移行ヘルパー関数
 */
export function withFallback<T>(
  primaryFn: () => Promise<T>,
  fallbackFn: () => Promise<T>,
  feature: string
): Promise<T> {
  return primaryFn().catch(async (error) => {
    console.warn(`[Migration] Primary source failed for ${feature}, falling back:`, error);
    return fallbackFn();
  });
}

/**
 * データ互換性チェッカー
 */
export class CompatibilityChecker {
  static checkDataStructure(expected: any, actual: any): boolean {
    if (typeof expected !== typeof actual) return false;

    if (Array.isArray(expected)) {
      return Array.isArray(actual);
    }

    if (typeof expected === 'object' && expected !== null) {
      for (const key in expected) {
        if (!(key in actual)) return false;
        if (!this.checkDataStructure(expected[key], actual[key])) {
          return false;
        }
      }
    }

    return true;
  }

  static validateApiResponse<T>(data: T, schema: any): { valid: boolean; errors: string[] } {
    const errors: string[] = [];

    try {
      if (!this.checkDataStructure(schema, data)) {
        errors.push('Data structure mismatch');
      }
    } catch (error) {
      errors.push(`Validation error: ${error}`);
    }

    return { valid: errors.length === 0, errors };
  }
}

/**
 * 開発者用デバッグ情報
 */
export function getMigrationDebugInfo() {
  if (!import.meta.env.DEV) return null;

  return {
    migrations: Object.fromEntries(migrationManager['migrations']),
    adapters: Object.fromEntries(
      Array.from(migrationManager['adapters']).map(([key, adapters]) => [
        key,
        adapters.map(a => a.source)
      ])
    )
  };
}
