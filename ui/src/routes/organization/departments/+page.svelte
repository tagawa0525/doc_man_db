<script lang="ts">
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import Button from "$lib/components/ui/Button.svelte";
  import Input from "$lib/components/ui/Input.svelte";
  import ResponsiveTable from "$lib/components/mobile/ResponsiveTable.svelte";
  import { graphqlClient } from "$lib/api/client";
  import {
    GET_DEPARTMENTS,
    type DepartmentWithManager,
  } from "$lib/api/queries/departments";

  // 部署データ
  let departments: DepartmentWithManager[] = [];
  let filteredDepartments: DepartmentWithManager[] = [];

  // 状態管理
  let isLoading = true;
  let searchTerm = "";
  let error: string | null = null;

  // テーブルヘッダー定義
  const headers = [
    { key: "code", label: "部署コード", sortable: true },
    { key: "name", label: "部署名", sortable: true },
    { key: "description", label: "説明", sortable: false, mobileHidden: true },
    { key: "managerName", label: "責任者", sortable: true, mobileHidden: true },
    { key: "employeeCount", label: "人数", sortable: true },
    { key: "status", label: "状態", sortable: true },
    { key: "actions", label: "操作", sortable: false, class: "w-32" },
  ];

  // データ読み込み
  async function loadDepartments() {
    try {
      isLoading = true;
      error = null;

      const response = await graphqlClient.request(GET_DEPARTMENTS);
      departments = (response as any).departments;
      filteredDepartments = departments;
    } catch (err) {
      console.error("Failed to load departments:", err);
      error = "部署データの読み込みに失敗しました";
    } finally {
      isLoading = false;
    }
  }

  // 検索処理
  function handleSearch() {
    filteredDepartments = departments.filter(
      (dept) =>
        dept.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
        dept.code.toLowerCase().includes(searchTerm.toLowerCase()) ||
        (dept.managerName &&
          dept.managerName.toLowerCase().includes(searchTerm.toLowerCase())),
    );
  }

  // 検索クリア
  function clearSearch() {
    searchTerm = "";
    filteredDepartments = departments;
  }

  // 行クリック（詳細ページへ）
  function handleRowClick(department: any) {
    goto(`/organization/departments/${department.id}`);
  }

  // ソート処理
  function handleSort(key: string, direction: string) {
    filteredDepartments.sort((a: any, b: any) => {
      const aVal = a[key];
      const bVal = b[key];

      if (typeof aVal === "string" && typeof bVal === "string") {
        return direction === "asc"
          ? aVal.localeCompare(bVal)
          : bVal.localeCompare(aVal);
      } else {
        return direction === "asc" ? aVal - bVal : bVal - aVal;
      }
    });
    filteredDepartments = [...filteredDepartments];
  }

  // 新規部署作成
  function goToCreateDepartment() {
    goto("/organization/departments/create");
  }

  // 初期化
  onMount(() => {
    loadDepartments();
  });

  // 検索実行
  $: if (searchTerm !== undefined) {
    handleSearch();
  }
</script>

