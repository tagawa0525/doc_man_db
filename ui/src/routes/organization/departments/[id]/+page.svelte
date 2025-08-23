<script lang="ts">
  import { page } from "$app/stores";
  import { onMount } from "svelte";
  import Button from "$lib/components/ui/Button.svelte";

  // パラメータから部署IDを取得
  $: departmentId = $page.params.id;

  // 状態管理
  let department: any = null;
  let employees: any[] = [];
  let isLoading = true;
  let error = "";
  let showEditDialog = false;
  let activeTab = "overview";

  // 仮の部署データ
  const mockDepartments: Record<string, any> = {
    "1": {
      id: 1,
      name: "情報システム部",
      code: "IS",
      parentId: null,
      parentName: null,
      level: 0,
      managerName: "田中部長",
      managerId: 10,
      employeeCount: 15,
      isActive: true,
      createdDate: "2020-04-01",
      description:
        "システム開発・運用・保守を担当する部署です。\n社内のデジタル化推進と情報セキュリティの維持を主な業務としています。",
      budget: "50,000,000",
      location: "本社3F",
      phoneNumber: "03-1234-5600",
      email: "info-sys@company.com",
      responsibilities: [
        "システム開発・保守",
        "情報セキュリティ管理",
        "IT戦略立案",
        "デジタル化推進",
        "ユーザーサポート",
      ],
      kpis: [
        {
          name: "システム稼働率",
          value: "99.9%",
          target: "99.5%",
          status: "good",
        },
        {
          name: "プロジェクト完了率",
          value: "95%",
          target: "90%",
          status: "good",
        },
        {
          name: "ユーザー満足度",
          value: "4.2/5.0",
          target: "4.0/5.0",
          status: "good",
        },
        {
          name: "セキュリティ事故件数",
          value: "0件",
          target: "0件",
          status: "good",
        },
      ],
    },
    "2": {
      id: 2,
      name: "システム開発課",
      code: "IS-DEV",
      parentId: 1,
      parentName: "情報システム部",
      level: 1,
      managerName: "山田課長",
      managerId: 1,
      employeeCount: 8,
      isActive: true,
      createdDate: "2020-04-01",
      description: "アプリケーション開発とシステム設計を担当する課です。",
      budget: "25,000,000",
      location: "本社3F-A",
      phoneNumber: "03-1234-5601",
      email: "dev@company.com",
      responsibilities: [
        "Webアプリケーション開発",
        "データベース設計",
        "システム設計",
        "コードレビュー",
        "技術検証",
      ],
      kpis: [
        {
          name: "コード品質スコア",
          value: "8.5/10",
          target: "8.0/10",
          status: "good",
        },
        { name: "開発速度", value: "120%", target: "100%", status: "good" },
        { name: "バグ発見率", value: "2%", target: "5%", status: "good" },
      ],
    },
  };

  // 部署の社員データ
  const departmentEmployees: Record<string, any[]> = {
    "1": [
      {
        id: 10,
        name: "田中部長",
        position: "部長",
        email: "tanaka.manager@company.com",
        hireDate: "2015-04-01",
        roles: ["管理者", "承認者"],
      },
    ],
    "2": [
      {
        id: 1,
        name: "山田太郎",
        position: "課長",
        email: "yamada@company.com",
        hireDate: "2018-04-01",
        roles: ["承認者", "文書作成"],
      },
      {
        id: 2,
        name: "佐藤花子",
        position: "主任",
        email: "sato@company.com",
        hireDate: "2020-04-01",
        roles: ["文書作成"],
      },
      {
        id: 11,
        name: "鈴木次郎",
        position: "係長",
        email: "suzuki@company.com",
        hireDate: "2019-07-01",
        roles: ["文書作成"],
      },
      {
        id: 12,
        name: "田村美子",
        position: "主任",
        email: "tamura@company.com",
        hireDate: "2021-04-01",
        roles: ["文書作成"],
      },
    ],
  };

  // 部署データ読み込み
  async function loadDepartment() {
    isLoading = true;
    error = "";

    try {
      await new Promise((resolve) => setTimeout(resolve, 1000));

      const dept = departmentId ? mockDepartments[departmentId] : null;
      if (!dept) {
        error = "指定された部署が見つかりません。";
        return;
      }

      department = dept;
      employees = departmentId ? departmentEmployees[departmentId] || [] : [];
    } catch (err) {
      error = "部署データの読み込みに失敗しました。";
      console.error("Failed to load department:", err);
    } finally {
      isLoading = false;
    }
  }

  // KPIステータスの色
  function getKpiStatusColor(status: string): string {
    switch (status) {
      case "good":
        return "text-green-600";
      case "warning":
        return "text-yellow-600";
      case "danger":
        return "text-red-600";
      default:
        return "text-gray-600";
    }
  }

  // KPIステータスアイコン
  function getKpiStatusIcon(status: string): string {
    switch (status) {
      case "good":
        return "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z";
      case "warning":
        return "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z";
      case "danger":
        return "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z";
      default:
        return "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z";
    }
  }

  // 権限バッジの色
  function getRoleBadgeColor(role: string): string {
    switch (role) {
      case "管理者":
        return "bg-red-100 text-red-800";
      case "承認者":
        return "bg-blue-100 text-blue-800";
      case "文書作成":
        return "bg-green-100 text-green-800";
      default:
        return "bg-gray-100 text-gray-800";
    }
  }

  // 社員詳細表示
  function viewEmployeeDetails(employeeId: number) {
    window.location.href = `/organization/employees/${employeeId}`;
  }

  // 部署編集
  function editDepartment() {
    showEditDialog = true;
  }

  // 初期読み込み
  onMount(() => {
    loadDepartment();
  });
