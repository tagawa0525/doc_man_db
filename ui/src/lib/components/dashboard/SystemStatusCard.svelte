<script lang="ts">
  import { onMount } from "svelte";

  interface SystemStatus {
    service: string;
    status: "healthy" | "warning" | "error" | "maintenance";
    lastCheck: string;
    uptime: string;
    responseTime?: number;
    details?: string;
  }

  let systemStatuses: SystemStatus[] = [];
  let isLoading = true;
  let overallStatus: "healthy" | "warning" | "error" = "healthy";

  // 仮のシステム状況データ
  const mockStatuses: SystemStatus[] = [
    {
      service: "Webサーバー",
      status: "healthy",
      lastCheck: "2024-08-17 10:30:00",
      uptime: "99.98%",
      responseTime: 120,
      details: "すべてのエンドポイントが正常に応答",
    },
    {
      service: "データベース",
      status: "healthy",
      lastCheck: "2024-08-17 10:29:45",
      uptime: "99.95%",
      responseTime: 45,
      details: "接続プール: 8/20 使用中",
    },
    {
      service: "ファイルサーバー",
      status: "warning",
      lastCheck: "2024-08-17 10:25:30",
      uptime: "98.72%",
      responseTime: 2300,
      details: "レスポンス時間が平均より高い",
    },
    {
      service: "バックアップシステム",
      status: "healthy",
      lastCheck: "2024-08-17 09:00:00",
      uptime: "99.99%",
      details: "日次バックアップ正常完了",
    },
    {
      service: "Teamsコネクター",
      status: "error",
      lastCheck: "2024-08-17 10:15:22",
      uptime: "95.24%",
      details: "Webhook URL応答なし、再設定が必要",
    },
    {
      service: "メール送信",
      status: "healthy",
      lastCheck: "2024-08-17 10:28:15",
      uptime: "99.87%",
      responseTime: 850,
      details: "SMTP接続正常",
    },
  ];

  // システム状況読み込み
  async function loadSystemStatus() {
    isLoading = true;

    try {
      // TODO: 実際のAPI呼び出しに置き換え
      await new Promise((resolve) => setTimeout(resolve, 1000));

      systemStatuses = mockStatuses;

      // 全体ステータス計算
      const hasError = systemStatuses.some((s) => s.status === "error");
      const hasWarning = systemStatuses.some((s) => s.status === "warning");

      if (hasError) {
        overallStatus = "error";
      } else if (hasWarning) {
        overallStatus = "warning";
      } else {
        overallStatus = "healthy";
      }
    } catch (error) {
      console.error("Failed to load system status:", error);
      overallStatus = "error";
    } finally {
      isLoading = false;
    }
  }

  // ステータス表示用関数
  function getStatusLabel(status: string): string {
    const labels: Record<string, string> = {
      healthy: "正常",
      warning: "警告",
      error: "エラー",
      maintenance: "メンテナンス中",
    };
    return labels[status] || status;
  }

  function getStatusColor(status: string): string {
    const colors: Record<string, string> = {
      healthy: "bg-green-100 text-green-800",
      warning: "bg-yellow-100 text-yellow-800",
      error: "bg-red-100 text-red-800",
      maintenance: "bg-blue-100 text-blue-800",
    };
    return colors[status] || "bg-gray-100 text-gray-800";
  }

  function getStatusIcon(status: string): string {
    const icons: Record<string, string> = {
      healthy: "M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
      warning:
        "M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z",
      error:
        "M12 9v3.75m9-.75a9 9 0 11-18 0 9 9 0 0118 0zm-9 3.75h.008v.008H12v-.008z",
      maintenance:
        "M11.42 15.17L17.25 21A2.652 2.652 0 0021 17.25l-5.877-5.877M11.42 15.17l2.496-3.03c.317-.384.74-.626 1.208-.766M11.42 15.17l-4.655 5.653a2.548 2.548 0 11-3.586-3.586l6.837-5.63m5.108-.233c.55-.164 1.163-.188 1.743-.14a4.5 4.5 0 004.486-6.336l-3.276 3.277a3.004 3.004 0 01-2.25-2.25l3.276-3.276a4.5 4.5 0 00-6.336 4.486c.091 1.076-.071 2.264-.904 2.95l-.102.085m-1.745 1.437L5.909 7.5H4.5L2.25 3.75l1.5-1.5L7.5 4.5v1.409l4.26 4.26m-1.745 1.437l1.745-1.437m6.615 8.206L15.75 15.75M4.867 19.125h.008v.008h-.008v-.008z",
    };
    return icons[status] || icons.healthy;
  }

  function getOverallStatusColor(): string {
    return getStatusColor(overallStatus);
  }

  function getOverallStatusIcon(): string {
    return getStatusIcon(overallStatus);
  }

  // 初期読み込み
  onMount(() => {
    loadSystemStatus();

    // 定期的にステータスを更新
    const interval = setInterval(loadSystemStatus, 60000); // 1分間隔

    return () => clearInterval(interval);
  });
