<script lang="ts">
  import { goto } from "$app/navigation";
  import Button from "$lib/components/ui/Button.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Select from "$lib/components/ui/Select.svelte";
  import TextArea from "$lib/components/ui/TextArea.svelte";

  // フォームデータ
  let formData = {
    title: "",
    documentTypeId: "",
    businessNumber: "",
    createdDate: new Date().toISOString().split("T")[0],
    confidentiality: {
      internalExternal: "internal",
      importanceClass: "class2",
      personalInfo: "none",
    },
    notes: "",
  };

  // 状態管理
  let isSubmitting = false;
  let errors: Record<string, string> = {};
  let generatedNumber = "";
  let showPreview = false;

  // 文書種別オプション
  const documentTypeOptions = [
    { value: "", label: "文書種別を選択...", disabled: true },
    { value: "technical", label: "技術文書" },
    { value: "plan", label: "計画書" },
    { value: "report", label: "レポート" },
    { value: "manual", label: "マニュアル" },
    { value: "specification", label: "仕様書" },
    { value: "procedure", label: "手順書" },
    { value: "policy", label: "ポリシー" },
  ];

  // 機密レベルオプション
  const internalExternalOptions = [
    { value: "internal", label: "社内" },
    { value: "external", label: "社外" },
  ];

  const importanceClassOptions = [
    { value: "class1", label: "情報クラスⅠ（重要）" },
    { value: "class2", label: "情報クラスⅡ（通常）" },
  ];

  const personalInfoOptions = [
    { value: "none", label: "なし" },
    { value: "present", label: "あり" },
  ];

  // 文書番号自動生成プレビュー
  async function generateNumberPreview() {
    if (!formData.documentTypeId || !formData.createdDate) {
      generatedNumber = "";
      return;
    }

    try {
      // TODO: 実際のAPI呼び出しに置き換え
      await new Promise((resolve) => setTimeout(resolve, 500));

      // 仮の番号生成ロジック
      const year = new Date(formData.createdDate).getFullYear();
      const month = String(
        new Date(formData.createdDate).getMonth() + 1,
      ).padStart(2, "0");
      const docTypePrefix = getDocumentTypePrefix(formData.documentTypeId);
      const sequence = "001"; // 実際は最新の連番を取得

      generatedNumber = `${docTypePrefix}-${year.toString().slice(2)}${month}${sequence}`;
    } catch (error) {
      console.error("Failed to generate number preview:", error);
      generatedNumber = "";
    }
  }

  function getDocumentTypePrefix(typeId: string): string {
    const prefixes: Record<string, string> = {
      technical: "CTA",
      plan: "PLN",
      report: "REP",
      manual: "MAN",
      specification: "SPC",
      procedure: "PRC",
      policy: "POL",
    };
    return prefixes[typeId] || "DOC";
  }

  // バリデーション
  function validateForm(): boolean {
    errors = {};

    if (!formData.title.trim()) {
      errors.title = "タイトルは必須です";
    } else if (formData.title.length > 200) {
      errors.title = "タイトルは200文字以内で入力してください";
    }

    if (!formData.documentTypeId) {
      errors.documentTypeId = "文書種別は必須です";
    }

    if (!formData.createdDate) {
      errors.createdDate = "作成日は必須です";
    } else {
      const selectedDate = new Date(formData.createdDate);
      const today = new Date();
      today.setHours(0, 0, 0, 0);

      if (selectedDate > today) {
        errors.createdDate = "未来の日付は選択できません";
      }
    }

    if (formData.businessNumber && formData.businessNumber.length > 50) {
      errors.businessNumber = "業務番号は50文字以内で入力してください";
    }

    if (formData.notes && formData.notes.length > 1000) {
      errors.notes = "備考は1000文字以内で入力してください";
    }

    return Object.keys(errors).length === 0;
  }

  // フォーム送信
  async function handleSubmit() {
    if (!validateForm()) return;

    isSubmitting = true;

    try {
      // TODO: 実際のAPI呼び出しに置き換え
      await new Promise((resolve) => setTimeout(resolve, 2000));

      // 成功時の処理
      const documentId = Math.floor(Math.random() * 1000); // 仮のID

      // 成功メッセージの表示（実際は通知システムを使用）
      alert(
        `文書「${formData.title}」を作成しました。\n文書番号: ${generatedNumber}`,
      );

      // 詳細画面にリダイレクト
      goto(`/documents/${documentId}`);
    } catch (error) {
      console.error("Document creation failed:", error);
      alert("文書の作成に失敗しました。再度お試しください。");
    } finally {
      isSubmitting = false;
    }
  }

  // キャンセル処理
  function handleCancel() {
    if (hasUnsavedChanges()) {
      if (confirm("入力内容が破棄されます。よろしいですか？")) {
        goto("/documents");
      }
    } else {
      goto("/documents");
    }
  }

  // 未保存の変更があるかチェック
  function hasUnsavedChanges(): boolean {
    return (
      formData.title.trim() !== "" ||
      formData.businessNumber.trim() !== "" ||
      formData.notes.trim() !== "" ||
      formData.documentTypeId !== ""
    );
  }

  // リアクティブな処理
  $: if (formData.documentTypeId || formData.createdDate) {
    generateNumberPreview();
  }

  // 機密レベル表示用関数
  function getConfidentialityLabel(): string {
    const parts = [];
    if (formData.confidentiality.internalExternal === "external")
      parts.push("社外");
    if (formData.confidentiality.importanceClass === "class1")
      parts.push("重要");
    if (formData.confidentiality.personalInfo === "present")
      parts.push("個人情報");
    return parts.length > 0 ? parts.join("・") : "通常";
  }

  function getConfidentialityColor(): string {
    if (
      formData.confidentiality.importanceClass === "class1" ||
      formData.confidentiality.personalInfo === "present"
    ) {
      return "bg-red-100 text-red-800 border-red-200";
    }
    if (formData.confidentiality.internalExternal === "external") {
      return "bg-yellow-100 text-yellow-800 border-yellow-200";
    }
    return "bg-gray-100 text-gray-800 border-gray-200";
  }
