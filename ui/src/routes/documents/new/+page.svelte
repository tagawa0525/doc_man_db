<script lang="ts">
  import { goto } from "$app/navigation";
  import Button from "$lib/components/ui/Button.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Select from "$lib/components/ui/Select.svelte";
  // import TextArea from "$lib/components/ui/TextArea.svelte";

  // API統合
  import {
    createDocument,
    isLoadingDocuments,
    documentsError
  } from "$lib/stores/documents.js";
  import { showError, showInfo } from "$lib/stores/errors.js";
  import type { CreateDocumentInput } from "$lib/api/queries/documents.js";

  // フォームデータ
  let formData = {
    title: "",
    documentTypeCode: "",
    departmentCode: "",
    createdBy: 1, // TODO: 実際のユーザーIDに置き換え
    createdDate: new Date().toISOString().split("T")[0],
  };

  // 状態管理
  let errors: Record<string, string> = {};
  let generatedNumber = "";
  let showPreview = false;
  let createdDocument: any = null;

  // 文書種別オプション（実APIと合わせる）
  const documentTypeOptions = [
    { value: "", label: "文書種別を選択...", disabled: true },
    { value: "TECH", label: "技術文書" },
    { value: "PLAN", label: "計画書" },
    { value: "REPORT", label: "レポート" },
    { value: "MANUAL", label: "マニュアル" },
    { value: "SPEC", label: "仕様書" },
    { value: "PROC", label: "手順書" },
    { value: "POLICY", label: "ポリシー" },
  ];

  // 部署コードオプション（実APIと合わせる）
  const departmentOptions = [
    { value: "", label: "部署を選択...", disabled: true },
    { value: "IT", label: "情報システム部" },
    { value: "DEV", label: "システム開発部" },
    { value: "OPS", label: "運用管理部" },
    { value: "SALES", label: "営業部" },
    { value: "HR", label: "人事部" },
    { value: "FINANCE", label: "経理部" },
    { value: "GENERAL", label: "総務部" },
  ];

  // バリデーション
  function validateForm(): boolean {
    errors = {};

    if (!formData.title.trim()) {
      errors.title = "文書名は必須です";
    } else if (formData.title.length > 200) {
      errors.title = "文書名は200文字以内で入力してください";
    }

    if (!formData.documentTypeCode) {
      errors.documentTypeCode = "文書種別は必須です";
    }

    if (!formData.departmentCode) {
      errors.departmentCode = "部署は必須です";
    }

    if (!formData.createdDate) {
      errors.createdDate = "作成日は必須です";
    } else {
      const selectedDate = new Date(formData.createdDate);
      const today = new Date();
      today.setHours(0, 0, 0, 0);
      
      if (selectedDate > today) {
        errors.createdDate = "作成日は今日以前の日付を選択してください";
      }
    }

    return Object.keys(errors).length === 0;
  }

  // プレビュー表示
  function togglePreview() {
    if (validateForm()) {
      showPreview = !showPreview;
    }
  }

  // フォーム送信
  async function handleSubmit() {
    if (!validateForm()) {
      showError("入力内容に誤りがあります。修正してください。");
      return;
    }

    try {
      const input: CreateDocumentInput = {
        title: formData.title.trim(),
        documentTypeCode: formData.documentTypeCode,
        departmentCode: formData.departmentCode,
        createdBy: formData.createdBy,
        createdDate: formData.createdDate
      };

      const result = await createDocument(input);
      
      if (result) {
        createdDocument = result;
        generatedNumber = result.documentNumber;
        showInfo(`文書が正常に作成されました。文書番号: ${result.documentNumber}`);
        
        // 3秒後に詳細ページに遷移
        setTimeout(() => {
          goto(`/documents/${result.document.id}`);
        }, 3000);
      } else {
        showError("文書の作成に失敗しました");
      }
    } catch (error: any) {
      showError(error.message || "文書の作成中にエラーが発生しました");
    }
  }

  // フォームリセット
  function resetForm() {
    formData = {
      title: "",
      documentTypeCode: "",
      departmentCode: "",
      createdBy: 1,
      createdDate: new Date().toISOString().split("T")[0],
    };
    errors = {};
    generatedNumber = "";
    showPreview = false;
    createdDocument = null;
  }

  // キャンセル
  function handleCancel() {
    goto('/documents');
  }

  // エラーハンドリング
  $: if ($documentsError) {
    showError($documentsError);
  }
</script>

<svelte:head>
  <title>新規文書作成 - 文書管理システム</title>
</svelte:head>

