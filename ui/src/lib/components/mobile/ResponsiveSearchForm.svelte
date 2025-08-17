<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import Input from '$lib/components/ui/Input.svelte';
  import Select from '$lib/components/ui/Select.svelte';
  
  const dispatch = createEventDispatcher();
  
  export let filters: any = {};
  export let isSearching = false;
  export let showAdvancedFilters = false;
  export let documentTypeOptions: any[] = [];
  export let internalExternalOptions: any[] = [];
  export let importanceClassOptions: any[] = [];
  export let personalInfoOptions: any[] = [];
  
  function handleSearch() {
    dispatch('search', filters);
  }
  
  function handleClear() {
    dispatch('clear');
  }
  
  function toggleAdvanced() {
    showAdvancedFilters = !showAdvancedFilters;
    dispatch('toggle-advanced', showAdvancedFilters);
  }
</script>

<div class="bg-white shadow rounded-lg p-4 sm:p-6">
  <form on:submit|preventDefault={handleSearch}>
    <!-- 基本検索 -->
    <div class="grid grid-cols-1 gap-4 mb-4 sm:grid-cols-2 lg:grid-cols-3">
      <div>
        <label
          for="title"
          class="block text-sm font-medium text-gray-700 mb-1"
        >
          文書タイトル
        </label>
        <Input
          id="title"
          bind:value={filters.title}
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
          bind:value={filters.businessNumber}
          placeholder="業務番号で検索..."
        />
      </div>

      <div class="sm:col-span-2 lg:col-span-1">
        <label
          for="documentType"
          class="block text-sm font-medium text-gray-700 mb-1"
        >
          文書種別
        </label>
        <Select
          id="documentType"
          bind:value={filters.documentTypeId}
          options={documentTypeOptions}
          placeholder="文書種別を選択..."
        />
      </div>
    </div>

    <!-- 詳細検索 -->
    {#if showAdvancedFilters}
      <div class="border-t border-gray-200 pt-4">
        <h3 class="text-lg font-medium text-gray-900 mb-4">詳細検索</h3>

        <div class="grid grid-cols-1 gap-4 mb-4 sm:grid-cols-2">
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
              bind:value={filters.createdDateFrom}
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
              bind:value={filters.createdDateTo}
            />
          </div>
        </div>

        <!-- 機密レベル -->
        <div class="mb-4">
          <h4 class="text-sm font-medium text-gray-700 mb-2">機密レベル</h4>
          <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
            <div>
              <label
                for="internalExternal"
                class="block text-xs font-medium text-gray-600 mb-1"
              >
                社内外区分
              </label>
              <Select
                id="internalExternal"
                bind:value={filters.confidentiality.internalExternal}
                options={internalExternalOptions}
                placeholder="社内外区分"
              />
            </div>
            <div>
              <label
                for="importanceClass"
                class="block text-xs font-medium text-gray-600 mb-1"
              >
                重要度
              </label>
              <Select
                id="importanceClass"
                bind:value={filters.confidentiality.importanceClass}
                options={importanceClassOptions}
                placeholder="重要度"
              />
            </div>
            <div class="sm:col-span-2 lg:col-span-1">
              <label
                for="personalInfo"
                class="block text-xs font-medium text-gray-600 mb-1"
              >
                個人情報
              </label>
              <Select
                id="personalInfo"
                bind:value={filters.confidentiality.personalInfo}
                options={personalInfoOptions}
                placeholder="個人情報"
              />
            </div>
          </div>
        </div>
      </div>
    {/if}

    <!-- 検索ボタン -->
    <div class="flex flex-col sm:flex-row sm:justify-between sm:items-center pt-4 space-y-3 sm:space-y-0">
      <button
        type="button"
        on:click={toggleAdvanced}
        class="text-blue-600 hover:text-blue-700 text-sm font-medium"
      >
        {showAdvancedFilters ? "詳細検索を閉じる" : "詳細検索"}
      </button>

      <div class="flex space-x-2">
        <Button type="button" variant="secondary" on:click={handleClear}>
          クリア
        </Button>
        <Button type="submit" loading={isSearching} class="flex-1 sm:flex-none">
          検索
        </Button>
      </div>
    </div>
  </form>
</div>