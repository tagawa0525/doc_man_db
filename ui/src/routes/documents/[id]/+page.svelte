<script lang="ts">
  import { page } from "$app/stores";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import Button from "$lib/components/ui/Button.svelte";

  // API統合
  import {
    currentDocument,
    isLoadingCurrentDocument,
    documentsError,
    loadDocument,
  } from "$lib/stores/documents.js";
  import { showError } from "$lib/stores/errors.js";

  // パラメータからドキュメントIDを取得
  $: documentId = $page.params.id ? parseInt($page.params.id) : null;

  // 状態管理
  let showDeleteDialog = false;

  // 文書種別表示用（仮データ、後で実APIから取得）
  const documentTypeLabels: Record<number, string> = {
    1: "技術文書",
    2: "計画書",
    3: "レポート",
    4: "提案書",
    5: "手順書",
  };

  // 文書編集ページへ
  function goToEdit() {
    if (documentId) {
      goto(`/documents/${documentId}/edit`);
    }
  }

  // 文書削除処理（プレースホルダー）
  function handleDelete() {
    showDeleteDialog = true;
  }

  // 削除確認処理
  function confirmDelete() {
    // TODO: 実装時に削除API呼び出し
    console.log("Delete document:", documentId);
    showDeleteDialog = false;
    goto("/documents");
  }

  // ファイルダウンロード（プレースホルダー）
  function downloadFile() {
    // TODO: ファイルダウンロード機能実装
    showError("ファイルダウンロード機能は準備中です");
  }

  // 回覧開始（プレースホルダー）
  function startCirculation() {
    // TODO: 回覧機能実装
    showError("回覧機能は準備中です");
  }

  // 初期化
  onMount(async () => {
    if (documentId && !isNaN(documentId)) {
      await loadDocument(documentId);
    } else {
      showError("無効な文書IDです");
      goto("/documents");
    }
  });

  // エラーハンドリング
  $: if ($documentsError) {
    showError($documentsError);
  }

  // 文書が見つからない場合の処理
  $: if (!$isLoadingCurrentDocument && !$currentDocument && documentId) {
    showError("文書が見つかりません");
    goto("/documents");
  }
</script>

<svelte:head>
  <title>{$currentDocument?.title || "文書詳細"} - 文書管理システム</title>
</svelte:head>

