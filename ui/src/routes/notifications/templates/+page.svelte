<script lang="ts">
  import { onMount } from 'svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import Input from '$lib/components/ui/Input.svelte';
  import Select from '$lib/components/ui/Select.svelte';
  import TextArea from '$lib/components/ui/TextArea.svelte';
  
  // 通知テンプレートデータ
  interface NotificationTemplate {
    id: string;
    name: string;
    type: 'email' | 'teams';
    category: 'document' | 'system' | 'report' | 'custom';
    subject: string;
    content: string;
    variables: string[];
    isActive: boolean;
    createdAt: string;
    updatedAt: string;
  }
  
  let templates: NotificationTemplate[] = [];
  let isLoading = true;
  let showCreateModal = false;
  let showEditModal = false;
  let editingTemplate: NotificationTemplate | null = null;
  
  // 新規作成・編集用フォームデータ
  let formData = {
    name: '',
    type: 'email' as 'email' | 'teams',
    category: 'document' as 'document' | 'system' | 'report' | 'custom',
    subject: '',
    content: '',
    isActive: true
  };
  
  // バリデーションエラー
  let errors: Record<string, string> = {};
  
  // 仮のテンプレートデータ
  const mockTemplates: NotificationTemplate[] = [
    {
      id: '1',
      name: '文書承認依頼',
      type: 'email',
      category: 'document',
      subject: '文書承認依頼: {{documentTitle}}',
      content: `{{requesterName}}さんから文書の承認依頼があります。

文書名: {{documentTitle}}
文書番号: {{documentNumber}}
作成日: {{createdDate}}

承認画面: {{approvalUrl}}

よろしくお願いいたします。`,
      variables: ['documentTitle', 'documentNumber', 'requesterName', 'createdDate', 'approvalUrl'],
      isActive: true,
      createdAt: '2024-08-01',
      updatedAt: '2024-08-10'
    },
    {
      id: '2',
      name: 'ファイル確認結果通知',
      type: 'teams',
      category: 'report',
      subject: 'ファイル存在確認結果',
      content: `📁 **ファイル存在確認が完了しました**

📊 **結果サマリー**
- 対象ファイル: {{totalFiles}}件
- 存在確認済み: {{existingFiles}}件
- 不存在ファイル: {{missingFiles}}件

{{#if missingFiles > 0}}
⚠️ **不存在ファイルあり**
詳細レポート: {{reportUrl}}
{{/if}}

実行日時: {{executedAt}}`,
      variables: ['totalFiles', 'existingFiles', 'missingFiles', 'reportUrl', 'executedAt'],
      isActive: true,
      createdAt: '2024-08-05',
      updatedAt: '2024-08-12'
    },
    {
      id: '3',
      name: 'システムエラー通知',
      type: 'email',
      category: 'system',
      subject: '【緊急】システムエラー発生: {{errorType}}',
      content: `システムエラーが発生しました。

エラー種別: {{errorType}}
発生日時: {{occurredAt}}
影響範囲: {{affectedArea}}

詳細:
{{errorDetails}}

対応状況: {{responseStatus}}

管理者による確認をお願いします。`,
      variables: ['errorType', 'occurredAt', 'affectedArea', 'errorDetails', 'responseStatus'],
      isActive: true,
      createdAt: '2024-07-20',
      updatedAt: '2024-08-01'
    },
    {
      id: '4',
      name: '日次バックアップ完了',
      type: 'teams',
      category: 'system',
      subject: '日次バックアップ完了',
      content: `✅ **日次バックアップが正常に完了しました**

📅 実行日: {{backupDate}}
🕐 実行時間: {{executionTime}}
💾 バックアップサイズ: {{backupSize}}
📍 保存場所: {{backupLocation}}

次回実行予定: {{nextSchedule}}`,
      variables: ['backupDate', 'executionTime', 'backupSize', 'backupLocation', 'nextSchedule'],
      isActive: true,
      createdAt: '2024-08-08',
      updatedAt: '2024-08-08'
    }
  ];
  
  // タイプオプション
  const typeOptions = [
    { value: 'email', label: 'Email' },
    { value: 'teams', label: 'Teams' }
  ];
  
  // カテゴリオプション
  const categoryOptions = [
    { value: 'document', label: '文書関連' },
    { value: 'system', label: 'システム' },
    { value: 'report', label: 'レポート' },
    { value: 'custom', label: 'カスタム' }
  ];
  
  // テンプレート読み込み
  async function loadTemplates() {
    isLoading = true;
    
    try {
      // TODO: 実際のAPI呼び出しに置き換え
      await new Promise(resolve => setTimeout(resolve, 1000));
      templates = mockTemplates;
    } catch (error) {
      console.error('Failed to load templates:', error);
    } finally {
      isLoading = false;
    }
  }
  
  // 新規作成モーダル表示
  function showCreateTemplate() {
    formData = {
      name: '',
      type: 'email',
      category: 'document',
      subject: '',
      content: '',
      isActive: true
    };
    errors = {};
    showCreateModal = true;
  }
  
  // 編集モーダル表示
  function showEditTemplate(template: NotificationTemplate) {
    editingTemplate = template;
    formData = {
      name: template.name,
      type: template.type,
      category: template.category,
      subject: template.subject,
      content: template.content,
      isActive: template.isActive
    };
    errors = {};
    showEditModal = true;
  }
  
  // モーダル閉じる
  function closeModals() {
    showCreateModal = false;
    showEditModal = false;
    editingTemplate = null;
    formData = {
      name: '',
      type: 'email',
      category: 'document',
      subject: '',
      content: '',
      isActive: true
    };
    errors = {};
  }
  
  // バリデーション
  function validateForm(): boolean {
    errors = {};
    
    if (!formData.name.trim()) {
      errors.name = 'テンプレート名は必須です';
    }
    
    if (!formData.subject.trim()) {
      errors.subject = '件名は必須です';
    }
    
    if (!formData.content.trim()) {
      errors.content = '内容は必須です';
    }
    
    return Object.keys(errors).length === 0;
  }
  
  // テンプレート保存
  async function saveTemplate() {
    if (!validateForm()) return;
    
    try {
      // TODO: 実際のAPI呼び出しに置き換え
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      if (editingTemplate) {
        // 更新
        const index = templates.findIndex(t => t.id === editingTemplate.id);
        if (index !== -1) {
          templates[index] = {
            ...editingTemplate,
            ...formData,
            variables: extractVariables(formData.content),
            updatedAt: new Date().toISOString().split('T')[0]
          };
        }
        alert('テンプレートを更新しました。');
      } else {
        // 新規作成
        const newTemplate: NotificationTemplate = {
          id: Date.now().toString(),
          ...formData,
          variables: extractVariables(formData.content),
          createdAt: new Date().toISOString().split('T')[0],
          updatedAt: new Date().toISOString().split('T')[0]
        };
        templates = [...templates, newTemplate];
        alert('テンプレートを作成しました。');
      }
      
      closeModals();
    } catch (error) {
      alert('テンプレートの保存に失敗しました。');
      console.error('Failed to save template:', error);
    }
  }
  
  // テンプレート削除
  async function deleteTemplate(template: NotificationTemplate) {
    if (!confirm(`テンプレート「${template.name}」を削除しますか？`)) return;
    
    try {
      // TODO: 実際のAPI呼び出しに置き換え
      await new Promise(resolve => setTimeout(resolve, 500));
      
      templates = templates.filter(t => t.id !== template.id);
      alert('テンプレートを削除しました。');
    } catch (error) {
      alert('テンプレートの削除に失敗しました。');
      console.error('Failed to delete template:', error);
    }
  }
  
  // テンプレート複製
  function duplicateTemplate(template: NotificationTemplate) {
    formData = {
      name: `${template.name}のコピー`,
      type: template.type,
      category: template.category,
      subject: template.subject,
      content: template.content,
      isActive: true
    };
    errors = {};
    showCreateModal = true;
  }
  
  // 変数抽出
  function extractVariables(content: string): string[] {
    const matches = content.match(/\{\{(\w+)\}\}/g);
    if (!matches) return [];
    
    return [...new Set(matches.map(match => match.replace(/\{\{|\}\}/g, '')))];
  }
  
  // カテゴリ表示
  function getCategoryLabel(category: string): string {
    const labels: Record<string, string> = {
      document: '文書関連',
      system: 'システム',
      report: 'レポート',
      custom: 'カスタム'
    };
    return labels[category] || category;
  }
  
  function getCategoryColor(category: string): string {
    const colors: Record<string, string> = {
      document: 'bg-blue-100 text-blue-800',
      system: 'bg-red-100 text-red-800',
      report: 'bg-green-100 text-green-800',
      custom: 'bg-purple-100 text-purple-800'
    };
    return colors[category] || 'bg-gray-100 text-gray-800';
  }
  
  // 初期読み込み
  onMount(() => {
    loadTemplates();
  });
