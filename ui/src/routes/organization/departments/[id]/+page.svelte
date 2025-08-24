<script lang="ts">
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import { graphqlClient } from "$lib/api/client";
  import {
    GET_DEPARTMENT,
    type DepartmentWithManager,
  } from "$lib/api/queries/departments";

  // パラメータから部署IDを取得
  $: departmentId = $page.params.id;

  // 状態管理
  let department: DepartmentWithManager | null = null;
  let isLoading = true;
  let error = "";
  let activeTab = "overview";

  // 部署データ読み込み
  async function loadDepartment() {
    if (!departmentId) return;

    isLoading = true;
    error = "";

    try {
      const response = await graphqlClient.request(GET_DEPARTMENT, {
        id: parseInt(departmentId),
      });

      if (!response.department) {
        error = "指定された部署が見つかりません。";
        return;
      }

      department = response.department;
    } catch (err) {
      error = "部署データの読み込みに失敗しました。";
      console.error("Failed to load department:", err);
    } finally {
      isLoading = false;
    }
  }

  // 初期化
  onMount(() => {
    loadDepartment();
  });

  // 部署IDが変わったら再読み込み
  $: if (departmentId) {
    loadDepartment();
  }

  // 編集ページへ遷移
  function editDepartment() {
    goto(`/organization/departments/${departmentId}/edit`);
  }
</script>

<svelte:head>
  <title>
    {department ? `${department.name} - 部署詳細` : "部署詳細"} | 文書管理システム
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
      <Button on:click={loadDepartment} variant="primary">再試行</Button>
    </div>
  {:else if department}
    <!-- ページヘッダー -->
    <div class="bg-white shadow rounded-lg">
      <div class="px-6 py-4">
        <div class="flex items-center justify-between">
          <div>
            <h1 class="text-2xl font-bold text-gray-900">
              {department.name}
              <span class="text-sm font-normal text-gray-500 ml-2"
                >({department.code})</span
              >
            </h1>
            <div class="mt-2 flex items-center space-x-4">
              <span
                class="inline-flex px-2 py-1 text-xs font-semibold rounded-full {department.isActive
                  ? 'bg-green-100 text-green-800'
                  : 'bg-red-100 text-red-800'}"
              >
                {department.isActive ? "アクティブ" : "無効"}
              </span>
              <span class="text-sm text-gray-500"
                >従業員数: {department.employeeCount}名</span
              >
            </div>
          </div>
          <div class="flex space-x-2">
            <Button on:click={editDepartment} variant="primary">編集</Button>
          </div>
        </div>
      </div>
    </div>

    <!-- タブナビゲーション -->
    <div class="bg-white shadow rounded-lg">
      <div class="border-b border-gray-200">
        <nav class="-mb-px flex space-x-8 px-6">
          <button
            type="button"
            class="py-4 px-1 border-b-2 font-medium text-sm {activeTab ===
            'overview'
              ? 'border-blue-500 text-blue-600'
              : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}"
            on:click={() => (activeTab = "overview")}
          >
            概要
          </button>
          <button
            type="button"
            class="py-4 px-1 border-b-2 font-medium text-sm {activeTab ===
            'members'
              ? 'border-blue-500 text-blue-600'
              : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}"
            on:click={() => (activeTab = "members")}
          >
            メンバー
          </button>
        </nav>
      </div>

      <div class="p-6">
        {#if activeTab === "overview"}
          <!-- 概要タブ -->
          <div class="space-y-6">
            <!-- 基本情報 -->
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div>
                <h3 class="text-lg font-medium text-gray-900 mb-4">基本情報</h3>
                <dl class="space-y-3">
                  <div>
                    <dt class="text-sm font-medium text-gray-500">
                      部署コード
                    </dt>
                    <dd class="text-sm text-gray-900">{department.code}</dd>
                  </div>
                  <div>
                    <dt class="text-sm font-medium text-gray-500">部署名</dt>
                    <dd class="text-sm text-gray-900">{department.name}</dd>
                  </div>
                  {#if department.parentName}
                    <div>
                      <dt class="text-sm font-medium text-gray-500">
                        上位部署
                      </dt>
                      <dd class="text-sm text-gray-900">
                        {department.parentName}
                      </dd>
                    </div>
                  {/if}
                  <div>
                    <dt class="text-sm font-medium text-gray-500">レベル</dt>
                    <dd class="text-sm text-gray-900">{department.level}</dd>
                  </div>
                  {#if department.managerName}
                    <div>
                      <dt class="text-sm font-medium text-gray-500">責任者</dt>
                      <dd class="text-sm text-gray-900">
                        {department.managerName}
                      </dd>
                    </div>
                  {/if}
                  <div>
                    <dt class="text-sm font-medium text-gray-500">従業員数</dt>
                    <dd class="text-sm text-gray-900">
                      {department.employeeCount}名
                    </dd>
                  </div>
                  <div>
                    <dt class="text-sm font-medium text-gray-500">作成日</dt>
                    <dd class="text-sm text-gray-900">
                      {department.createdDate || "未設定"}
                    </dd>
                  </div>
                </dl>
              </div>

              <div>
                <h3 class="text-lg font-medium text-gray-900 mb-4">連絡先</h3>
                <dl class="space-y-3">
                  {#if department.location}
                    <div>
                      <dt class="text-sm font-medium text-gray-500">所在地</dt>
                      <dd class="text-sm text-gray-900">
                        {department.location}
                      </dd>
                    </div>
                  {/if}
                  {#if department.phoneNumber}
                    <div>
                      <dt class="text-sm font-medium text-gray-500">
                        電話番号
                      </dt>
                      <dd class="text-sm text-gray-900">
                        <a
                          href="tel:{department.phoneNumber}"
                          class="text-blue-600 hover:text-blue-900"
                        >
                          {department.phoneNumber}
                        </a>
                      </dd>
                    </div>
                  {/if}
                  {#if department.email}
                    <div>
                      <dt class="text-sm font-medium text-gray-500">
                        メールアドレス
                      </dt>
                      <dd class="text-sm text-gray-900">
                        <a
                          href="mailto:{department.email}"
                          class="text-blue-600 hover:text-blue-900"
                        >
                          {department.email}
                        </a>
                      </dd>
                    </div>
                  {/if}
                  {#if department.budget}
                    <div>
                      <dt class="text-sm font-medium text-gray-500">予算</dt>
                      <dd class="text-sm text-gray-900">
                        ¥{department.budget.toLocaleString()}
                      </dd>
                    </div>
                  {/if}
                </dl>
              </div>
            </div>

            <!-- 説明 -->
            {#if department.description}
              <div>
                <h3 class="text-lg font-medium text-gray-900 mb-4">説明</h3>
                <div class="bg-gray-50 rounded-lg p-4">
                  <p class="text-sm text-gray-700 whitespace-pre-wrap">
                    {department.description}
                  </p>
                </div>
              </div>
            {/if}
          </div>
        {:else if activeTab === "members"}
          <!-- メンバータブ -->
          <div>
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
                  d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"
                />
              </svg>
              <h3 class="mt-2 text-sm font-medium text-gray-900">
                メンバー情報
              </h3>
              <p class="mt-1 text-sm text-gray-500">
                メンバー管理機能は開発中です。従業員数: {department.employeeCount}名
              </p>
            </div>
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>