<div class="space-y-6">
  <!-- ナビゲーション -->
  <div class="flex items-center space-x-2 text-sm text-gray-500">
    <a href="/documents" class="hover:text-gray-700">文書管理</a>
    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
      <path
        fill-rule="evenodd"
        d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 111.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
        clip-rule="evenodd"
      />
    </svg>
    <span class="text-gray-900">文書詳細</span>
  </div>

  {#if $isLoadingCurrentDocument}
    <!-- ローディング表示 -->
    <div class="bg-white shadow rounded-lg">
      <div class="p-12 text-center">
        <div class="inline-flex items-center">
          <svg
            class="animate-spin -ml-1 mr-3 h-8 w-8 text-gray-600"
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
              d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l-3-2.647z"
            ></path>
          </svg>
          <span class="text-lg">文書情報を読み込み中...</span>
        </div>
      </div>
    </div>
  {:else if $currentDocument}
    <!-- 文書詳細表示 -->
    <div class="bg-white shadow rounded-lg">
      <!-- ヘッダー -->
      <div class="px-6 py-4 border-b border-gray-200">
        <div class="flex items-start justify-between">
          <div>
            <h1 class="text-2xl font-bold text-gray-900">
              {$currentDocument.title}
            </h1>
            <p class="mt-1 text-sm text-gray-500">
              ID: {$currentDocument.id} | 種別: {documentTypeLabels[
                $currentDocument.documentTypeId
              ] || "不明"}
            </p>
          </div>

          <div class="flex items-center space-x-2">
            <Button on:click={goToEdit} variant="primary">編集</Button>
            <Button on:click={downloadFile} variant="secondary">
              ダウンロード
            </Button>
            <Button on:click={startCirculation} variant="secondary">
              回覧開始
            </Button>
            <Button on:click={handleDelete} variant="danger">削除</Button>
          </div>
        </div>
      </div>

      <!-- 基本情報 -->
      <div class="px-6 py-4">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div class="space-y-4">
            <div>
              <h3 class="text-sm font-medium text-gray-500 mb-2">基本情報</h3>
              <dl class="space-y-2">
                <div class="flex">
                  <dt class="w-24 text-sm font-medium text-gray-900">
                    作成者:
                  </dt>
                  <dd class="text-sm text-gray-700">
                    {$currentDocument.createdBy}
                  </dd>
                </div>
                <div class="flex">
                  <dt class="w-24 text-sm font-medium text-gray-900">
                    作成日:
                  </dt>
                  <dd class="text-sm text-gray-700">
                    {$currentDocument.createdDate}
                  </dd>
                </div>
                <div class="flex">
                  <dt class="w-24 text-sm font-medium text-gray-900">
                    更新日:
                  </dt>
                  <dd class="text-sm text-gray-700">
                    {new Date($currentDocument.updatedAt).toLocaleString(
                      "ja-JP",
                    )}
                  </dd>
                </div>
              </dl>
            </div>
          </div>

          <div class="space-y-4">
            <div>
              <h3 class="text-sm font-medium text-gray-500 mb-2">
                システム情報
              </h3>
              <dl class="space-y-2">
                <div class="flex">
                  <dt class="w-24 text-sm font-medium text-gray-900">
                    登録日:
                  </dt>
                  <dd class="text-sm text-gray-700">
                    {new Date($currentDocument.createdAt).toLocaleString(
                      "ja-JP",
                    )}
                  </dd>
                </div>
                <div class="flex">
                  <dt class="w-24 text-sm font-medium text-gray-900">
                    種別ID:
                  </dt>
                  <dd class="text-sm text-gray-700">
                    {$currentDocument.documentTypeId}
                  </dd>
                </div>
              </dl>
            </div>
          </div>
        </div>
      </div>

      <!-- 操作ボタン（モバイル用） -->
      <div class="px-6 py-4 border-t border-gray-200 md:hidden">
        <div class="grid grid-cols-2 gap-2">
          <Button on:click={goToEdit} variant="primary" class="w-full">
            編集
          </Button>
          <Button on:click={downloadFile} variant="secondary" class="w-full">
            ダウンロード
          </Button>
          <Button
            on:click={startCirculation}
            variant="secondary"
            class="w-full"
          >
            回覧開始
          </Button>
          <Button on:click={handleDelete} variant="danger" class="w-full">
            削除
          </Button>
        </div>
      </div>
    </div>

    <!-- プレースホルダー情報（将来実装予定） -->
    <div class="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
      <div class="flex">
        <div class="flex-shrink-0">
          <svg
            class="h-5 w-5 text-yellow-400"
            viewBox="0 0 20 20"
            fill="currentColor"
          >
            <path
              fill-rule="evenodd"
              d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z"
              clip-rule="evenodd"
            />
          </svg>
        </div>
        <div class="ml-3">
          <h3 class="text-sm font-medium text-yellow-800">機能準備中</h3>
          <div class="mt-2 text-sm text-yellow-700">
            <p>以下の機能は今後実装予定です：</p>
            <ul class="list-disc list-inside mt-1">
              <li>ファイルダウンロード</li>
              <li>回覧開始機能</li>
              <li>文書削除機能</li>
              <li>機密性レベル表示</li>
              <li>承認状態表示</li>
              <li>バージョン履歴</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>

<!-- 削除確認ダイアログ -->
{#if showDeleteDialog}
  <div
    class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50"
  >
    <div
      class="relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white"
    >
      <div class="mt-3 text-center">
        <div
          class="mx-auto flex items-center justify-center h-12 w-12 rounded-full bg-red-100"
        >
          <svg
            class="h-6 w-6 text-red-600"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z"
            />
          </svg>
        </div>
        <h3 class="text-lg font-medium text-gray-900 mt-2">文書の削除</h3>
        <div class="mt-2 px-7 py-3">
          <p class="text-sm text-gray-500">
            この文書を削除してもよろしいですか？<br />
            この操作は取り消すことができません。
          </p>
        </div>
        <div class="flex justify-center space-x-3 mt-4">
          <Button on:click={confirmDelete} variant="danger" size="sm">
            削除する
          </Button>
          <Button
            on:click={() => (showDeleteDialog = false)}
            variant="secondary"
            size="sm"
          >
            キャンセル
          </Button>
        </div>
      </div>
    </div>
  </div>
{/if}
