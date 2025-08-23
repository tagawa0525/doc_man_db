<script lang="ts">
  import { onMount } from "svelte";

  let usageData = {
    totalDocuments: 1250,
    documentsThisMonth: 85,
    totalUsers: 42,
    activeUsersThisMonth: 28,
    storageUsed: 2.4,
    downloadCount: 456,
    searchCount: 1024,
  };

  let monthlyStats = [
    {
      month: "2024-01",
      documents: 45,
      users: 18,
      searches: 324,
      downloads: 156,
    },
    {
      month: "2024-02",
      documents: 52,
      users: 22,
      searches: 398,
      downloads: 189,
    },
    {
      month: "2024-03",
      documents: 68,
      users: 25,
      searches: 456,
      downloads: 234,
    },
    {
      month: "2024-04",
      documents: 41,
      users: 20,
      searches: 389,
      downloads: 167,
    },
    {
      month: "2024-05",
      documents: 59,
      users: 26,
      searches: 512,
      downloads: 245,
    },
    {
      month: "2024-06",
      documents: 73,
      users: 29,
      searches: 634,
      downloads: 298,
    },
    {
      month: "2024-07",
      documents: 81,
      users: 31,
      searches: 724,
      downloads: 356,
    },
    {
      month: "2024-08",
      documents: 85,
      users: 28,
      searches: 1024,
      downloads: 456,
    },
  ];

  let topUsers = [
    {
      name: "田中 太郎",
      department: "技術部",
      documents: 23,
      searches: 145,
      lastActive: "2024-08-22",
    },
    {
      name: "佐藤 花子",
      department: "営業部",
      documents: 18,
      searches: 98,
      lastActive: "2024-08-21",
    },
    {
      name: "鈴木 次郎",
      department: "企画部",
      documents: 15,
      searches: 87,
      lastActive: "2024-08-22",
    },
    {
      name: "高橋 三郎",
      department: "技術部",
      documents: 12,
      searches: 76,
      lastActive: "2024-08-20",
    },
    {
      name: "伊藤 四郎",
      department: "総務部",
      documents: 10,
      searches: 65,
      lastActive: "2024-08-19",
    },
  ];

  let topDocuments = [
    {
      title: "技術仕様書 v2.0",
      documentNumber: "CTA-2508001",
      views: 89,
      downloads: 45,
      type: "技術仕様書",
    },
    {
      title: "プロジェクト計画書",
      documentNumber: "CTA-2508002",
      views: 67,
      downloads: 34,
      type: "プロジェクト計画",
    },
    {
      title: "システム設計書",
      documentNumber: "CTA-2508003",
      views: 54,
      downloads: 28,
      type: "設計書",
    },
    {
      title: "品質管理手順書",
      documentNumber: "CTA-2508004",
      views: 43,
      downloads: 23,
      type: "手順書",
    },
    {
      title: "顧客提案書テンプレート",
      documentNumber: "CTA-2508005",
      views: 38,
      downloads: 19,
      type: "提案書",
    },
  ];

  let searchTerms = [
    { term: "技術仕様", count: 156, trend: "up" },
    { term: "設計", count: 134, trend: "up" },
    { term: "プロジェクト", count: 98, trend: "stable" },
    { term: "手順書", count: 87, trend: "down" },
    { term: "契約", count: 76, trend: "up" },
    { term: "提案書", count: 65, trend: "stable" },
    { term: "API", count: 54, trend: "up" },
    { term: "データベース", count: 43, trend: "down" },
  ];

  let isLoading = true;
  let selectedPeriod = "month";

  onMount(() => {
    setTimeout(() => {
      isLoading = false;
    }, 500);
  });

  const formatMonth = (monthStr: string) => {
    const [year, month] = monthStr.split("-");
    return `${year}年${month}月`;
  };

  const getTrendIcon = (trend: string) => {
    switch (trend) {
      case "up":
        return "↗️";
      case "down":
        return "↘️";
      case "stable":
        return "➡️";
      default:
        return "";
    }
  };

  const getTrendColor = (trend: string) => {
    switch (trend) {
      case "up":
        return "text-green-600";
      case "down":
        return "text-red-600";
      case "stable":
        return "text-gray-600";
      default:
        return "text-gray-600";
    }
  };
</script>

<svelte:head>
  <title>利用状況レポート - 文書管理システム</title>
</svelte:head>