<div class="space-y-6">
  <!-- ページヘッダー -->
  <div class="sm:flex sm:items-center sm:justify-between">
    <div>
      <h1 class="text-2xl font-bold text-gray-900">部署管理</h1>
      <p class="mt-2 text-sm text-gray-700">
        組織の部署情報を管理・編集できます
      </p>
    </div>
    <div class="mt-4 sm:mt-0">
      <Button on:click={goToCreateDepartment} variant="primary">
        新規部署作成
      </Button>
    </div>
  </div>

  <!-- 検索フィルター -->
  <div class="bg-white shadow rounded-lg">
    <div class="px-6 py-4 border-b border-gray-200">
      <h2 class="text-lg font-medium text-gray-900">部署検索</h2>
    </div>

    <div class="p-6">
      <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
        <div class="md:col-span-2">
          <Input
            label="部署名・コード・責任者"
            bind:value={searchTerm}
            placeholder="検索キーワード..."
          />
        </div>

        <div class="flex items-end space-x-2">
          <Button on:click={handleSearch} variant="primary">検索</Button>
          <Button on:click={clearSearch} variant="secondary">クリア</Button>
        </div>
      </div>
    </div>
  </div>

  <!-- 検索結果 -->
  <div class="bg-white shadow rounded-lg">
    <div class="px-6 py-4 border-b border-gray-200">
      <div class="flex items-center justify-between">
        <h2 class="text-lg font-medium text-gray-900">
          部署一覧
          {#if filteredDepartments.length > 0}
            <span class="text-sm text-gray-500"
              >（{filteredDepartments.length}件）</span
            >
          {/if}
        </h2>

        {#if isLoading}
          <div class="text-sm text-gray-500">検索中...</div>
        {/if}
      </div>
    </div>

    <div class="p-6">
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
      {:else if error}
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
          <Button on:click={loadDepartments} variant="primary">再試行</Button>
        </div>
      {:else}
        <!-- テーブル表示 -->
        <ResponsiveTable
          {headers}
          data={filteredDepartments}
          onRowClick={handleRowClick}
          onSort={handleSort}
        >
          <svelte:fragment slot="cell" let:item let:header>
            {#if header.key === "status"}
              <span
                class="inline-flex px-2 py-1 text-xs font-semibold rounded-full {item.isActive
                  ? 'bg-green-100 text-green-800'
                  : 'bg-red-100 text-red-800'}"
              >
                {item.isActive ? "アクティブ" : "無効"}
              </span>
            {:else if header.key === "actions"}
              <div class="flex space-x-2">
                <button
                  type="button"
                  class="text-blue-600 hover:text-blue-900 text-sm"
                  on:click|stopPropagation={() =>
                    goto(`/organization/departments/${item.id}/edit`)}
                >
                  編集
                </button>
              </div>
            {:else}
              {item[header.key] || "-"}
            {/if}
          </svelte:fragment>

          <svelte:fragment slot="mobile-card" let:item>
            <div class="space-y-2">
              <div class="flex items-center justify-between">
                <div class="font-medium text-gray-900">
                  {item.name} ({item.code})
                </div>
                <span
                  class="inline-flex px-2 py-1 text-xs font-semibold rounded-full {item.isActive
                    ? 'bg-green-100 text-green-800'
                    : 'bg-red-100 text-red-800'}"
                >
                  {item.isActive ? "アクティブ" : "無効"}
                </span>
              </div>
              <div class="text-sm text-gray-500">
                責任者: {item.managerName || "未設定"} | 人数: {item.employeeCount}名
              </div>
              <div class="text-sm text-gray-500">
                {item.description}
              </div>
              <div class="flex justify-end space-x-2">
                <button
                  type="button"
                  class="text-blue-600 hover:text-blue-900 text-sm"
                  on:click|stopPropagation={() =>
                    goto(`/organization/departments/${item.id}/edit`)}
                >
                  編集
                </button>
              </div>
            </div>
          </svelte:fragment>

          <svelte:fragment slot="empty">
            <div class="text-center py-12">
              <svg
                class="mx-auto h-12 w-12 text-gray-400"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-4m-5 0H3m2 0h3M9 7h6m-6 4h6m-2 4h2M9 15h4"
                />
              </svg>
              <h3 class="mt-2 text-sm font-medium text-gray-900">
                部署が見つかりません
              </h3>
              <p class="mt-1 text-sm text-gray-500">
                検索条件を変更するか、新しい部署を作成してください。
              </p>
              <div class="mt-6">
                <Button on:click={goToCreateDepartment} variant="primary">
                  新規部署作成
                </Button>
              </div>
            </div>
          </svelte:fragment>
        </ResponsiveTable>
      {/if}
    </div>
  </div>
</div>

<style>
  /* カスタムスタイル（必要に応じて） */
</style>
