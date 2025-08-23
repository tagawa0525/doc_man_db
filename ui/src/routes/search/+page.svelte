<script lang="ts">
  import { onMount } from "svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import SearchResults from "$lib/components/search/SearchResults.svelte";
  import SearchFilters from "$lib/components/search/SearchFilters.svelte";
  import SavedSearches from "$lib/components/search/SavedSearches.svelte";

  // API統合
  import {
    documents,
    totalDocuments,
    isLoadingDocuments,
    documentsError,
    searchDocuments,
    updateSearchFilters,
  } from "$lib/stores/documents.js";
  import { showError } from "$lib/stores/errors.js";
  import type { DocumentSearchFilters } from "$lib/api/queries/documents.js";

  // ローカル検索フィルター（UI用）
  let localFilters: DocumentSearchFilters = {
    title: "",
    documentTypeId: undefined,
    createdBy: undefined,
    createdDateFrom: undefined,
    createdDateTo: undefined,
    limit: 20,
    offset: 0,
  };

  let savedSearches: any[] = [];
  let showAdvancedFilters = false;
  let searchHistory: string[] = [];
  let searchTerm = "";

  onMount(() => {
    loadSavedSearches();
    loadSearchHistory();
  });

  // エラーハンドリング
  $: if ($documentsError) {
    showError($documentsError);
  }

  function loadSavedSearches() {
    const saved = localStorage.getItem("savedSearches");
    if (saved) {
      savedSearches = JSON.parse(saved);
    }
  }

  function loadSearchHistory() {
    const history = localStorage.getItem("searchHistory");
    if (history) {
      searchHistory = JSON.parse(history);
    }
  }

  function saveToHistory(query: string) {
    if (query && !searchHistory.includes(query)) {
      searchHistory = [query, ...searchHistory.slice(0, 9)]; // Keep last 10
      localStorage.setItem("searchHistory", JSON.stringify(searchHistory));
    }
  }

  async function handleSearch() {
    if (
      !localFilters.title &&
      !localFilters.documentTypeId &&
      !localFilters.createdBy
    ) {
      return;
    }

    try {
      if (localFilters.title) {
        saveToHistory(localFilters.title);
      }

      // ストアのフィルターを更新してGraphQL検索実行
      updateSearchFilters(localFilters);
      await searchDocuments();
    } catch (error: any) {
      console.error("Search failed:", error);
      showError(error.message || "検索中にエラーが発生しました");
    }
  }

  function handleLoadMore() {
    if ($documents && $documents.length > 0) {
      localFilters.offset =
        (localFilters.offset || 0) + (localFilters.limit || 20);
      updateSearchFilters(localFilters);
      searchDocuments();
    }
  }

  function handleFilterChange(event: CustomEvent) {
    localFilters = { ...localFilters, ...event.detail };
    localFilters.offset = 0; // Reset pagination
  }

  function saveCurrentSearch() {
    const searchName = prompt("検索条件に名前を付けてください:");
    if (searchName) {
      const savedSearch = {
        id: Date.now(),
        name: searchName,
        filters: { ...localFilters },
        createdAt: new Date().toISOString(),
      };
      savedSearches = [...savedSearches, savedSearch];
      localStorage.setItem("savedSearches", JSON.stringify(savedSearches));
    }
  }

  function loadSavedSearch(savedSearch: any) {
    localFilters = { ...savedSearch.filters };
    handleSearch();
  }

  function deleteSavedSearch(id: number) {
    savedSearches = savedSearches.filter((s) => s.id !== id);
    localStorage.setItem("savedSearches", JSON.stringify(savedSearches));
  }

  function clearSearch() {
    localFilters = {
      title: "",
      documentTypeId: undefined,
      createdBy: undefined,
      createdDateFrom: undefined,
      createdDateTo: undefined,
      limit: 20,
      offset: 0,
    };
  }
</script>

<svelte:head>
  <title>高度検索</title>
</svelte:head>

