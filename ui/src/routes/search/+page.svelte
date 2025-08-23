<script lang="ts">
  import { onMount } from "svelte";
  import { writable } from "svelte/store";
  import Button from "$lib/components/ui/Button.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Select from "$lib/components/ui/Select.svelte";
  import SearchResults from "$lib/components/search/SearchResults.svelte";
  import SearchFilters from "$lib/components/search/SearchFilters.svelte";
  import SavedSearches from "$lib/components/search/SavedSearches.svelte";

  interface SearchFilters {
    title?: string;
    document_type_id?: number;
    created_by?: number;
    created_date_from?: string;
    created_date_to?: string;
    department_id?: number;
    business_id?: number;
    content?: string;
    file_exists?: boolean;
    confidentiality_level?: string;
    limit: number;
    offset: number;
  }

  interface SearchResult {
    documents: any[];
    total: number;
    took_ms: number;
  }

  let filters: SearchFilters = {
    limit: 20,
    offset: 0,
  };

  let searchResults: SearchResult | null = null;
  let isSearching = false;
  let savedSearches: any[] = [];
  let showAdvancedFilters = false;
  let searchHistory: string[] = [];

  onMount(() => {
    loadSavedSearches();
    loadSearchHistory();
  });

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
    if (!filters.title && !filters.content && !filters.document_type_id) {
      return;
    }

    try {
      isSearching = true;

      if (filters.title) {
        saveToHistory(filters.title);
      }

      const queryParams = new URLSearchParams();
      Object.entries(filters).forEach(([key, value]) => {
        if (value !== undefined && value !== null && value !== "") {
          queryParams.append(key, value.toString());
        }
      });

      const response = await fetch(`/api/documents/search?${queryParams}`);
      if (response.ok) {
        const data = await response.json();
        searchResults = {
          documents: data.documents || [],
          total: data.total || 0,
          took_ms: data.took_ms || 0,
        };
      } else {
        console.error("Search failed");
      }
    } catch (error) {
      console.error("Search error:", error);
    } finally {
      isSearching = false;
    }
  }

  function handleLoadMore() {
    if (searchResults) {
      filters.offset += filters.limit;
      handleSearch();
    }
  }

  function handleFilterChange(event: CustomEvent) {
    filters = { ...filters, ...event.detail };
    filters.offset = 0; // Reset pagination
  }

  function saveCurrentSearch() {
    const searchName = prompt("検索条件に名前を付けてください:");
    if (searchName) {
      const savedSearch = {
        id: Date.now(),
        name: searchName,
        filters: { ...filters },
        createdAt: new Date().toISOString(),
      };
      savedSearches = [...savedSearches, savedSearch];
      localStorage.setItem("savedSearches", JSON.stringify(savedSearches));
    }
  }

  function loadSavedSearch(savedSearch: any) {
    filters = { ...savedSearch.filters };
    handleSearch();
  }

  function deleteSavedSearch(id: number) {
    savedSearches = savedSearches.filter((s) => s.id !== id);
    localStorage.setItem("savedSearches", JSON.stringify(savedSearches));
  }

  function clearSearch() {
    filters = { limit: 20, offset: 0 };
    searchResults = null;
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
                    filters.title = query;
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
                  bind:value={filters.title}
                  placeholder="タイトルで検索..."
                />
              </div>

              <div>
                <label
                  for="search-content"
                  class="block text-sm font-medium text-gray-700 mb-1"
                >
                  文書内容
                </label>
                <Input
                  id="search-content"
                  bind:value={filters.content}
                  placeholder="内容で検索..."
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
              <SearchFilters bind:filters on:change={handleFilterChange} />
            {/if}

            <!-- Search Button -->
            <div class="flex justify-between items-center">
              <div class="text-sm text-gray-500">
                {#if searchResults}
                  {searchResults.total}件の文書が見つかりました ({searchResults.took_ms}ms)
                {/if}
              </div>

              <Button type="submit" variant="primary" disabled={isSearching}>
                {#if isSearching}
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
        {#if searchResults}
          <SearchResults
            results={searchResults}
            on:loadMore={handleLoadMore}
            hasMore={searchResults.documents.length < searchResults.total}
            loading={isSearching}
          />
        {/if}
      </div>
    </div>
  </div>
</div>
