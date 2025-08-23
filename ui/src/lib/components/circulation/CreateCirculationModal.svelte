<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import TextArea from "$lib/components/ui/TextArea.svelte";
  import WorkflowSelector from "./WorkflowSelector.svelte";

  interface Document {
    id: number;
    title: string;
    document_type_id: number;
    created_by: number;
    created_date: string;
    created_at: string;
    updated_at: string;
  }

  const dispatch = createEventDispatcher();

  let documentSearchQuery = "";
  let selectedDocumentId: number | null = null;
  let selectedWorkflowId: number | null = null;
  let notes = "";
  let searchResults: Document[] = [];
  let isSearching = false;
  let selectedDocument: Document | null = null;

  $: canCreate = selectedDocumentId && selectedWorkflowId;

  async function searchDocuments() {
    if (!documentSearchQuery.trim()) {
      searchResults = [];
      return;
    }

    try {
      isSearching = true;
      const response = await fetch(
        `/api/documents/search?title=${encodeURIComponent(documentSearchQuery)}&limit=10`,
      );
      if (response.ok) {
        const data = await response.json();
        searchResults = data.documents || [];
      }
    } catch (error) {
      console.error("Failed to search documents:", error);
    } finally {
      isSearching = false;
    }
  }

  function selectDocument(document: Document) {
    selectedDocumentId = document.id;
    selectedDocument = document;
    documentSearchQuery = document.title;
    searchResults = [];
  }

  function handleCreate() {
    if (canCreate) {
      dispatch("create", {
        document_id: selectedDocumentId,
        workflow_id: selectedWorkflowId,
        notes: notes.trim() || undefined,
      });
    }
  }

  function handleCancel() {
    dispatch("cancel");
  }

  // Debounced search
  let searchTimeout: NodeJS.Timeout;
  $: if (documentSearchQuery) {
    clearTimeout(searchTimeout);
    searchTimeout = setTimeout(searchDocuments, 300);
  }
</script>

<div class="fixed inset-0 z-50 overflow-y-auto">
  <div
    class="flex items-center justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0"
  >
    <!-- Background overlay -->
    <div class="fixed inset-0 transition-opacity" aria-hidden="true">
      <button
        type="button"
        class="absolute inset-0 bg-gray-500 opacity-75"
        on:click={handleCancel}
        aria-label="モーダルを閉じる"
      ></button>
    </div>

    <!-- Modal panel -->
    <div
      class="inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-2xl sm:w-full"
    >
      <div class="bg-white px-6 pt-6 pb-4">
        <!-- Header -->
        <div class="flex justify-between items-center mb-6">
          <h3 class="text-xl font-semibold text-gray-900">新規回覧開始</h3>
          <button
            type="button"
            class="text-gray-400 hover:text-gray-600 focus:outline-none"
            on:click={handleCancel}
            aria-label="閉じる"
          >
            <svg
              class="w-6 h-6"
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

        <div class="space-y-6">
          <!-- Document Selection -->
          <div>
            <label
              for="document-search"
              class="block text-sm font-medium text-gray-700 mb-1"
            >
              文書を選択 *
            </label>
            <div class="relative">
              <Input
                id="document-search"
                bind:value={documentSearchQuery}
                placeholder="文書タイトルで検索..."
                class="w-full"
              />

              {#if isSearching}
                <div
                  class="absolute right-3 top-1/2 transform -translate-y-1/2"
                >
                  <div
                    class="animate-spin rounded-full h-4 w-4 border-b-2 border-blue-600"
                  ></div>
                </div>
              {/if}

              {#if searchResults.length > 0}
                <div
                  class="absolute z-10 mt-1 w-full bg-white shadow-lg max-h-60 rounded-md py-1 text-base ring-1 ring-black ring-opacity-5 overflow-auto"
                >
                  {#each searchResults as document (document.id)}
                    <button
                      type="button"
                      class="w-full text-left px-4 py-2 hover:bg-gray-100 focus:bg-gray-100 focus:outline-none"
                      on:click={() => selectDocument(document)}
                    >
                      <div class="font-medium text-gray-900">
                        {document.title}
                      </div>
                      <div class="text-sm text-gray-500">
                        ID: {document.id} | 作成日: {new Date(
                          document.created_date,
                        ).toLocaleDateString("ja-JP")}
                      </div>
                    </button>
                  {/each}
                </div>
              {/if}
            </div>

            {#if selectedDocument}
              <div class="mt-2 p-3 bg-blue-50 rounded-md">
                <p class="text-sm font-medium text-blue-900">選択された文書:</p>
                <p class="text-sm text-blue-800">{selectedDocument.title}</p>
                <p class="text-xs text-blue-600">ID: {selectedDocument.id}</p>
              </div>
            {/if}
          </div>

          <!-- Workflow Selection -->
          <div>
            <label
              for="workflow-selector"
              class="block text-sm font-medium text-gray-700 mb-1"
            >
              ワークフロー *
            </label>
            <WorkflowSelector bind:selectedWorkflowId />
          </div>

          <!-- Notes -->
          <div>
            <label
              for="notes"
              class="block text-sm font-medium text-gray-700 mb-1"
            >
              備考
            </label>
            <TextArea
              id="notes"
              bind:value={notes}
              placeholder="回覧に関する備考を入力してください（任意）"
              rows={3}
            />
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div class="bg-gray-50 px-6 py-4 flex justify-end space-x-3">
        <Button variant="secondary" on:click={handleCancel}>キャンセル</Button>
        <Button variant="primary" disabled={!canCreate} on:click={handleCreate}>
          回覧開始
        </Button>
      </div>
    </div>
  </div>
</div>
