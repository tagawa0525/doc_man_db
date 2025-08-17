<script lang="ts">
  export let headers: Array<{
    key: string;
    label: string;
    sortable?: boolean;
    mobileHidden?: boolean;
    class?: string;
  }>;
  export let data: Array<Record<string, any>>;
  export let mobileCardMode = true;
  export let onRowClick: ((item: any) => void) | undefined = undefined;
  export let onSort: ((key: string, direction: string) => void) | undefined = undefined;
  
  let sortColumn = '';
  let sortDirection: 'asc' | 'desc' = 'asc';
  
  function handleSort(key: string) {
    if (!onSort) return;
    
    if (sortColumn === key) {
      sortDirection = sortDirection === 'asc' ? 'desc' : 'asc';
    } else {
      sortColumn = key;
      sortDirection = 'asc';
    }
    
    onSort(key, sortDirection);
  }
  
  function handleRowClick(item: any) {
    if (onRowClick) {
      onRowClick(item);
    }
  }
  
  // Custom cell renderer
  function renderCell(item: any, header: any, index: number) {
    return item[header.key] || '-';
  }
</script>

<!-- デスクトップ・タブレット表示 -->
<div class="hidden sm:block overflow-x-auto">
  <table class="min-w-full divide-y divide-gray-200">
    <thead class="bg-gray-50">
      <tr>
        {#each headers as header}
          <th 
            class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider {header.class || ''}"
            class:cursor-pointer={header.sortable}
            on:click={header.sortable ? () => handleSort(header.key) : undefined}
            role={header.sortable ? 'button' : undefined}
            tabindex={header.sortable ? 0 : -1}
          >
            <div class="flex items-center space-x-1">
              <span>{header.label}</span>
              {#if header.sortable}
                <svg 
                  class="w-4 h-4 text-gray-400 {sortColumn === header.key ? 'text-gray-600' : ''}"
                  class:rotate-180={sortColumn === header.key && sortDirection === 'desc'}
                  fill="none" 
                  stroke="currentColor" 
                  viewBox="0 0 24 24"
                >
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 15l7-7 7 7" />
                </svg>
              {/if}
            </div>
          </th>
        {/each}
      </tr>
    </thead>
    <tbody class="bg-white divide-y divide-gray-200">
      {#each data as item, index}
        <tr 
          class="hover:bg-gray-50 {onRowClick ? 'cursor-pointer' : ''}"
          on:click={onRowClick ? () => handleRowClick(item) : undefined}
          role={onRowClick ? 'button' : undefined}
          tabindex={onRowClick ? 0 : -1}
        >
          {#each headers as header}
            <td class="px-6 py-4 whitespace-nowrap {header.class || ''}">
              <slot name="cell" {item} {header} {index}>
                {renderCell(item, header, index)}
              </slot>
            </td>
          {/each}
        </tr>
      {/each}
    </tbody>
  </table>
</div>

<!-- モバイル表示（カードモード） -->
{#if mobileCardMode}
  <div class="sm:hidden space-y-3">
    {#each data as item, index}
      <div 
        class="bg-white shadow rounded-lg p-4 {onRowClick ? 'cursor-pointer hover:shadow-md' : ''}"
        on:click={onRowClick ? () => handleRowClick(item) : undefined}
        role={onRowClick ? 'button' : undefined}
        tabindex={onRowClick ? 0 : -1}
      >
        <slot name="mobile-card" {item} {index}>
          {#each headers.filter(h => !h.mobileHidden) as header}
            <div class="flex justify-between items-center py-1">
              <span class="text-sm font-medium text-gray-500">{header.label}:</span>
              <span class="text-sm text-gray-900">
                <slot name="cell" {item} {header} {index}>
                  {renderCell(item, header, index)}
                </slot>
              </span>
            </div>
          {/each}
        </slot>
      </div>
    {/each}
  </div>
{:else}
  <!-- モバイル表示（簡易テーブル） -->
  <div class="sm:hidden overflow-x-auto">
    <table class="min-w-full divide-y divide-gray-200">
      <thead class="bg-gray-50">
        <tr>
          {#each headers.filter(h => !h.mobileHidden) as header}
            <th class="px-3 py-2 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              {header.label}
            </th>
          {/each}
        </tr>
      </thead>
      <tbody class="bg-white divide-y divide-gray-200">
        {#each data as item, index}
          <tr 
            class="hover:bg-gray-50 {onRowClick ? 'cursor-pointer' : ''}"
            on:click={onRowClick ? () => handleRowClick(item) : undefined}
            role={onRowClick ? 'button' : undefined}
            tabindex={onRowClick ? 0 : -1}
          >
            {#each headers.filter(h => !h.mobileHidden) as header}
              <td class="px-3 py-2 text-sm">
                <slot name="cell" {item} {header} {index}>
                  {renderCell(item, header, index)}
                </slot>
              </td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
{/if}

<!-- データが空の場合 -->
{#if data.length === 0}
  <div class="text-center py-12">
    <slot name="empty">
      <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2M4 13h2m4-8v12m4-8v8" />
      </svg>
      <h3 class="mt-2 text-sm font-medium text-gray-900">データがありません</h3>
      <p class="mt-1 text-sm text-gray-500">該当するデータが見つかりませんでした。</p>
    </slot>
  </div>
{/if}