<div class="min-h-screen bg-gray-50">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <!-- Header -->
    <div class="mb-8">
      <h1 class="text-3xl font-bold text-gray-900">高度検索</h1>
      <p class="mt-2 text-gray-600">
        詳細な検索条件を指定して文書を検索できます。
      </p>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-4 gap-8">
      <!-- Sidebar -->
      <div class="lg:col-span-1 space-y-6">
        <!-- Saved Searches -->
        {#if savedSearches.length > 0}
          <SavedSearches
            searches={savedSearches}
            on:load={(event) => loadSavedSearch(event.detail)}
            on:delete={(event) => deleteSavedSearch(event.detail)}
          />
        {/if}

        <!-- Search History -->
        {#if searchHistory.length > 0}
          <div class="bg-white rounded-lg shadow p-4">
            <h3 class="text-lg font-medium text-gray-900 mb-3">検索履歴</h3>
            <div class="space-y-2">
              {#each searchHistory as query (query)}
                <button
                  type="button"
                  class="w-full text-left text-sm text-blue-600 hover:text-blue-800 truncate"
                  on:click={() => {
                    localFilters.title = query;
                    handleSearch();
                  }}
                >
                  {query}
                </button>
              {/each}
            </div>
          </div>
        {/if}
      </div>

      <!-- Main Content -->
      <div class="lg:col-span-3 space-y-6">
        <!-- Search Form -->
        <div class="bg-white shadow rounded-lg p-6">
          <div class="flex justify-between items-center mb-6">
            <h2 class="text-lg font-medium text-gray-900">検索条件</h2>
            <div class="space-x-2">
              <button
                type="button"
                on:click={saveCurrentSearch}
                class="text-blue-600 hover:text-blue-700 text-sm font-medium"
              >
                検索条件を保存
              </button>
              <button
                type="button"
                on:click={clearSearch}
                class="text-gray-600 hover:text-gray-700 text-sm font-medium"
              >
                クリア
              </button>
            </div>
          </div>

          <form on:submit|preventDefault={handleSearch} class="space-y-6">
            <!-- Basic Search -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label
                  for="search-title"
                  class="block text-sm font-medium text-gray-700 mb-1"
                >
                  文書タイトル
                </label>
                <Input
                  id="search-title"
                  bind:value={localFilters.title}
                  placeholder="タイトルで検索..."
                />
              </div>

              <div>
                <label
                  for="search-content"
                  class="block text-sm font-medium text-gray-700 mb-1"
                >
                  作成者ID
                </label>
                <Input
                  id="search-content"
                  bind:value={localFilters.createdBy}
                  placeholder="作成者IDで検索..."
                />
              </div>
            </div>

            <!-- Advanced Filters Toggle -->
            <div>
              <button
                type="button"
                class="flex items-center text-sm font-medium text-blue-600 hover:text-blue-700"
                on:click={() => (showAdvancedFilters = !showAdvancedFilters)}
              >
                <span>詳細フィルタ</span>
                <svg
                  class="ml-1 w-4 h-4 transform transition-transform {showAdvancedFilters
                    ? 'rotate-180'
                    : ''}"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M19 9l-7 7-7-7"
                  />
                </svg>
              </button>
            </div>

            <!-- Advanced Filters -->
            {#if showAdvancedFilters}
              <SearchFilters
                bind:filters={localFilters}
                on:change={handleFilterChange}
              />
            {/if}

            <!-- Search Button -->
            <div class="flex justify-between items-center">
              <div class="text-sm text-gray-500">
                {#if $totalDocuments > 0}
                  {$totalDocuments}件の文書が見つかりました
                {/if}
              </div>

              <Button
                type="submit"
                variant="primary"
                disabled={$isLoadingDocuments}
              >
                {#if $isLoadingDocuments}
                  <div class="flex items-center">
                    <div
                      class="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"
                    ></div>
                    検索中...
                  </div>
                {:else}
                  検索実行
                {/if}
              </Button>
            </div>
          </form>
        </div>

        <!-- Search Results -->
        {#if $documents && $documents.length > 0}
          <SearchResults
            results={{
              documents: $documents,
              total: $totalDocuments,
              took_ms: 0, // TODO: GraphQL response時間を追加
            }}
            on:loadMore={() => {
              localFilters.offset =
                (localFilters.offset || 0) + (localFilters.limit || 20);
              updateSearchFilters(localFilters);
              searchDocuments();
            }}
            hasMore={$documents.length < $totalDocuments}
            loading={$isLoadingDocuments}
          />
        {:else if $isLoadingDocuments}
          <div class="bg-white shadow rounded-lg p-8">
            <div class="text-center">
              <div
                class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto mb-4"
              ></div>
              <p class="text-gray-500">検索中...</p>
            </div>
          </div>
        {:else}
          <div class="bg-white shadow rounded-lg p-8">
            <div class="text-center text-gray-500">
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
                  d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                />
              </svg>
              <p class="text-lg font-medium text-gray-900 mb-1">
                検索結果がありません
              </p>
              <p class="text-sm text-gray-500">
                検索条件を変更して再度お試しください。
              </p>
            </div>
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>
