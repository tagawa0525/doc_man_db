<script lang="ts">
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import Input from '$lib/components/ui/Input.svelte';
  import Select from '$lib/components/ui/Select.svelte';
  
  // パラメータから社員IDを取得
  $: employeeId = $page.params.id;
  
  // 状態管理
  let employee: any = null;
  let isLoading = true;
  let error = '';
  let isEditing = false;
  let showDeleteDialog = false;
  
  // 編集用フォームデータ
  let editFormData = {
    name: '',
    email: '',
    position: '',
    departmentId: '',
    phone: '',
    joinDate: '',
    status: 'active'
  };
  
  // 仮の社員データ
  const mockEmployees: Record<string, any> = {
    '1': {
      id: 1,
      name: '山田太郎',
      email: 'yamada@company.com',
      position: '主任',
      department: { id: 1, name: '情報システム部' },
      phone: '03-1234-5678',
      joinDate: '2020-04-01',
      status: 'active',
      permissions: ['document_read', 'document_write', 'system_admin'],
      recentActivity: [
        { date: '2024-08-15', action: '文書「システム設計書」を作成', type: 'create' },
        { date: '2024-08-14', action: '文書「運用手順書」を更新', type: 'update' },
        { date: '2024-08-13', action: 'システムにログイン', type: 'login' }
      ],
      documents: [
        { id: 1, title: 'システム設計書 v2.1', type: '技術文書', created: '2024-08-15' },
        { id: 2, title: 'データベース移行計画書', type: '計画書', created: '2024-08-10' }
      ]
    },
    '2': {
      id: 2,
      name: '佐藤花子',
      email: 'sato@company.com',
      position: '課長',
      department: { id: 2, name: 'システム開発部' },
      phone: '03-1234-5679',
      joinDate: '2018-04-01',
      status: 'active',
      permissions: ['document_read', 'document_write', 'department_admin'],
      recentActivity: [
        { date: '2024-08-15', action: '部署メンバーの権限を更新', type: 'admin' },
        { date: '2024-08-14', action: '文書承認プロセスを実行', type: 'approval' }
      ],
      documents: [
        { id: 3, title: '開発標準書', type: 'マニュアル', created: '2024-08-12' }
      ]
    }
  };
  
  // 部署オプション
  const departmentOptions = [
    { value: '1', label: '情報システム部' },
    { value: '2', label: 'システム開発部' },
    { value: '3', label: '運用管理部' },
    { value: '4', label: '総務部' }
  ];
  
  // ステータスオプション
  const statusOptions = [
    { value: 'active', label: '在職中' },
    { value: 'inactive', label: '休職中' },
    { value: 'resigned', label: '退職' }
  ];
  
  // 社員データ読み込み
  async function loadEmployee() {
    isLoading = true;
    error = '';
    
    try {
      // TODO: 実際のAPI呼び出しに置き換え
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      const emp = mockEmployees[employeeId];
      if (!emp) {
        error = '指定された社員が見つかりません。';
        return;
      }
      
      employee = emp;
      // 編集フォームデータを初期化
      editFormData = {
        name: emp.name,
        email: emp.email,
        position: emp.position,
        departmentId: emp.department.id.toString(),
        phone: emp.phone,
        joinDate: emp.joinDate,
        status: emp.status
      };
    } catch (err) {
      error = '社員情報の読み込みに失敗しました。';
      console.error('Failed to load employee:', err);
    } finally {
      isLoading = false;
    }
  }
  
  // 編集開始
  function startEdit() {
    isEditing = true;
  }
  
  // 編集キャンセル
  function cancelEdit() {
    isEditing = false;
    // フォームデータをリセット
    editFormData = {
      name: employee.name,
      email: employee.email,
      position: employee.position,
      departmentId: employee.department.id.toString(),
      phone: employee.phone,
      joinDate: employee.joinDate,
      status: employee.status
    };
  }
  
  // 更新保存
  async function saveEmployee() {
    try {
      // TODO: 実際のAPI呼び出しに置き換え
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      // 社員データを更新
      employee = {
        ...employee,
        name: editFormData.name,
        email: editFormData.email,
        position: editFormData.position,
        department: { 
          id: parseInt(editFormData.departmentId), 
          name: departmentOptions.find(d => d.value === editFormData.departmentId)?.label || '' 
        },
        phone: editFormData.phone,
        joinDate: editFormData.joinDate,
        status: editFormData.status
      };
      
      isEditing = false;
      alert('社員情報を更新しました。');
    } catch (error) {
      alert('社員情報の更新に失敗しました。');
      console.error('Failed to update employee:', error);
    }
  }
  
  // 社員削除
  async function deleteEmployee() {
    try {
      // TODO: 実際のAPI呼び出しに置き換え
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      alert('社員を削除しました。');
      window.location.href = '/organization';
    } catch (error) {
      alert('社員の削除に失敗しました。');
      console.error('Failed to delete employee:', error);
    }
  }
  
  // ステータス表示
  function getStatusLabel(status: string): string {
    const labels: Record<string, string> = {
      active: '在職中',
      inactive: '休職中',
      resigned: '退職'
    };
    return labels[status] || status;
  }
  
  function getStatusColor(status: string): string {
    const colors: Record<string, string> = {
      active: 'bg-green-100 text-green-800',
      inactive: 'bg-yellow-100 text-yellow-800',
      resigned: 'bg-red-100 text-red-800'
    };
    return colors[status] || 'bg-gray-100 text-gray-800';
  }
  
  // アクティビティタイプ表示
  function getActivityTypeLabel(type: string): string {
    const labels: Record<string, string> = {
      create: '作成',
      update: '更新',
      login: 'ログイン',
      admin: '管理',
      approval: '承認'
    };
    return labels[type] || type;
  }
  
  function getActivityTypeColor(type: string): string {
    const colors: Record<string, string> = {
      create: 'bg-blue-100 text-blue-800',
      update: 'bg-yellow-100 text-yellow-800',
      login: 'bg-gray-100 text-gray-800',
      admin: 'bg-purple-100 text-purple-800',
      approval: 'bg-green-100 text-green-800'
    };
    return colors[type] || 'bg-gray-100 text-gray-800';
  }
  
  // 初期読み込み
  onMount(() => {
    loadEmployee();
  });