</script>

<div class="space-y-6">
  <!-- ページヘッダー -->
  <div class="md:flex md:items-center md:justify-between">
    <div class="min-w-0 flex-1">
      <h1
        class="text-2xl font-bold leading-7 text-gray-900 sm:truncate sm:text-3xl sm:tracking-tight"
      >
        新規文書作成
      </h1>
      <p class="mt-1 text-sm text-gray-500">
        新しい文書を作成します。必須項目を入力してください。
      </p>
    </div>

    <!-- プレビューボタン -->
    <div class="mt-4 flex md:ml-4 md:mt-0">
      <Button
        variant="secondary"
        size="sm"
        on:click={() => (showPreview = !showPreview)}
      >
        <svg
          class="mr-2 h-4 w-4"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M2.036 12.322a1.012 1.012 0 010-.639C3.423 7.51 7.36 4.5 12 4.5c4.639 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.639 0-8.573-3.007-9.963-7.178z"
          />
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
          />
        </svg>
        {showPreview ? "プレビューを閉じる" : "プレビュー"}
      </Button>
    </div>
  </div>

  <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
    <!-- メインフォーム -->
    <div class="lg:col-span-2">
      <form on:submit|preventDefault={handleSubmit} class="space-y-6">
        <!-- 基本情報 -->
        <div class="bg-white shadow rounded-lg p-6">
          <h2 class="text-lg font-medium text-gray-900 mb-4">基本情報</h2>

          <div class="space-y-4">
            <div>
              <label
                for="title"
                class="block text-sm font-medium text-gray-700 mb-1"
              >
                文書タイトル <span class="text-red-500">*</span>
              </label>
              <Input
                id="title"
                bind:value={formData.title}
                error={errors.title}
                placeholder="文書のタイトルを入力してください"
                required
              />
              <p class="mt-1 text-xs text-gray-500">
                {formData.title.length}/200文字
              </p>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label
                  for="documentType"
                  class="block text-sm font-medium text-gray-700 mb-1"
                >
                  文書種別 <span class="text-red-500">*</span>
                </label>
                <Select
                  id="documentType"
                  bind:value={formData.documentTypeId}
                  options={documentTypeOptions}
                  error={errors.documentTypeId}
                  required
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
                  bind:value={formData.businessNumber}
                  error={errors.businessNumber}
                  placeholder="PJ2024-001（任意）"
                />
              </div>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div>
                <label
                  for="createdDate"
                  class="block text-sm font-medium text-gray-700 mb-1"
                >
                  作成日 <span class="text-red-500">*</span>
                </label>
                <Input
                  id="createdDate"
                  type="date"
                  bind:value={formData.createdDate}
                  error={errors.createdDate}
                  required
                />
              </div>

              {#if generatedNumber}
                <div>
                  <label
                    for="generated-number-display"
                    class="block text-sm font-medium text-gray-700 mb-1"
                  >
                    生成される文書番号
                  </label>
                  <div
                    id="generated-number-display"
                    class="px-3 py-2 bg-blue-50 border border-blue-200 rounded-md"
                  >
                    <span class="text-sm font-mono text-blue-800"
                      >{generatedNumber}</span
                    >
                  </div>
                </div>
              {/if}
            </div>
          </div>
        </div>

        <!-- 機密レベル設定 -->
        <div class="bg-white shadow rounded-lg p-6">
          <h2 class="text-lg font-medium text-gray-900 mb-4">機密レベル設定</h2>

          <div class="space-y-4">
            <div>
              <fieldset>
                <legend class="block text-sm font-medium text-gray-700 mb-2">
                  社内外区分 <span class="text-red-500">*</span>
                </legend>
                <div class="grid grid-cols-2 gap-4">
                  {#each internalExternalOptions as option}
                    <label
                      class="flex items-center p-3 border rounded-md cursor-pointer hover:bg-gray-50
                                {formData.confidentiality.internalExternal ===
                      option.value
                        ? 'border-blue-500 bg-blue-50'
                        : 'border-gray-300'}"
                    >
                      <input
                        type="radio"
                        bind:group={formData.confidentiality.internalExternal}
                        value={option.value}
                        class="mr-3 text-blue-600 focus:ring-blue-500"
                      />
                      <span class="text-sm font-medium text-gray-900"
                        >{option.label}</span
                      >
                    </label>
                  {/each}
                </div>
              </fieldset>
            </div>

            <div>
              <fieldset>
                <legend class="block text-sm font-medium text-gray-700 mb-2">
                  重要度 <span class="text-red-500">*</span>
                </legend>
                <div class="grid grid-cols-2 gap-4">
                  {#each importanceClassOptions as option}
                    <label
                      class="flex items-center p-3 border rounded-md cursor-pointer hover:bg-gray-50
                                {formData.confidentiality.importanceClass ===
                      option.value
                        ? 'border-blue-500 bg-blue-50'
                        : 'border-gray-300'}"
                    >
                      <input
                        type="radio"
                        bind:group={formData.confidentiality.importanceClass}
                        value={option.value}
                        class="mr-3 text-blue-600 focus:ring-blue-500"
                      />
                      <span class="text-sm font-medium text-gray-900"
                        >{option.label}</span
                      >
                    </label>
                  {/each}
                </div>
              </fieldset>
            </div>

            <div>
              <fieldset>
                <legend class="block text-sm font-medium text-gray-700 mb-2">
                  個人情報 <span class="text-red-500">*</span>
                </legend>
                <div class="grid grid-cols-2 gap-4">
                  {#each personalInfoOptions as option}
                    <label
                      class="flex items-center p-3 border rounded-md cursor-pointer hover:bg-gray-50
                                {formData.confidentiality.personalInfo ===
                      option.value
                        ? 'border-blue-500 bg-blue-50'
                        : 'border-gray-300'}"
                    >
                      <input
                        type="radio"
                        bind:group={formData.confidentiality.personalInfo}
                        value={option.value}
                        class="mr-3 text-blue-600 focus:ring-blue-500"
                      />
                      <span class="text-sm font-medium text-gray-900"
                        >{option.label}</span
                      >
                    </label>
                  {/each}
                </div>
              </fieldset>
            </div>

            <!-- 機密レベルプレビュー -->
            <div class="p-3 bg-gray-50 rounded-md">
              <div class="flex items-center justify-between">
                <span class="text-sm font-medium text-gray-700"
                  >設定された機密レベル:</span
                >
                <span
                  class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border {getConfidentialityColor()}"
                >
                  {getConfidentialityLabel()}
                </span>
              </div>
            </div>
          </div>
        </div>

        <!-- 備考 -->
        <div class="bg-white shadow rounded-lg p-6">
          <h2 class="text-lg font-medium text-gray-900 mb-4">備考</h2>

          <TextArea
            bind:value={formData.notes}
            error={errors.notes}
            placeholder="必要に応じて備考を入力してください..."
            rows={4}
            maxlength={1000}
          />
        </div>

        <!-- 送信ボタン -->
        <div class="bg-white shadow rounded-lg p-6">
          <div class="flex justify-end space-x-4">
            <Button
              type="button"
              variant="secondary"
              on:click={handleCancel}
              disabled={isSubmitting}
            >
              キャンセル
            </Button>
            <Button
              type="submit"
              loading={isSubmitting}
              disabled={!formData.title || !formData.documentTypeId}
            >
              {isSubmitting ? "作成中..." : "文書を作成"}
            </Button>
          </div>
        </div>
      </form>
    </div>

    <!-- プレビューサイドバー -->
    {#if showPreview}
      <div class="lg:col-span-1">
        <div class="bg-white shadow rounded-lg p-6 sticky top-6">
          <h3 class="text-lg font-medium text-gray-900 mb-4">プレビュー</h3>

          <div class="space-y-4 text-sm">
            <div>
              <span class="font-medium text-gray-700">タイトル:</span>
              <p class="mt-1 text-gray-900">{formData.title || "（未入力）"}</p>
            </div>

            <div>
              <span class="font-medium text-gray-700">文書番号:</span>
              <p class="mt-1 font-mono text-blue-600">
                {generatedNumber || "（文書種別と作成日を選択してください）"}
              </p>
            </div>

            <div>
              <span class="font-medium text-gray-700">文書種別:</span>
              <p class="mt-1 text-gray-900">
                {documentTypeOptions.find(
                  (opt) => opt.value === formData.documentTypeId,
                )?.label || "（未選択）"}
              </p>
            </div>

            {#if formData.businessNumber}
              <div>
                <span class="font-medium text-gray-700">業務番号:</span>
                <p class="mt-1 text-gray-900">{formData.businessNumber}</p>
              </div>
            {/if}

            <div>
              <span class="font-medium text-gray-700">作成日:</span>
              <p class="mt-1 text-gray-900">
                {formData.createdDate || "（未入力）"}
              </p>
            </div>

            <div>
              <span class="font-medium text-gray-700">機密レベル:</span>
              <div class="mt-1">
                <span
                  class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border {getConfidentialityColor()}"
                >
                  {getConfidentialityLabel()}
                </span>
              </div>
            </div>

            {#if formData.notes}
              <div>
                <span class="font-medium text-gray-700">備考:</span>
                <p class="mt-1 text-gray-900 whitespace-pre-wrap">
                  {formData.notes}
                </p>
              </div>
            {/if}
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>
