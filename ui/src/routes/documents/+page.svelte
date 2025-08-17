<script lang="ts">
  import { onMount } from "svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Select from "$lib/components/ui/Select.svelte";

  // 検索フィルター
  let searchFilters = {
    title: "",
    businessNumber: "",
    documentTypeId: "",
    createdDateFrom: "",
    createdDateTo: "",
    confidentiality: {
      internalExternal: "",
      importanceClass: "",
      personalInfo: "",
    },
  };

  // ページング
  let currentPage = 1;
  const pageSize = 20;
  let totalCount = 0;

  // 状態管理
  let isLoading = false;
  let isSearching = false;
  let showAdvancedFilters = false;

  // 仮の文書データ
  let documents = [
    {
      id: 1,
      number: "CTA-2508001",
      title: "システム設計書 v2.1",
      documentType: "技術文書",
      businessNumber: "PJ2024-001",
      createdDate: "2024-08-15",
      createdBy: "山田太郎",
      department: "情報システム部",
      confidentiality: {
        internalExternal: "internal",
        importanceClass: "class1",
        personalInfo: "none",
      },
      networkPath: "\\\\server\\documents\\2024\\08\\CTA-2508001",
      hasFile: true,
      hasApproval: true,
    },
    {
      id: 2,
      number: "技術-25001",
      title: "データベース移行計画書",
      documentType: "計画書",
      businessNumber: "PJ2024-002",
      createdDate: "2024-08-10",
      createdBy: "佐藤花子",
      department: "システム開発部",
      confidentiality: {
        internalExternal: "internal",
        importanceClass: "class2",
        personalInfo: "none",
      },
      networkPath: "\\\\server\\documents\\2024\\08\\技術-25001",
      hasFile: true,
      hasApproval: false,
    },
    {
      id: 3,
      number: "REP-2508001",
      title: "月次運用レポート 2024年7月",
      documentType: "レポート",
      businessNumber: "",
      createdDate: "2024-08-01",
      createdBy: "田中一郎",
      department: "運用管理部",
      confidentiality: {
        internalExternal: "internal",
        importanceClass: "class2",
        personalInfo: "none",
      },
      networkPath: "\\\\server\\documents\\2024\\08\\REP-2508001",
      hasFile: false,
      hasApproval: true,
    },
  ];

  // 文書種別オプション
  const documentTypeOptions = [
    { value: "", label: "全て" },
    { value: "technical", label: "技術文書" },
    { value: "plan", label: "計画書" },
    { value: "report", label: "レポート" },
    { value: "manual", label: "マニュアル" },
    { value: "specification", label: "仕様書" },
  ];

  // 機密レベルオプション
  const internalExternalOptions = [
    { value: "", label: "全て" },
    { value: "internal", label: "社内" },
    { value: "external", label: "社外" },
  ];

  const importanceClassOptions = [
    { value: "", label: "全て" },
    { value: "class1", label: "情報クラスⅠ（重要）" },
    { value: "class2", label: "情報クラスⅡ（通常）" },
  ];

  const personalInfoOptions = [
    { value: "", label: "全て" },
    { value: "none", label: "なし" },
    { value: "present", label: "あり" },
  ];

  // 検索実行
  async function handleSearch() {
    isSearching = true;

    try {
      // TODO: 実際のAPI呼び出しに置き換え
      await new Promise((resolve) => setTimeout(resolve, 1000));

      // フィルタリング処理（仮実装）
      let filteredDocs = documents;
      if (searchFilters.title) {
        filteredDocs = filteredDocs.filter((doc) =>
          doc.title.toLowerCase().includes(searchFilters.title.toLowerCase()),
        );
      }
      if (searchFilters.businessNumber) {
        filteredDocs = filteredDocs.filter((doc) =>
          doc.businessNumber.includes(searchFilters.businessNumber),
        );
      }

      totalCount = filteredDocs.length;
      documents = filteredDocs;
      currentPage = 1;
    } catch (error) {
      console.error("Search failed:", error);
    } finally {
      isSearching = false;
    }
  }

  // フィルタークリア
  function clearFilters() {
    searchFilters = {
      title: "",
      businessNumber: "",
      documentTypeId: "",
      createdDateFrom: "",
      createdDateTo: "",
      confidentiality: {
        internalExternal: "",
        importanceClass: "",
        personalInfo: "",
      },
    };
    handleSearch();
  }

  // ページ変更
  function handlePageChange(page: number) {
    currentPage = page;
    // TODO: ページングされたデータの取得
  }

  // 文書詳細表示
  function viewDocument(documentId: number) {
    window.location.href = `/documents/${documentId}`;
  }

  // 初期読み込み
  onMount(() => {
    handleSearch();
  });

  // ページ数計算
  $: totalPages = Math.ceil(totalCount / pageSize);
  $: startIndex = (currentPage - 1) * pageSize + 1;
  $: endIndex = Math.min(currentPage * pageSize, totalCount);

  // 機密レベル表示
  function getConfidentialityLabel(confidentiality: any): string {
    const parts = [];
    if (confidentiality.internalExternal === "external") parts.push("社外");
    if (confidentiality.importanceClass === "class1") parts.push("重要");
    if (confidentiality.personalInfo === "present") parts.push("個人情報");
    return parts.length > 0 ? parts.join("・") : "通常";
  }

  function getConfidentialityColor(confidentiality: any): string {
    if (
      confidentiality.importanceClass === "class1" ||
      confidentiality.personalInfo === "present"
    ) {
      return "bg-red-100 text-red-800";
    }
    if (confidentiality.internalExternal === "external") {
      return "bg-yellow-100 text-yellow-800";
    }
    return "bg-gray-100 text-gray-800";
  }
