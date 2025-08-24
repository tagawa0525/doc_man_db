<script lang="ts">
  import { onMount } from "svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import StatsCard from "$lib/components/dashboard/StatsCard.svelte";
  import SystemStatusCard from "$lib/components/dashboard/SystemStatusCard.svelte";
  import ActivityFeed from "$lib/components/dashboard/ActivityFeed.svelte";

  // API統合
  import {
    dashboardStats,
    isLoadingStats,
    pendingApprovals,
    isLoadingApprovals,
    dashboardError,
    initializeDashboard,
    formatStatValue,
  } from "$lib/stores/dashboard.js";
  import { showError } from "$lib/stores/errors.js";

  // リアクティブな統計データ生成
  $: stats = $dashboardStats
    ? [
        {
          title: "総文書数",
          value: formatStatValue($dashboardStats.totalDocuments),
          change: "+2.1%", // TODO: 前月比データを取得
          trend: "up" as const,
          icon: "documents",
          description: "全部署の管理文書",
          color: "blue" as const,
        },
        {
          title: "今月作成",
          value: formatStatValue($dashboardStats.monthlyCreated),
          change: "+12.3%", // TODO: 前月比データを取得
          trend: "up" as const,
          icon: "new",
          description:
            new Date().toLocaleString("ja-JP", { month: "long" }) +
            "の新規作成文書",
          color: "green" as const,
        },
        {
          title: "ファイル不存在",
          value: formatStatValue($dashboardStats.missingFiles),
          change: "-25.0%", // TODO: 前月比データを取得
          trend:
            $dashboardStats.missingFiles > 0
              ? ("up" as const)
              : ("down" as const),
          icon: "warning",
          description: "最新ファイル確認結果",
          color:
            $dashboardStats.missingFiles > 0
              ? ("red" as const)
              : ("green" as const),
        },
        {
          title: "アクティブユーザー",
          value: formatStatValue($dashboardStats.activeUsers),
          change: "+8.7%", // TODO: 前月比データを取得
          trend: "up" as const,
          icon: "users",
          description: "過去30日間のアクティブユーザー",
          color: "purple" as const,
        },
        {
          title: "承認待ち",
          value: formatStatValue($dashboardStats.pendingApprovals),
          change: "+2", // TODO: 前日比データを取得
          trend: "up" as const,
          icon: "clock",
          description: "承認待ちの文書",
          color: "yellow" as const,
        },
        {
          title: "システム稼働率",
          value: ($dashboardStats.systemUptime * 100).toFixed(1) + "%",
          change: "+0.1%", // TODO: 前月比データを取得
          trend: "up" as const,
          icon: "check",
          description: "過去30日間の平均稼働率",
          color: "green" as const,
        },
      ]
    : [];

  // 初期読み込み
  onMount(async () => {
    await initializeDashboard();
  });

  // エラーハンドリング
  $: if ($dashboardError) {
    showError($dashboardError);
  }
</script>

