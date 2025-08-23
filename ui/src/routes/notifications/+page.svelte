<script lang="ts">
  import { onMount } from "svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Select from "$lib/components/ui/Select.svelte";
  import PlaceholderBanner from "$lib/components/ui/PlaceholderBanner.svelte";

  // 通知履歴データ
  interface NotificationHistory {
    id: string;
    type: "email" | "teams" | "app";
    recipient: string;
    subject: string;
    content: string;
    status: "sent" | "failed" | "pending";
    sentAt: string;
    templateId?: string;
    documentId?: string;
  }

  let notifications: NotificationHistory[] = [];
  let isLoading = true;
  let currentPage = 1;
  const pageSize = 20;
  let totalCount = 0;

  // フィルター
  let filters = {
    type: "",
    status: "",
    dateFrom: "",
    dateTo: "",
    recipient: "",
  };

  // 仮の通知履歴データ
  const mockNotifications: NotificationHistory[] = [
    {
      id: "1",
      type: "email",
      recipient: "yamada@company.com",
      subject: "文書承認依頼: システム設計書",
      content: "文書「システム設計書」の承認依頼があります。",
      status: "sent",
      sentAt: "2024-08-15 14:30:00",
      templateId: "document_approval",
      documentId: "1",
    },
    {
      id: "2",
      type: "teams",
      recipient: "General Channel",
      subject: "ファイル存在確認結果",
      content: "ファイル存在確認が完了しました。不存在ファイル: 3件",
      status: "sent",
      sentAt: "2024-08-15 12:00:00",
      templateId: "file_check_result",
    },
    {
      id: "3",
      type: "email",
      recipient: "admin@company.com",
      subject: "システムエラー通知",
      content: "データベース接続エラーが発生しました。",
      status: "failed",
      sentAt: "2024-08-15 09:15:00",
      templateId: "system_alert",
    },
    {
      id: "4",
      type: "app",
      recipient: "sato@company.com",
      subject: "文書更新通知",
      content: "文書「開発標準書」が更新されました。",
      status: "sent",
      sentAt: "2024-08-14 16:45:00",
    },
  ];

  // タイプオプション
  const typeOptions = [
    { value: "", label: "全て" },
    { value: "email", label: "Email" },
    { value: "teams", label: "Teams" },
    { value: "app", label: "アプリ内" },
  ];

  // ステータスオプション
  const statusOptions = [
    { value: "", label: "全て" },
    { value: "sent", label: "送信済み" },
    { value: "failed", label: "失敗" },
    { value: "pending", label: "送信中" },
  ];

  // 通知履歴読み込み
  async function loadNotifications() {
    isLoading = true;

    try {
      // TODO: 実際のAPI呼び出しに置き換え
      await new Promise((resolve) => setTimeout(resolve, 1000));

      let filteredNotifications = mockNotifications;

      // フィルタリング
      if (filters.type) {
        filteredNotifications = filteredNotifications.filter(
          (n) => n.type === filters.type,
        );
      }
      if (filters.status) {
        filteredNotifications = filteredNotifications.filter(
          (n) => n.status === filters.status,
        );
      }
      if (filters.recipient) {
        filteredNotifications = filteredNotifications.filter((n) =>
          n.recipient.toLowerCase().includes(filters.recipient.toLowerCase()),
        );
      }

      totalCount = filteredNotifications.length;
      notifications = filteredNotifications.slice(
        (currentPage - 1) * pageSize,
        currentPage * pageSize,
      );
    } catch (error) {
      console.error("Failed to load notifications:", error);
    } finally {
      isLoading = false;
    }
  }

  // フィルター適用
  function applyFilters() {
    currentPage = 1;
    loadNotifications();
  }

  // フィルタークリア
  function clearFilters() {
    filters = {
      type: "",
      status: "",
      dateFrom: "",
      dateTo: "",
      recipient: "",
    };
    applyFilters();
  }

  // ページ変更
  function handlePageChange(page: number) {
    currentPage = page;
    loadNotifications();
  }

  // 通知詳細表示
  function viewNotificationDetail(notification: NotificationHistory) {
    // TODO: 詳細モーダルまたは詳細ページの実装
    alert(`通知詳細: ${notification.subject}`);
  }

  // 再送信
  async function resendNotification(_notif: NotificationHistory) {
    try {
      // TODO: 実際の再送信API呼び出しに置き換え
      await new Promise((resolve) => setTimeout(resolve, 1000));

      alert("通知を再送信しました。");
      loadNotifications();
    } catch (error) {
      alert("再送信に失敗しました。");
      console.error("Failed to resend notification:", error);
    }
  }

  // ステータス表示
  function getStatusLabel(status: string): string {
    const labels: Record<string, string> = {
      sent: "送信済み",
      failed: "失敗",
      pending: "送信中",
    };
    return labels[status] || status;
  }

  function getStatusColor(status: string): string {
    const colors: Record<string, string> = {
      sent: "bg-green-100 text-green-800",
      failed: "bg-red-100 text-red-800",
      pending: "bg-yellow-100 text-yellow-800",
    };
    return colors[status] || "bg-gray-100 text-gray-800";
  }

  // タイプ表示
  function getTypeLabel(type: string): string {
    const labels: Record<string, string> = {
      email: "Email",
      teams: "Teams",
      app: "アプリ内",
    };
    return labels[type] || type;
  }

  function getTypeColor(type: string): string {
    const colors: Record<string, string> = {
      email: "bg-blue-100 text-blue-800",
      teams: "bg-purple-100 text-purple-800",
      app: "bg-gray-100 text-gray-800",
    };
    return colors[type] || "bg-gray-100 text-gray-800";
  }

  // 初期読み込み
  onMount(() => {
    loadNotifications();
  });

  // ページ数計算
  $: totalPages = Math.ceil(totalCount / pageSize);
  $: startIndex = (currentPage - 1) * pageSize + 1;
  $: endIndex = Math.min(currentPage * pageSize, totalCount);
