<script lang="ts">
  import { onMount } from "svelte";

  let systemStatus = {
    overallHealth: "healthy",
    uptime: "15日 7時間 23分",
    lastUpdate: new Date(),
    components: [
      {
        name: "Webサーバー",
        status: "healthy",
        responseTime: "45ms",
        uptime: "99.9%",
        details: "すべてのエンドポイントが正常に応答",
      },
      {
        name: "データベース",
        status: "healthy",
        responseTime: "12ms",
        uptime: "99.8%",
        details: "すべてのクエリが正常に実行",
      },
      {
        name: "ファイルストレージ",
        status: "warning",
        responseTime: "85ms",
        uptime: "98.5%",
        details: "ストレージ使用率が80%を超過",
      },
      {
        name: "AD認証サーバー",
        status: "healthy",
        responseTime: "120ms",
        uptime: "99.7%",
        details: "認証処理が正常に動作",
      },
      {
        name: "メール送信サーバー",
        status: "error",
        responseTime: "N/A",
        uptime: "95.2%",
        details: "SMTP接続エラーが発生中",
      },
    ],
  };

  let metrics = {
    activeUsers: 24,
    documentsProcessed: 156,
    storageUsed: 2.4,
    totalStorage: 10.0,
    memoryUsage: 68,
    cpuUsage: 42,
  };

  let recentAlerts = [
    {
      id: 1,
      severity: "error",
      component: "メール送信サーバー",
      message: "SMTP接続タイムアウトが発生しています",
      timestamp: "2024-08-22T14:32:00Z",
      acknowledged: false,
    },
    {
      id: 2,
      severity: "warning",
      component: "ファイルストレージ",
      message: "ストレージ使用率が80%を超過しました",
      timestamp: "2024-08-22T12:15:00Z",
      acknowledged: true,
    },
    {
      id: 3,
      severity: "info",
      component: "データベース",
      message: "定期メンテナンスが完了しました",
      timestamp: "2024-08-22T09:00:00Z",
      acknowledged: true,
    },
  ];

  let isLoading = true;

  onMount(() => {
    setTimeout(() => {
      isLoading = false;
    }, 500);

    // 定期的にデータを更新
    const interval = setInterval(() => {
      systemStatus.lastUpdate = new Date();
      // TODO: 実際のAPIからデータを取得
    }, 30000);

    return () => clearInterval(interval);
  });

  const getStatusColor = (status: string) => {
    switch (status) {
      case "healthy":
        return "text-green-600 bg-green-100";
      case "warning":
        return "text-yellow-600 bg-yellow-100";
      case "error":
        return "text-red-600 bg-red-100";
      default:
        return "text-gray-600 bg-gray-100";
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case "healthy":
        return "✓";
      case "warning":
        return "⚠";
      case "error":
        return "✗";
      default:
        return "?";
    }
  };

  const getStatusText = (status: string) => {
    switch (status) {
      case "healthy":
        return "正常";
      case "warning":
        return "警告";
      case "error":
        return "エラー";
      default:
        return "不明";
    }
  };

  const getSeverityColor = (severity: string) => {
    switch (severity) {
      case "error":
        return "bg-red-100 text-red-800";
      case "warning":
        return "bg-yellow-100 text-yellow-800";
      case "info":
        return "bg-blue-100 text-blue-800";
      default:
        return "bg-gray-100 text-gray-800";
    }
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleString("ja-JP");
  };

  const acknowledgeAlert = (alertId: number) => {
    recentAlerts = recentAlerts.map((alert) =>
      alert.id === alertId ? { ...alert, acknowledged: true } : alert,
    );
  };
</script>

<svelte:head>
  <title>システムステータス - 文書管理システム</title>
</svelte:head>

