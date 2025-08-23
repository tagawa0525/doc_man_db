<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import Button from "$lib/components/ui/Button.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Select from "$lib/components/ui/Select.svelte";
  import ResponsiveTable from "$lib/components/mobile/ResponsiveTable.svelte";

  // API統合
  import {
    documents,
    totalDocuments,
    isLoadingDocuments,
    documentsError,
    paginationInfo,
    searchDocuments,
    updateSearchFilters,
    setPage,
    initializeDocuments,
  } from "$lib/stores/documents.js";
  import { showError } from "$lib/stores/errors.js";

  // ローカル検索フィルター（UI用）
  let localFilters = {
    title: "",
    documentTypeId: "",
    createdDateFrom: "",
    createdDateTo: "",
  };

  // 状態管理
  let showAdvancedFilters = false;
  let debounceTimer: ReturnType<typeof setTimeout>;

  // 文書タイプ選択肢（仮データ、後で実APIから取得）
  const documentTypeOptions = [
    { value: "", label: "すべて" },
    { value: "1", label: "技術文書" },
    { value: "2", label: "計画書" },
    { value: "3", label: "レポート" },
    { value: "4", label: "提案書" },
    { value: "5", label: "手順書" },
  ];

  // テーブルヘッダー定義
  const headers = [
    { key: "id", label: "ID", sortable: true, mobileHidden: true },
    { key: "title", label: "文書名", sortable: true },
    {
      key: "documentTypeId",
      label: "種別",
      sortable: true,
      mobileHidden: true,
    },
    { key: "createdBy", label: "作成者", sortable: false, mobileHidden: true },
    { key: "createdDate", label: "作成日", sortable: true },
    { key: "actions", label: "操作", sortable: false, class: "w-32" },
  ];

  // 検索実行（デバウンス付き）
  function performSearch() {
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(async () => {
      // ローカルフィルターをストアに反映
      updateSearchFilters({
        title: localFilters.title || undefined,
        documentTypeId: localFilters.documentTypeId
          ? parseInt(localFilters.documentTypeId)
          : undefined,
        createdDateFrom: localFilters.createdDateFrom || undefined,
        createdDateTo: localFilters.createdDateTo || undefined,
        offset: 0, // 検索時は最初のページに戻る
      });

      await searchDocuments();
    }, 500);
  }

  // 高速検索（Enterキー）
  function handleQuickSearch() {
    clearTimeout(debounceTimer);
    performSearch();
  }

  // クリア
  function clearFilters() {
    localFilters = {
      title: "",
      documentTypeId: "",
      createdDateFrom: "",
      createdDateTo: "",
    };
    performSearch();
  }

  // ページ変更
  function handlePageChange(page: number) {
    setPage(page);
  }

  // 行クリック（詳細ページへ）
  function handleRowClick(document: any) {
    goto(`/documents/${document.id}`);
  }

  // ソート処理
  function handleSort(key: string, direction: string) {
    // TODO: バックエンドでソート実装後に対応
    console.log("Sort:", key, direction);
  }

  // 文書作成ページへ
  function goToCreateDocument() {
    goto("/documents/new");
  }

  // 初期化
  onMount(() => {
    initializeDocuments();
  });

  // エラーハンドリング
  $: if ($documentsError) {
    showError($documentsError);
  }
</script>

