<script lang="ts">
  import { onMount } from "svelte";
  import Button from "../ui/Button.svelte";
  
  // API統合
  import {
    unreadNotifications,
    unreadCount,
    isLoadingUnread,
    notificationsError,
    loadUnreadNotifications,
    markNotificationAsRead,
    markAllNotificationsAsRead
  } from "$lib/stores/notifications.js";
  import { showError } from "$lib/stores/errors.js";

  interface Notification {
    id: string;
    type: "info" | "success" | "warning" | "error";
    title: string;
    message: string;
    timestamp: string;
    read: boolean;
    actions?: Array<{
      label: string;
      action: string;
    }>;
  }

  let notifications: Notification[] = [];
  let showNotifications = false;
  let unreadCount = 0;

  // 仮の通知データ
  const mockNotifications: Notification[] = [
    {
      id: "1",
      type: "info",
      title: "文書承認依頼",
      message: "山田太郎さんから「システム設計書」の承認依頼があります。",
      timestamp: "2024-08-15 14:30",
      read: false,
      actions: [
        { label: "承認", action: "approve" },
        { label: "詳細", action: "view" },
      ],
    },
    {
      id: "2",
      type: "warning",
      title: "ファイル存在確認",
      message: "3件の文書でファイルが見つかりませんでした。",
      timestamp: "2024-08-15 12:00",
      read: false,
      actions: [{ label: "確認", action: "check" }],
    },
    {
      id: "3",
      type: "success",
      title: "バックアップ完了",
      message: "データベースの日次バックアップが正常に完了しました。",
      timestamp: "2024-08-15 09:00",
      read: true,
    },
    {
      id: "4",
      type: "error",
      title: "システムエラー",
      message: "Teams通知の送信に失敗しました。管理者に連絡してください。",
      timestamp: "2024-08-14 16:45",
      read: false,
      actions: [
        { label: "再試行", action: "retry" },
        { label: "管理者連絡", action: "contact" },
      ],
    },
  ];

  // 通知読み込み
  async function loadNotifications() {
    try {
      // TODO: 実際のAPI呼び出しに置き換え
      await new Promise((resolve) => setTimeout(resolve, 500));
      notifications = mockNotifications;
      updateUnreadCount();
    } catch (error) {
      console.error("Failed to load notifications:", error);
    }
  }

  // 未読数更新
  function updateUnreadCount() {
    unreadCount = notifications.filter((n) => !n.read).length;
  }

  // 通知パネル表示切り替え
  function toggleNotifications() {
    showNotifications = !showNotifications;
  }

  // 通知を既読にする
  async function markAsRead(notificationId: string) {
    try {
      // TODO: 実際のAPI呼び出しに置き換え
      await new Promise((resolve) => setTimeout(resolve, 200));

      const notification = notifications.find((n) => n.id === notificationId);
      if (notification) {
        notification.read = true;
        notifications = [...notifications];
        updateUnreadCount();
      }
    } catch (error) {
      console.error("Failed to mark notification as read:", error);
    }
  }

  // 全て既読にする
  async function markAllAsRead() {
    try {
      // TODO: 実際のAPI呼び出しに置き換え
      await new Promise((resolve) => setTimeout(resolve, 500));

      notifications = notifications.map((n) => ({ ...n, read: true }));
      updateUnreadCount();
    } catch (error) {
      console.error("Failed to mark all notifications as read:", error);
    }
  }

  // 通知削除
  async function deleteNotification(notificationId: string) {
    try {
      // TODO: 実際のAPI呼び出しに置き換え
      await new Promise((resolve) => setTimeout(resolve, 200));

      notifications = notifications.filter((n) => n.id !== notificationId);
      updateUnreadCount();
    } catch (error) {
      console.error("Failed to delete notification:", error);
    }
  }

  // 通知アクション実行
  async function executeAction(notificationId: string, action: string) {
    try {
      // TODO: 実際のアクション実行に置き換え
      console.log(
        `Executing action: ${action} for notification: ${notificationId}`,
      );

      // 通知を既読にする
      await markAsRead(notificationId);

      // アクション別の処理
      switch (action) {
        case "approve":
          alert("承認処理を実行しました。");
          break;
        case "view":
          window.location.href = `/documents/${notificationId}`;
          break;
        case "check":
          window.location.href = "/reports/file-check";
          break;
        case "retry":
          alert("再試行しました。");
          break;
        case "contact":
          alert("管理者に連絡しました。");
          break;
      }
    } catch (error) {
      console.error("Failed to execute action:", error);
    }
  }

  // 通知タイプ別のアイコン
  function getNotificationIcon(type: string): string {
    const icons: Record<string, string> = {
      info: "M11.25 11.25l.041-.02a.75.75 0 011.063.852l-.708 2.836a.75.75 0 001.063.853l.041-.021M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9-3.75h.008v.008H12V8.25z",
      success: "M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
      warning:
        "M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z",
      error:
        "M12 9v3.75m9-.75a9 9 0 11-18 0 9 9 0 0118 0zm-9 3.75h.008v.008H12v-.008z",
    };
    return icons[type] || icons.info;
  }

  // 通知タイプ別の色
  function getNotificationColor(type: string): string {
    const colors: Record<string, string> = {
      info: "text-blue-500",
      success: "text-green-500",
      warning: "text-yellow-500",
      error: "text-red-500",
    };
    return colors[type] || colors.info;
  }

  // 初期読み込み
  onMount(() => {
    loadNotifications();

    // 定期的に通知を更新
    const interval = setInterval(loadNotifications, 30000); // 30秒間隔

    return () => clearInterval(interval);
  });