</script>

<div class="space-y-6">
  <!-- ページヘッダー -->
  <div class="md:flex md:items-center md:justify-between">
    <div class="min-w-0 flex-1">
      <h1 class="text-2xl font-bold leading-7 text-gray-900 sm:truncate sm:text-3xl sm:tracking-tight">
        通知テンプレート管理
      </h1>
      <p class="mt-1 text-sm text-gray-500">Email・Teams通知のテンプレート作成と管理</p>
    </div>
    <div class="mt-4 flex md:ml-4 md:mt-0">
      <Button variant="primary" size="sm" on:click={showCreateTemplate}>
        <svg class="mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
        </svg>
        新規テンプレート作成
      </Button>
    </div>
  </div>

  <!-- テンプレート一覧 -->
  <div class="bg-white shadow rounded-lg">
    {#if isLoading}
      <div class="px-6 py-12 text-center">
        <div class="inline-flex items-center">
          <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-gray-600" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          読み込み中...
        </div>
      </div>
    {:else if templates.length === 0}
      <div class="px-6 py-12 text-center">
        <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z" />
        </svg>
        <h3 class="mt-2 text-sm font-medium text-gray-900">テンプレートがありません</h3>
        <p class="mt-1 text-sm text-gray-500">最初のテンプレートを作成してください。</p>
        <div class="mt-6">
          <Button variant="primary" on:click={showCreateTemplate}>
            <svg class="mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
            </svg>
            新規テンプレート作成
          </Button>
        </div>
      </div>
    {:else}
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 p-6">
        {#each templates as template}
          <div class="border border-gray-200 rounded-lg p-6 hover:shadow-md transition-shadow">
            <!-- ヘッダー -->
            <div class="flex items-start justify-between mb-4">
              <div class="flex-1">
                <div class="flex items-center space-x-2 mb-2">
                  <h3 class="text-lg font-medium text-gray-900">{template.name}</h3>
                  {#if !template.isActive}
                    <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-gray-100 text-gray-800">
                      無効
                    </span>
                  {/if}
                </div>
                <div class="flex items-center space-x-2">
                  <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                    {template.type === 'email' ? 'Email' : 'Teams'}
                  </span>
                  <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium {getCategoryColor(template.category)}">
                    {getCategoryLabel(template.category)}
                  </span>
                </div>
              </div>
              
              <!-- アクションボタン -->
              <div class="flex items-center space-x-2">
                <button
                  type="button"
                  class="text-gray-400 hover:text-gray-600"
                  on:click={() => showEditTemplate(template)}
                  title="編集"
                >
                  <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                  </svg>
                </button>
                <button
                  type="button"
                  class="text-gray-400 hover:text-gray-600"
                  on:click={() => duplicateTemplate(template)}
                  title="複製"
                >
                  <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                  </svg>
                </button>
                <button
                  type="button"
                  class="text-red-400 hover:text-red-600"
                  on:click={() => deleteTemplate(template)}
                  title="削除"
                >
                  <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                  </svg>
                </button>
              </div>
            </div>
            
            <!-- 件名 -->
            <div class="mb-3">
              <label class="block text-xs font-medium text-gray-500 mb-1">件名</label>
              <p class="text-sm text-gray-900 truncate">{template.subject}</p>
            </div>
            
            <!-- 内容プレビュー -->
            <div class="mb-4">
              <label class="block text-xs font-medium text-gray-500 mb-1">内容</label>
              <p class="text-sm text-gray-600 line-clamp-3">{template.content}</p>
            </div>
            
            <!-- 変数 -->
            {#if template.variables.length > 0}
              <div class="mb-4">
                <label class="block text-xs font-medium text-gray-500 mb-1">使用変数</label>
                <div class="flex flex-wrap gap-1">
                  {#each template.variables.slice(0, 3) as variable}
                    <span class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-gray-100 text-gray-800">
                      {variable}
                    </span>
                  {/each}
                  {#if template.variables.length > 3}
                    <span class="text-xs text-gray-500">+{template.variables.length - 3}個</span>
                  {/if}
                </div>
              </div>
            {/if}
            
            <!-- メタ情報 -->
            <div class="text-xs text-gray-500">
              <p>作成: {template.createdAt}</p>
              <p>更新: {template.updatedAt}</p>
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

<!-- 新規作成モーダル -->
{#if showCreateModal}
  <div class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
    <div class="relative top-20 mx-auto p-5 border w-full max-w-2xl shadow-lg rounded-md bg-white">
      <div class="mt-3">
        <h3 class="text-lg font-medium text-gray-900 mb-4">新規テンプレート作成</h3>
        
        <form on:submit|preventDefault={saveTemplate} class="space-y-4">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label for="name" class="block text-sm font-medium text-gray-700 mb-1">
                テンプレート名 <span class="text-red-500">*</span>
              </label>
              <Input
                id="name"
                bind:value={formData.name}
                error={errors.name}
                placeholder="文書承認依頼"
                required
              />
            </div>
            
            <div>
              <label for="type" class="block text-sm font-medium text-gray-700 mb-1">
                通知タイプ <span class="text-red-500">*</span>
              </label>
              <Select
                id="type"
                bind:value={formData.type}
                options={typeOptions}
                required
              />
            </div>
          </div>
          
          <div>
            <label for="category" class="block text-sm font-medium text-gray-700 mb-1">
              カテゴリ <span class="text-red-500">*</span>
            </label>
            <Select
              id="category"
              bind:value={formData.category}
              options={categoryOptions}
              required
            />
          </div>
          
          <div>
            <label for="subject" class="block text-sm font-medium text-gray-700 mb-1">
              件名 <span class="text-red-500">*</span>
            </label>
            <Input
              id="subject"
              bind:value={formData.subject}
              error={errors.subject}
              placeholder="文書承認依頼: {{documentTitle}}"
              required
            />
            <p class="mt-1 text-xs text-gray-500">
              変数は {{variableName}} の形式で記述してください
            </p>
          </div>
          
          <div>
            <label for="content" class="block text-sm font-medium text-gray-700 mb-1">
              内容 <span class="text-red-500">*</span>
            </label>
            <TextArea
              id="content"
              bind:value={formData.content}
              error={errors.content}
              rows={8}
              placeholder="{{requesterName}}さんから文書の承認依頼があります。&#10;&#10;文書名: {{documentTitle}}&#10;文書番号: {{documentNumber}}"
              required
            />
            <p class="mt-1 text-xs text-gray-500">
              利用可能な変数: documentTitle, documentNumber, requesterName, createdDate, approvalUrl など
            </p>
          </div>
          
          <div class="flex items-center">
            <input
              id="isActive"
              type="checkbox"
              bind:checked={formData.isActive}
              class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
            />
            <label for="isActive" class="ml-2 block text-sm text-gray-900">
              有効にする
            </label>
          </div>
          
          <div class="flex justify-end space-x-3 pt-4">
            <Button
              type="button"
              variant="secondary"
              on:click={closeModals}
            >
              キャンセル
            </Button>
            <Button type="submit">
              作成
            </Button>
          </div>
        </form>
      </div>
    </div>
  </div>
{/if}

<!-- 編集モーダル -->
{#if showEditModal}
  <div class="fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
    <div class="relative top-20 mx-auto p-5 border w-full max-w-2xl shadow-lg rounded-md bg-white">
      <div class="mt-3">
        <h3 class="text-lg font-medium text-gray-900 mb-4">テンプレート編集</h3>
        
        <form on:submit|preventDefault={saveTemplate} class="space-y-4">
          <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div>
              <label for="edit-name" class="block text-sm font-medium text-gray-700 mb-1">
                テンプレート名 <span class="text-red-500">*</span>
              </label>
              <Input
                id="edit-name"
                bind:value={formData.name}
                error={errors.name}
                required
              />
            </div>
            
            <div>
              <label for="edit-type" class="block text-sm font-medium text-gray-700 mb-1">
                通知タイプ <span class="text-red-500">*</span>
              </label>
              <Select
                id="edit-type"
                bind:value={formData.type}
                options={typeOptions}
                required
              />
            </div>
          </div>
          
          <div>
            <label for="edit-category" class="block text-sm font-medium text-gray-700 mb-1">
              カテゴリ <span class="text-red-500">*</span>
            </label>
            <Select
              id="edit-category"
              bind:value={formData.category}
              options={categoryOptions}
              required
            />
          </div>
          
          <div>
            <label for="edit-subject" class="block text-sm font-medium text-gray-700 mb-1">
              件名 <span class="text-red-500">*</span>
            </label>
            <Input
              id="edit-subject"
              bind:value={formData.subject}
              error={errors.subject}
              required
            />
          </div>
          
          <div>
            <label for="edit-content" class="block text-sm font-medium text-gray-700 mb-1">
              内容 <span class="text-red-500">*</span>
            </label>
            <TextArea
              id="edit-content"
              bind:value={formData.content}
              error={errors.content}
              rows={8}
              required
            />
          </div>
          
          <div class="flex items-center">
            <input
              id="edit-isActive"
              type="checkbox"
              bind:checked={formData.isActive}
              class="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
            />
            <label for="edit-isActive" class="ml-2 block text-sm text-gray-900">
              有効にする
            </label>
          </div>
          
          <div class="flex justify-end space-x-3 pt-4">
            <Button
              type="button"
              variant="secondary"
              on:click={closeModals}
            >
              キャンセル
            </Button>
            <Button type="submit">
              更新
            </Button>
          </div>
        </form>
      </div>
    </div>
  </div>
{/if}