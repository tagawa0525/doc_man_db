<script lang="ts">
  import { onMount } from "svelte";

  let fileCheckResults = [
    {
      documentId: "CTA-2508001",
      fileName: "技術仕様書_v1.0.pdf",
      expectedPath: "\\\\server\\documents\\2024\\tech\\CTA-2508001.pdf",
      status: "found",
      lastChecked: "2024-08-22T10:30:00Z",
      fileSize: "2.4 MB",
    },
    {
      documentId: "CTA-2508002",
      fileName: "設計書_DB設計.pdf",
      expectedPath: "\\\\server\\documents\\2024\\design\\CTA-2508002.pdf",
      status: "not_found",
      lastChecked: "2024-08-22T10:30:05Z",
      fileSize: null,
    },
    {
      documentId: "CTA-2508003",
      fileName: "プロジェクト計画書.xlsx",
      expectedPath: "\\\\server\\documents\\2024\\project\\CTA-2508003.xlsx",
      status: "found",
      lastChecked: "2024-08-22T10:30:10Z",
      fileSize: "1.2 MB",
    },
  ];

  let summary = {
    totalFiles: 1250,
    foundFiles: 1198,
    missingFiles: 52,
    lastCheckDate: "2024-08-22T10:00:00Z",
  };

  let isLoading = true;
  let isRunningCheck = false;

  onMount(() => {
    setTimeout(() => {
      isLoading = false;
    }, 500);
  });

  const getStatusColor = (status: string) => {
    switch (status) {
      case "found":
        return "bg-green-100 text-green-800";
      case "not_found":
        return "bg-red-100 text-red-800";
      case "checking":
        return "bg-yellow-100 text-yellow-800";
      default:
        return "bg-gray-100 text-gray-800";
    }
  };

  const getStatusText = (status: string) => {
    switch (status) {
      case "found":
        return "ファイル存在";
      case "not_found":
        return "ファイル不存在";
      case "checking":
        return "チェック中";
      default:
        return "不明";
    }
  };

  const startFileCheck = async () => {
    isRunningCheck = true;
    // TODO: 実際のファイルチェック処理
    setTimeout(() => {
      isRunningCheck = false;
      summary.lastCheckDate = new Date().toISOString();
    }, 3000);
  };

  const formatDate = (dateString: string) => {
    return new Date(dateString).toLocaleString("ja-JP");
  };
</script>

<svelte:head>
  <title>ファイル存在チェック - 文書管理システム</title>
</svelte:head>

<div class="max-w-6xl mx-auto p-6">
  <div class="mb-6 flex justify-between items-center">
    <div>
      <h1 class="text-3xl font-bold text-gray-900">ファイル存在チェック</h1>
      <p class="text-gray-600 mt-2">文書ファイルの存在確認と整合性チェック</p>
    </div>
    <button
      on:click={startFileCheck}
      disabled={isRunningCheck}
      class="btn btn-primary disabled:opacity-50"
    >
      {#if isRunningCheck}
        <div
          class="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"
        ></div>
        チェック中...
      {:else}
        ファイルチェック実行
      {/if}
    </button>
  </div>

  {#if isLoading}
    <div class="bg-white rounded-lg shadow p-8 text-center">
      <div
        class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500 mx-auto"
      ></div>
      <p class="mt-4 text-gray-600">ファイルチェック結果を読み込み中...</p>
    </div>
  {:else}
    <!-- サマリーカード -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-6 mb-6">
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
              {summary.totalFiles.toLocaleString()}
            </div>
            <div class="text-sm text-gray-500">総ファイル数</div>
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
                d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
              ></path>
            </svg>
          </div>
          <div class="ml-4">
            <div class="text-2xl font-bold text-green-600">
              {summary.foundFiles.toLocaleString()}
            </div>
            <div class="text-sm text-gray-500">存在ファイル数</div>
          </div>
        </div>
      </div>

      <div class="bg-white rounded-lg shadow p-6">
        <div class="flex items-center">
          <div class="p-2 bg-red-100 rounded-lg">
            <svg
              class="w-6 h-6 text-red-600"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"
              ></path>
            </svg>
          </div>
          <div class="ml-4">
            <div class="text-2xl font-bold text-red-600">
              {summary.missingFiles}
            </div>
            <div class="text-sm text-gray-500">不存在ファイル数</div>
          </div>
        </div>
      </div>

      <div class="bg-white rounded-lg shadow p-6">
        <div class="flex items-center">
          <div class="p-2 bg-yellow-100 rounded-lg">
            <svg
              class="w-6 h-6 text-yellow-600"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
              ></path>
            </svg>
          </div>
          <div class="ml-4">
            <div class="text-sm font-medium text-gray-900">最終チェック</div>
            <div class="text-sm text-gray-500">
              {formatDate(summary.lastCheckDate)}
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- チェック結果テーブル -->
    <div class="bg-white rounded-lg shadow overflow-hidden">
      <div class="p-6 border-b border-gray-200">
        <div class="flex items-center space-x-4">
          <div class="flex-1">
            <input
              type="text"
              placeholder="文書IDやファイル名で検索..."
              class="input w-full"
            />
          </div>
          <div>
            <select class="input">
              <option value="">すべてのステータス</option>
              <option value="found">存在</option>
              <option value="not_found">不存在</option>
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
                文書ID
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                ファイル名
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                ステータス
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                ファイルサイズ
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                最終チェック
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                操作
              </th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            {#each fileCheckResults as result}
              <tr class="hover:bg-gray-50">
                <td
                  class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900"
                >
                  {result.documentId}
                </td>
                <td class="px-6 py-4 whitespace-nowrap">
                  <div>
                    <div class="text-sm font-medium text-gray-900">
                      {result.fileName}
                    </div>
                    <div
                      class="text-sm text-gray-500 truncate"
                      style="max-width: 300px;"
                    >
                      {result.expectedPath}
                    </div>
                  </div>
                </td>
                <td class="px-6 py-4 whitespace-nowrap">
                  <span
                    class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {getStatusColor(
                      result.status,
                    )}"
                  >
                    {getStatusText(result.status)}
                  </span>
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                  {result.fileSize || "-"}
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  {formatDate(result.lastChecked)}
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                  <div class="flex space-x-2">
                    <button class="text-blue-600 hover:text-blue-800"
                      >再チェック</button
                    >
                    {#if result.status === "not_found"}
                      <button class="text-green-600 hover:text-green-800"
                        >パス修正</button
                      >
                    {/if}
                  </div>
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </div>

    <div class="mt-6 bg-yellow-50 border border-yellow-200 rounded-lg p-4">
      <div class="flex">
        <svg
          class="w-5 h-5 text-yellow-400 mt-0.5 mr-3"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"
          ></path>
        </svg>
        <div>
          <h3 class="text-sm font-medium text-yellow-800">
            ファイルチェックについて
          </h3>
          <p class="text-sm text-yellow-700 mt-1">
            ファイルチェックは毎月第1営業日の深夜に自動実行されます。不存在ファイルが検出された場合は、担当者に通知が送信されます。
          </p>
        </div>
      </div>
    </div>
  {/if}
</div>
