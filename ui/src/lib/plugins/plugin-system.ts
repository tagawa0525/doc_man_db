/**
 * プラグインシステム - 拡張可能なアーキテクチャ
 * TASK-056: 未実装機能の対応方針設計
 * 
 * 将来の機能追加を容易にするプラグインベースの設計
 */

import type { ComponentType } from 'svelte';

/**
 * プラグイン基底インターフェース
 */
export interface Plugin {
  id: string;
  name: string;
  version: string;
  description: string;
  author: string;
  dependencies?: string[];

  // ライフサイクルフック
  initialize?(): Promise<void>;
  destroy?(): Promise<void>;

  // 機能提供
  provides?: PluginCapability[];
}

/**
 * プラグインが提供する機能の種類
 */
export type PluginCapability =
  | 'navigation'      // ナビゲーション拡張
  | 'dashboard'       // ダッシュボード拡張  
  | 'document-action' // 文書操作拡張
  | 'notification'    // 通知システム拡張
  | 'report'          // レポート機能拡張
  | 'integration'     // 外部連携
  | 'workflow'        // ワークフロー拡張
  | 'ui-component';   // UI コンポーネント拡張

/**
 * ナビゲーション拡張プラグイン
 */
export interface NavigationPlugin extends Plugin {
  getNavigationItems(): NavigationItem[];
}

export interface NavigationItem {
  id: string;
  label: string;
  icon?: string;
  path: string;
  order: number;
  permissions?: string[];
  badge?: string | number;
}

/**
 * ダッシュボードウィジェットプラグイン
 */
export interface DashboardPlugin extends Plugin {
  getWidgets(): DashboardWidget[];
}

export interface DashboardWidget {
  id: string;
  title: string;
  component: ComponentType;
  size: 'small' | 'medium' | 'large';
  order: number;
  refreshInterval?: number;
}

/**
 * 文書操作拡張プラグイン
 */
export interface DocumentActionPlugin extends Plugin {
  getActions(): DocumentAction[];
}

export interface DocumentAction {
  id: string;
  label: string;
  icon?: string;
  handler: (documentId: number) => Promise<void>;
  permissions?: string[];
  condition?: (document: any) => boolean;
}

/**
 * プラグインレジストリ
 */
class PluginRegistry {
  private plugins = new Map<string, Plugin>();
  private initializedPlugins = new Set<string>();

  /**
   * プラグインを登録
   */
  register(plugin: Plugin): void {
    if (this.plugins.has(plugin.id)) {
      throw new Error(`Plugin ${plugin.id} is already registered`);
    }

    // 依存関係チェック
    if (plugin.dependencies) {
      const missingDeps = plugin.dependencies.filter(dep => !this.plugins.has(dep));
      if (missingDeps.length > 0) {
        throw new Error(`Plugin ${plugin.id} has missing dependencies: ${missingDeps.join(', ')}`);
      }
    }

    this.plugins.set(plugin.id, plugin);
  }

  /**
   * プラグインを初期化
   */
  async initialize(pluginId: string): Promise<void> {
    const plugin = this.plugins.get(pluginId);
    if (!plugin) {
      throw new Error(`Plugin ${pluginId} not found`);
    }

    if (this.initializedPlugins.has(pluginId)) {
      return; // Already initialized
    }

    // 依存関係の初期化
    if (plugin.dependencies) {
      for (const dep of plugin.dependencies) {
        await this.initialize(dep);
      }
    }

    if (plugin.initialize) {
      await plugin.initialize();
    }

    this.initializedPlugins.add(pluginId);
    console.log(`[Plugin] Initialized: ${plugin.name} v${plugin.version}`);
  }

  /**
   * すべてのプラグインを初期化
   */
  async initializeAll(): Promise<void> {
    const plugins = Array.from(this.plugins.values());

    // 依存関係を考慮した順序でソート
    const sortedPlugins = this.topologicalSort(plugins);

    for (const plugin of sortedPlugins) {
      await this.initialize(plugin.id);
    }
  }

  /**
   * 特定の機能を提供するプラグインを取得
   */
  getPluginsByCapability<T extends Plugin>(capability: PluginCapability): T[] {
    return Array.from(this.plugins.values())
      .filter(plugin =>
        this.initializedPlugins.has(plugin.id) &&
        plugin.provides?.includes(capability)
      ) as T[];
  }

  /**
   * プラグインを取得
   */
  getPlugin<T extends Plugin>(id: string): T | undefined {
    return this.plugins.get(id) as T;
  }

  /**
   * 依存関係を考慮したトポロジカルソート
   */
  private topologicalSort(plugins: Plugin[]): Plugin[] {
    const visited = new Set<string>();
    const visiting = new Set<string>();
    const result: Plugin[] = [];
    const pluginMap = new Map(plugins.map(p => [p.id, p]));

    function visit(plugin: Plugin): void {
      if (visiting.has(plugin.id)) {
        throw new Error(`Circular dependency detected involving ${plugin.id}`);
      }
      if (visited.has(plugin.id)) return;

      visiting.add(plugin.id);

      // 依存関係を先に処理
      if (plugin.dependencies) {
        for (const depId of plugin.dependencies) {
          const dep = pluginMap.get(depId);
          if (dep) visit(dep);
        }
      }

      visiting.delete(plugin.id);
      visited.add(plugin.id);
      result.push(plugin);
    }

    for (const plugin of plugins) {
      visit(plugin);
    }

    return result;
  }
}

// グローバルプラグインレジストリ
export const pluginRegistry = new PluginRegistry();

/**
 * プラグイン開発支援ユーティリティ
 */
export class PluginUtils {
  /**
   * 設定値の取得（localStorage から）
   */
  static getConfig<T>(pluginId: string, key: string, defaultValue: T): T {
    try {
      const configKey = `plugin_${pluginId}_${key}`;
      const stored = localStorage.getItem(configKey);
      return stored ? JSON.parse(stored) : defaultValue;
    } catch {
      return defaultValue;
    }
  }

  /**
   * 設定値の保存
   */
  static setConfig<T>(pluginId: string, key: string, value: T): void {
    try {
      const configKey = `plugin_${pluginId}_${key}`;
      localStorage.setItem(configKey, JSON.stringify(value));
    } catch (error) {
      console.warn(`Failed to save plugin config: ${error}`);
    }
  }

  /**
   * プラグイン用のイベントエミッター
   */
  static createEventBus() {
    const listeners = new Map<string, Function[]>();

    return {
      on(event: string, callback: Function) {
        if (!listeners.has(event)) {
          listeners.set(event, []);
        }
        listeners.get(event)!.push(callback);
      },

      emit(event: string, ...args: any[]) {
        const callbacks = listeners.get(event) || [];
        callbacks.forEach(callback => {
          try {
            callback(...args);
          } catch (error) {
            console.error(`Plugin event handler error:`, error);
          }
        });
      },

      off(event: string, callback: Function) {
        const callbacks = listeners.get(event) || [];
        const index = callbacks.indexOf(callback);
        if (index > -1) {
          callbacks.splice(index, 1);
        }
      }
    };
  }
}

/**
 * 開発者用プラグイン情報表示
 */
export function getPluginDebugInfo() {
  if (!import.meta.env.DEV) return null;

  const plugins = Array.from(pluginRegistry['plugins'].values());
  return plugins.map(plugin => ({
    id: plugin.id,
    name: plugin.name,
    version: plugin.version,
    initialized: pluginRegistry['initializedPlugins'].has(plugin.id),
    provides: plugin.provides || [],
    dependencies: plugin.dependencies || []
  }));
}