</script>

<div class="space-y-6">
  {#if isLoading}
    <!-- ローディング状態 -->
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
            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
          ></path>
        </svg>
        読み込み中...
      </div>
    </div>
  {:else if error}
    <!-- エラー状態 -->
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
          d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
        />
      </svg>
      <h3 class="mt-2 text-sm font-medium text-gray-900">
        エラーが発生しました
      </h3>
      <p class="mt-1 text-sm text-gray-500">{error}</p>
      <div class="mt-6">
        <Button variant="primary" on:click={loadDepartment}>再読み込み</Button>
      </div>
    </div>
  {:else if department}
    <!-- 部署詳細 -->
    <!-- ページヘッダー -->
    <div class="md:flex md:items-center md:justify-between">
      <div class="min-w-0 flex-1">
        <nav class="flex" aria-label="Breadcrumb">
          <ol class="flex items-center space-x-4">
            <li>
              <a
                href="/organization"
                class="text-gray-400 hover:text-gray-500"
                aria-label="組織管理ホームに戻る"
              >
                <svg class="h-5 w-5" fill="currentColor" viewBox="0 0 20 20">
                  <path
                    d="M10.707 2.293a1 1 0 00-1.414 0l-7 7a1 1 0 001.414 1.414L4 10.414V17a1 1 0 001 1h2a1 1 0 001-1v-2a1 1 0 011-1h2a1 1 0 011 1v2a1 1 0 001 1h2a1 1 0 001-1v-6.586l.293.293a1 1 0 001.414-1.414l-7-7z"
                  />
                </svg>
              </a>
            </li>
            <li>
              <div class="flex items-center">
                <svg
                  class="h-5 w-5 text-gray-400"
                  fill="currentColor"
                  viewBox="0 0 20 20"
                >
                  <path
                    fill-rule="evenodd"
                    d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
                    clip-rule="evenodd"
                  />
                </svg>
                <a
                  href="/organization"
                  class="ml-4 text-sm font-medium text-gray-500 hover:text-gray-700"
                  >組織管理</a
                >
              </div>
            </li>
            {#if department.parentName}
              <li>
                <div class="flex items-center">
                  <svg
                    class="h-5 w-5 text-gray-400"
                    fill="currentColor"
                    viewBox="0 0 20 20"
                  >
                    <path
                      fill-rule="evenodd"
                      d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
                      clip-rule="evenodd"
                    />
                  </svg>
                  <span class="ml-4 text-sm font-medium text-gray-500"
                    >{department.parentName}</span
                  >
                </div>
              </li>
            {/if}
            <li>
              <div class="flex items-center">
                <svg
                  class="h-5 w-5 text-gray-400"
                  fill="currentColor"
                  viewBox="0 0 20 20"
                >
                  <path
                    fill-rule="evenodd"
                    d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
                    clip-rule="evenodd"
                  />
                </svg>
                <span class="ml-4 text-sm font-medium text-gray-500"
                  >{department.name}</span
                >
              </div>
            </li>
          </ol>
        </nav>

        <div class="mt-2 flex items-center">
          <h1
            class="text-2xl font-bold leading-7 text-gray-900 sm:truncate sm:text-3xl sm:tracking-tight"
          >
            {department.name}
          </h1>
          <span
            class="ml-3 inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-100 text-gray-800"
          >
            {department.code}
          </span>
        </div>

        <div class="mt-1 flex flex-col sm:flex-row sm:flex-wrap sm:space-x-6">
          <div class="mt-2 flex items-center text-sm text-gray-500">
            <svg
              class="mr-1.5 h-4 w-4 text-gray-400"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
              />
            </svg>
            責任者: {department.managerName}
          </div>
          <div class="mt-2 flex items-center text-sm text-gray-500">
            <svg
              class="mr-1.5 h-4 w-4 text-gray-400"
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
            {department.employeeCount}名
          </div>
          <div class="mt-2 flex items-center text-sm text-gray-500">
            <svg
              class="mr-1.5 h-4 w-4 text-gray-400"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"
              />
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"
              />
            </svg>
            {department.location}
          </div>
        </div>
      </div>

      <!-- アクションボタン -->
      <div class="mt-4 flex space-x-3 md:ml-4 md:mt-0">
        <Button variant="secondary" size="sm" on:click={editDepartment}>
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
              d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L6.832 19.82a4.5 4.5 0 01-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 011.13-1.897L16.863 4.487zm0 0L19.5 7.125"
            />
          </svg>
          編集
        </Button>
        <Button variant="primary" size="sm">
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
              d="M12 4.5v15m7.5-7.5h-15"
            />
          </svg>
          社員追加
        </Button>
      </div>
    </div>

    <!-- タブナビゲーション -->
    <div class="bg-white shadow rounded-lg">
      <div class="border-b border-gray-200">
        <nav class="-mb-px flex space-x-8 px-6" aria-label="Tabs">
          <button
            class="py-4 px-1 border-b-2 font-medium text-sm {activeTab ===
            'overview'
              ? 'border-blue-500 text-blue-600'
              : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}"
            on:click={() => (activeTab = "overview")}
          >
            概要
          </button>
          <button
            class="py-4 px-1 border-b-2 font-medium text-sm {activeTab ===
            'members'
              ? 'border-blue-500 text-blue-600'
              : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}"
            on:click={() => (activeTab = "members")}
          >
            メンバー ({employees.length})
          </button>
          <button
            class="py-4 px-1 border-b-2 font-medium text-sm {activeTab ===
            'kpis'
              ? 'border-blue-500 text-blue-600'
              : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}"
            on:click={() => (activeTab = "kpis")}
          >
            KPI・実績
          </button>
        </nav>
      </div>

      <div class="p-6">
        {#if activeTab === "overview"}
          <!-- 概要タブ -->
          <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
            <!-- 基本情報 -->
            <div class="lg:col-span-2 space-y-6">
              <div>
                <h3 class="text-lg font-medium text-gray-900 mb-4">基本情報</h3>
                <dl class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                  <div>
                    <dt class="text-sm font-medium text-gray-500">
                      部署コード
                    </dt>
                    <dd class="mt-1 text-sm text-gray-900">
                      {department.code}
                    </dd>
                  </div>
                  <div>
                    <dt class="text-sm font-medium text-gray-500">設立日</dt>
                    <dd class="mt-1 text-sm text-gray-900">
                      {department.createdDate}
                    </dd>
                  </div>
                  <div>
                    <dt class="text-sm font-medium text-gray-500">予算</dt>
                    <dd class="mt-1 text-sm text-gray-900">
                      ¥{department.budget}
                    </dd>
                  </div>
                  <div>
                    <dt class="text-sm font-medium text-gray-500">電話番号</dt>
                    <dd class="mt-1 text-sm text-gray-900">
                      {department.phoneNumber}
                    </dd>
                  </div>
                  <div class="sm:col-span-2">
                    <dt class="text-sm font-medium text-gray-500">
                      メールアドレス
                    </dt>
                    <dd class="mt-1 text-sm text-gray-900">
                      {department.email}
                    </dd>
                  </div>
                </dl>
              </div>

              <div>
                <h4 class="text-sm font-medium text-gray-500 mb-2">部署概要</h4>
                <p class="text-sm text-gray-900 whitespace-pre-wrap">
                  {department.description}
                </p>
              </div>

              <div>
                <h4 class="text-sm font-medium text-gray-500 mb-3">
                  主な業務・責任
                </h4>
                <ul class="space-y-2">
                  {#each department.responsibilities as responsibility}
                    <li class="flex items-start">
                      <svg
                        class="mr-2 h-4 w-4 text-green-500 mt-0.5"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                      >
                        <path
                          stroke-linecap="round"
                          stroke-linejoin="round"
                          stroke-width="2"
                          d="M5 13l4 4L19 7"
                        />
                      </svg>
                      <span class="text-sm text-gray-900">{responsibility}</span
                      >
                    </li>
                  {/each}
                </ul>
              </div>
            </div>

            <!-- サイドバー情報 -->
            <div class="space-y-6">
              <div class="bg-gray-50 rounded-lg p-4">
                <h4 class="text-sm font-medium text-gray-900 mb-3">
                  クイック情報
                </h4>
                <div class="space-y-3">
                  <div class="flex justify-between">
                    <span class="text-sm text-gray-500">総人数</span>
                    <span class="text-sm font-medium text-gray-900"
                      >{department.employeeCount}名</span
                    >
                  </div>
                  <div class="flex justify-between">
                    <span class="text-sm text-gray-500">ステータス</span>
                    <span
                      class="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-green-100 text-green-800"
                    >
                      有効
                    </span>
                  </div>
                  <div class="flex justify-between">
                    <span class="text-sm text-gray-500">所在地</span>
                    <span class="text-sm font-medium text-gray-900"
                      >{department.location}</span
                    >
                  </div>
                </div>
              </div>

              <div class="bg-blue-50 rounded-lg p-4">
                <h4 class="text-sm font-medium text-blue-900 mb-2">
                  責任者情報
                </h4>
                <div class="flex items-center">
                  <div
                    class="h-10 w-10 rounded-full bg-blue-200 flex items-center justify-center"
                  >
                    <span class="text-sm font-medium text-blue-700">
                      {department.managerName.charAt(0)}
                    </span>
                  </div>
                  <div class="ml-3">
                    <p class="text-sm font-medium text-blue-900">
                      {department.managerName}
                    </p>
                    <p class="text-xs text-blue-700">部署責任者</p>
                  </div>
                </div>
                <div class="mt-3">
                  <Button variant="secondary" size="sm">詳細を表示</Button>
                </div>
              </div>
            </div>
          </div>
        {:else if activeTab === "members"}
          <!-- メンバータブ -->
          <div class="space-y-4">
            <div class="flex justify-between items-center">
              <h3 class="text-lg font-medium text-gray-900">部署メンバー</h3>
              <Button variant="primary" size="sm">新規メンバー追加</Button>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              {#each employees as employee}
                <div
                  class="bg-white border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow"
                >
                  <div class="flex items-center mb-3">
                    <div
                      class="h-10 w-10 rounded-full bg-gray-200 flex items-center justify-center"
                    >
                      <span class="text-sm font-medium text-gray-700">
                        {employee.name.charAt(0)}
                      </span>
                    </div>
                    <div class="ml-3">
                      <h4 class="text-sm font-medium text-gray-900">
                        {employee.name}
                      </h4>
                      <p class="text-xs text-gray-500">{employee.position}</p>
                    </div>
                  </div>

                  <div class="space-y-2">
                    <div class="text-xs text-gray-600">
                      <span class="font-medium">メール:</span>
                      {employee.email}
                    </div>
                    <div class="text-xs text-gray-600">
                      <span class="font-medium">入社日:</span>
                      {employee.hireDate}
                    </div>
                    <div class="flex flex-wrap gap-1 mt-2">
                      {#each employee.roles as role}
                        <span
                          class="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium {getRoleBadgeColor(
                            role,
                          )}"
                        >
                          {role}
                        </span>
                      {/each}
                    </div>
                  </div>

                  <div class="mt-4">
                    <Button
                      variant="secondary"
                      size="sm"
                      on:click={() => viewEmployeeDetails(employee.id)}
                    >
                      詳細表示
                    </Button>
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {:else if activeTab === "kpis"}
          <!-- KPI・実績タブ -->
          <div class="space-y-6">
            <h3 class="text-lg font-medium text-gray-900">KPI・実績指標</h3>

            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
              {#each department.kpis as kpi}
                <div class="bg-white border border-gray-200 rounded-lg p-6">
                  <div class="flex items-center justify-between mb-4">
                    <h4 class="text-sm font-medium text-gray-900">
                      {kpi.name}
                    </h4>
                    <svg
                      class="h-5 w-5 {getKpiStatusColor(kpi.status)}"
                      fill="none"
                      viewBox="0 0 24 24"
                      stroke="currentColor"
                    >
                      <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d={getKpiStatusIcon(kpi.status)}
                      />
                    </svg>
                  </div>

                  <div class="space-y-2">
                    <div class="flex justify-between items-center">
                      <span class="text-sm text-gray-500">現在値</span>
                      <span
                        class="text-lg font-semibold {getKpiStatusColor(
                          kpi.status,
                        )}">{kpi.value}</span
                      >
                    </div>
                    <div class="flex justify-between items-center">
                      <span class="text-sm text-gray-500">目標値</span>
                      <span class="text-sm text-gray-900">{kpi.target}</span>
                    </div>
                  </div>

                  <!-- プログレスバー（簡略化） -->
                  <div class="mt-4">
                    <div class="bg-gray-200 rounded-full h-2">
                      <div
                        class="bg-{kpi.status === 'good'
                          ? 'green'
                          : kpi.status === 'warning'
                            ? 'yellow'
                            : 'red'}-500 h-2 rounded-full"
                        style="width: {kpi.status === 'good'
                          ? '90'
                          : kpi.status === 'warning'
                            ? '60'
                            : '30'}%"
                      ></div>
                    </div>
                  </div>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    </div>
  {/if}
</div>
