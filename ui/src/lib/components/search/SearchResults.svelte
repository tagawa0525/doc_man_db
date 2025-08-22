<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import Button from '$lib/components/ui/Button.svelte';
  
  interface SearchResult {
    documents: any[];
    total: number;
    took_ms: number;
  }
  
  export let results: SearchResult;
  export let hasMore: boolean = false;
  export let loading: boolean = false;
  
  const dispatch = createEventDispatcher();
  
  function formatDate(dateString: string): string {
    return new Date(dateString).toLocaleDateString('ja-JP');
  }
  
  function formatDateTime(dateString: string): string {
    return new Date(dateString).toLocaleString('ja-JP');
  }
  
  function handleDocumentClick(documentId: number) {
    // Navigate to document detail page
    window.location.href = `/documents/${documentId}`;
  }
  
  function handleLoadMore() {
    dispatch('loadMore');
  }
</script>

<div class="bg-white shadow rounded-lg">
  <!-- Results Header -->
  <div class="px-6 py-4 border-b border-gray-200">
    <div class="flex justify-between items-center">
      <h3 class="text-lg font-medium text-gray-900">
        検索結果 ({results.total}件)
      </h3>
      <div class="text-sm text-gray-500">
        検索時間: {results.took_ms}ms
      </div>
    </div>
  </div>
  
  <!-- Results List -->
  <div class="divide-y divide-gray-200">
    {#each results.documents as document (document.id)}
      <div class="p-6 hover:bg-gray-50 cursor-pointer" on:click={() => handleDocumentClick(document.id)}>
        <div class="flex justify-between items-start">
          <div class="flex-1 min-w-0">
            <h4 class="text-lg font-medium text-blue-600 hover:text-blue-800 truncate">
              {document.title}
            </h4>
            
            <div class="mt-2 flex items-center space-x-4 text-sm text-gray-500">
              <span>ID: {document.id}</span>
              <span>作成日: {formatDate(document.created_date)}</span>
              <span>更新日: {formatDateTime(document.updated_at)}</span>
            </div>
            
            {#if document.description}
              <p class="mt-2 text-sm text-gray-700 line-clamp-2">
                {document.description}
              </p>
            {/if}
            
            <!-- Document metadata -->
            <div class="mt-3 flex flex-wrap gap-2">
              {#if document.document_type_name}
                <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                  {document.document_type_name}
                </span>
              {/if}
              
              {#if document.department_name}
                <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-100 text-gray-800">
                  {document.department_name}
                </span>
              {/if}
              
              {#if document.confidentiality_level}
                <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium 
                  {document.confidentiality_level === 'confidential' ? 'bg-red-100 text-red-800' : 
                    document.confidentiality_level === 'internal' ? 'bg-yellow-100 text-yellow-800' : 
                    'bg-green-100 text-green-800'}">
                  {document.confidentiality_level === 'confidential' ? '機密' : 
                    document.confidentiality_level === 'internal' ? '社内' : '公開'}
                </span>
              {/if}
              
              {#if document.file_exists === false}
                <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-red-100 text-red-800">
                  ファイル不存在
                </span>
              {/if}
            </div>
            
            <!-- File path if available -->
            {#if document.file_path}
              <div class="mt-2 text-xs text-gray-500 font-mono truncate">
                {document.file_path}
              </div>
            {/if}
          </div>
          
          <!-- Actions -->
          <div class="flex-shrink-0 ml-4">
            <div class="flex items-center space-x-2">
              <button
                type="button"
                class="text-blue-600 hover:text-blue-800 text-sm font-medium"
                on:click|stopPropagation={() => handleDocumentClick(document.id)}
              >
                詳細
              </button>
              
              {#if document.file_path && document.file_exists}
                <a
                  href="file://{document.file_path}"
                  class="text-green-600 hover:text-green-800 text-sm font-medium"
                  on:click|stopPropagation
                >
                  開く
                </a>
              {/if}
            </div>
          </div>
        </div>
      </div>
    {:else}
      <div class="p-12 text-center">
        <div class="text-gray-400 text-lg mb-2">検索結果が見つかりませんでした</div>
        <p class="text-gray-500">検索条件を変更して再度お試しください。</p>
      </div>
    {/each}
  </div>
  
  <!-- Load More Button -->
  {#if hasMore}
    <div class="px-6 py-4 border-t border-gray-200 text-center">
      <Button
        variant="outline"
        on:click={handleLoadMore}
        disabled={loading}
      >
        {#if loading}
          <div class="flex items-center">
            <div class="animate-spin rounded-full h-4 w-4 border-b-2 border-blue-600 mr-2"></div>
            読み込み中...
          </div>
        {:else}
          さらに読み込む
        {/if}
      </Button>
    </div>
  {/if}
</div>