</script>

<div class="space-y-6">
  <!-- ページヘッダー -->
  <div class="md:flex md:items-center md:justify-between">
    <div class="min-w-0 flex-1">
      <h1
        class="text-2xl font-bold leading-7 text-gray-900 sm:truncate sm:text-3xl sm:tracking-tight"
      >
        文書検索
      </h1>
      <p class="mt-1 text-sm text-gray-500">登録済み文書の検索と閲覧</p>
    </div>
    <div class="mt-4 flex md:ml-4 md:mt-0">
      <Button variant="primary" size="sm">
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
        新規文書作成
      </Button>
    </div>
  </div>

  <!-- 検索フォーム -->
  <div class="bg-white shadow rounded-lg p-6">
    <form on:submit|preventDefault={handleSearch}>
      <!-- 基本検索 -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 mb-4">
        <div>
          <label
            for="title"
            class="block text-sm font-medium text-gray-700 mb-1"
          >
            文書タイトル
          </label>
          <Input
            id="title"
            bind:value={searchFilters.title}
            placeholder="タイトルで検索..."
          />
        </div>

        <div>
          <label
            for="businessNumber"
            class="block text-sm font-medium text-gray-700 mb-1"
          >
            業務番号
          </label>
          <Input
            id="businessNumber"
            bind:value={searchFilters.businessNumber}
            placeholder="業務番号で検索..."
          />
        </div>

        <div>
          <label
            for="documentType"
            class="block text-sm font-medium text-gray-700 mb-1"
          >
            文書種別
          </label>
          <Select
            id="documentType"
            bind:value={searchFilters.documentTypeId}
            options={documentTypeOptions}
            placeholder="文書種別を選択..."
          />
        </div>
      </div>

      <!-- 詳細検索 -->
      {#if showAdvancedFilters}
        <div class="border-t border-gray-200 pt-4">
          <h3 class="text-lg font-medium text-gray-900 mb-4">詳細検索</h3>

          <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
            <div>
              <label
                for="dateFrom"
                class="block text-sm font-medium text-gray-700 mb-1"
              >
                作成日（開始）
              </label>
              <Input
                id="dateFrom"
                type="date"
                bind:value={searchFilters.createdDateFrom}
              />
            </div>

            <div>
              <label
                for="dateTo"
                class="block text-sm font-medium text-gray-700 mb-1"
              >
                作成日（終了）
              </label>
              <Input
                id="dateTo"
                type="date"
                bind:value={searchFilters.createdDateTo}
              />
            </div>
          </div>

          <!-- 機密レベル -->
          <div class="mb-4">
            <h4 class="text-sm font-medium text-gray-700 mb-2">機密レベル</h4>
            <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
              <div>
                <label
                  for="internalExternal"
                  class="block text-xs font-medium text-gray-600 mb-1"
                  >社内外区分</label
                >
                <Select
                  id="internalExternal"
                  bind:value={searchFilters.confidentiality.internalExternal}
                  options={internalExternalOptions}
                  placeholder="社内外区分"
                />
              </div>
              <div>
                <label
                  for="importanceClass"
                  class="block text-xs font-medium text-gray-600 mb-1"
                  >重要度</label
                >
                <Select
                  id="importanceClass"
                  bind:value={searchFilters.confidentiality.importanceClass}
                  options={importanceClassOptions}
                  placeholder="重要度"
                />
              </div>
              <div>
                <label
                  for="personalInfo"
                  class="block text-xs font-medium text-gray-600 mb-1"
                  >個人情報</label
                >
                <Select
                  id="personalInfo"
                  bind:value={searchFilters.confidentiality.personalInfo}
                  options={personalInfoOptions}
                  placeholder="個人情報"
                />
              </div>
            </div>
          </div>
        </div>
      {/if}

      <!-- 検索ボタン -->
      <div class="flex justify-between items-center pt-4">
        <button
          type="button"
          on:click={() => (showAdvancedFilters = !showAdvancedFilters)}
          class="text-blue-600 hover:text-blue-700 text-sm font-medium"
        >
          {showAdvancedFilters ? "詳細検索を閉じる" : "詳細検索"}
        </button>

        <div class="space-x-2">
          <Button type="button" variant="secondary" on:click={clearFilters}>
            クリア
          </Button>
          <Button type="submit" loading={isSearching}>検索</Button>
        </div>
      </div>
    </form>
  </div>

  <!-- 検索結果 -->
  <div class="bg-white shadow rounded-lg">
    <!-- 結果ヘッダー -->
    <div class="px-6 py-4 border-b border-gray-200">
      <div class="flex justify-between items-center">
        <div>
          <h3 class="text-lg font-medium text-gray-900">検索結果</h3>
          {#if totalCount > 0}
            <p class="text-sm text-gray-500">
              {totalCount}件中 {startIndex}-{endIndex}件を表示
            </p>
          {/if}
        </div>

        {#if totalCount > 0}
          <div class="flex items-center space-x-2">
            <label for="sort" class="text-sm font-medium text-gray-700"
              >並び順:</label
            >
            <select
              id="sort"
              class="text-sm border-gray-300 rounded-md focus:ring-blue-500 focus:border-blue-500"
            >
              <option value="created_desc">作成日（新しい順）</option>
              <option value="created_asc">作成日（古い順）</option>
              <option value="title_asc">タイトル（昇順）</option>
              <option value="number_asc">文書番号（昇順）</option>
            </select>
          </div>
        {/if}
      </div>
    </div>

    <!-- 結果テーブル -->
    {#if isSearching}
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
          検索中...
        </div>
      </div>
    {:else if documents.length === 0}
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
            d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
          />
        </svg>
        <h3 class="mt-2 text-sm font-medium text-gray-900">検索結果なし</h3>
        <p class="mt-1 text-sm text-gray-500">
          検索条件に一致する文書が見つかりませんでした。
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
                文書番号・タイトル
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                文書種別
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                作成者・部署
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                作成日
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                機密レベル
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                ファイル状況
              </th>
              <th
                class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
              >
                操作
              </th>
            </tr>
          </thead>
          <tbody class="bg-white divide-y divide-gray-200">
            {#each documents as document}
              <tr class="hover:bg-gray-50">
                <td class="px-6 py-4">
                  <div>
                    <div class="text-sm font-medium text-gray-900">
                      {document.number}
                    </div>
                    <div class="text-sm text-gray-500">{document.title}</div>
                    {#if document.businessNumber}
                      <div class="text-xs text-blue-600">
                        業務番号: {document.businessNumber}
                      </div>
                    {/if}
                  </div>
                </td>
                <td class="px-6 py-4 whitespace-nowrap">
                  <span
                    class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800"
                  >
                    {document.documentType}
                  </span>
                </td>
                <td class="px-6 py-4">
                  <div class="text-sm text-gray-900">{document.createdBy}</div>
                  <div class="text-sm text-gray-500">{document.department}</div>
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                  {document.createdDate}
                </td>
                <td class="px-6 py-4 whitespace-nowrap">
                  <span
                    class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {getConfidentialityColor(
                      document.confidentiality,
                    )}"
                  >
                    {getConfidentialityLabel(document.confidentiality)}
                  </span>
                </td>
                <td class="px-6 py-4">
                  <div class="flex items-center space-x-2">
                    <div class="flex items-center">
                      <div
                        class="h-2 w-2 rounded-full {document.hasFile
                          ? 'bg-green-400'
                          : 'bg-red-400'} mr-1"
                      ></div>
                      <span class="text-xs text-gray-600">ファイル</span>
                    </div>
                    <div class="flex items-center">
                      <div
                        class="h-2 w-2 rounded-full {document.hasApproval
                          ? 'bg-green-400'
                          : 'bg-red-400'} mr-1"
                      ></div>
                      <span class="text-xs text-gray-600">承認書</span>
                    </div>
                  </div>
                </td>
                <td class="px-6 py-4 whitespace-nowrap text-sm font-medium">
                  <button
                    type="button"
                    class="text-blue-600 hover:text-blue-900"
                    on:click={() => viewDocument(document.id)}
                  >
                    詳細
                  </button>
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