<div class="max-w-6xl mx-auto p-6">
  <div class="mb-6 flex justify-between items-center">
    <div>
      <h1 class="text-3xl font-bold text-gray-900">利用状況レポート</h1>
      <p class="text-gray-600 mt-2">システムの利用状況とトレンド分析</p>
    </div>
    <div class="flex space-x-2">
      <select bind:value={selectedPeriod} class="input">
        <option value="week">過去1週間</option>
        <option value="month">過去1ヶ月</option>
        <option value="quarter">過去3ヶ月</option>
        <option value="year">過去1年</option>
      </select>
      <button class="btn btn-primary">レポート出力</button>
    </div>
  </div>

  {#if isLoading}
    <div class="bg-white rounded-lg shadow p-8 text-center">
      <div
        class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500 mx-auto"
      ></div>
      <p class="mt-4 text-gray-600">利用状況データを読み込み中...</p>
    </div>
  {:else}
    <!-- 利用統計サマリー -->
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
                d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
              ></path>
            </svg>
          </div>
          <div class="ml-4">
            <div class="text-2xl font-bold text-gray-900">
              {usageData.totalDocuments.toLocaleString()}
            </div>
            <div class="text-sm text-gray-500">総文書数</div>
            <div class="text-xs text-green-600">
              今月 +{usageData.documentsThisMonth}
            </div>
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
                d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197m13.5-9a2.5 2.5 0 11-5 0 2.5 2.5 0 015 0z"
              ></path>
            </svg>
          </div>
          <div class="ml-4">
            <div class="text-2xl font-bold text-gray-900">
              {usageData.activeUsersThisMonth}
            </div>
            <div class="text-sm text-gray-500">月間アクティブユーザー</div>
            <div class="text-xs text-gray-600">
              総ユーザー数 {usageData.totalUsers}
            </div>
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
                d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
              ></path>
            </svg>
          </div>
          <div class="ml-4">
            <div class="text-2xl font-bold text-gray-900">
              {usageData.searchCount.toLocaleString()}
            </div>
            <div class="text-sm text-gray-500">今月の検索回数</div>
            <div class="text-xs text-green-600">先月比 +12%</div>
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
                d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10"
              ></path>
            </svg>
          </div>
          <div class="ml-4">
            <div class="text-2xl font-bold text-gray-900">
              {usageData.downloadCount}
            </div>
            <div class="text-sm text-gray-500">今月のダウンロード数</div>
            <div class="text-xs text-green-600">先月比 +8%</div>
          </div>
        </div>
      </div>
    </div>

    <!-- 月次トレンド -->
    <div class="bg-white rounded-lg shadow mb-6">
      <div class="p-6 border-b border-gray-200">
        <h2 class="text-lg font-semibold text-gray-900">月次利用トレンド</h2>
      </div>
      <div class="p-6">
        <div class="overflow-x-auto">
          <table class="min-w-full">
            <thead>
              <tr class="border-b border-gray-200">
                <th class="text-left py-2 text-sm font-medium text-gray-500"
                  >月</th
                >
                <th class="text-right py-2 text-sm font-medium text-gray-500"
                  >新規文書</th
                >
                <th class="text-right py-2 text-sm font-medium text-gray-500"
                  >アクティブユーザー</th
                >
                <th class="text-right py-2 text-sm font-medium text-gray-500"
                  >検索回数</th
                >
                <th class="text-right py-2 text-sm font-medium text-gray-500"
                  >ダウンロード数</th
                >
              </tr>
            </thead>
            <tbody class="divide-y divide-gray-200">
              {#each monthlyStats.slice(-6) as stat}
                <tr>
                  <td class="py-3 text-sm text-gray-900"
                    >{formatMonth(stat.month)}</td
                  >
                  <td class="py-3 text-sm text-gray-900 text-right"
                    >{stat.documents}</td
                  >
                  <td class="py-3 text-sm text-gray-900 text-right"
                    >{stat.users}</td
                  >
                  <td class="py-3 text-sm text-gray-900 text-right"
                    >{stat.searches.toLocaleString()}</td
                  >
                  <td class="py-3 text-sm text-gray-900 text-right"
                    >{stat.downloads}</td
                  >
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mb-6">
      <!-- トップユーザー -->
      <div class="bg-white rounded-lg shadow">
        <div class="p-6 border-b border-gray-200">
          <h2 class="text-lg font-semibold text-gray-900">
            アクティブユーザー TOP5
          </h2>
        </div>
        <div class="divide-y divide-gray-200">
          {#each topUsers as user, index}
            <div class="p-4 flex items-center justify-between">
              <div class="flex items-center">
                <div
                  class="flex-shrink-0 w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center text-blue-600 font-medium text-sm"
                >
                  {index + 1}
                </div>
                <div class="ml-3">
                  <div class="text-sm font-medium text-gray-900">
                    {user.name}
                  </div>
                  <div class="text-sm text-gray-500">{user.department}</div>
                </div>
              </div>
              <div class="text-right">
                <div class="text-sm text-gray-900">{user.documents}件作成</div>
                <div class="text-xs text-gray-500">{user.searches}回検索</div>
              </div>
            </div>
          {/each}
        </div>
      </div>

      <!-- 人気文書 -->
      <div class="bg-white rounded-lg shadow">
        <div class="p-6 border-b border-gray-200">
          <h2 class="text-lg font-semibold text-gray-900">人気文書 TOP5</h2>
        </div>
        <div class="divide-y divide-gray-200">
          {#each topDocuments as doc, index}
            <div class="p-4 flex items-center justify-between">
              <div class="flex items-center">
                <div
                  class="flex-shrink-0 w-8 h-8 bg-green-100 rounded-full flex items-center justify-center text-green-600 font-medium text-sm"
                >
                  {index + 1}
                </div>
                <div class="ml-3">
                  <div class="text-sm font-medium text-gray-900">
                    {doc.title}
                  </div>
                  <div class="text-sm text-gray-500">{doc.documentNumber}</div>
                </div>
              </div>
              <div class="text-right">
                <div class="text-sm text-gray-900">{doc.views}回表示</div>
                <div class="text-xs text-gray-500">
                  {doc.downloads}回ダウンロード
                </div>
              </div>
            </div>
          {/each}
        </div>
      </div>
    </div>

    <!-- 検索キーワードトレンド -->
    <div class="bg-white rounded-lg shadow">
      <div class="p-6 border-b border-gray-200">
        <h2 class="text-lg font-semibold text-gray-900">人気検索キーワード</h2>
      </div>
      <div class="p-6">
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
          {#each searchTerms as term}
            <div
              class="flex items-center justify-between p-3 bg-gray-50 rounded-lg"
            >
              <div>
                <div class="text-sm font-medium text-gray-900">{term.term}</div>
                <div class="text-xs text-gray-500">{term.count}回</div>
              </div>
              <div class="text-lg {getTrendColor(term.trend)}">
                {getTrendIcon(term.trend)}
              </div>
            </div>
          {/each}
        </div>
      </div>
    </div>
  {/if}
</div>
