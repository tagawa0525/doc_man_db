/**
 * 未実装機能のプレースホルダー設定
 * TASK-056: 未実装機能の対応方針設計
 */

export interface PlaceholderConfig {
  feature: string;
  description: string;
  roadmapPhase: string;
  estimatedCompletion: string;
  implementationPriority: 'high' | 'medium' | 'low';
  dependsOn: string[];
  businessImpact: 'critical' | 'high' | 'medium' | 'low';
  technicalComplexity: 'high' | 'medium' | 'low';
}

/**
 * Phase 10以降での実装予定機能
 */
export const PLACEHOLDER_CONFIGS: Record<string, PlaceholderConfig> = {
  // 回覧・承認機能
  circulation: {
    feature: '回覧・承認フロー',
    description: '文書の承認プロセスやワークフロー管理機能です。',
    roadmapPhase: 'Phase 10',
    estimatedCompletion: '2024年Q2',
    implementationPriority: 'high',
    dependsOn: ['organization', 'notifications'],
    businessImpact: 'critical',
    technicalComplexity: 'high'
  },

  // 組織管理
  organization: {
    feature: '組織管理',
    description: '部署・従業員管理、権限設定、Active Directory連携機能です。',
    roadmapPhase: 'Phase 10',
    estimatedCompletion: '2024年Q1',
    implementationPriority: 'high',
    dependsOn: ['ad_integration'],
    businessImpact: 'critical',
    technicalComplexity: 'high'
  },

  // 通知システム
  notifications: {
    feature: 'リアルタイム通知',
    description: 'Email・Teams連携、リアルタイム通知配信機能です。',
    roadmapPhase: 'Phase 10',
    estimatedCompletion: '2024年Q1',
    implementationPriority: 'high',
    dependsOn: ['email_service', 'teams_integration'],
    businessImpact: 'high',
    technicalComplexity: 'medium'
  },

  // レポート機能
  reports: {
    feature: 'レポート・分析',
    description: 'システム使用統計、監査ログ、パフォーマンス分析機能です。',
    roadmapPhase: 'Phase 11',
    estimatedCompletion: '2024年Q2',
    implementationPriority: 'medium',
    dependsOn: ['audit_logging', 'metrics_collection'],
    businessImpact: 'medium',
    technicalComplexity: 'medium'
  },

  // 設定管理
  settings: {
    feature: 'システム設定管理',
    description: '動的設定変更、システム管理者向け設定画面です。',
    roadmapPhase: 'Phase 11',
    estimatedCompletion: '2024年Q3',
    implementationPriority: 'medium',
    dependsOn: ['role_based_access'],
    businessImpact: 'medium',
    technicalComplexity: 'low'
  },

  // ファイル管理
  file_management: {
    feature: 'ファイル管理',
    description: 'ファイルアップロード、バージョン管理、ストレージ連携機能です。',
    roadmapPhase: 'Phase 10',
    estimatedCompletion: '2024年Q2',
    implementationPriority: 'high',
    dependsOn: ['cloud_storage'],
    businessImpact: 'high',
    technicalComplexity: 'medium'
  },

  // 監査ログ
  audit: {
    feature: '監査ログ',
    description: 'すべての操作ログ、セキュリティ監査、コンプライアンス対応です。',
    roadmapPhase: 'Phase 11',
    estimatedCompletion: '2024年Q2',
    implementationPriority: 'medium',
    dependsOn: ['security_framework'],
    businessImpact: 'high',
    technicalComplexity: 'medium'
  }
};

/**
 * 機能別の代替案・現状対応
 */
export const WORKAROUND_SOLUTIONS: Record<string, string> = {
  circulation: '現在は文書作成者による手動での承認依頼をお願いしています。',
  organization: 'システム管理者が手動で権限設定を行います。',
  notifications: 'メールでの手動通知またはシステム内通知をご確認ください。',
  reports: 'データベース管理者にカスタムレポートの作成を依頼してください。',
  settings: 'システム設定変更は管理者にお問い合わせください。',
  file_management: '現在はファイルパス管理のみ対応しています。',
  audit: '基本的な操作ログは記録されています。詳細は管理者にお問い合わせください。'
};

/**
 * 優先度別の実装スケジュール
 */
export const IMPLEMENTATION_ROADMAP = {
  'Phase 10': {
    period: '2024年Q1-Q2',
    features: ['organization', 'circulation', 'notifications', 'file_management'],
    focus: 'コアビジネス機能の実装'
  },
  'Phase 11': {
    period: '2024年Q2-Q3',
    features: ['reports', 'settings', 'audit'],
    focus: '管理・分析機能の充実'
  },
  'Phase 12': {
    period: '2024年Q3-Q4',
    features: ['mobile_app', 'advanced_search', 'ai_features'],
    focus: '先進機能・モバイル対応'
  }
};

/**
 * ヘルプ・問い合わせ情報
 */
export const SUPPORT_INFO = {
  email: 'support@company.com',
  phone: '03-1234-5678',
  hours: '平日 9:00-17:00',
  docs: '/docs/user-manual',
  admin: 'システム管理部'
};