</script>

<!-- 通知ベルアイコン -->
<div class="relative">
  <button
    type="button"
    class="relative p-2 text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
    on:click={toggleNotifications}
  >
    <span class="sr-only">通知を表示</span>
    <svg
      class="h-6 w-6"
      fill="none"
      viewBox="0 0 24 24"
      stroke-width="1.5"
      stroke="currentColor"
    >
      <path
        stroke-linecap="round"
        stroke-linejoin="round"
        d="M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0"
      />
    </svg>

    <!-- 未読数バッジ -->
    {#if unreadCount > 0}
      <span
        class="absolute top-0 right-0 inline-flex items-center justify-center px-2 py-1 text-xs font-bold leading-none text-white transform translate-x-1/2 -translate-y-1/2 bg-red-600 rounded-full"
      >
        {unreadCount > 99 ? "99+" : unreadCount}
      </span>
    {/if}
  </button>

  <!-- 通知パネル -->
  {#if showNotifications}
    <div
      class="origin-top-right absolute right-0 mt-2 w-96 bg-white rounded-md shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none z-50"
    >
      <div class="py-1">
        <!-- ヘッダー -->
        <div class="px-4 py-3 border-b border-gray-200">
          <div class="flex items-center justify-between">
            <h3 class="text-lg font-medium text-gray-900">通知</h3>
            {#if unreadCount > 0}
              <Button variant="secondary" size="sm" on:click={markAllAsRead}>
                全て既読
              </Button>
            {/if}
          </div>
        </div>

        <!-- 通知一覧 -->
        <div class="max-h-96 overflow-y-auto">
          {#if notifications.length > 0}
            {#each notifications as notification}
              <div
                class="px-4 py-3 border-b border-gray-100 hover:bg-gray-50 {!notification.read
                  ? 'bg-blue-50'
                  : ''}"
              >
                <div class="flex">
                  <!-- アイコン -->
                  <div class="flex-shrink-0">
                    <svg
                      class="h-6 w-6 {getNotificationColor(notification.type)}"
                      fill="none"
                      viewBox="0 0 24 24"
                      stroke-width="1.5"
                      stroke="currentColor"
                    >
                      <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d={getNotificationIcon(notification.type)}
                      />
                    </svg>
                  </div>

                  <!-- 内容 -->
                  <div class="ml-3 flex-1">
                    <div class="flex items-start justify-between">
                      <div class="flex-1">
                        <p class="text-sm font-medium text-gray-900">
                          {notification.title}
                          {#if !notification.read}
                            <span
                              class="ml-1 inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800"
                            >
                              未読
                            </span>
                          {/if}
                        </p>
                        <p class="mt-1 text-sm text-gray-600">
                          {notification.message}
                        </p>
                        <p class="mt-1 text-xs text-gray-500">
                          {notification.timestamp}
                        </p>

                        <!-- アクションボタン -->
                        {#if notification.actions && notification.actions.length > 0}
                          <div class="mt-2 flex space-x-2">
                            {#each notification.actions as action}
                              <button
                                type="button"
                                class="text-xs text-blue-600 hover:text-blue-900 font-medium"
                                on:click={() =>
                                  executeAction(notification.id, action.action)}
                              >
                                {action.label}
                              </button>
                            {/each}
                          </div>
                        {/if}
                      </div>

                      <!-- 削除ボタン -->
                      <button
                        type="button"
                        class="ml-2 text-gray-400 hover:text-gray-600"
                        on:click={() => deleteNotification(notification.id)}
                        aria-label="通知を削除"
                      >
                        <svg
                          class="h-4 w-4"
                          fill="none"
                          viewBox="0 0 24 24"
                          stroke="currentColor"
                        >
                          <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M6 18L18 6M6 6l12 12"
                          />
                        </svg>
                      </button>
                    </div>
                  </div>
                </div>
              </div>
            {/each}
          {:else}
            <div class="px-4 py-8 text-center">
              <svg
                class="mx-auto h-12 w-12 text-gray-400"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M15 17h5l-5 5v-5zM4 19h10a2 2 0 002-2V7a2 2 0 00-2-2H4a2 2 0 00-2 2v10a2 2 0 002 2z"
                />
              </svg>
              <h3 class="mt-2 text-sm font-medium text-gray-900">
                通知はありません
              </h3>
              <p class="mt-1 text-sm text-gray-500">
                新しい通知があるとここに表示されます。
              </p>
            </div>
          {/if}
        </div>

        <!-- フッター -->
        {#if notifications.length > 0}
          <div class="px-4 py-3 border-t border-gray-200">
            <a
              href="/notifications"
              class="text-sm text-blue-600 hover:text-blue-900 font-medium"
            >
              すべての通知を表示
            </a>
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>

<!-- クリック外でパネルを閉じる -->
{#if showNotifications}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div
    class="fixed inset-0 z-40"
    on:click={() => (showNotifications = false)}
  ></div>
{/if}
