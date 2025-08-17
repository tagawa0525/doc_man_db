<script lang="ts">
  import { onMount } from 'svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import Input from '$lib/components/ui/Input.svelte';
  import ResponsiveTable from '$lib/components/mobile/ResponsiveTable.svelte';
  
  // 状態管理
  let activeTab = 'departments';
  let searchTerm = '';
  let isLoading = false;
  
  // 部署データ
  let departments = [
    {
      id: 1,
      name: '情報システム部',
      code: 'IS',
      parentId: null,
      level: 0,
      managerName: '田中部長',
      employeeCount: 15,
      isActive: true,
      createdDate: '2020-04-01',
      description: 'システム開発・運用・保守を担当'
    },
    {
      id: 2,
      name: 'システム開発課',
      code: 'IS-DEV',
      parentId: 1,
      level: 1,
      managerName: '山田課長',
      employeeCount: 8,
      isActive: true,
      createdDate: '2020-04-01',
      description: 'アプリケーション開発'
    },
    {
      id: 3,
      name: 'システム運用課',
      code: 'IS-OPS',
      parentId: 1,
      level: 1,
      managerName: '佐藤課長',
      employeeCount: 7,
      isActive: true,
      createdDate: '2020-04-01',
      description: 'システム運用・保守'
    },
    {
      id: 4,
      name: '経営企画部',
      code: 'PL',
      parentId: null,
      level: 0,
      managerName: '高橋部長',
      employeeCount: 12,
      isActive: true,
      createdDate: '2020-04-01',
      description: '経営戦略・企画立案'
    },
    {
      id: 5,
      name: '総務部',
      code: 'GA',
      parentId: null,
      level: 0,
      managerName: '鈴木部長',
      employeeCount: 10,
      isActive: true,
      createdDate: '2020-04-01',
      description: '総務・人事・労務管理'
    }
  ];
  
  // 社員データ
  let employees = [
    {
      id: 1,
      employeeNumber: 'EMP001',
      name: '山田太郎',
      departmentId: 2,
      departmentName: 'システム開発課',
      position: '課長',
      email: 'yamada@company.com',
      phoneNumber: '03-1234-5678',
      hireDate: '2018-04-01',
      isActive: true,
      roles: ['文書作成', '承認者'],
      lastLogin: '2024-08-17 09:30:00'
    },
    {
      id: 2,
      employeeNumber: 'EMP002',
      name: '佐藤花子',
      departmentId: 2,
      departmentName: 'システム開発課',
      position: '主任',
      email: 'sato@company.com',
      phoneNumber: '03-1234-5679',
      hireDate: '2020-04-01',
      isActive: true,
      roles: ['文書作成'],
      lastLogin: '2024-08-17 08:45:00'
    },
    {
      id: 3,
      employeeNumber: 'EMP003',
      name: '田中一郎',
      departmentId: 3,
      departmentName: 'システム運用課',
      position: '課長',
      email: 'tanaka@company.com',
      phoneNumber: '03-1234-5680',
      hireDate: '2019-04-01',
      isActive: true,
      roles: ['文書作成', '承認者'],
      lastLogin: '2024-08-16 17:20:00'
    },
    {
      id: 4,
      employeeNumber: 'EMP004',
      name: '高橋美子',
      departmentId: 4,
      departmentName: '経営企画部',
      position: '部長',
      email: 'takahashi@company.com',
      phoneNumber: '03-1234-5681',
      hireDate: '2015-04-01',
      isActive: true,
      roles: ['文書作成', '承認者', '管理者'],
      lastLogin: '2024-08-17 10:15:00'
    }
  ];
  
  // フィルタリング
  $: filteredDepartments = departments.filter(dept => 
    dept.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
    dept.code.toLowerCase().includes(searchTerm.toLowerCase()) ||
    dept.managerName.toLowerCase().includes(searchTerm.toLowerCase())
  );
  
  $: filteredEmployees = employees.filter(emp => 
    emp.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
    emp.employeeNumber.toLowerCase().includes(searchTerm.toLowerCase()) ||
    emp.departmentName.toLowerCase().includes(searchTerm.toLowerCase()) ||
    emp.email.toLowerCase().includes(searchTerm.toLowerCase())
  );
  
  // 部署階層表示用関数
  function getDepartmentHierarchy() {
    const hierarchy: any[] = [];
    const departmentMap = new Map();
    
    // 部署をマップに格納
    departments.forEach(dept => {
      departmentMap.set(dept.id, { ...dept, children: [] });
    });
    
    // 階層構造を構築
    departments.forEach(dept => {
      const deptObj = departmentMap.get(dept.id);
      if (dept.parentId) {
        const parent = departmentMap.get(dept.parentId);
        if (parent) {
          parent.children.push(deptObj);
        }
      } else {
        hierarchy.push(deptObj);
      }
    });
    
    return hierarchy;
  }
  
  // 部署詳細表示
  function viewDepartmentDetails(departmentId: number) {
    window.location.href = `/organization/departments/${departmentId}`;
  }
  
  // 社員詳細表示
  function viewEmployeeDetails(employeeId: number) {
    window.location.href = `/organization/employees/${employeeId}`;
  }
  
  // 権限バッジの色
  function getRoleBadgeColor(role: string): string {
    switch (role) {
      case '管理者':
        return 'bg-red-100 text-red-800';
      case '承認者':
        return 'bg-blue-100 text-blue-800';
      case '文書作成':
        return 'bg-green-100 text-green-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  }
  
  // ステータス表示
  function getStatusBadge(isActive: boolean) {
    return isActive 
      ? 'bg-green-100 text-green-800' 
      : 'bg-gray-100 text-gray-800';
  }
  
  function getStatusText(isActive: boolean) {
    return isActive ? '有効' : '無効';
  }
  
  // テーブルヘッダー定義
  const departmentHeaders = [
    {
      key: 'department_info',
      label: '部署名・コード',
      sortable: true
    },
    {
      key: 'managerName',
      label: '責任者',
      sortable: true,
      mobileHidden: true
    },
    {
      key: 'employeeCount',
      label: '人数',
      sortable: true
    },
    {
      key: 'isActive',
      label: 'ステータス',
      mobileHidden: true
    },
    {
      key: 'createdDate',
      label: '作成日',
      mobileHidden: true
    },
    {
      key: 'actions',
      label: '操作'
    }
  ];

  const employeeHeaders = [
    {
      key: 'employee_info',
      label: '社員情報',
      sortable: true
    },
    {
      key: 'department_position',
      label: '所属・役職',
      sortable: true,
      mobileHidden: true
    },
    {
      key: 'roles',
      label: '権限',
      mobileHidden: true
    },
    {
      key: 'lastLogin',
      label: '最終ログイン',
      mobileHidden: true
    },
    {
      key: 'isActive',
      label: 'ステータス'
    },
    {
      key: 'actions',
      label: '操作'
    }
  ];

  // 初期読み込み
  onMount(() => {
    // TODO: 実際のAPI呼び出し
  });
</script>

<div class="space-y-6">
  <!-- ページヘッダー -->
  <div class="md:flex md:items-center md:justify-between">
    <div class="min-w-0 flex-1">
      <h1 class="text-2xl font-bold leading-7 text-gray-900 sm:truncate sm:text-3xl sm:tracking-tight">
        組織管理
      </h1>
      <p class="mt-1 text-sm text-gray-500">
        部署と社員の管理・権限設定
      </p>
    </div>
    
    <div class="mt-4 flex space-x-3 md:ml-4 md:mt-0">
      <Button variant="secondary" size="sm">
        <svg class="mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" d="M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5M16.5 12L12 16.5m0 0L7.5 12m4.5 4.5V3" />
        </svg>
        データ出力
      </Button>
      <Button variant="primary" size="sm">
        <svg class="mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
        </svg>
        新規追加
      </Button>
    </div>
  </div>

  <!-- タブナビゲーション -->
  <div class="bg-white shadow rounded-lg">
    <div class="border-b border-gray-200">
      <nav class="-mb-px flex space-x-8 px-6" aria-label="Tabs">
        <button
          class="py-4 px-1 border-b-2 font-medium text-sm {activeTab === 'departments' 
            ? 'border-blue-500 text-blue-600' 
            : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}"
          on:click={() => activeTab = 'departments'}
        >
          <svg class="mr-2 h-4 w-4 inline" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 21h16.5M4.5 3h15l-.75 18H5.25L4.5 3z" />
          </svg>
          部署管理
        </button>
        <button
          class="py-4 px-1 border-b-2 font-medium text-sm {activeTab === 'employees' 
            ? 'border-blue-500 text-blue-600' 
            : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}"
          on:click={() => activeTab = 'employees'}
        >
          <svg class="mr-2 h-4 w-4 inline" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z" />
          </svg>
          社員管理
        </button>
        <button
          class="py-4 px-1 border-b-2 font-medium text-sm {activeTab === 'hierarchy' 
            ? 'border-blue-500 text-blue-600' 
            : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}"
          on:click={() => activeTab = 'hierarchy'}
        >
          <svg class="mr-2 h-4 w-4 inline" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 6.75h12M8.25 12h12m-12 5.25h12M3.75 6.75h.007v.008H3.75V6.75zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zM3.75 12h.007v.008H3.75V12zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0zm-.375 5.25h.007v.008H3.75v-.008zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0z" />
          </svg>
          組織図
        </button>
      </nav>
    </div>
    
    <!-- 検索バー -->
    <div class="p-6 border-b border-gray-200">
      <div class="max-w-md">
        <Input
          bind:value={searchTerm}
          placeholder="検索（名前、コード、メールアドレスなど）"
        >
          <svg slot="icon" class="h-4 w-4 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
          </svg>
        </Input>
      </div>
    </div>
    
    <!-- コンテンツエリア -->
    <div class="p-6">
      {#if activeTab === 'departments'}
        <!-- 部署管理 -->
        <div class="space-y-4">
          <div class="flex justify-between items-center">
            <h3 class="text-lg font-medium text-gray-900">部署一覧</h3>
            <Button variant="primary" size="sm">
              新規部署追加
            </Button>
          </div>
          
          <ResponsiveTable
            headers={departmentHeaders}
            data={filteredDepartments}
            onSort={(column, direction) => {
              console.log('Sort departments:', column, direction);
            }}
            let:item
          >
            <!-- 部署名・コード列 -->
            <svelte:fragment slot="department_info" let:item>
              <div style="margin-left: {item.level * 20}px;">
                {#if item.level > 0}
                  <span class="text-gray-400 mr-2">└</span>
                {/if}
                <div class="text-sm font-medium text-gray-900">{item.name}</div>
                <div class="text-sm text-gray-500">{item.code}</div>
                {#if item.description}
                  <div class="text-xs text-gray-400 mt-1">{item.description}</div>
                {/if}
              </div>
            </svelte:fragment>

            <!-- 責任者列 -->
            <svelte:fragment slot="managerName" let:item>
              <span class="text-sm text-gray-900">{item.managerName}</span>
            </svelte:fragment>

            <!-- 人数列 -->
            <svelte:fragment slot="employeeCount" let:item>
              <span class="text-sm text-gray-900">{item.employeeCount}名</span>
            </svelte:fragment>

            <!-- ステータス列 -->
            <svelte:fragment slot="isActive" let:item>
              <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {getStatusBadge(item.isActive)}">
                {getStatusText(item.isActive)}
              </span>
            </svelte:fragment>

            <!-- 作成日列 -->
            <svelte:fragment slot="createdDate" let:item>
              <span class="text-sm text-gray-900">{item.createdDate}</span>
            </svelte:fragment>

            <!-- 操作列 -->
            <svelte:fragment slot="actions" let:item>
              <div class="flex space-x-2">
                <button
                  class="text-blue-600 hover:text-blue-900 text-sm font-medium"
                  on:click={() => viewDepartmentDetails(item.id)}
                >
                  詳細
                </button>
                <button class="text-gray-600 hover:text-gray-900 text-sm font-medium">
                  編集
                </button>
              </div>
            </svelte:fragment>
          </ResponsiveTable>

          <div class="overflow-hidden shadow ring-1 ring-black ring-opacity-5 md:rounded-lg" style="display: none;">
            <table class="min-w-full divide-y divide-gray-300">
              <thead class="bg-gray-50">
                <tr>
                  <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                    部署名・コード
                  </th>
                </tr>
              </thead>
            </table>
          </div>
        </div>
        
      {:else if activeTab === 'employees'}
        <!-- 社員管理 -->
        <div class="space-y-4">
          <div class="flex justify-between items-center">
            <h3 class="text-lg font-medium text-gray-900">社員一覧</h3>
            <Button variant="primary" size="sm">
              新規社員追加
            </Button>
          </div>
          
          <ResponsiveTable
            headers={employeeHeaders}
            data={filteredEmployees}
            onSort={(column, direction) => {
              console.log('Sort employees:', column, direction);
            }}
            let:item
          >
            <!-- 社員情報列 -->
            <svelte:fragment slot="employee_info" let:item>
              <div>
                <div class="text-sm font-medium text-gray-900">{item.name}</div>
                <div class="text-sm text-gray-500">{item.employeeNumber}</div>
                <div class="text-xs text-gray-400">{item.email}</div>
              </div>
            </svelte:fragment>

            <!-- 所属・役職列 -->
            <svelte:fragment slot="department_position" let:item>
              <div class="text-sm text-gray-900">{item.departmentName}</div>
              <div class="text-sm text-gray-500">{item.position}</div>
            </svelte:fragment>

            <!-- 権限列 -->
            <svelte:fragment slot="roles" let:item>
              <div class="flex flex-wrap gap-1">
                {#each item.roles as role}
                  <span class="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium {getRoleBadgeColor(role)}">
                    {role}
                  </span>
                {/each}
              </div>
            </svelte:fragment>

            <!-- 最終ログイン列 -->
            <svelte:fragment slot="lastLogin" let:item>
              <span class="text-sm text-gray-900">{item.lastLogin}</span>
            </svelte:fragment>

            <!-- ステータス列 -->
            <svelte:fragment slot="isActive" let:item>
              <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {getStatusBadge(item.isActive)}">
                {getStatusText(item.isActive)}
              </span>
            </svelte:fragment>

            <!-- 操作列 -->
            <svelte:fragment slot="actions" let:item>
              <div class="flex space-x-2">
                <button
                  class="text-blue-600 hover:text-blue-900 text-sm font-medium"
                  on:click={() => viewEmployeeDetails(item.id)}
                >
                  詳細
                </button>
                <button class="text-gray-600 hover:text-gray-900 text-sm font-medium">
                  編集
                </button>
              </div>
            </svelte:fragment>
          </ResponsiveTable>
        </div>
        
      {:else if activeTab === 'hierarchy'}
        <!-- 組織図 -->
        <div class="space-y-6">
          <h3 class="text-lg font-medium text-gray-900">組織階層図</h3>
          
          <div class="bg-gray-50 rounded-lg p-6">
            {#each getDepartmentHierarchy() as rootDept}
              <div class="mb-6">
                <!-- 部署カード -->
                <div class="bg-white rounded-lg shadow p-4 mb-4">
                  <div class="flex items-center justify-between">
                    <div>
                      <h4 class="text-lg font-medium text-gray-900">{rootDept.name}</h4>
                      <p class="text-sm text-gray-500">{rootDept.code} | {rootDept.managerName}</p>
                      <p class="text-xs text-gray-400 mt-1">{rootDept.description}</p>
                    </div>
                    <div class="text-right">
                      <div class="text-2xl font-bold text-blue-600">{rootDept.employeeCount}</div>
                      <div class="text-xs text-gray-500">名</div>
                    </div>
                  </div>
                </div>
                
                <!-- 子部署 -->
                {#if rootDept.children && rootDept.children.length > 0}
                  <div class="ml-8 space-y-3">
                    {#each rootDept.children as childDept}
                      <div class="bg-white rounded-lg shadow-sm border border-gray-200 p-3">
                        <div class="flex items-center justify-between">
                          <div>
                            <h5 class="font-medium text-gray-900">{childDept.name}</h5>
                            <p class="text-sm text-gray-500">{childDept.code} | {childDept.managerName}</p>
                            <p class="text-xs text-gray-400">{childDept.description}</p>
                          </div>
                          <div class="text-right">
                            <div class="text-lg font-semibold text-blue-600">{childDept.employeeCount}</div>
                            <div class="text-xs text-gray-500">名</div>
                          </div>
                        </div>
                      </div>
                    {/each}
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        </div>
      {/if}
    </div>
  </div>
</div>