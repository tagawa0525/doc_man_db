<script lang="ts">
  import { goto } from "$app/navigation";
  import Button from "$lib/components/ui/Button.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import SearchableSelect from "$lib/components/ui/SearchableSelect.svelte";
  import { graphqlClient } from "$lib/api/client";
  import {
    CREATE_DEPARTMENT,
    GET_DEPARTMENTS,
    type CreateDepartmentInput,
    type DepartmentWithManager,
  } from "$lib/api/queries/departments";
  import { onMount } from "svelte";

  // フォームの状態管理
  let formData: CreateDepartmentInput = {
    code: "",
    name: "",
    parentId: undefined,
    managerId: undefined,
    description: "",
    location: "",
    phoneNumber: "",
    email: "",
    budget: undefined,
    createdDate: new Date().toISOString().split("T")[0], // 今日の日付をデフォルト
  };

  // 状態管理
  let isLoading = false;
  let isSubmitting = false;
  let error = "";
  let formErrors: Record<string, string> = {};

  // 選択肢データ
  let departments: DepartmentWithManager[] = [];
  let employees: Array<{ id: number; name: string }> = [];

  // 部署一覧を読み込み（上位部署選択用）
  async function loadDepartments() {
    try {
      isLoading = true;
      error = "";

      const response = await graphqlClient.request(GET_DEPARTMENTS);
      departments = (response as any).departments;
    } catch (err) {
      console.error("Failed to load departments:", err);
      error = "部署データの読み込みに失敗しました";
    } finally {
      isLoading = false;
    }
  }

  // 従業員一覧を読み込み（責任者選択用）
  async function loadEmployees() {
    try {
      // TODO: 従業員取得APIが実装されたら置き換える
      employees = [
        { id: 1, name: "山田太郎" },
        { id: 2, name: "佐藤花子" },
        { id: 3, name: "田中一郎" },
        { id: 4, name: "鈴木二郎" },
        { id: 11, name: "鈴木次郎" },
        { id: 12, name: "田村美子" },
        { id: 13, name: "高橋健太" },
        { id: 14, name: "松本美香" },
        { id: 15, name: "渡辺聡子" },
        { id: 16, name: "小林達也" },
        { id: 17, name: "伊藤明子" },
      ];
    } catch (err) {
      console.error("Failed to load employees:", err);
    }
  }

  // バリデーション
  function validateForm(): boolean {
    formErrors = {};

    // 必須項目チェック
    if (!formData.code.trim()) {
      formErrors.code = "部署コードは必須です";
    } else if (!/^[A-Z0-9_-]+$/.test(formData.code)) {
      formErrors.code = "部署コードは英大文字、数字、_、-のみ使用可能です";
    }

    if (!formData.name.trim()) {
      formErrors.name = "部署名は必須です";
    }

    // メールアドレス形式チェック
    if (formData.email && !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(formData.email)) {
      formErrors.email = "正しいメールアドレス形式で入力してください";
    }

    // 予算の数値チェック
    if (
      formData.budget !== undefined &&
      formData.budget !== null &&
      formData.budget < 0
    ) {
      formErrors.budget = "予算は0以上の値を入力してください";
    }

    return Object.keys(formErrors).length === 0;
  }

  // フォーム送信
  async function handleSubmit() {
    if (!validateForm()) {
      return;
    }

    isSubmitting = true;
    error = "";

    try {
      // 空文字列をnullに変換
      const cleanedData = {
        ...formData,
        description: formData.description?.trim() || null,
        location: formData.location?.trim() || null,
        phoneNumber: formData.phoneNumber?.trim() || null,
        email: formData.email?.trim() || null,
      };

      await graphqlClient.request(CREATE_DEPARTMENT, {
        input: cleanedData,
      });

      // 成功したら部署一覧ページに戻る
      goto("/organization/departments");
    } catch (err) {
      console.error("Failed to create department:", err);
      error = "部署の作成に失敗しました。入力内容を確認してください。";
    } finally {
      isSubmitting = false;
    }
  }

  // キャンセル
  function handleCancel() {
    goto("/organization/departments");
  }

  // 初期化
  onMount(() => {
    loadDepartments();
    loadEmployees();
  });
</script>

<svelte:head>
  <title>新規部署作成 | 文書管理システム</title>
</svelte:head>