</script>

<div class="bg-white shadow rounded-lg">
  <div class="px-6 py-4 border-b border-gray-200">
    <div class="flex items-center justify-between">
      <h3 class="text-lg font-medium text-gray-900">システム稼働状況</h3>
      {#if !isLoading}
        <div class="flex items-center space-x-2">
          <span
            class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {getOverallStatusColor()}"
          >
            <svg
              class="mr-1 h-3 w-3"
              fill="none"
              viewBox="0 0 24 24"
              stroke-width="1.5"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d={getOverallStatusIcon()}
              />
            </svg>
            全体: {getStatusLabel(overallStatus)}
          </span>
          <button
            type="button"
            class="text-sm text-gray-500 hover:text-gray-700"
            on:click={loadSystemStatus}
            aria-label="更新"
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
                d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
              />
            </svg>
          </button>
        </div>
      {/if}
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
          ステータス確認中...
        </div>
      </div>
    {:else}
      <div class="space-y-4">
        {#each systemStatuses as status}
          <div
            class="flex items-center justify-between p-4 border border-gray-200 rounded-lg hover:bg-gray-50"
          >
            <div class="flex items-center space-x-3">
              <div class="flex-shrink-0">
                <svg
                  class="h-5 w-5 {status.status === 'healthy'
                    ? 'text-green-500'
                    : status.status === 'warning'
                      ? 'text-yellow-500'
                      : status.status === 'error'
                        ? 'text-red-500'
                        : 'text-blue-500'}"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke-width="1.5"
                  stroke="currentColor"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d={getStatusIcon(status.status)}
                  />
                </svg>
              </div>
              <div>
                <h4 class="text-sm font-medium text-gray-900">
                  {status.service}
                </h4>
                <p class="text-xs text-gray-500">{status.details}</p>
              </div>
            </div>

            <div class="flex items-center space-x-4 text-sm">
              {#if status.responseTime}
                <div class="text-center">
                  <p class="text-xs text-gray-500">応答時間</p>
                  <p
                    class="font-medium {status.responseTime > 1000
                      ? 'text-red-600'
                      : status.responseTime > 500
                        ? 'text-yellow-600'
                        : 'text-green-600'}"
                  >
                    {status.responseTime}ms
                  </p>
                </div>
              {/if}

              <div class="text-center">
                <p class="text-xs text-gray-500">稼働率</p>
                <p class="font-medium text-gray-900">{status.uptime}</p>
              </div>

              <div class="text-center">
                <p class="text-xs text-gray-500">最終確認</p>
                <p class="font-medium text-gray-900">
                  {status.lastCheck.split(" ")[1]}
                </p>
              </div>

              <span
                class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {getStatusColor(
                  status.status,
                )}"
              >
                {getStatusLabel(status.status)}
              </span>
            </div>
          </div>
        {/each}
      </div>

      <div class="mt-6 pt-4 border-t border-gray-200">
        <div class="flex items-center justify-between text-sm text-gray-500">
          <span>最終更新: {new Date().toLocaleString("ja-JP")}</span>
          <a
            href="/reports/system-status"
            class="text-blue-600 hover:text-blue-900 font-medium"
          >
            詳細レポートを表示
          </a>
        </div>
      </div>
    {/if}
  </div>
</div>
