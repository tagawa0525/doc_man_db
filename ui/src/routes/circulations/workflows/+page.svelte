<script lang="ts">
  import { onMount } from "svelte";

  let workflows = [
    {
      id: 1,
      name: "技術文書承認フロー",
      description: "技術仕様書や設計書の承認プロセス",
      steps: ["作成者", "技術主任", "部長", "役員"],
      status: "active",
      documents: 12,
    },
    {
      id: 2,
      name: "契約書承認フロー",
      description: "契約書類の承認プロセス",
      steps: ["作成者", "法務", "営業部長", "役員"],
      status: "active",
      documents: 8,
    },
    {
      id: 3,
      name: "提案書レビューフロー",
      description: "顧客向け提案書のレビュープロセス",
      steps: ["作成者", "営業主任", "営業部長"],
      status: "draft",
      documents: 0,
    },
  ];

  let isLoading = true;

  onMount(() => {
    setTimeout(() => {
      isLoading = false;
    }, 500);
  });

  const getStatusColor = (status: string) => {
    switch (status) {
      case "active":
        return "bg-green-100 text-green-800";
      case "draft":
        return "bg-yellow-100 text-yellow-800";
      case "inactive":
        return "bg-gray-100 text-gray-800";
      default:
        return "bg-gray-100 text-gray-800";
    }
  };

  const getStatusText = (status: string) => {
    switch (status) {
      case "active":
        return "有効";
      case "draft":
        return "下書き";
      case "inactive":
        return "無効";
      default:
        return "不明";
    }
  };
</script>

<svelte:head>
  <title>承認ワークフロー - 文書管理システム</title>
</svelte:head>

<div class="max-w-6xl mx-auto p-6">
  <div class="mb-6 flex justify-between items-center">
    <div>
      <h1 class="text-3xl font-bold text-gray-900">承認ワークフロー</h1>
      <p class="text-gray-600 mt-2">文書の承認プロセスを管理</p>
    </div>
    <button class="btn btn-primary"> 新規ワークフロー作成 </button>
  </div>

  {#if isLoading}
    <div class="bg-white rounded-lg shadow p-8 text-center">
      <div
        class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500 mx-auto"
      ></div>
      <p class="mt-4 text-gray-600">ワークフローを読み込み中...</p>
    </div>
  {:else}
    <div class="bg-white rounded-lg shadow overflow-hidden">
      <div class="p-6 border-b border-gray-200">
        <div class="flex items-center space-x-4">
          <div class="flex-1">
            <input
              type="text"
              placeholder="ワークフローを検索..."
              class="input w-full"
            />
          </div>
          <div>
            <select class="input">
              <option value="">すべてのステータス</option>
              <option value="active">有効</option>
              <option value="draft">下書き</option>
              <option value="inactive">無効</option>
            </select>
          </div>
        </div>
      </div>

      <div class="overflow-x-auto">
        <table class="min-w-full divide-y divide-gray-200">
          <thead class="bg-gray-50">
            <tr>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                ワークフロー名
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                承認ステップ
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                ステータス
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                使用文書数
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                操作
              </th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            {#each workflows as workflow}
              <tr class="hover:bg-gray-50">
                <td class="px-6 py-4 whitespace-nowrap">
                  <div>
                    <div class="text-sm font-medium text-gray-900">
                      {workflow.name}
                    </div>
                    <div class="text-sm text-gray-500">
                      {workflow.description}
                    </div>
                  </div>
                </td>
                <td class="px-6 py-4">
                  <div class="flex flex-wrap gap-1">
                    {#each workflow.steps as step, index}
                      <span
                        class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800"
                      >
                        {index + 1}. {step}
                      </span>
                      {#if index < workflow.steps.length - 1}
                        <span class="text-gray-400 mx-1">→</span>
                      {/if}
                    {/each}
                  </div>
                </td>
                <td class="px-6 py-4 whitespace-nowrap">
                  <span
                    class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {getStatusColor(
                      workflow.status,
                    )}"
                  >
                    {getStatusText(workflow.status)}
                  </span>
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                  {workflow.documents}件
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  <div class="flex space-x-2">
                    <button class="text-blue-600 hover:text-blue-800"
                      >編集</button
                    >
                    <button class="text-green-600 hover:text-green-800"
                      >複製</button
                    >
                    <button class="text-red-600 hover:text-red-800">削除</button
                    >
                  </div>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>

    <div class="mt-6 grid grid-cols-1 md:grid-cols-3 gap-6">
      <div class="bg-white rounded-lg shadow p-6">
        <h3 class="text-lg font-semibold text-gray-900 mb-4">
          ワークフロー統計
        </h3>
        <div class="space-y-3">
          <div class="flex justify-between">
            <span class="text-gray-600">有効なワークフロー:</span>
            <span class="font-medium"
              >{workflows.filter((w) => w.status === "active").length}件</span
            >
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">下書きワークフロー:</span>
            <span class="font-medium"
              >{workflows.filter((w) => w.status === "draft").length}件</span
            >
          </div>
          <div class="flex justify-between">
            <span class="text-gray-600">総文書数:</span>
            <span class="font-medium"
              >{workflows.reduce((sum, w) => sum + w.documents, 0)}件</span
            >
          </div>
        </div>
      </div>

      <div class="bg-white rounded-lg shadow p-6">
        <h3 class="text-lg font-semibold text-gray-900 mb-4">承認待ち</h3>
        <div class="space-y-2">
          <div class="text-2xl font-bold text-orange-600">15件</div>
          <p class="text-gray-600 text-sm">現在承認待ちの文書</p>
          <button class="text-blue-600 hover:text-blue-800 text-sm"
            >詳細を見る →</button
          >
        </div>
      </div>

      <div class="bg-white rounded-lg shadow p-6">
        <h3 class="text-lg font-semibold text-gray-900 mb-4">今月の承認完了</h3>
        <div class="space-y-2">
          <div class="text-2xl font-bold text-green-600">42件</div>
          <p class="text-gray-600 text-sm">今月承認完了した文書</p>
          <button class="text-blue-600 hover:text-blue-800 text-sm"
            >レポートを見る →</button
          >
        </div>
      </div>
    </div>
  {/if}
</div>
