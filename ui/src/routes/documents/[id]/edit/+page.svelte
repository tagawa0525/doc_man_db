<script lang="ts">
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";

  const documentId = $page.params.id || "1";

  let documentData = {
    id: documentId,
    title: "",
    documentNumber: "",
    documentType: "",
    createdDate: "",
    filePath: "",
    description: "",
    confidentiality: "internal",
    importance: "normal",
  };

  let isLoading = true;

  onMount(async () => {
    // TODO: 実際のAPIからデータを取得
    setTimeout(() => {
      documentData = {
        id: documentId,
        title: "既存文書タイトル",
        documentNumber: `CTA-2508${documentId.padStart(3, "0")}`,
        documentType: "技術仕様書",
        createdDate: "2024-01-15",
        filePath: `/documents/CTA-2508${documentId.padStart(3, "0")}.pdf`,
        description: "既存の文書の説明文",
        confidentiality: "internal",
        importance: "normal",
      };
      isLoading = false;
    }, 500);
  });

  const handleSubmit = async () => {
    try {
      console.log("文書を更新:", documentData);
      // TODO: 実際のAPI呼び出し
      goto(`/documents/${documentId}`);
    } catch (error) {
      console.error("文書更新エラー:", error);
    }
  };

  const handleCancel = () => {
    goto(`/documents/${documentId}`);
  };
</script>

<svelte:head>
  <title>文書編集 - 文書管理システム</title>
</svelte:head>

<div class="max-w-4xl mx-auto p-6">
  <div class="mb-6">
    <h1 class="text-3xl font-bold text-gray-900">文書編集</h1>
    <p class="text-gray-600 mt-2">文書 #{documentId} の情報を編集</p>
  </div>

  {#if isLoading}
    <div class="bg-white rounded-lg shadow p-8 text-center">
      <div
        class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500 mx-auto"
      ></div>
      <p class="mt-4 text-gray-600">文書データを読み込み中...</p>
    </div>
  {:else}
    <div class="bg-white rounded-lg shadow">
      <form on:submit|preventDefault={handleSubmit} class="p-6 space-y-6">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <label
              for="title"
              class="block text-sm font-medium text-gray-700 mb-2"
            >
              文書タイトル *
            </label>
            <input
              id="title"
              type="text"
              bind:value={documentData.title}
              required
              class="input w-full"
              placeholder="文書タイトルを入力"
            />
          </div>

          <div>
            <label
              for="documentNumber"
              class="block text-sm font-medium text-gray-700 mb-2"
            >
              文書番号
            </label>
            <input
              id="documentNumber"
              type="text"
              bind:value={documentData.documentNumber}
              readonly
              class="input w-full bg-gray-50"
            />
          </div>

          <div>
            <label
              for="documentType"
              class="block text-sm font-medium text-gray-700 mb-2"
            >
              文書種別 *
            </label>
            <select
              id="documentType"
              bind:value={documentData.documentType}
              required
              class="input w-full"
            >
              <option value="">選択してください</option>
              <option value="技術仕様書">技術仕様書</option>
              <option value="設計書">設計書</option>
              <option value="手順書">手順書</option>
              <option value="報告書">報告書</option>
              <option value="提案書">提案書</option>
              <option value="契約書">契約書</option>
              <option value="その他">その他</option>
            </select>
          </div>

          <div>
            <label
              for="createdDate"
              class="block text-sm font-medium text-gray-700 mb-2"
            >
              作成日
            </label>
            <input
              id="createdDate"
              type="date"
              bind:value={documentData.createdDate}
              class="input w-full"
            />
          </div>

          <div>
            <label
              for="confidentiality"
              class="block text-sm font-medium text-gray-700 mb-2"
            >
              機密区分
            </label>
            <select
              id="confidentiality"
              bind:value={documentData.confidentiality}
              class="input w-full"
            >
              <option value="public">公開</option>
              <option value="internal">社内限定</option>
              <option value="confidential">機密</option>
              <option value="secret">極秘</option>
            </select>
          </div>

          <div>
            <label
              for="importance"
              class="block text-sm font-medium text-gray-700 mb-2"
            >
              重要度
            </label>
            <select
              id="importance"
              bind:value={documentData.importance}
              class="input w-full"
            >
              <option value="low">低</option>
              <option value="normal">普通</option>
              <option value="high">高</option>
              <option value="urgent">緊急</option>
            </select>
          </div>
        </div>

        <div>
          <label
            for="filePath"
            class="block text-sm font-medium text-gray-700 mb-2"
          >
            ファイルパス
          </label>
          <input
            id="filePath"
            type="text"
            bind:value={documentData.filePath}
            class="input w-full"
            placeholder="ファイルパスを入力"
          />
        </div>

        <div>
          <label
            for="description"
            class="block text-sm font-medium text-gray-700 mb-2"
          >
            説明
          </label>
          <textarea
            id="description"
            bind:value={documentData.description}
            rows="4"
            class="input w-full resize-y"
            placeholder="文書の説明を入力"
          ></textarea>
        </div>

        <div class="flex justify-end space-x-4 pt-6 border-t">
          <button
            type="button"
            on:click={handleCancel}
            class="btn btn-secondary"
          >
            キャンセル
          </button>
          <button type="submit" class="btn btn-primary"> 更新 </button>
        </div>
      </form>
    </div>
  {/if}
</div>
