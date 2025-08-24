// ダッシュボード統計API
export const DASHBOARD_STATS = `
  query DashboardStats {
    dashboardStats {
      totalDocuments
      monthlyCreated
      missingFiles
      activeUsers
      pendingApprovals
      systemUptime
    }
  }
`;

// ダッシュボード統計レスポンス型
export interface DashboardStats {
  totalDocuments: number;
  monthlyCreated: number;
  missingFiles: number;
  activeUsers: number;
  pendingApprovals: number;
  systemUptime: number;
}

// システム稼働状況API
export const SYSTEM_STATUS = `
  query SystemStatus {
    systemStatus {
      apiStatus
      databaseStatus
      fileSystemStatus
      lastBackup
      serverUptime
      memoryUsage
      diskUsage
    }
  }
`;

// システム稼働状況型
export interface SystemStatus {
  apiStatus: 'healthy' | 'warning' | 'error';
  databaseStatus: 'healthy' | 'warning' | 'error';
  fileSystemStatus: 'healthy' | 'warning' | 'error';
  lastBackup: string;
  serverUptime: string;
  memoryUsage: number;
  diskUsage: number;
}

// アクティビティフィードAPI
export const RECENT_ACTIVITIES = `
  query RecentActivities($limit: Int = 10) {
    recentActivities(limit: $limit) {
      id
      type
      message
      user
      timestamp
      documentId
      documentTitle
    }
  }
`;

// アクティビティレスポンス型
export interface Activity {
  id: string;
  type: 'create' | 'update' | 'delete' | 'approval' | 'circulation';
  message: string;
  user: string;
  timestamp: string;
  documentId?: string;
  documentTitle?: string;
}

// 承認待ち文書API
export const PENDING_APPROVALS = `
  query PendingApprovals($limit: Int = 5) {
    pendingApprovals(limit: $limit) {
      id
      documentId
      documentTitle
      requesterName
      requestedAt
      approvalType
    }
  }
`;

// 承認待ち文書型
export interface PendingApproval {
  id: string;
  documentId: string;
  documentTitle: string;
  requesterName: string;
  requestedAt: string;
  approvalType: 'review' | 'approval' | 'acknowledge';
}
