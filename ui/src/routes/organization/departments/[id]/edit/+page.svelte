<script lang="ts">
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import Button from "$lib/components/ui/Button.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import Select from "$lib/components/ui/Select.svelte";
  import SearchableSelect from "$lib/components/ui/SearchableSelect.svelte";
  import { graphqlClient } from "$lib/api/client";
  import {
    GET_DEPARTMENT,
    UPDATE_DEPARTMENT,
    GET_DEPARTMENTS,
    type UpdateDepartmentInput,
    type DepartmentWithManager,
  } from "$lib/api/queries/departments";
  import { onMount } from "svelte";

  // パラメータから部署IDを取得
  $: departmentId = $page.params.id;

  // フォームの状態管理
  let formData: UpdateDepartmentInput = {
    code: "",
    name: "",
    parentId: undefined,
    managerId: undefined,
    description: "",
    location: "",
    phoneNumber: "",
    email: "",
    budget: undefined,
    isActive: true,
  };

  // 状態管理
  let isLoading = true;
  let isSubmitting = false;
  let error = "";
  let formErrors: Record<string, string> = {};

  // 選択肢データ
  let departments: DepartmentWithManager[] = [];
  let employees: Array<{ id: number; name: string }> = [];
  let originalDepartment: DepartmentWithManager | null = null;

  // 部署データを読み込み
  async function loadDepartment() {
    if (!departmentId) return;

    try {
      isLoading = true;
      error = "";

      const response = await graphqlClient.request(GET_DEPARTMENT, {
        id: parseInt(departmentId || ""),
      });

      if (!(response as any).department) {
        error = "指定された部署が見つかりません。";
        return;
      }

      originalDepartment = (response as any).department;

      // フォームデータに設定
      if (originalDepartment) {
        formData = {
          code: originalDepartment.code,
          name: originalDepartment.name,
          parentId: originalDepartment.parentId,
          managerId: originalDepartment.managerId,
          description: originalDepartment.description || "",
          location: originalDepartment.location || "",
          phoneNumber: originalDepartment.phoneNumber || "",
          email: originalDepartment.email || "",
          budget: originalDepartment.budget,
          isActive: originalDepartment.isActive,
        };
      }
    } catch (err) {
      console.error("Failed to load department:", err);
      error = "部署データの読み込みに失敗しました。";
    } finally {
      isLoading = false;
    }
  }

  // 部署一覧を読み込み（上位部署選択用）
  async function loadDepartments() {
    try {
      const response = await graphqlClient.request(GET_DEPARTMENTS);
      // 自分自身と子部署は除外
      departments = (response as any).departments.filter(
        (dept: DepartmentWithManager) => dept.id !== parseInt(departmentId || ""),
      );
    } catch (err) {
      console.error("Failed to load departments:", err);
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
    if (!formData.code?.trim()) {
      formErrors.code = "部署コードは必須です";
    } else if (!/^[A-Z0-9_-]+$/.test(formData.code)) {
      formErrors.code = "部署コードは英大文字、数字、_、-のみ使用可能です";
    }

    if (!formData.name?.trim()) {
      formErrors.name = "部署名は必須です";
    }

    // メールアドレス形式チェック
    if (formData.email && !/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(formData.email)) {
      formErrors.email = "正しいメールアドレス形式で入力してください";
    }

    // 予算の数値チェック
    if (
      formData.budget !== null &&
      formData.budget !== undefined &&
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
      // 空文字列をnullに変換し、変更されたフィールドのみ送信
      const cleanedData: UpdateDepartmentInput = {};

      if (formData.code !== originalDepartment?.code) {
        cleanedData.code = formData.code?.trim() || undefined;
      }
      if (formData.name !== originalDepartment?.name) {
        cleanedData.name = formData.name?.trim() || undefined;
      }
      if (formData.parentId !== originalDepartment?.parentId) {
        cleanedData.parentId = formData.parentId;
      }
      if (formData.managerId !== originalDepartment?.managerId) {
        cleanedData.managerId = formData.managerId;
      }
      if (formData.description !== (originalDepartment?.description || "")) {
        cleanedData.description = formData.description?.trim() || undefined;
      }
      if (formData.location !== (originalDepartment?.location || "")) {
        cleanedData.location = formData.location?.trim() || undefined;
      }
      if (formData.phoneNumber !== (originalDepartment?.phoneNumber || "")) {
        cleanedData.phoneNumber = formData.phoneNumber?.trim() || undefined;
      }
      if (formData.email !== (originalDepartment?.email || "")) {
        cleanedData.email = formData.email?.trim() || undefined;
      }
      if (formData.budget !== originalDepartment?.budget) {
        cleanedData.budget = formData.budget;
      }
      if (formData.isActive !== originalDepartment?.isActive) {
        cleanedData.isActive = formData.isActive;
      }

      await graphqlClient.request(UPDATE_DEPARTMENT, {
        id: parseInt(departmentId || ""),
        input: cleanedData,
      });

      // 成功したら部署詳細ページに戻る
      goto(`/organization/departments/${departmentId}`);
    } catch (err) {
      console.error("Failed to update department:", err);
      error = "部署の更新に失敗しました。入力内容を確認してください。";
    } finally {
      isSubmitting = false;
    }
  }

  // キャンセル
  function handleCancel() {
    goto(`/organization/departments/${departmentId}`);
  }

  // 初期化
  onMount(() => {
    loadDepartment();
    loadDepartments();
    loadEmployees();
  });

  // 部署IDが変わったら再読み込み
  $: if (departmentId) {
    loadDepartment();
  }
</script>

<svelte:head>
  <title>
    {originalDepartment ? `${originalDepartment.name} - 部署編集` : "部署編集"} |
    文書管理システム
  </title>
</svelte:head>

<div class="space-y-6">
  {#if isLoading}
    <!-- ローディング表示 -->
    <div class="text-center py-12">
      <div class="inline-flex items-center">
        <svg
          class="animate-spin -ml-1 mr-3 h-5 w-5 text-gray-600"
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
        部署データを読み込み中...
      </div>
    </div>
  {:else if error && !originalDepartment}
    <!-- エラー表示 -->
    <div class="text-center py-12">
      <div class="text-red-600 mb-4">
        <svg
          class="mx-auto h-12 w-12"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.864-.833-2.634 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"
          />
        </svg>
      </div>
      <h3 class="text-lg font-medium text-gray-900 mb-2">
        エラーが発生しました
      </h3>
      <p class="text-sm text-gray-500 mb-4">{error}</p>
      <Button on:click={loadDepartment} variant="primary">再試行</Button>
    </div>
  {:else if originalDepartment}
    <!-- ページヘッダー -->
    <div class="sm:flex sm:items-center sm:justify-between">
      <div>
        <h1 class="text-2xl font-bold text-gray-900">
          {originalDepartment.name} の編集
          <span class="text-sm font-normal text-gray-500 ml-2"
            >({originalDepartment.code})</span
          >
        </h1>
        <p class="mt-2 text-sm text-gray-700">部署の情報を編集してください</p>
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
            <div class="md:col-span-1">
              <Select
                label="状態"
                bind:value={formData.isActive}
                options={[
                  { value: true, label: "アクティブ" },
                  { value: false, label: "無効" },
                ]}
              />
            </div>
            <div></div>
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
          <Button type="submit" variant="primary" disabled={isSubmitting}>
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
              更新中...
            {:else}
              変更を保存
            {/if}
          </Button>
        </div>
      </form>
    </div>
  {/if}
</div>
