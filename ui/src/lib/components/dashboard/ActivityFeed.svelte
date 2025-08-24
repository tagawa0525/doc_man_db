<script lang="ts">
  import { onMount } from "svelte";

  interface Activity {
    id: string;
    type:
      | "document_create"
      | "document_update"
      | "document_delete"
      | "user_login"
      | "system_backup"
      | "file_check"
      | "approval_request"
      | "approval_complete";
    title: string;
    description: string;
    user: string;
    timestamp: string;
    relativeTime: string;
    documentId?: string;
    priority: "low" | "medium" | "high";
  }

  let activities: Activity[] = [];
  let isLoading = true;
  let selectedFilter = "all";

  // 仮のアクティビティデータ
  const mockActivities: Activity[] = [
    {
      id: "1",
      type: "document_create",
      title: "新規文書作成",
      description: "システム設計書 v3.0を作成しました",
      user: "山田太郎",
      timestamp: "2024-08-17 14:30:00",
      relativeTime: "30分前",
      documentId: "DOC-001",
      priority: "medium",
    },
    {
      id: "2",
      type: "approval_request",
      title: "承認依頼",
      description: "データベース移行計画書の承認を依頼しました",
      user: "佐藤花子",
      timestamp: "2024-08-17 13:45:00",
      relativeTime: "1時間前",
      documentId: "DOC-002",
      priority: "high",
    },
    {
      id: "3",
      type: "system_backup",
      title: "システムバックアップ",
      description: "日次バックアップが正常に完了しました",
      user: "システム",
      timestamp: "2024-08-17 12:00:00",
      relativeTime: "2時間前",
      priority: "low",
    },
    {
      id: "4",
      type: "document_update",
      title: "文書更新",
      description: "運用手順書 v2.1を更新しました",
      user: "田中一郎",
      timestamp: "2024-08-17 11:15:00",
      relativeTime: "3時間前",
      documentId: "DOC-003",
      priority: "medium",
    },
    {
      id: "5",
      type: "file_check",
      title: "ファイル存在確認",
      description: "ファイル確認処理で3件の不存在ファイルを検出",
      user: "システム",
      timestamp: "2024-08-17 10:30:00",
      relativeTime: "4時間前",
      priority: "high",
    },
    {
      id: "6",
      type: "approval_complete",
      title: "承認完了",
      description: "セキュリティポリシー文書が承認されました",
      user: "部長",
      timestamp: "2024-08-17 09:45:00",
      relativeTime: "5時間前",
      documentId: "DOC-004",
      priority: "medium",
    },
    {
      id: "7",
      type: "user_login",
      title: "ユーザーログイン",
      description: "新規ユーザーが初回ログインしました",
      user: "鈴木太郎",
      timestamp: "2024-08-17 09:00:00",
      relativeTime: "6時間前",
      priority: "low",
    },
    {
      id: "8",
      type: "document_delete",
      title: "文書削除",
      description: "古いテスト文書を削除しました",
      user: "山田太郎",
      timestamp: "2024-08-17 08:30:00",
      relativeTime: "6時間前",
      documentId: "DOC-005",
      priority: "low",
    },
  ];

  // フィルターオプション
  const filterOptions = [
    { value: "all", label: "全て" },
    { value: "document", label: "文書関連" },
    { value: "system", label: "システム" },
    { value: "approval", label: "承認" },
    { value: "high", label: "重要度：高" },
  ];

  // アクティビティ読み込み
  async function loadActivities() {
    isLoading = true;

    try {
      // TODO: 実際のAPI呼び出しに置き換え
      await new Promise((resolve) => setTimeout(resolve, 800));

      let filteredActivities = mockActivities;

      // フィルター適用
      if (selectedFilter !== "all") {
        switch (selectedFilter) {
          case "document":
            filteredActivities = filteredActivities.filter((a) =>
              [
                "document_create",
                "document_update",
                "document_delete",
              ].includes(a.type),
            );
            break;
          case "system":
            filteredActivities = filteredActivities.filter((a) =>
              ["system_backup", "file_check", "user_login"].includes(a.type),
            );
            break;
          case "approval":
            filteredActivities = filteredActivities.filter((a) =>
              ["approval_request", "approval_complete"].includes(a.type),
            );
            break;
          case "high":
            filteredActivities = filteredActivities.filter(
              (a) => a.priority === "high",
            );
            break;
        }
      }

      activities = filteredActivities;
    } catch (error) {
      console.error("Failed to load activities:", error);
    } finally {
      isLoading = false;
    }
  }

  // アクティビティアイコン取得
  function getActivityIcon(type: string): string {
    const icons: Record<string, string> = {
      document_create: "M12 4.5v15m7.5-7.5h-15",
      document_update:
        "M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L6.832 19.82a4.5 4.5 0 01-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 011.13-1.897L16.863 4.487zm0 0L19.5 7.125",
      document_delete:
        "M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0",
      user_login:
        "M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z",
      system_backup:
        "M20.25 6.375c0 2.278-3.694 4.125-8.25 4.125S3.75 8.653 3.75 6.375m16.5 0c0-2.278-3.694-4.125-8.25-4.125S3.75 4.097 3.75 6.375m16.5 0v11.25c0 2.278-3.694 4.125-8.25 4.125s-8.25-1.847-8.25-4.125V6.375m16.5 0v3.75m-16.5-3.75v3.75m16.5 0v3.75C20.25 16.153 16.556 18 12 18s-8.25-1.847-8.25-4.125v-3.75m16.5 0c0 2.278-3.694 4.125-8.25 4.125s-8.25-1.847-8.25-4.125",
      file_check: "M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
      approval_request:
        "M9 12h3.75M9 15h3.75M9 18h3.75m3 .75H18a2.25 2.25 0 002.25-2.25V6.108c0-1.135-.845-2.098-1.976-2.192a48.424 48.424 0 00-1.123-.08m-5.801 0c-.065.21-.1.433-.1.664 0 .414.336.75.75.75h4.5a.75.75 0 00.75-.75 2.25 2.25 0 00-.1-.664m-5.8 0A2.251 2.251 0 0113.5 2.25H15c1.012 0 1.867.668 2.15 1.586m-5.8 0c-.376.023-.75.05-1.124.08C9.095 4.01 8.25 4.973 8.25 6.108V8.25m0 0H4.875c-.621 0-1.125.504-1.125 1.125v11.25c0 .621.504 1.125 1.125 1.125h9.75c.621 0 1.125-.504 1.125-1.125V9.375c0-.621-.504-1.125-1.125-1.125H8.25zM6.75 12h.008v.008H6.75V12zm0 3h.008v.008H6.75V15zm0 3h.008v.008H6.75V18z",
      approval_complete:
        "M4.26 10.147a60.436 60.436 0 00-.491 6.347A48.627 48.627 0 0112 20.904a48.627 48.627 0 018.232-4.41 60.46 60.46 0 00-.491-6.347m-15.482 0a50.57 50.57 0 00-2.658-.813A59.905 59.905 0 0112 3.493a59.902 59.902 0 0110.399 5.84c-.896.248-1.783.52-2.658.814m-15.482 0A50.697 50.697 0 0112 13.489a50.702 50.702 0 017.74-3.342M6.75 15a.75.75 0 100-1.5.75.75 0 000 1.5zm0 0v-3.675A55.378 55.378 0 0112 8.443m-7.007 11.55A5.981 5.981 0 006.75 15.75v-1.5",
    };
    return icons[type] || icons.document_create;
  }

  // アクティビティ色取得
  function getActivityColor(type: string, priority: string): string {
    if (priority === "high") return "bg-red-100 text-red-600";

    const colors: Record<string, string> = {
      document_create: "bg-green-100 text-green-600",
      document_update: "bg-blue-100 text-blue-600",
      document_delete: "bg-gray-100 text-gray-600",
      user_login: "bg-purple-100 text-purple-600",
      system_backup: "bg-green-100 text-green-600",
      file_check: "bg-yellow-100 text-yellow-600",
      approval_request: "bg-orange-100 text-orange-600",
      approval_complete: "bg-emerald-100 text-emerald-600",
    };
    return colors[type] || "bg-gray-100 text-gray-600";
  }

  // 文書詳細表示
  function viewDocument(documentId: string) {
    if (documentId) {
      window.location.href = `/documents/${documentId}`;
    }
  }

  // フィルター変更
  function handleFilterChange() {
    loadActivities();
  }

  // 初期読み込み
  onMount(() => {
    loadActivities();

    // 定期的に更新
    const interval = setInterval(loadActivities, 30000); // 30秒間隔

    return () => clearInterval(interval);
  });
