<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import Button from "$lib/components/ui/Button.svelte";

  interface SavedSearch {
    id: number;
    name: string;
    filters: any;
    createdAt: string;
  }

  export let searches: SavedSearch[] = [];

  const dispatch = createEventDispatcher();

  function formatDate(dateString: string): string {
    return new Date(dateString).toLocaleDateString("ja-JP");
  }

  function handleLoad(search: SavedSearch) {
    dispatch("load", search);
  }

  function handleDelete(id: number, event: Event) {
    event.stopPropagation();
    if (confirm("この保存された検索を削除しますか？")) {
      dispatch("delete", id);
    }
  }
</script>

<div class="bg-white rounded-lg shadow p-4">
  <h3 class="text-lg font-medium text-gray-900 mb-3">保存された検索</h3>

  {#if searches.length === 0}
    <p class="text-sm text-gray-500">保存された検索はありません</p>
  {:else}
    <div class="space-y-2">
      {#each searches as search (search.id)}
        <div
          class="group relative border border-gray-200 rounded-md p-3 hover:bg-gray-50"
        >
          <button
            type="button"
            class="w-full text-left"
            on:click={() => handleLoad(search)}
          >
            <h4 class="font-medium text-gray-900 text-sm">{search.name}</h4>
            <p class="text-xs text-gray-500 mt-1">
              保存日: {formatDate(search.createdAt)}
            </p>

            <!-- Preview of filters -->
            <div class="mt-2 flex flex-wrap gap-1">
              {#if search.filters.title}
                <span
                  class="inline-flex items-center px-2 py-1 rounded text-xs bg-blue-100 text-blue-800"
                >
                  タイトル: {search.filters.title.length > 10
                    ? search.filters.title.substring(0, 10) + "..."
                    : search.filters.title}
                </span>
              {/if}

              {#if search.filters.document_type_id}
                <span
                  class="inline-flex items-center px-2 py-1 rounded text-xs bg-green-100 text-green-800"
                >
                  文書種別
                </span>
              {/if}

              {#if search.filters.department_id}
                <span
                  class="inline-flex items-center px-2 py-1 rounded text-xs bg-purple-100 text-purple-800"
                >
                  部署
                </span>
              {/if}

              {#if search.filters.created_date_from || search.filters.created_date_to}
                <span
                  class="inline-flex items-center px-2 py-1 rounded text-xs bg-yellow-100 text-yellow-800"
                >
                  期間
                </span>
              {/if}
            </div>
          </button>

          <!-- Delete button -->
          <button
            type="button"
            class="absolute top-2 right-2 opacity-0 group-hover:opacity-100 text-red-500 hover:text-red-700 transition-opacity"
            on:click={(event) => handleDelete(search.id, event)}
            aria-label="削除"
          >
            <svg
              class="w-4 h-4"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
        </div>
      {/each}
    </div>
  {/if}
</div>