<div class="max-w-6xl mx-auto p-6">
  <div class="mb-6 flex justify-between items-center">
    <div>
      <h1 class="text-3xl font-bold text-gray-900">システムステータス</h1>
      <p class="text-gray-600 mt-2">システム全体の稼働状況とメトリクス</p>
    </div>
    <div class="text-sm text-gray-500">
      最終更新: {systemStatus.lastUpdate.toLocaleTimeString("ja-JP")}
    </div>
  </div>

  {#if isLoading}
    <div class="bg-white rounded-lg shadow p-8 text-center">
      <div
        class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500 mx-auto"
      ></div>
      <p class="mt-4 text-gray-600">システムステータスを読み込み中...</p>
    </div>
  {:else}
    <!-- システム全体ステータス -->
    <div class="bg-white rounded-lg shadow p-6 mb-6">
      <div class="flex items-center justify-between">
        <div class="flex items-center">
          <div
            class="w-4 h-4 rounded-full {systemStatus.overallHealth ===
            'healthy'
              ? 'bg-green-500'
              : 'bg-red-500'} mr-3"
          ></div>
          <div>
            <h2 class="text-xl font-semibold text-gray-900">
              システム全体: {systemStatus.overallHealth === "healthy"
                ? "正常稼働中"
                : "エラー発生中"}
            </h2>
            <p class="text-gray-600">稼働時間: {systemStatus.uptime}</p>
          </div>
        </div>
        <button class="btn btn-secondary">詳細ログを表示</button>
      </div>
    </div>

    <!-- メトリクス -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-6">
      <div class="bg-white rounded-lg shadow p-6">
        <div class="flex items-center">
          <div class="p-2 bg-blue-100 rounded-lg">
            <svg
              class="w-6 h-6 text-blue-600"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197m13.5-9a2.5 2.5 0 11-5 0 2.5 2.5 0 015 0z"
              ></path>
            </svg>
          </div>
          <div class="ml-4">
            <div class="text-2xl font-bold text-gray-900">
              {metrics.activeUsers}
            </div>
            <div class="text-sm text-gray-500">アクティブユーザー</div>
          </div>
        </div>
      </div>

      <div class="bg-white rounded-lg shadow p-6">
        <div class="flex items-center">
          <div class="p-2 bg-green-100 rounded-lg">
            <svg
              class="w-6 h-6 text-green-600"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
              ></path>
            </svg>
          </div>
          <div class="ml-4">
            <div class="text-2xl font-bold text-gray-900">
              {metrics.documentsProcessed}
            </div>
            <div class="text-sm text-gray-500">今日の処理文書数</div>
          </div>
        </div>
      </div>

      <div class="bg-white rounded-lg shadow p-6">
        <div class="flex items-center">
          <div class="p-2 bg-purple-100 rounded-lg">
            <svg
              class="w-6 h-6 text-purple-600"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M9 3v2m6-2v2M9 19v2m6-2v2M5 9H3m2 6H3m18-6h-2m2 6h-2M7 19h10a2 2 0 002-2V7a2 2 0 00-2-2H7a2 2 0 00-2 2v10a2 2 0 002 2zM9 9h6v6H9V9z"
              ></path>
            </svg>
          </div>
          <div class="ml-4">
            <div class="text-2xl font-bold text-gray-900">
              {metrics.cpuUsage}%
            </div>
            <div class="text-sm text-gray-500">CPU使用率</div>
          </div>
        </div>
      </div>

      <div class="bg-white rounded-lg shadow p-6">
        <div class="flex items-center">
          <div class="p-2 bg-orange-100 rounded-lg">
            <svg
              class="w-6 h-6 text-orange-600"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4"
              ></path>
            </svg>
          </div>
          <div class="ml-4">
            <div class="text-2xl font-bold text-gray-900">
              {metrics.storageUsed}GB
            </div>
            <div class="text-sm text-gray-500">ストレージ使用量</div>
          </div>
        </div>
      </div>
    </div>

    <!-- コンポーネントステータス -->
    <div class="bg-white rounded-lg shadow mb-6">
      <div class="p-6 border-b border-gray-200">
        <h2 class="text-lg font-semibold text-gray-900">
          コンポーネントステータス
        </h2>
      </div>
      <div class="divide-y divide-gray-200">
        {#each systemStatus.components as component}
          <div class="p-6 flex items-center justify-between">
            <div class="flex items-center">
              <div class="flex-shrink-0">
                <span
                  class="inline-flex items-center justify-center w-8 h-8 rounded-full {getStatusColor(
                    component.status,
                  )}"
                >
                  {getStatusIcon(component.status)}
                </span>
              </div>
              <div class="ml-4">
                <div class="text-sm font-medium text-gray-900">
                  {component.name}
                </div>
                <div class="text-sm text-gray-500">{component.details}</div>
              </div>
            </div>
            <div class="flex items-center space-x-6 text-sm">
              <div>
                <div class="text-gray-500">応答時間</div>
                <div class="font-medium">{component.responseTime}</div>
              </div>
              <div>
                <div class="text-gray-500">稼働率</div>
                <div class="font-medium">{component.uptime}</div>
              </div>
              <div>
                <span
                  class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {getStatusColor(
                    component.status,
                  )}"
                >
                  {getStatusText(component.status)}
                </span>
              </div>
            </div>
          </div>
        {/each}
      </div>
    </div>

    <!-- 最近のアラート -->
    <div class="bg-white rounded-lg shadow">
      <div class="p-6 border-b border-gray-200">
        <h2 class="text-lg font-semibold text-gray-900">最近のアラート</h2>
      </div>
      <div class="divide-y divide-gray-200">
        {#each recentAlerts as alert}
          <div
            class="p-6 flex items-center justify-between {alert.acknowledged
              ? 'opacity-60'
              : ''}"
          >
            <div class="flex items-center">
              <span
                class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {getSeverityColor(
                  alert.severity,
                )} mr-3"
              >
                {alert.severity.toUpperCase()}
              </span>
              <div>
                <div class="text-sm font-medium text-gray-900">
                  [{alert.component}] {alert.message}
                </div>
                <div class="text-sm text-gray-500">
                  {formatDate(alert.timestamp)}
                </div>
              </div>
            </div>
            <div class="flex items-center space-x-3">
              {#if alert.acknowledged}
                <span class="text-sm text-green-600">確認済み</span>
              {:else}
                <button
                  on:click={() => acknowledgeAlert(alert.id)}
                  class="text-sm text-blue-600 hover:text-blue-800"
                >
                  確認
                </button>
              {/if}
            </div>
          </div>
        {/each}
      </div>
    </div>
  {/if}
</div>