<div class="space-y-6">
  <!-- ページヘッダー -->
  <div class="sm:flex sm:items-center sm:justify-between">
    <div>
      <h1 class="text-2xl font-bold text-gray-900">文書管理</h1>
      <p class="mt-2 text-sm text-gray-700">組織の文書を検索・管理できます</p>
    </div>
    <div class="mt-4 sm:mt-0">
      <Button on:click={goToCreateDocument} variant="primary">
        新規文書作成
      </Button>
    </div>
  </div>

  <!-- 検索フィルター -->
  <div class="bg-white shadow rounded-lg">
    <div class="px-6 py-4 border-b border-gray-200">
      <div class="flex items-center justify-between">
        <h2 class="text-lg font-medium text-gray-900">文書検索</h2>
        <button
          type="button"
          class="text-sm text-blue-600 hover:text-blue-900"
          on:click={() => (showAdvancedFilters = !showAdvancedFilters)}
        >
          {showAdvancedFilters ? "詳細検索を閉じる" : "詳細検索"}
        </button>
      </div>
    </div>

    <div class="p-6 space-y-4">
      <!-- 基本検索 -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div>
          <Input
            label="文書名"
            bind:value={localFilters.title}
            placeholder="文書名で検索..."
            on:input={performSearch}
            on:keydown={(e) =>
              e.detail && e.detail.key === "Enter" && handleQuickSearch()}
          />
        </div>

        <div>
          <Select
            label="文書種別"
            bind:value={localFilters.documentTypeId}
            options={documentTypeOptions}
            on:change={performSearch}
          />
        </div>

        <div class="flex items-end space-x-2">
          <Button on:click={handleQuickSearch} variant="primary">検索</Button>
          <Button on:click={clearFilters} variant="secondary">クリア</Button>
        </div>
      </div>

      <!-- 詳細検索 -->
      {#if showAdvancedFilters}
        <div class="pt-4 border-t border-gray-200">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <Input
                label="作成日（開始）"
                type="date"
                bind:value={localFilters.createdDateFrom}
                on:change={performSearch}
              />
            </div>
            <div>
              <Input
                label="作成日（終了）"
                type="date"
                bind:value={localFilters.createdDateTo}
                on:change={performSearch}
              />
            </div>
          </div>
        </div>
      {/if}
    </div>
  </div>

  <!-- 検索結果 -->
  <div class="bg-white shadow rounded-lg">
    <div class="px-6 py-4 border-b border-gray-200">
      <div class="flex items-center justify-between">
        <h2 class="text-lg font-medium text-gray-900">
          検索結果
          {#if $totalDocuments > 0}
            <span class="text-sm text-gray-500">（{$totalDocuments}件）</span>
          {/if}
        </h2>

        {#if $isLoadingDocuments}
          <div class="text-sm text-gray-500">検索中...</div>
        {/if}
      </div>
    </div>

    <div class="p-6">
      {#if $isLoadingDocuments}
        <!-- ローディング表示 -->
        <div class="text-center py-12">
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
                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l-3-2.647z"
              ></path>
            </svg>
            文書を検索中...
          </div>
        </div>
      {:else}
        <!-- テーブル表示 -->
        <ResponsiveTable
          {headers}
          data={$documents}
          onRowClick={handleRowClick}
          onSort={handleSort}
        >
          <svelte:fragment slot="cell" let:item let:header>
            {#if header.key === "documentTypeId"}
              {documentTypeOptions.find(
                (opt) => opt.value === item.documentTypeId.toString(),
              )?.label || "不明"}
            {:else if header.key === "actions"}
              <div class="flex space-x-2">
                <button
                  type="button"
                  class="text-blue-600 hover:text-blue-900 text-sm"
                  on:click|stopPropagation={() =>
                    goto(`/documents/${item.id}/edit`)}
                >
                  編集
                </button>
              </div>
            {:else}
              {item[header.key] || "-"}
            {/if}
          </svelte:fragment>

          <svelte:fragment slot="mobile-card" let:item>
            <div class="space-y-2">
              <div class="font-medium text-gray-900">{item.title}</div>
              <div class="text-sm text-gray-500">
                ID: {item.id} | 種別: {documentTypeOptions.find(
                  (opt) => opt.value === item.documentTypeId.toString(),
                )?.label || "不明"}
              </div>
              <div class="text-sm text-gray-500">
                作成日: {item.createdDate}
              </div>
              <div class="flex justify-end space-x-2">
                <button
                  type="button"
                  class="text-blue-600 hover:text-blue-900 text-sm"
                  on:click|stopPropagation={() =>
                    goto(`/documents/${item.id}/edit`)}
                >
                  編集
                </button>
              </div>
            </div>
          </svelte:fragment>

          <svelte:fragment slot="empty">
            <div class="text-center py-12">
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
                  d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                />
              </svg>
              <h3 class="mt-2 text-sm font-medium text-gray-900">
                文書が見つかりません
              </h3>
              <p class="mt-1 text-sm text-gray-500">
                検索条件を変更するか、新しい文書を作成してください。
              </p>
              <div class="mt-6">
                <Button on:click={goToCreateDocument} variant="primary">
                  新規文書作成
                </Button>
              </div>
            </div>
          </svelte:fragment>
        </ResponsiveTable>
      {/if}
    </div>

    <!-- ページング -->
    {#if $totalDocuments > 0 && !$isLoadingDocuments}
      <div class="px-6 py-3 border-t border-gray-200 bg-gray-50">
        <div class="flex items-center justify-between">
          <div class="text-sm text-gray-700">
            {$paginationInfo.total}件中
            {Math.min(
              ($paginationInfo.currentPage - 1) * $paginationInfo.pageSize + 1,
              $paginationInfo.total,
            )}〜
            {Math.min(
              $paginationInfo.currentPage * $paginationInfo.pageSize,
              $paginationInfo.total,
            )}件を表示
          </div>

          <div class="flex items-center space-x-2">
            <Button
              on:click={() => handlePageChange($paginationInfo.currentPage - 1)}
              disabled={!$paginationInfo.hasPrevPage}
              variant="secondary"
              size="sm"
            >
              前へ
            </Button>

            <span class="text-sm text-gray-700">
              {$paginationInfo.currentPage} / {$paginationInfo.totalPages}
            </span>

            <Button
              on:click={() => handlePageChange($paginationInfo.currentPage + 1)}
              disabled={!$paginationInfo.hasNextPage}
              variant="secondary"
              size="sm"
            >
              次へ
            </Button>
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  /* カスタムスタイル（必要に応じて） */
</style>
