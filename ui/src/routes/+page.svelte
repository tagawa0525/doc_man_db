<script lang="ts">
  import Button from '$lib/components/ui/Button.svelte';
  
  // ダッシュボード統計データ（仮データ）
  const stats = [
    {
      title: '総文書数',
      value: '12,547',
      change: '+2.1%',
      trend: 'up',
      icon: 'documents'
    },
    {
      title: '今月作成',
      value: '89',
      change: '+12.3%',
      trend: 'up',
      icon: 'new'
    },
    {
      title: 'ファイル不存在',
      value: '3',
      change: '-25.0%',
      trend: 'down',
      icon: 'warning'
    },
    {
      title: 'アクティブユーザー',
      value: '24',
      change: '+8.7%',
      trend: 'up',
      icon: 'users'
    }
  ];
  
  const recentActivities = [
    {
      id: 1,
      action: '文書作成',
      document: 'システム設計書_v2.1',
      user: '山田太郎',
      time: '2時間前',
      type: 'create'
    },
    {
      id: 2,
      action: '文書更新',
      document: 'プロジェクト計画書',
      user: '佐藤花子',
      time: '4時間前',
      type: 'update'
    },
    {
      id: 3,
      action: 'ファイル確認',
      document: '月次レポート_2024年1月',
      user: 'システム',
      time: '6時間前',
      type: 'check'
    }
  ];
</script>

<div class="space-y-6">
  <!-- ページヘッダー -->
  <div class="md:flex md:items-center md:justify-between">
    <div class="min-w-0 flex-1">
      <h1 class="text-2xl font-bold leading-7 text-gray-900 sm:truncate sm:text-3xl sm:tracking-tight">
        ダッシュボード
      </h1>
      <p class="mt-1 text-sm text-gray-500">
        文書管理システムの概要と最新の活動状況
      </p>
    </div>
    <div class="mt-4 flex md:ml-4 md:mt-0">
      <Button variant="primary" size="sm">
        <svg class="mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
        </svg>
        新規文書作成
      </Button>
    </div>
  </div>

  <!-- 統計カード -->
  <div class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4">
    {#each stats as stat}
      <div class="relative overflow-hidden rounded-lg bg-white px-4 py-5 shadow sm:px-6 sm:py-6">
        <dt>
          <div class="absolute rounded-md bg-blue-500 p-3">
            <svg class="h-6 w-6 text-white" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
              {#if stat.icon === 'documents'}
                <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z" />
              {:else if stat.icon === 'new'}
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
              {:else if stat.icon === 'warning'}
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z" />
              {:else}
                <path stroke-linecap="round" stroke-linejoin="round" d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z" />
              {/if}
            </svg>
          </div>
          <p class="ml-16 truncate text-sm font-medium text-gray-500">{stat.title}</p>
        </dt>
        <dd class="ml-16 flex items-baseline">
          <p class="text-2xl font-semibold text-gray-900">{stat.value}</p>
          <p class="ml-2 flex items-baseline text-sm font-semibold {stat.trend === 'up' ? 'text-green-600' : 'text-red-600'}">
            <svg class="h-4 w-4 flex-shrink-0 self-center {stat.trend === 'up' ? '' : 'rotate-180'}" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 15.75l6-6 3.75 3.75 6-6" />
            </svg>
            <span class="ml-1">{stat.change}</span>
          </p>
        </dd>
      </div>
    {/each}
  </div>

  <!-- メインコンテンツエリア -->
  <div class="grid grid-cols-1 gap-6 lg:grid-cols-3">
    <!-- 最近の活動 -->
    <div class="lg:col-span-2">
      <div class="rounded-lg bg-white shadow">
        <div class="p-6">
          <h3 class="text-lg font-medium leading-6 text-gray-900">最近の活動</h3>
          <div class="mt-6 flow-root">
            <ul role="list" class="-my-5 divide-y divide-gray-200">
              {#each recentActivities as activity}
                <li class="py-4">
                  <div class="flex items-center space-x-4">
                    <div class="flex-shrink-0">
                      <div class="h-8 w-8 rounded-full {activity.type === 'create' ? 'bg-green-100' : activity.type === 'update' ? 'bg-blue-100' : 'bg-gray-100'} flex items-center justify-center">
                        <svg class="h-4 w-4 {activity.type === 'create' ? 'text-green-600' : activity.type === 'update' ? 'text-blue-600' : 'text-gray-600'}" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                          {#if activity.type === 'create'}
                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
                          {:else if activity.type === 'update'}
                            <path stroke-linecap="round" stroke-linejoin="round" d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L6.832 19.82a4.5 4.5 0 01-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 011.13-1.897L16.863 4.487zm0 0L19.5 7.125" />
                          {:else}
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                          {/if}
                        </svg>
                      </div>
                    </div>
                    <div class="min-w-0 flex-1">
                      <p class="text-sm font-medium text-gray-900">
                        {activity.action}: {activity.document}
                      </p>
                      <p class="text-sm text-gray-500">
                        {activity.user} • {activity.time}
                      </p>
                    </div>
                  </div>
                </li>
              {/each}
            </ul>
          </div>
          <div class="mt-6">
            <a href="/reports/activity" class="w-full flex justify-center items-center px-4 py-2 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50">
              すべての活動を表示
            </a>
          </div>
        </div>
      </div>
    </div>

    <!-- クイックアクション -->
    <div class="space-y-6">
      <div class="rounded-lg bg-white shadow">
        <div class="p-6">
          <h3 class="text-lg font-medium leading-6 text-gray-900">クイックアクション</h3>
          <div class="mt-6 space-y-4">
            <Button variant="primary" size="md">
              <svg class="mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
              </svg>
              新規文書作成
            </Button>
            <Button variant="secondary" size="md">
              <svg class="mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" d="m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z" />
              </svg>
              文書検索
            </Button>
            <Button variant="secondary" size="md">
              <svg class="mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" d="M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z" />
              </svg>
              レポート確認
            </Button>
          </div>
        </div>
      </div>

      <!-- システム状況 -->
      <div class="rounded-lg bg-white shadow">
        <div class="p-6">
          <h3 class="text-lg font-medium leading-6 text-gray-900">システム状況</h3>
          <div class="mt-6 space-y-4">
            <div class="flex items-center justify-between">
              <span class="text-sm font-medium text-gray-700">サーバー状況</span>
              <span class="inline-flex items-center rounded-full bg-green-100 px-2.5 py-0.5 text-xs font-medium text-green-800">
                正常
              </span>
            </div>
            <div class="flex items-center justify-between">
              <span class="text-sm font-medium text-gray-700">データベース</span>
              <span class="inline-flex items-center rounded-full bg-green-100 px-2.5 py-0.5 text-xs font-medium text-green-800">
                正常
              </span>
            </div>
            <div class="flex items-center justify-between">
              <span class="text-sm font-medium text-gray-700">ファイル確認</span>
              <span class="inline-flex items-center rounded-full bg-yellow-100 px-2.5 py-0.5 text-xs font-medium text-yellow-800">
                実行中
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</div>