</script>

<div class="space-y-6">
  {#if isLoading}
    <!-- ローディング状態 -->
    <div class="text-center py-12">
      <div class="inline-flex items-center">
        <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-gray-600" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
          <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
          <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
        </svg>
        読み込み中...
      </div>
    </div>
  {:else if error}
    <!-- エラー状態 -->
    <div class="text-center py-12">
      <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
      </svg>
      <h3 class="mt-2 text-sm font-medium text-gray-900">エラーが発生しました</h3>
      <p class="mt-1 text-sm text-gray-500">{error}</p>
      <div class="mt-6">
        <Button variant="primary" on:click={loadEmployee}>
          再読み込み
        </Button>
      </div>
    </div>
  {:else if employee}
    <!-- 社員詳細 -->
    <!-- ページヘッダー -->
    <div class="md:flex md:items-center md:justify-between">
      <div class="min-w-0 flex-1">
        <div class="flex items-center">
          <h1 class="text-2xl font-bold leading-7 text-gray-900 sm:truncate sm:text-3xl sm:tracking-tight">
            {employee.name}
          </h1>
          <span class="ml-3 inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {getStatusColor(employee.status)}">
            {getStatusLabel(employee.status)}
          </span>
        </div>
        <div class="mt-1 flex flex-col sm:flex-row sm:flex-wrap sm:space-x-6">
          <div class="mt-2 flex items-center text-sm text-gray-500">
            <svg class="mr-1.5 h-4 w-4 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4" />
            </svg>
            {employee.department.name}
          </div>
          <div class="mt-2 flex items-center text-sm text-gray-500">
            <svg class="mr-1.5 h-4 w-4 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
            </svg>
            {employee.position}
          </div>
        </div>
      </div>
      
      <!-- アクションボタン -->
      <div class="mt-4 flex space-x-3 md:ml-4 md:mt-0">
        {#if isEditing}
          <Button variant="secondary" size="sm" on:click={cancelEdit}>
            キャンセル
          </Button>
          <Button variant="primary" size="sm" on:click={saveEmployee}>
            保存
          </Button>
        {:else}
          <Button variant="secondary" size="sm" on:click={startEdit}>
            <svg class="mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L6.832 19.82a4.5 4.5 0 01-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 011.13-1.897L16.863 4.487zm0 0L19.5 7.125" />
            </svg>
            編集
          </Button>
          <Button variant="danger" size="sm" on:click={() => showDeleteDialog = true}>
            <svg class="mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" d="M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0" />
            </svg>
            削除
          </Button>
        {/if}
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- メイン情報 -->
      <div class="lg:col-span-2 space-y-6">
        <!-- 基本情報 -->
        <div class="bg-white shadow rounded-lg p-6">
          <h3 class="text-lg font-medium text-gray-900 mb-4">基本情報</h3>
          
          {#if isEditing}
            <!-- 編集フォーム -->
            <div class="space-y-4">
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label for="name" class="block text-sm font-medium text-gray-700 mb-1">
                    氏名 <span class="text-red-500">*</span>
                  </label>
                  <Input id="name" bind:value={editFormData.name} required />
                </div>
                <div>
                  <label for="email" class="block text-sm font-medium text-gray-700 mb-1">
                    メールアドレス <span class="text-red-500">*</span>
                  </label>
                  <Input id="email" type="email" bind:value={editFormData.email} required />
                </div>
              </div>
              
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label for="position" class="block text-sm font-medium text-gray-700 mb-1">
                    役職
                  </label>
                  <Input id="position" bind:value={editFormData.position} />
                </div>
                <div>
                  <label for="department" class="block text-sm font-medium text-gray-700 mb-1">
                    部署 <span class="text-red-500">*</span>
                  </label>
                  <Select id="department" bind:value={editFormData.departmentId} options={departmentOptions} required />
                </div>
              </div>
              
              <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label for="phone" class="block text-sm font-medium text-gray-700 mb-1">
                    電話番号
                  </label>
                  <Input id="phone" bind:value={editFormData.phone} />
                </div>
                <div>
                  <label for="joinDate" class="block text-sm font-medium text-gray-700 mb-1">
                    入社日
                  </label>
                  <Input id="joinDate" type="date" bind:value={editFormData.joinDate} />
                </div>
              </div>
              
              <div>
                <label for="status" class="block text-sm font-medium text-gray-700 mb-1">
                  ステータス
                </label>
                <Select id="status" bind:value={editFormData.status} options={statusOptions} />
              </div>
            </div>
          {:else}
            <!-- 表示モード -->
            <dl class="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <div>
                <dt class="text-sm font-medium text-gray-500">メールアドレス</dt>
                <dd class="mt-1 text-sm text-gray-900">{employee.email}</dd>
              </div>
              <div>
                <dt class="text-sm font-medium text-gray-500">電話番号</dt>
                <dd class="mt-1 text-sm text-gray-900">{employee.phone}</dd>
              </div>
              <div>
                <dt class="text-sm font-medium text-gray-500">入社日</dt>
                <dd class="mt-1 text-sm text-gray-900">{employee.joinDate}</dd>
              </div>
              <div>
                <dt class="text-sm font-medium text-gray-500">権限</dt>
                <dd class="mt-1">
                  <div class="flex flex-wrap gap-1">
                    {#each employee.permissions as permission}
                      <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                        {permission}
                      </span>
                    {/each}
                  </div>
                </dd>
              </div>
            </dl>
          {/if}
        </div>
        
        <!-- 作成文書 -->
        <div class="bg-white shadow rounded-lg p-6">
          <h3 class="text-lg font-medium text-gray-900 mb-4">作成文書</h3>
          
          {#if employee.documents.length > 0}
            <div class="space-y-3">
              {#each employee.documents as document}
                <div class="border rounded-lg p-4 hover:bg-gray-50">
                  <div class="flex items-center justify-between">
                    <div>
                      <h4 class="text-sm font-medium text-gray-900">{document.title}</h4>
                      <p class="text-xs text-gray-500">
                        {document.type} • 作成日: {document.created}
                      </p>
                    </div>
                    <Button variant="secondary" size="sm">
                      <a href="/documents/{document.id}" class="text-blue-600 hover:text-blue-900">
                        詳細
                      </a>
                    </Button>
                  </div>
                </div>
              {/each}
            </div>
          {:else}
            <p class="text-sm text-gray-500">作成した文書はありません。</p>
          {/if}
        </div>
        
        <!-- 最近のアクティビティ -->
        <div class="bg-white shadow rounded-lg p-6">
          <h3 class="text-lg font-medium text-gray-900 mb-4">最近のアクティビティ</h3>
          
          <div class="flow-root">
            <ul role="list" class="-mb-8">
              {#each employee.recentActivity as activity, index}
                <li>
                  <div class="relative pb-8">
                    {#if index !== employee.recentActivity.length - 1}
                      <span class="absolute top-4 left-4 -ml-px h-full w-0.5 bg-gray-200" aria-hidden="true"></span>
                    {/if}
                    <div class="relative flex space-x-3">
                      <div>
                        <span class="h-8 w-8 rounded-full bg-blue-500 flex items-center justify-center ring-8 ring-white">
                          <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {getActivityTypeColor(activity.type)}">
                            {getActivityTypeLabel(activity.type)}
                          </span>
                        </span>
                      </div>
                      <div class="min-w-0 flex-1 pt-1.5 flex justify-between space-x-4">
                        <div>
                          <p class="text-sm text-gray-900">{activity.action}</p>
                        </div>
                        <div class="text-right text-sm whitespace-nowrap text-gray-500">
                          {activity.date}
                        </div>
                      </div>
                    </div>
                  </div>
                </li>
              {/each}
            </ul>
          </div>
        </div>
      </div>
      
      <!-- サイドバー -->
      <div class="lg:col-span-1">
        <div class="bg-white shadow rounded-lg p-6">
          <h3 class="text-lg font-medium text-gray-900 mb-4">クイックアクション</h3>
          
          <div class="space-y-3">
            <Button variant="primary" size="sm">
              <svg class="mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" d="M21.75 6.75v10.5a2.25 2.25 0 01-2.25 2.25h-15a2.25 2.25 0 01-2.25-2.25V6.75m19.5 0A2.25 2.25 0 0019.5 4.5h-15a2.25 2.25 0 00-2.25 2.25m19.5 0v.243a2.25 2.25 0 01-1.07 1.916l-7.5 4.615a2.25 2.25 0 01-2.36 0L3.32 8.91a2.25 2.25 0 01-1.07-1.916V6.75" />
              </svg>
              メール送信
            </Button>
            
            <Button variant="secondary" size="sm">
              <svg class="mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" d="M9 12h3.75M9 15h3.75M9 18h3.75m3 .75H18a2.25 2.25 0 002.25-2.25V6.108c0-1.135-.845-2.098-1.976-2.192a48.424 48.424 0 00-1.123-.08m-5.801 0c-.065.21-.1.433-.1.664 0 .414.336.75.75.75h4.5a.75.75 0 00.75-.75 2.25 2.25 0 00-.1-.664m-5.8 0A2.251 2.251 0 0113.5 2.25H15c1.012 0 1.867.668 2.15 1.586m-5.8 0c-.376.023-.75.05-1.124.08C9.095 4.01 8.25 4.973 8.25 6.108V8.25m0 0H4.875c-.621 0-1.125.504-1.125 1.125v11.25c0 .621.504 1.125 1.125 1.125h9.75c.621 0 1.125-.504 1.125-1.125V9.375c0-.621-.504-1.125-1.125-1.125H8.25zM6.75 12h.008v.008H6.75V12zm0 3h.008v.008H6.75V15zm0 3h.008v.008H6.75V18z" />
              </svg>
              権限管理
            </Button>
            
            <Button variant="secondary" size="sm">
              <svg class="mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" d="M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5M16.5 12L12 16.5m0 0L7.5 12m4.5 4.5V3" />
              </svg>
              レポート出力
            </Button>
          </div>
        </div>
      </div>
    </div>
  {/if}
  
  <!-- 削除確認ダイアログ -->
  {#if showDeleteDialog}
    <div class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
      <div class="relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white">
        <div class="mt-3 text-center">
          <div class="mx-auto flex items-center justify-center h-12 w-12 rounded-full bg-red-100">
            <svg class="h-6 w-6 text-red-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z" />
            </svg>
          </div>
          <h3 class="text-lg font-medium text-gray-900 mt-4">社員を削除しますか？</h3>
          <div class="mt-2 px-7 py-3">
            <p class="text-sm text-gray-500">
              この操作は取り消すことができません。社員「{employee?.name}」を完全に削除します。
            </p>
          </div>
          <div class="items-center px-4 py-3">
            <div class="flex space-x-3">
              <Button
                variant="secondary"
                size="sm"
                on:click={() => showDeleteDialog = false}
              >
                キャンセル
              </Button>
              <Button
                variant="danger"
                size="sm"
                on:click={deleteEmployee}
              >
                削除する
              </Button>
            </div>
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>