</script>

<div class="space-y-6">
  <!-- Placeholder Banner -->
  <PlaceholderBanner feature="notifications" />

  <!-- ページヘッダー -->
  <div class="md:flex md:items-center md:justify-between">
    <div class="min-w-0 flex-1">
      <h1
        class="text-2xl font-bold leading-7 text-gray-900 sm:truncate sm:text-3xl sm:tracking-tight"
      >
        通知履歴
      </h1>
      <p class="mt-1 text-sm text-gray-500">送信された通知の履歴と管理</p>
    </div>
    <div class="mt-4 flex space-x-3 md:ml-4 md:mt-0">
      <Button variant="secondary" size="sm">
        <svg
          class="mr-2 h-4 w-4"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.324.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 011.37.49l1.296 2.247a1.125 1.125 0 01-.26 1.431l-1.003.827c-.293.24-.438.613-.431.992a6.759 6.759 0 010 .255c-.007.378.138.75.43.99l1.005.828c.424.35.534.954.26 1.43l-1.298 2.247a1.125 1.125 0 01-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.57 6.57 0 01-.22.128c-.331.183-.581.495-.644.869l-.213 1.28c-.09.543-.56.941-1.11.941h-2.594c-.55 0-1.02-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 01-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 01-1.369-.49l-1.297-2.247a1.125 1.125 0 01.26-1.431l1.004-.827c.292-.24.437-.613.43-.992a6.932 6.932 0 010-.255c.007-.378-.138-.75-.43-.99l-1.004-.828a1.125 1.125 0 01-.26-1.43l1.297-2.247a1.125 1.125 0 011.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.087.22-.128.332-.183.582-.495.644-.869l.214-1.281z M15 12a3 3 0 11-6 0 3 3 0 016 0z"
          />
        </svg>
        通知設定
      </Button>
      <Button variant="primary" size="sm">
        <a href="/notifications/templates">
          <svg
            class="mr-2 h-4 w-4"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z"
            />
          </svg>
          テンプレート管理
        </a>
      </Button>
    </div>
  </div>

  <!-- フィルター -->
  <div class="bg-white shadow rounded-lg p-6">
    <h3 class="text-lg font-medium text-gray-900 mb-4">フィルター</h3>

    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4">
      <div>
        <label for="type" class="block text-sm font-medium text-gray-700 mb-1">
          通知タイプ
        </label>
        <Select id="type" bind:value={filters.type} options={typeOptions} />
      </div>

      <div>
        <label
          for="status"
          class="block text-sm font-medium text-gray-700 mb-1"
        >
          ステータス
        </label>
        <Select
          id="status"
          bind:value={filters.status}
          options={statusOptions}
        />
      </div>

      <div>
        <label
          for="recipient"
          class="block text-sm font-medium text-gray-700 mb-1"
        >
          送信先
        </label>
        <Input
          id="recipient"
          bind:value={filters.recipient}
          placeholder="送信先で検索..."
        />
      </div>

      <div>
        <label
          for="dateFrom"
          class="block text-sm font-medium text-gray-700 mb-1"
        >
          送信日（開始）
        </label>
        <Input id="dateFrom" type="date" bind:value={filters.dateFrom} />
      </div>

      <div>
        <label
          for="dateTo"
          class="block text-sm font-medium text-gray-700 mb-1"
        >
          送信日（終了）
        </label>
        <Input id="dateTo" type="date" bind:value={filters.dateTo} />
      </div>
    </div>

    <div class="mt-4 flex space-x-3">
      <Button variant="primary" on:click={applyFilters}>検索</Button>
      <Button variant="secondary" on:click={clearFilters}>クリア</Button>
    </div>
  </div>

  <!-- 通知履歴一覧 -->
  <div class="bg-white shadow rounded-lg">
    <!-- ヘッダー -->
    <div class="px-6 py-4 border-b border-gray-200">
      <div class="flex justify-between items-center">
        <div>
          <h3 class="text-lg font-medium text-gray-900">通知履歴</h3>
          {#if totalCount > 0}
            <p class="text-sm text-gray-500">
              {totalCount}件中 {startIndex}-{endIndex}件を表示
            </p>
          {/if}
        </div>
      </div>
    </div>

    <!-- テーブル -->
    {#if isLoading}
      <div class="px-6 py-12 text-center">
        <div class="inline-flex items-center">
          <svg
            class="animate-spin -ml-1 mr-3 h-5 w-5 text-gray-600"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
          >
            <circle
              class="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              stroke-width="4"
            ></circle>
            <path
              class="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
            ></path>
          </svg>
          読み込み中...
        </div>
      </div>
    {:else if notifications.length === 0}
      <div class="px-6 py-12 text-center">
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
          通知履歴がありません
        </h3>
        <p class="mt-1 text-sm text-gray-500">
          送信された通知があるとここに表示されます。
        </p>
      </div>
    {:else}
      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                タイプ・ステータス
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                送信先
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                件名・内容
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                送信日時
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                操作
              </th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            {#each notifications as notification}
              <tr class="hover:bg-gray-50">
                <td class="px-6 py-4">
                  <div class="space-y-1">
                    <span
                      class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {getTypeColor(
                        notification.type,
                      )}"
                    >
                      {getTypeLabel(notification.type)}
                    </span>
                    <div>
                      <span
                        class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {getStatusColor(
                          notification.status,
                        )}"
                      >
                        {getStatusLabel(notification.status)}
                      </span>
                    </div>
                  </div>
                </td>
                <td class="px-6 py-4">
                  <div class="text-sm font-medium text-gray-900">
                    {notification.recipient}
                  </div>
                  {#if notification.templateId}
                    <div class="text-xs text-gray-500">
                      テンプレート: {notification.templateId}
                    </div>
                  {/if}
                </td>
                <td class="px-6 py-4">
                  <div class="text-sm font-medium text-gray-900">
                    {notification.subject}
                  </div>
                  <div class="text-sm text-gray-500 truncate max-w-xs">
                    {notification.content}
                  </div>
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                  {notification.sentAt}
                </td>
                <td
                  class="px-6 py-4 whitespace-nowrap text-sm font-medium space-x-2"
                >
                  <button
                    type="button"
                    class="text-blue-600 hover:text-blue-900"
                    on:click={() => viewNotificationDetail(notification)}
                  >
                    詳細
                  </button>
                  {#if notification.status === "failed"}
                    <button
                      type="button"
                      class="text-green-600 hover:text-green-900"
                      on:click={() => resendNotification(notification)}
                    >
                      再送信
                    </button>
                  {/if}
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>

      <!-- ページング -->
      {#if totalPages > 1}
        <div class="px-6 py-4 border-t border-gray-200">
          <nav class="flex items-center justify-between">
            <div class="flex-1 flex justify-between sm:hidden">
              <button
                disabled={currentPage <= 1}
                class="relative inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
                on:click={() => handlePageChange(currentPage - 1)}
              >
                前へ
              </button>
              <button
                disabled={currentPage >= totalPages}
                class="ml-3 relative inline-flex items-center px-4 py-2 border border-gray-300 text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
                on:click={() => handlePageChange(currentPage + 1)}
              >
                次へ
              </button>
            </div>
            <div
              class="hidden sm:flex-1 sm:flex sm:items-center sm:justify-between"
            >
              <div>
                <p class="text-sm text-gray-700">
                  <span class="font-medium">{startIndex}</span>
                  ～
                  <span class="font-medium">{endIndex}</span>
                  件を表示（全
                  <span class="font-medium">{totalCount}</span>
                  件）
                </p>
              </div>
              <div>
                <nav
                  class="relative z-0 inline-flex rounded-md shadow-sm -space-x-px"
                >
                  <button
                    disabled={currentPage <= 1}
                    class="relative inline-flex items-center px-2 py-2 rounded-l-md border border-gray-300 bg-white text-sm font-medium text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
                    on:click={() => handlePageChange(currentPage - 1)}
                    aria-label="前のページ"
                  >
                    <svg
                      class="h-5 w-5"
                      viewBox="0 0 20 20"
                      fill="currentColor"
                    >
                      <path
                        fill-rule="evenodd"
                        d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z"
                        clip-rule="evenodd"
                      />
                    </svg>
                  </button>

                  {#each Array.from({ length: Math.min(5, totalPages) }, (_, i) => i + Math.max(1, currentPage - 2)) as page}
                    {#if page <= totalPages}
                      <button
                        class="relative inline-flex items-center px-4 py-2 border text-sm font-medium {page ===
                        currentPage
                          ? 'z-10 bg-blue-50 border-blue-500 text-blue-600'
                          : 'bg-white border-gray-300 text-gray-500 hover:bg-gray-50'}"
                        on:click={() => handlePageChange(page)}
                      >
                        {page}
                      </button>
                    {/if}
                  {/each}

                  <button
                    disabled={currentPage >= totalPages}
                    class="relative inline-flex items-center px-2 py-2 rounded-r-md border border-gray-300 bg-white text-sm font-medium text-gray-500 hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
                    on:click={() => handlePageChange(currentPage + 1)}
                    aria-label="次のページ"
                  >
                    <svg
                      class="h-5 w-5"
                      viewBox="0 0 20 20"
                      fill="currentColor"
                    >
                      <path
                        fill-rule="evenodd"
                        d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
                        clip-rule="evenodd"
                      />
                    </svg>
                  </button>
                </nav>
              </div>
            </div>
          </nav>
        </div>
      {/if}
    {/if}
  </div>
</div>
