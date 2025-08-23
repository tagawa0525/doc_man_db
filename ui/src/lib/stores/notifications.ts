import { writable } from 'svelte/store';

export interface ToastNotification {
  id: string;
  type: 'info' | 'success' | 'warning' | 'error';
  title: string;
  message?: string;
  duration?: number;
  actions?: Array<{
    label: string;
    action: () => void;
  }>;
}

// トースト通知ストア
export const toastNotifications = writable<ToastNotification[]>([]);

// 通知追加
export function addToast(notification: Omit<ToastNotification, 'id'>) {
  const toast: ToastNotification = {
    ...notification,
    id: Date.now().toString() + Math.random().toString(36).substring(2, 11)
  };

  toastNotifications.update(notifications => [...notifications, toast]);

  return toast.id;
}

// 通知削除
export function removeToast(id: string) {
  toastNotifications.update(notifications =>
    notifications.filter(notification => notification.id !== id)
  );
}

// すべての通知をクリア
export function clearAllToasts() {
  toastNotifications.set([]);
}

// 便利な関数群
export const notificationHelpers = {
  // 成功通知
  success: (title: string, message?: string, duration?: number) =>
    addToast({ type: 'success', title, message, duration }),

  // エラー通知
  error: (title: string, message?: string, duration?: number) =>
    addToast({ type: 'error', title, message, duration }),

  // 情報通知
  info: (title: string, message?: string, duration?: number) =>
    addToast({ type: 'info', title, message, duration }),

  // 警告通知
  warning: (title: string, message?: string, duration?: number) =>
    addToast({ type: 'warning', title, message, duration }),

  // アクション付き通知
  withActions: (
    type: ToastNotification['type'],
    title: string,
    message: string,
    actions: ToastNotification['actions']
  ) => addToast({ type, title, message, actions })
};

// Email/Teams通知設定ストア
export interface NotificationSettings {
  email: {
    enabled: boolean;
    address: string;
    documentApproval: boolean;
    systemAlerts: boolean;
    dailyReport: boolean;
  };
  teams: {
    enabled: boolean;
    webhookUrl: string;
    documentApproval: boolean;
    systemAlerts: boolean;
    fileCheck: boolean;
  };
  inApp: {
    enabled: boolean;
    soundEnabled: boolean;
    documentApproval: boolean;
    systemAlerts: boolean;
    mentions: boolean;
  };
}

const defaultNotificationSettings: NotificationSettings = {
  email: {
    enabled: true,
    address: '',
    documentApproval: true,
    systemAlerts: true,
    dailyReport: false
  },
  teams: {
    enabled: false,
    webhookUrl: '',
    documentApproval: false,
    systemAlerts: true,
    fileCheck: true
  },
  inApp: {
    enabled: true,
    soundEnabled: true,
    documentApproval: true,
    systemAlerts: true,
    mentions: true
  }
};

export const notificationSettings = writable<NotificationSettings>(defaultNotificationSettings);

// 通知設定更新
export function updateNotificationSettings(settings: Partial<NotificationSettings>) {
  notificationSettings.update(current => ({
    ...current,
    ...settings
  }));
}

// 通知送信API（仮実装）
export class NotificationService {
  // Email通知送信
  static async sendEmail(
    recipient: string,
    subject: string,
    body: string,
    templateId?: string
  ): Promise<boolean> {
    try {
      // TODO: 実際のAPI呼び出しに置き換え
      console.log('Sending email:', { recipient, subject, body, templateId });
      await new Promise(resolve => setTimeout(resolve, 1000));

      // 成功通知
      notificationHelpers.success('メール送信完了', `${recipient}にメールを送信しました`);
      return true;
    } catch (error) {
      console.error('Email sending failed:', error);
      notificationHelpers.error('メール送信失敗', 'メールの送信に失敗しました');
      return false;
    }
  }