<div class="space-y-6">
  <!-- ページヘッダー -->
  <div class="sm:flex sm:items-center sm:justify-between">
    <div>
      <h1 class="text-2xl font-bold text-gray-900">新規部署作成</h1>
      <p class="mt-2 text-sm text-gray-700">
        新しい部署の情報を入力してください
      </p>
    </div>
  </div>

  <!-- フォーム -->
  <div class="bg-white shadow rounded-lg">
    <form on:submit|preventDefault={handleSubmit} class="space-y-6 p-6">
      <!-- 基本情報セクション -->
      <div>
        <h2 class="text-lg font-medium text-gray-900 mb-4">基本情報</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="md:col-span-1">
            <Input
              label="部署コード"
              bind:value={formData.code}
              placeholder="例: DEV, SALES"
              required
              error={formErrors.code}
            />
          </div>
          <div class="md:col-span-1">
            <Input
              label="部署名"
              bind:value={formData.name}
              placeholder="例: 開発部"
              required
              error={formErrors.name}
            />
          </div>
          <div class="md:col-span-1">
            <SearchableSelect
              label="上位部署"
              bind:value={formData.parentId}
              placeholder="部署を検索..."
              options={[
                { value: null, label: "なし（トップレベル）" },
                ...departments.map((dept) => ({
                  value: dept.id,
                  label: `${dept.name} (${dept.code})`,
                })),
              ]}
            />
          </div>
          <div class="md:col-span-1">
            <SearchableSelect
              label="責任者"
              bind:value={formData.managerId}
              placeholder="責任者を検索..."
              options={[
                { value: null, label: "未設定" },
                ...employees.map((emp) => ({
                  value: emp.id,
                  label: emp.name,
                })),
              ]}
            />
          </div>
          <div class="md:col-span-2">
            <label
              for="description"
              class="block text-sm font-medium text-gray-700 mb-2"
            >
              説明
            </label>
            <textarea
              id="description"
              bind:value={formData.description}
              rows="3"
              class="w-full px-3 py-2 border border-gray-300 rounded-lg shadow-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              placeholder="部署の役割や業務内容を記述してください"
            ></textarea>
          </div>
        </div>
      </div>

      <!-- 連絡先情報セクション -->
      <div>
        <h2 class="text-lg font-medium text-gray-900 mb-4">連絡先情報</h2>
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="md:col-span-1">
            <Input
              label="所在地"
              bind:value={formData.location}
              placeholder="例: 本社3F"
            />
          </div>
          <div class="md:col-span-1">
            <Input
              label="電話番号"
              bind:value={formData.phoneNumber}
              placeholder="例: 03-1234-5600"
            />
          </div>
          <div class="md:col-span-1">
            <Input
              label="メールアドレス"
              type="email"
              bind:value={formData.email}
              placeholder="例: dept@company.com"
              error={formErrors.email}
            />
          </div>
          <div class="md:col-span-1">
            <Input
              label="予算"
              type="number"
              bind:value={formData.budget}
              placeholder="例: 50000000"
              error={formErrors.budget}
            />
          </div>
          <div class="md:col-span-1">
            <Input
              label="設立日"
              type="date"
              bind:value={formData.createdDate}
            />
          </div>
        </div>
      </div>

      <!-- エラー表示 -->
      {#if error}
        <div class="bg-red-50 border border-red-200 rounded-lg p-4">
          <div class="flex">
            <div class="flex-shrink-0">
              <svg
                class="h-5 w-5 text-red-400"
                viewBox="0 0 20 20"
                fill="currentColor"
              >
                <path
                  fill-rule="evenodd"
                  d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z"
                  clip-rule="evenodd"
                />
              </svg>
            </div>
            <div class="ml-3">
              <h3 class="text-sm font-medium text-red-800">エラー</h3>
              <div class="mt-2 text-sm text-red-700">
                <p>{error}</p>
              </div>
            </div>
          </div>
        </div>
      {/if}

      <!-- アクションボタン -->
      <div class="flex justify-end space-x-3 pt-6 border-t border-gray-200">
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
          variant="primary"
          disabled={isSubmitting || isLoading}
        >
          {#if isSubmitting}
            <svg
              class="animate-spin -ml-1 mr-3 h-5 w-5 text-white"
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
            作成中...
          {:else}
            部署を作成
          {/if}
        </Button>
      </div>
    </form>
  </div>
</div>
