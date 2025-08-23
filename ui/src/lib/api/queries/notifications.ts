// 通知履歴取得API
export const NOTIFICATION_HISTORY = `
  query NotificationHistory($filters: NotificationFilters, $limit: Int = 20, $offset: Int = 0) {
    notificationHistory(filters: $filters, limit: $limit, offset: $offset) {
      notifications {
        id
        type
        recipient
        subject
        content
        status
        sentAt
        templateId
        documentId
      }
      total
    }
  }
`;

// 未読通知取得API
export const UNREAD_NOTIFICATIONS = `
  query UnreadNotifications($limit: Int = 10) {
    unreadNotifications(limit: $limit) {
      id
      type
      title
      message
      createdAt
      documentId
      isRead
    }
  }
`;

// 通知テンプレート取得API
export const NOTIFICATION_TEMPLATES = `
  query NotificationTemplates {
    notificationTemplates {
      id
      name
      description
      type
      subject
      content
      variables
      isActive
    }
  }
`;

// 通知送信API
export const SEND_NOTIFICATION = `
  mutation SendNotification($input: NotificationInput!) {
    sendNotification(input: $input) {
      id
      status
      message
    }
  }
`;

// 通知既読マークAPI
export const MARK_NOTIFICATION_READ = `
  mutation MarkNotificationRead($notificationId: String!) {
    markNotificationRead(notificationId: $notificationId) {
      id
      isRead
    }
  }
`;

// 全通知既読API
export const MARK_ALL_NOTIFICATIONS_READ = `
  mutation MarkAllNotificationsRead {
    markAllNotificationsRead {
      count
    }
  }
`;

// 通知履歴フィルター型
export interface NotificationFilters {
  type?: 'email' | 'teams' | 'app';
  status?: 'sent' | 'failed' | 'pending';
  dateFrom?: string;
  dateTo?: string;
  recipient?: string;
}

// 通知履歴型
export interface NotificationHistory {
  id: string;
  type: 'email' | 'teams' | 'app';
  recipient: string;
  subject: string;
  content: string;
  status: 'sent' | 'failed' | 'pending';
  sentAt: string;
  templateId?: string;
  documentId?: string;
}

// 未読通知型
export interface UnreadNotification {
  id: string;
  type: 'document' | 'approval' | 'system';
  title: string;
  message: string;
  createdAt: string;
  documentId?: string;
  isRead: boolean;
}

// 通知テンプレート型
export interface NotificationTemplate {
  id: string;
  name: string;
  description: string;
  type: 'email' | 'teams' | 'app';
  subject: string;
  content: string;
  variables: string[];
  isActive: boolean;
}

// 通知送信入力型
export interface NotificationInput {
  type: 'email' | 'teams' | 'app';
  recipients: string[];
  subject: string;
  content: string;
  templateId?: string;
  documentId?: string;
  priority?: 'low' | 'normal' | 'high';
}

// 通知履歴レスポンス型
export interface NotificationHistoryResult {
  notifications: NotificationHistory[];
  total: number;
}