  // Teams通知送信
  static async sendTeamsMessage(
    webhookUrl: string,
    title: string,
    message: string,
    actionUrl?: string
  ): Promise<boolean> {
    try {
      // TODO: 実際のTeams Webhook呼び出しに置き換え
      console.log('Sending Teams message:', { webhookUrl, title, message, actionUrl });
      await new Promise(resolve => setTimeout(resolve, 800));

      // 成功通知
      notificationHelpers.success('Teams通知送信完了', 'Teamsチャネルに通知を送信しました');
      return true;
    } catch (error) {
      console.error('Teams message sending failed:', error);
      notificationHelpers.error('Teams通知送信失敗', 'Teams通知の送信に失敗しました');
      return false;
    }
  }

  // アプリ内通知送信
  static sendInAppNotification(
    type: ToastNotification['type'],
    title: string,
    message?: string,
    actions?: ToastNotification['actions']
  ): string {
    return addToast({ type, title, message, actions });
  }

  // 文書承認通知
  static async sendDocumentApprovalNotification(
    documentId: string,
    documentTitle: string,
    approverEmail: string,
    _requesterId: string
  ): Promise<void> {
    const settings = defaultNotificationSettings; // 実際は現在の設定を取得

    // Email通知
    if (settings.email.enabled && settings.email.documentApproval) {
      await this.sendEmail(
        approverEmail,
        `文書承認依頼: ${documentTitle}`,
        `文書「${documentTitle}」の承認依頼があります。\n\n承認画面: ${window.location.origin}/documents/${documentId}`,
        'document_approval'
      );
    }

    // Teams通知
    if (settings.teams.enabled && settings.teams.documentApproval && settings.teams.webhookUrl) {
      await this.sendTeamsMessage(
        settings.teams.webhookUrl,
        '文書承認依頼',
        `文書「${documentTitle}」の承認依頼があります。`,
        `${window.location.origin}/documents/${documentId}`
      );
    }

    // アプリ内通知
    if (settings.inApp.enabled && settings.inApp.documentApproval) {
      this.sendInAppNotification(
        'info',
        '文書承認依頼',
        `${documentTitle}の承認依頼があります`,
        [
          {
            label: '承認画面を開く',
            action: () => window.location.href = `/documents/${documentId}`
          }
        ]
      );
    }
  }

  // システムアラート通知
  static async sendSystemAlert(
    alertType: 'error' | 'warning' | 'info',
    title: string,
    message: string,
    details?: string
  ): Promise<void> {
    const settings = defaultNotificationSettings; // 実際は現在の設定を取得

    // Email通知
    if (settings.email.enabled && settings.email.systemAlerts) {
      await this.sendEmail(
        settings.email.address,
        `システムアラート: ${title}`,
        `${message}\n\n詳細: ${details || 'なし'}`,
        'system_alert'
      );
    }

    // Teams通知
    if (settings.teams.enabled && settings.teams.systemAlerts && settings.teams.webhookUrl) {
      await this.sendTeamsMessage(
        settings.teams.webhookUrl,
        `システムアラート: ${title}`,
        message
      );
    }

    // アプリ内通知
    if (settings.inApp.enabled && settings.inApp.systemAlerts) {
      this.sendInAppNotification(alertType, title, message);
    }
  }

  // ファイル確認結果通知
  static async sendFileCheckResults(
    totalFiles: number,
    missingFiles: number,
    _errorFiles: string[]
  ): Promise<void> {
    const settings = defaultNotificationSettings; // 実際は現在の設定を取得

    const message = `ファイル存在確認が完了しました。\n対象ファイル: ${totalFiles}件\n不存在ファイル: ${missingFiles}件`;

    // Teams通知（ファイル確認は主にTeamsで通知）
    if (settings.teams.enabled && settings.teams.fileCheck && settings.teams.webhookUrl) {
      await this.sendTeamsMessage(
        settings.teams.webhookUrl,
        'ファイル存在確認結果',
        message,
        `${window.location.origin}/reports/file-check`
      );
    }

    // アプリ内通知
    if (settings.inApp.enabled) {
      const notificationType = missingFiles > 0 ? 'warning' : 'success';
      this.sendInAppNotification(
        notificationType,
        'ファイル確認完了',
        message,
        [
          {
            label: '詳細レポートを見る',
            action: () => window.location.href = '/reports/file-check'
          }
        ]
      );
    }
  }
}