<div class="space-y-6">
  <!-- ナビゲーション -->
  <div class="flex items-center space-x-2 text-sm text-gray-500">
    <a href="/documents" class="hover:text-gray-700">文書管理</a>
    <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
      <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 111.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd"/>
    </svg>
    <span class="text-gray-900">新規文書作成</span>
  </div>

  <!-- 成功メッセージ -->
  {#if createdDocument}
    <div class="bg-green-50 border border-green-200 rounded-lg p-4">
      <div class="flex">
        <div class="flex-shrink-0">
          <svg class="h-5 w-5 text-green-400" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
          </svg>
        </div>
        <div class="ml-3">
          <h3 class="text-sm font-medium text-green-800">文書作成完了</h3>
          <div class="mt-2 text-sm text-green-700">
            <p>文書が正常に作成されました。</p>
            <p class="font-medium">文書番号: {generatedNumber}</p>
            <p class="mt-1">3秒後に詳細ページに遷移します...</p>
          </div>
        </div>
      </div>
    </div>
  {/if}

  <!-- メインフォーム -->
  <div class="bg-white shadow rounded-lg">
    <div class="px-6 py-4 border-b border-gray-200">
      <h1 class="text-2xl font-bold text-gray-900">新規文書作成</h1>
      <p class="mt-1 text-sm text-gray-600">
        新しい文書を登録します。文書番号は自動生成されます。
      </p>
    </div>

    <div class="p-6">
      {#if !showPreview && !createdDocument}
        <!-- 入力フォーム -->
        <form on:submit|preventDefault={handleSubmit} class="space-y-6">
          <!-- 基本情報 -->
          <div class="space-y-4">
            <h2 class="text-lg font-medium text-gray-900">基本情報</h2>
            
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div class="md:col-span-2">
                <Input
                  label="文書名"
                  bind:value={formData.title}
                  placeholder="文書名を入力してください"
                  required
                  error={errors.title}
                  maxlength={200}
                />
              </div>

              <div>
                <Select
                  label="文書種別"
                  bind:value={formData.documentTypeCode}
                  options={documentTypeOptions}
                  required
                  error={errors.documentTypeCode}
                />
              </div>

              <div>
                <Select
                  label="所属部署"
                  bind:value={formData.departmentCode}
                  options={departmentOptions}
                  required
                  error={errors.departmentCode}
                />
              </div>

              <div>
                <Input
                  label="作成日"
                  type="date"
                  bind:value={formData.createdDate}
                  required
                  error={errors.createdDate}
                />
              </div>
            </div>
          </div>

          <!-- アクションボタン -->
          <div class="flex flex-col sm:flex-row justify-end space-y-2 sm:space-y-0 sm:space-x-3 pt-6 border-t border-gray-200">
            <Button
              type="button"
              on:click={resetForm}
              variant="secondary"
              disabled={$isLoadingDocuments}
            >
              リセット
            </Button>
            
            <Button
              type="button"
              on:click={togglePreview}
              variant="secondary"
              disabled={$isLoadingDocuments}
            >
              プレビュー
            </Button>
            
            <Button
              type="button"
              on:click={handleCancel}
              variant="secondary"
              disabled={$isLoadingDocuments}
            >
              キャンセル
            </Button>
            
            <Button
              type="submit"
              variant="primary"
              loading={$isLoadingDocuments}
              disabled={$isLoadingDocuments}
            >
              文書を作成
            </Button>
          </div>
        </form>

      {:else if showPreview && !createdDocument}
        <!-- プレビュー表示 -->
        <div class="space-y-6">
          <div class="flex items-center justify-between">
            <h2 class="text-lg font-medium text-gray-900">作成内容の確認</h2>
            <Button
              on:click={togglePreview}
              variant="secondary"
              size="sm"
            >
              編集に戻る
            </Button>
          </div>

          <div class="bg-gray-50 p-6 rounded-lg space-y-4">
            <dl class="space-y-3">
              <div>
                <dt class="text-sm font-medium text-gray-900">文書名</dt>
                <dd class="text-sm text-gray-700">{formData.title}</dd>
              </div>
              <div>
                <dt class="text-sm font-medium text-gray-900">文書種別</dt>
                <dd class="text-sm text-gray-700">
                  {documentTypeOptions.find(opt => opt.value === formData.documentTypeCode)?.label || formData.documentTypeCode}
                </dd>
              </div>
              <div>
                <dt class="text-sm font-medium text-gray-900">所属部署</dt>
                <dd class="text-sm text-gray-700">
                  {departmentOptions.find(opt => opt.value === formData.departmentCode)?.label || formData.departmentCode}
                </dd>
              </div>
              <div>
                <dt class="text-sm font-medium text-gray-900">作成日</dt>
                <dd class="text-sm text-gray-700">{formData.createdDate}</dd>
              </div>
            </dl>
          </div>

          <!-- 確認・実行ボタン -->
          <div class="flex justify-end space-x-3">
            <Button
              on:click={togglePreview}
              variant="secondary"
              disabled={$isLoadingDocuments}
            >
              編集に戻る
            </Button>
            
            <Button
              on:click={handleSubmit}
              variant="primary"
              loading={$isLoadingDocuments}
              disabled={$isLoadingDocuments}
            >
              文書を作成
            </Button>
          </div>
        </div>
      {/if}
    </div>
  </div>

  <!-- 注意事項 -->
  <div class="bg-blue-50 border border-blue-200 rounded-lg p-4">
    <div class="flex">
      <div class="flex-shrink-0">
        <svg class="h-5 w-5 text-blue-400" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd" />
        </svg>
      </div>
      <div class="ml-3">
        <h3 class="text-sm font-medium text-blue-800">文書作成について</h3>
        <div class="mt-2 text-sm text-blue-700">
          <ul class="list-disc list-inside space-y-1">
            <li>文書番号は部署コードと文書種別に基づいて自動生成されます</li>
            <li>作成後の文書基本情報は編集可能です</li>
            <li>ファイルのアップロード機能は準備中です</li>
          </ul>
        </div>
      </div>
    </div>
  </div>
</div>