<script lang="ts">
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  export let currentPage: number;
  export let totalPages: number;
  export let totalCount: number;
  export let pageSize: number;

  $: startIndex = (currentPage - 1) * pageSize + 1;
  $: endIndex = Math.min(currentPage * pageSize, totalCount);

  function handlePageChange(page: number) {
    if (page >= 1 && page <= totalPages) {
      dispatch("page-change", page);
    }
  }

  function getPaginationPages(): number[] {
    const pages: number[] = [];
    const maxVisible = 5;

    if (totalPages <= maxVisible) {
      for (let i = 1; i <= totalPages; i++) {
        pages.push(i);
      }
    } else {
      const start = Math.max(1, currentPage - 2);
      const end = Math.min(totalPages, start + maxVisible - 1);

      for (let i = start; i <= end; i++) {
        pages.push(i);
      }
    }

    return pages;
  }
</script>

{#if totalPages > 1}
  <div class="px-4 py-3 border-t border-gray-200 sm:px-6">
    <div class="flex items-center justify-between">
      <!-- モバイル用簡易ページング -->
      <div class="flex flex-1 justify-between sm:hidden">
        <button
          disabled={currentPage <= 1}
          class="relative inline-flex items-center px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
          on:click={() => handlePageChange(currentPage - 1)}
        >
          前へ
        </button>

        <span class="text-sm text-gray-700 self-center">
          {currentPage} / {totalPages}
        </span>

        <button
          disabled={currentPage >= totalPages}
          class="relative inline-flex items-center px-4 py-2 ml-3 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
          on:click={() => handlePageChange(currentPage + 1)}
        >
          次へ
        </button>
      </div>

      <!-- デスクトップ用詳細ページング -->
      <div class="hidden sm:flex sm:flex-1 sm:items-center sm:justify-between">
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
            class="relative z-0 inline-flex -space-x-px rounded-md shadow-sm"
            aria-label="Pagination"
          >
            <!-- 前のページボタン -->
            <button
              disabled={currentPage <= 1}
              class="relative inline-flex items-center px-2 py-2 text-sm font-medium text-gray-500 bg-white border border-gray-300 rounded-l-md hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
              on:click={() => handlePageChange(currentPage - 1)}
              aria-label="前のページ"
            >
              <svg class="w-5 h-5" viewBox="0 0 20 20" fill="currentColor">
                <path
                  fill-rule="evenodd"
                  d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z"
                  clip-rule="evenodd"
                />
              </svg>
            </button>

            <!-- ページ番号 -->
            {#each getPaginationPages() as page}
              <button
                class="relative inline-flex items-center px-4 py-2 text-sm font-medium border {page ===
                currentPage
                  ? 'z-10 bg-blue-50 border-blue-500 text-blue-600'
                  : 'bg-white border-gray-300 text-gray-500 hover:bg-gray-50'}"
                on:click={() => handlePageChange(page)}
              >
                {page}
              </button>
            {/each}

            <!-- 次のページボタン -->
            <button
              disabled={currentPage >= totalPages}
              class="relative inline-flex items-center px-2 py-2 text-sm font-medium text-gray-500 bg-white border border-gray-300 rounded-r-md hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
              on:click={() => handlePageChange(currentPage + 1)}
              aria-label="次のページ"
            >
              <svg class="w-5 h-5" viewBox="0 0 20 20" fill="currentColor">
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
    </div>
  </div>
{/if}