<div class="space-y-6">
  <!-- ページヘッダー -->
  <div
    class="flex flex-col space-y-4 sm:flex-row sm:items-center sm:justify-between sm:space-y-0"
  >
    <div class="min-w-0 flex-1">
      <h1
        class="text-xl font-bold leading-7 text-gray-900 sm:text-2xl lg:text-3xl lg:tracking-tight"
      >
        ダッシュボード
      </h1>
      <p class="mt-1 text-sm text-gray-500">
        文書管理システムの概要と最新の活動状況
      </p>
    </div>
    <div class="flex flex-col space-y-2 sm:flex-row sm:space-y-0 sm:space-x-3">
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
            d="M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5M16.5 12L12 16.5m0 0L7.5 12m4.5 4.5V3"
          />
        </svg>
        <span class="hidden sm:inline">データエクスポート</span>
        <span class="sm:hidden">エクスポート</span>
      </Button>
      <Button variant="primary" size="sm">
        <a href="/documents/new" class="flex items-center">
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
              d="M12 4.5v15m7.5-7.5h-15"
            />
          </svg>
          <span class="hidden sm:inline">新規文書作成</span>
          <span class="sm:hidden">新規作成</span>
        </a>
      </Button>
    </div>
  </div>

  <!-- 統計カード -->
  {#if $isLoadingStats}
    <div class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3">
      {#each Array(6) as _}
        <div
          class="relative overflow-hidden rounded-lg bg-white px-4 py-5 shadow sm:px-6 sm:py-6"
        >
          <div class="animate-pulse">
            <div class="absolute rounded-md bg-gray-300 p-3 w-12 h-12"></div>
            <div class="ml-16 space-y-2">
              <div class="h-4 bg-gray-300 rounded w-3/4"></div>
              <div class="h-8 bg-gray-300 rounded w-1/2"></div>
            </div>
          </div>
        </div>
      {/each}
    </div>
  {:else if stats.length > 0}
    <div class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3">
      {#each stats as stat}
        <StatsCard
          title={stat.title}
          value={stat.value}
          change={stat.change}
          trend={stat.trend}
          icon={stat.icon}
          description={stat.description}
          color={stat.color}
        />
      {/each}
    </div>
  {:else}
    <div class="bg-gray-50 rounded-lg p-8 text-center">
      <div class="text-gray-500">
        <svg
          class="mx-auto h-12 w-12 text-gray-400 mb-4"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
          />
        </svg>
        <p class="text-lg font-medium text-gray-900 mb-1">データを取得中...</p>
        <p class="text-sm text-gray-500">
          統計情報を読み込んでいます。しばらくお待ちください。
        </p>
      </div>
    </div>
  {/if}

  <!-- メインコンテンツエリア -->
  <div class="grid grid-cols-1 gap-4 sm:gap-6 lg:grid-cols-3">
    <!-- アクティビティフィード -->
    <div class="lg:col-span-2">
      <ActivityFeed />
    </div>

    <!-- サイドバー -->
    <div class="space-y-4 sm:space-y-6">
      <!-- クイックアクション -->
      <div class="bg-white shadow rounded-lg">
        <div class="p-4 sm:p-6">
          <h3
            class="text-base sm:text-lg font-medium text-gray-900 mb-3 sm:mb-4"
          >
            クイックアクション
          </h3>
          <div
            class="grid grid-cols-2 gap-2 sm:grid-cols-1 sm:gap-3 lg:grid-cols-1"
          >
            <Button variant="primary" size="md">
              <a
                href="/documents/new"
                class="flex items-center justify-center w-full"
              >
                <svg
                  class="mr-1 sm:mr-2 h-4 w-4"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke-width="1.5"
                  stroke="currentColor"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M12 4.5v15m7.5-7.5h-15"
                  />
                </svg>
                <span class="text-xs sm:text-sm">新規作成</span>
              </a>
            </Button>
            <Button variant="secondary" size="md">
              <a href="/search" class="flex items-center justify-center w-full">
                <svg
                  class="mr-1 sm:mr-2 h-4 w-4"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke-width="1.5"
                  stroke="currentColor"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z"
                  />
                </svg>
                <span class="text-xs sm:text-sm">高度検索</span>
              </a>
            </Button>
            <Button variant="secondary" size="md">
              <a
                href="/reports"
                class="flex items-center justify-center w-full"
              >
                <svg
                  class="mr-1 sm:mr-2 h-4 w-4"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke-width="1.5"
                  stroke="currentColor"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z"
                  />
                </svg>
                <span class="text-xs sm:text-sm">レポート</span>
              </a>
            </Button>
            <Button variant="secondary" size="md">
              <a
                href="/circulations"
                class="flex items-center justify-center w-full"
              >
                <svg
                  class="mr-1 sm:mr-2 h-4 w-4"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke-width="1.5"
                  stroke="currentColor"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99"
                  />
                </svg>
                <span class="text-xs sm:text-sm">文書回覧</span>
              </a>
            </Button>
            <Button variant="secondary" size="md">
              <a
                href="/organization"
                class="flex items-center justify-center w-full"
              >
                <svg
                  class="mr-1 sm:mr-2 h-4 w-4"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke-width="1.5"
                  stroke="currentColor"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z"
                  />
                </svg>
                <span class="text-xs sm:text-sm">組織管理</span>
              </a>
            </Button>
          </div>
        </div>
      </div>

      <!-- 承認待ち回覧 -->
      <div class="bg-white shadow rounded-lg">
        <div class="p-4 sm:p-6">
          <h3
            class="text-base sm:text-lg font-medium text-gray-900 mb-3 sm:mb-4"
          >
            承認待ち回覧
          </h3>

          {#if $isLoadingApprovals}
            <div class="space-y-3">
              {#each Array(2) as _}
                <div class="animate-pulse p-3 bg-gray-50 rounded-lg">
                  <div class="space-y-2">
                    <div class="h-4 bg-gray-300 rounded w-3/4"></div>
                    <div class="h-3 bg-gray-300 rounded w-1/2"></div>
                  </div>
                </div>
              {/each}
            </div>
          {:else if $pendingApprovals && $pendingApprovals.length > 0}
            <div class="space-y-3">
              {#each $pendingApprovals as approval}
                <div
                  class="flex flex-col space-y-2 sm:flex-row sm:items-center sm:justify-between sm:space-y-0 p-3 bg-yellow-50 rounded-lg"
                >
                  <div class="min-w-0 flex-1">
                    <p class="text-sm font-medium text-gray-900 truncate">
                      {approval.documentTitle} - {approval.approvalType ===
                      "review"
                        ? "確認"
                        : approval.approvalType === "approval"
                          ? "承認"
                          : "確認"}
                    </p>
                    <p class="text-xs text-gray-500">
                      {approval.requesterName}からの回覧 • {new Date(
                        approval.requestedAt,
                      ).toLocaleDateString("ja-JP", {
                        month: "short",
                        day: "numeric",
                      })}
                    </p>
                  </div>
                  <Button variant="primary" size="sm">
                    <a
                      href="/circulations/{approval.id}"
                      class="flex items-center"
                    >
                      {approval.approvalType === "approval" ? "承認" : "確認"}
                    </a>
                  </Button>
                </div>
              {/each}
              <div class="pt-3 border-t border-gray-200">
                <a
                  href="/circulations"
                  class="text-sm text-blue-600 hover:text-blue-900 font-medium"
                >
                  すべての承認待ちを表示
                </a>
              </div>
            </div>
          {:else}
            <div class="text-center py-6">
              <svg
                class="mx-auto h-8 w-8 text-gray-400 mb-2"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                />
              </svg>
              <p class="text-sm text-gray-500">承認待ちの文書はありません</p>
            </div>
          {/if}
        </div>
      </div>
    </div>
  </div>

  <!-- システム稼働状況 -->
  <SystemStatusCard />
</div>