</script>

<div class="bg-white shadow rounded-lg">
  <div class="px-6 py-4 border-b border-gray-200">
    <div class="flex items-center justify-between">
      <h3 class="text-lg font-medium text-gray-900">最近のアクティビティ</h3>

      <!-- フィルター -->
      <select
        bind:value={selectedFilter}
        on:change={handleFilterChange}
        class="text-sm border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
      >
        {#each filterOptions as option}
          <option value={option.value}>{option.label}</option>
        {/each}
      </select>
    </div>
  </div>

  <div class="p-6">
    {#if isLoading}
      <div class="text-center py-8">
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
    {:else if activities.length === 0}
      <div class="text-center py-8">
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
            d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
          />
        </svg>
        <h3 class="mt-2 text-sm font-medium text-gray-900">
          アクティビティがありません
        </h3>
        <p class="mt-1 text-sm text-gray-500">
          選択したフィルターに該当するアクティビティが見つかりません。
        </p>
      </div>
    {:else}
      <div class="flow-root">
        <ul role="list" class="-mb-8">
          {#each activities as activity, index}
            <li>
              <div class="relative pb-8">
                {#if index !== activities.length - 1}
                  <span
                    class="absolute top-4 left-4 -ml-px h-full w-0.5 bg-gray-200"
                    aria-hidden="true"
                  ></span>
                {/if}
                <div class="relative flex space-x-3">
                  <div>
                    <span
                      class="h-8 w-8 rounded-full {getActivityColor(
                        activity.type,
                        activity.priority,
                      )} flex items-center justify-center ring-8 ring-white"
                    >
                      <svg
                        class="h-4 w-4"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke-width="1.5"
                        stroke="currentColor"
                      >
                        <path
                          stroke-linecap="round"
                          stroke-linejoin="round"
                          d={getActivityIcon(activity.type)}
                        />
                      </svg>
                    </span>
                  </div>
                  <div
                    class="min-w-0 flex-1 pt-1.5 flex justify-between space-x-4"
                  >
                    <div>
                      <p class="text-sm font-medium text-gray-900">
                        {activity.title}
                        {#if activity.priority === "high"}
                          <span
                            class="ml-2 inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-red-100 text-red-800"
                          >
                            重要
                          </span>
                        {/if}
                      </p>
                      <p class="text-sm text-gray-600 mt-1">
                        {activity.description}
                      </p>
                      <div
                        class="mt-2 flex items-center space-x-4 text-xs text-gray-500"
                      >
                        <span>実行者: {activity.user}</span>
                        {#if activity.documentId}
                          <button
                            type="button"
                            class="text-blue-600 hover:text-blue-900 font-medium"
                            on:click={() =>
                              viewDocument(activity.documentId || "")}
                          >
                            文書を表示
                          </button>
                        {/if}
                      </div>
                    </div>
                    <div
                      class="text-right text-sm whitespace-nowrap text-gray-500"
                    >
                      <time datetime={activity.timestamp}
                        >{activity.relativeTime}</time
                      >
                    </div>
                  </div>
                </div>
              </div>
            </li>
          {/each}
        </ul>
      </div>

      <div class="mt-6 pt-4 border-t border-gray-200">
        <a
          href="/reports/activity"
          class="w-full flex justify-center items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50"
        >
          すべてのアクティビティを表示
        </a>
      </div>
    {/if}
  </div>
</div>
