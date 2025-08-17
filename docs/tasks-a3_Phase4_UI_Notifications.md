# Phase 4: UI・通知システム実装 (Week 9-10)

## フェーズ概要

- **期間**: Week 9-10 (2週間)
- **目標**: Webユーザーインターフェースとメール・Teams通知機能の実装
- **成果物**: 完全なWebユーザーインターフェース、通知システム、ダッシュボード

## タスク一覧

### TASK-024: フロントエンド基盤

- **説明**: HTML/CSS/JS基盤・テンプレート
- **優先度**: High
- **見積工数**: 12h
- **状態**: 未着手
- **依存関係**: TASK-021

#### 実装内容

1. SvelteKit プロジェクト初期化
2. Tailwind CSS 設定
3. 基本コンポーネント作成
4. レイアウトシステム

#### ディレクトリ構成

```text
ui/
├── package.json
├── svelte.config.js
├── vite.config.js
├── tailwind.config.js
├── src/
│   ├── app.html
│   ├── routes/
│   │   ├── +layout.svelte
│   │   ├── +layout.ts
│   │   ├── +page.svelte
│   │   └── auth/
│   │       ├── +page.svelte
│   │       └── +page.ts
│   ├── lib/
│   │   ├── components/
│   │   │   ├── ui/
│   │   │   │   ├── Button.svelte
│   │   │   │   ├── Input.svelte
│   │   │   │   ├── Table.svelte
│   │   │   │   └── Modal.svelte
│   │   │   ├── forms/
│   │   │   │   ├── SearchForm.svelte
│   │   │   │   └── DocumentForm.svelte
│   │   │   └── layout/
│   │   │       ├── Header.svelte
│   │   │       ├── Navigation.svelte
│   │   │       └── Footer.svelte
│   │   ├── stores/
│   │   │   ├── auth.ts
│   │   │   ├── documents.ts
│   │   │   └── notifications.ts
│   │   ├── api/
│   │   │   ├── client.ts
│   │   │   └── endpoints.ts
│   │   └── utils/
│   │       ├── validation.ts
│   │       └── formatters.ts
│   └── styles/
│       └── app.css
```

#### 基本コンポーネント実装

```svelte
<!-- src/lib/components/ui/Button.svelte -->
<script lang="ts">
  export let variant: 'primary' | 'secondary' | 'danger' = 'primary';
  export let size: 'sm' | 'md' | 'lg' = 'md';
  export let loading = false;
  export let disabled = false;
  
  const variants = {
    primary: 'bg-blue-600 hover:bg-blue-700 text-white',
    secondary: 'bg-gray-200 hover:bg-gray-300 text-gray-900',
    danger: 'bg-red-600 hover:bg-red-700 text-white'
  };
  
  const sizes = {
    sm: 'px-3 py-1.5 text-sm',
    md: 'px-4 py-2 text-base',
    lg: 'px-6 py-3 text-lg'
  };
</script>

<button
  class="rounded-md font-medium transition-colors duration-200 
         {variants[variant]} {sizes[size]}
         {disabled || loading ? 'opacity-50 cursor-not-allowed' : ''}"
  {disabled}
  on:click
>
  {#if loading}
    <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white inline" 
         xmlns="http://www.w3.org/2000/svg" 
         fill="none" 
         viewBox="0 0 24 24">
      <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
      <path class="opacity-75" fill="currentColor" 
            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
    </svg>
  {/if}
  <slot />
</button>
```

#### 認証ストア

```typescript
// src/lib/stores/auth.ts
import { writable } from 'svelte/store';
import type { User, UserPermissions } from '../types/auth';

interface AuthState {
  user: User | null;
  permissions: UserPermissions | null;
  isAuthenticated: boolean;
  isLoading: boolean;
}

const initialState: AuthState = {
  user: null,
  permissions: null,
  isAuthenticated: false,
  isLoading: false,
};

export const authStore = writable<AuthState>(initialState);

export const authActions = {
  async login(username: string, password: string) {
    authStore.update(state => ({ ...state, isLoading: true }));
    
    try {
      const response = await fetch('/api/auth/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ username, password })
      });
      
      if (response.ok) {
        const { user, permissions, token } = await response.json();
        localStorage.setItem('auth_token', token);
        
        authStore.set({
          user,
          permissions,
          isAuthenticated: true,
          isLoading: false,
        });
      } else {
        throw new Error('Login failed');
      }
    } catch (error) {
      authStore.update(state => ({ ...state, isLoading: false }));
      throw error;
    }
  },
  
  logout() {
    localStorage.removeItem('auth_token');
    authStore.set(initialState);
  }
};
```

#### 成果物

- SvelteKit アプリケーション基盤
- 基本UIコンポーネント
- 認証システム統合
- レスポンシブレイアウト

---

### TASK-025: 文書検索画面

- **説明**: 検索フォーム・結果表示
- **優先度**: High
- **見積工数**: 16h
- **状態**: 未着手
- **依存関係**: TASK-024

#### 実装内容

1. 検索フォームコンポーネント
2. 検索結果テーブル
3. フィルタリングUI
4. ページング機能

#### 検索画面実装

```svelte
<!-- src/routes/documents/+page.svelte -->
<script lang="ts">
  import { onMount } from 'svelte';
  import SearchForm from '$lib/components/forms/SearchForm.svelte';
  import DocumentTable from '$lib/components/DocumentTable.svelte';
  import Pagination from '$lib/components/ui/Pagination.svelte';
  import { documentsStore, documentsActions } from '$lib/stores/documents';
  
  let searchFilters = {
    title: '',
    businessNumber: '',
    documentTypeId: '',
    createdDateFrom: '',
    createdDateTo: '',
    confidentiality: {
      internalExternal: '',
      importanceClass: '',
      personalInfo: ''
    }
  };
  
  let currentPage = 1;
  const pageSize = 20;
  
  $: documents = $documentsStore.documents;
  $: totalCount = $documentsStore.totalCount;
  $: isLoading = $documentsStore.isLoading;
  
  async function handleSearch() {
    await documentsActions.search({
      ...searchFilters,
      pagination: {
        offset: (currentPage - 1) * pageSize,
        limit: pageSize
      }
    });
  }
  
  function handlePageChange(page: number) {
    currentPage = page;
    handleSearch();
  }
  
  onMount(() => {
    handleSearch();
  });
</script>

<div class="container mx-auto px-4 py-8">
  <div class="mb-8">
    <h1 class="text-3xl font-bold text-gray-900 mb-4">文書検索</h1>
    
    <SearchForm
      bind:filters={searchFilters}
      on:search={handleSearch}
      {isLoading}
    />
  </div>
  
  <div class="bg-white shadow rounded-lg">
    <DocumentTable
      {documents}
      {isLoading}
      on:documentSelect={(event) => {
        window.location.href = `/documents/${event.detail.id}`;
      }}
    />
    
    {#if totalCount > pageSize}
      <div class="border-t border-gray-200 px-6 py-4">
        <Pagination
          {currentPage}
          {totalCount}
          {pageSize}
          on:pageChange={(event) => handlePageChange(event.detail)}
        />
      </div>
    {/if}
  </div>
</div>
```

#### 検索フォーム

```svelte
<!-- src/lib/components/forms/SearchForm.svelte -->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import Button from '$lib/components/ui/Button.svelte';
  import Input from '$lib/components/ui/Input.svelte';
  import Select from '$lib/components/ui/Select.svelte';
  
  export let filters: any;
  export let isLoading = false;
  
  const dispatch = createEventDispatcher();
  
  let showAdvanced = false;
  
  function handleSubmit() {
    dispatch('search');
  }
  
  function clearFilters() {
    filters = {
      title: '',
      businessNumber: '',
      documentTypeId: '',
      createdDateFrom: '',
      createdDateTo: '',
      confidentiality: {
        internalExternal: '',
        importanceClass: '',
        personalInfo: ''
      }
    };
    dispatch('search');
  }
</script>

<form on:submit|preventDefault={handleSubmit} class="bg-gray-50 p-6 rounded-lg">
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 mb-4">
    <div>
      <label for="title" class="block text-sm font-medium text-gray-700 mb-1">
        文書タイトル
      </label>
      <Input
        id="title"
        bind:value={filters.title}
        placeholder="タイトルで検索..."
      />
    </div>
    
    <div>
      <label for="businessNumber" class="block text-sm font-medium text-gray-700 mb-1">
        業務番号
      </label>
      <Input
        id="businessNumber"
        bind:value={filters.businessNumber}
        placeholder="業務番号で検索..."
      />
    </div>
    
    <div>
      <label for="documentType" class="block text-sm font-medium text-gray-700 mb-1">
        文書種別
      </label>
      <Select
        id="documentType"
        bind:value={filters.documentTypeId}
        options={documentTypeOptions}
        placeholder="文書種別を選択..."
      />
    </div>
  </div>
  
  {#if showAdvanced}
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4 p-4 bg-white rounded border">
      <div>
        <label for="dateFrom" class="block text-sm font-medium text-gray-700 mb-1">
          作成日（開始）
        </label>
        <Input
          id="dateFrom"
          type="date"
          bind:value={filters.createdDateFrom}
        />
      </div>
      
      <div>
        <label for="dateTo" class="block text-sm font-medium text-gray-700 mb-1">
          作成日（終了）
        </label>
        <Input
          id="dateTo"
          type="date"
          bind:value={filters.createdDateTo}
        />
      </div>
      
      <!-- 機密レベル選択 -->
      <div class="col-span-full">
        <h4 class="text-sm font-medium text-gray-700 mb-2">機密レベル</h4>
        <div class="grid grid-cols-3 gap-4">
          <Select
            bind:value={filters.confidentiality.internalExternal}
            options={internalExternalOptions}
            placeholder="社内外区分"
          />
          <Select
            bind:value={filters.confidentiality.importanceClass}
            options={importanceClassOptions}
            placeholder="重要度"
          />
          <Select
            bind:value={filters.confidentiality.personalInfo}
            options={personalInfoOptions}
            placeholder="個人情報"
          />
        </div>
      </div>
    </div>
  {/if}
  
  <div class="flex justify-between items-center">
    <button
      type="button"
      on:click={() => showAdvanced = !showAdvanced}
      class="text-blue-600 hover:text-blue-700 text-sm font-medium"
    >
      {showAdvanced ? '詳細検索を閉じる' : '詳細検索'}
    </button>
    
    <div class="space-x-2">
      <Button
        type="button"
        variant="secondary"
        on:click={clearFilters}
      >
        クリア
      </Button>
      <Button
        type="submit"
        loading={isLoading}
      >
        検索
      </Button>
    </div>
  </div>
</form>
```

#### 成果物

- 高機能検索フォーム
- 検索結果表示テーブル
- フィルタリング機能
- ページング機能

---

### TASK-026: 文書管理画面

- **説明**: 登録・編集・詳細画面
- **優先度**: High
- **見積工数**: 20h
- **状態**: 未着手
- **依存関係**: TASK-025

#### 実装内容

1. 文書登録フォーム
2. 文書編集画面
3. 文書詳細表示
4. ファイル操作UI

#### 文書登録画面

```svelte
<!-- src/routes/documents/new/+page.svelte -->
<script lang="ts">
  import { goto } from '$app/navigation';
  import DocumentForm from '$lib/components/forms/DocumentForm.svelte';
  import { documentsActions } from '$lib/stores/documents';
  import { addNotification } from '$lib/stores/notifications';
  
  let isSubmitting = false;
  
  async function handleSubmit(event: CustomEvent) {
    isSubmitting = true;
    
    try {
      const document = await documentsActions.create(event.detail);
      addNotification({
        type: 'success',
        message: `文書「${document.title}」を作成しました。文書番号: ${document.number}`
      });
      goto(`/documents/${document.id}`);
    } catch (error) {
      addNotification({
        type: 'error',
        message: '文書の作成に失敗しました。'
      });
    } finally {
      isSubmitting = false;
    }
  }
</script>

<div class="container mx-auto px-4 py-8">
  <div class="mb-8">
    <h1 class="text-3xl font-bold text-gray-900">新規文書作成</h1>
    <p class="mt-2 text-gray-600">新しい文書を作成します。必須項目を入力してください。</p>
  </div>
  
  <div class="max-w-4xl">
    <DocumentForm
      mode="create"
      loading={isSubmitting}
      on:submit={handleSubmit}
      on:cancel={() => goto('/documents')}
    />
  </div>
</div>
```

#### 文書フォーム

```svelte
<!-- src/lib/components/forms/DocumentForm.svelte -->
<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { authStore } from '$lib/stores/auth';
  import Button from '$lib/components/ui/Button.svelte';
  import Input from '$lib/components/ui/Input.svelte';
  import Select from '$lib/components/ui/Select.svelte';
  import TextArea from '$lib/components/ui/TextArea.svelte';
  import DocumentNumberDisplay from '$lib/components/DocumentNumberDisplay.svelte';
  
  export let mode: 'create' | 'edit' = 'create';
  export let initialData: any = null;
  export let loading = false;
  
  const dispatch = createEventDispatcher();
  
  let formData = {
    title: '',
    documentTypeId: '',
    businessNumber: '',
    createdDate: new Date().toISOString().split('T')[0],
    confidentiality: {
      internalExternal: 'internal',
      importanceClass: 'class2',
      personalInfo: 'none'
    },
    notes: '',
    ...initialData
  };
  
  let generatedNumber = '';
  let errors: Record<string, string> = {};
  
  $: user = $authStore.user;
  
  // 文書番号の自動生成プレビュー
  $: if (formData.documentTypeId && formData.createdDate) {
    generateNumberPreview();
  }
  
  async function generateNumberPreview() {
    try {
      const response = await fetch('/api/documents/preview-number', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          documentTypeId: formData.documentTypeId,
          createdDate: formData.createdDate
        })
      });
      
      if (response.ok) {
        const { number } = await response.json();
        generatedNumber = number;
      }
    } catch (error) {
      console.error('Failed to generate number preview:', error);
    }
  }
  
  function validateForm() {
    errors = {};
    
    if (!formData.title.trim()) {
      errors.title = 'タイトルは必須です';
    }
    
    if (!formData.documentTypeId) {
      errors.documentTypeId = '文書種別は必須です';
    }
    
    if (!formData.createdDate) {
      errors.createdDate = '作成日は必須です';
    }
    
    return Object.keys(errors).length === 0;
  }
  
  function handleSubmit() {
    if (!validateForm()) return;
    
    dispatch('submit', formData);
  }
</script>

<form on:submit|preventDefault={handleSubmit} class="space-y-8">
  <!-- 基本情報セクション -->
  <div class="bg-white shadow rounded-lg p-6">
    <h2 class="text-lg font-medium text-gray-900 mb-4">基本情報</h2>
    
    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
      <div class="md:col-span-2">
        <label for="title" class="block text-sm font-medium text-gray-700 mb-1">
          文書タイトル <span class="text-red-500">*</span>
        </label>
        <Input
          id="title"
          bind:value={formData.title}
          error={errors.title}
          placeholder="文書のタイトルを入力..."
        />
      </div>
      
      <div>
        <label for="documentType" class="block text-sm font-medium text-gray-700 mb-1">
          文書種別 <span class="text-red-500">*</span>
        </label>
        <Select
          id="documentType"
          bind:value={formData.documentTypeId}
          options={documentTypeOptions}
          error={errors.documentTypeId}
          placeholder="文書種別を選択..."
        />
      </div>
      
      <div>
        <label for="businessNumber" class="block text-sm font-medium text-gray-700 mb-1">
          業務番号
        </label>
        <Input
          id="businessNumber"
          bind:value={formData.businessNumber}
          placeholder="業務番号（任意）"
        />
      </div>
      
      <div>
        <label for="createdDate" class="block text-sm font-medium text-gray-700 mb-1">
          作成日 <span class="text-red-500">*</span>
        </label>
        <Input
          id="createdDate"
          type="date"
          bind:value={formData.createdDate}
          error={errors.createdDate}
        />
      </div>
      
      {#if mode === 'create' && generatedNumber}
        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">
            生成される文書番号
          </label>
          <DocumentNumberDisplay number={generatedNumber} />
        </div>
      {/if}
    </div>
  </div>
  
  <!-- 機密レベル設定 -->
  <div class="bg-white shadow rounded-lg p-6">
    <h2 class="text-lg font-medium text-gray-900 mb-4">機密レベル設定</h2>
    
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-2">
          社内外区分 <span class="text-red-500">*</span>
        </label>
        <div class="space-y-2">
          <label class="flex items-center">
            <input
              type="radio"
              bind:group={formData.confidentiality.internalExternal}
              value="internal"
              class="mr-2"
            />
            社内
          </label>
          <label class="flex items-center">
            <input
              type="radio"
              bind:group={formData.confidentiality.internalExternal}
              value="external"
              class="mr-2"
            />
            社外
          </label>
        </div>
      </div>
      
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-2">
          重要度 <span class="text-red-500">*</span>
        </label>
        <div class="space-y-2">
          <label class="flex items-center">
            <input
              type="radio"
              bind:group={formData.confidentiality.importanceClass}
              value="class1"
              class="mr-2"
            />
            情報クラスⅠ（重要）
          </label>
          <label class="flex items-center">
            <input
              type="radio"
              bind:group={formData.confidentiality.importanceClass}
              value="class2"
              class="mr-2"
            />
            情報クラスⅡ（通常）
          </label>
        </div>
      </div>
      
      <div>
        <label class="block text-sm font-medium text-gray-700 mb-2">
          個人情報 <span class="text-red-500">*</span>
        </label>
        <div class="space-y-2">
          <label class="flex items-center">
            <input
              type="radio"
              bind:group={formData.confidentiality.personalInfo}
              value="none"
              class="mr-2"
            />
            なし
          </label>
          <label class="flex items-center">
            <input
              type="radio"
              bind:group={formData.confidentiality.personalInfo}
              value="present"
              class="mr-2"
            />
            あり
          </label>
        </div>
      </div>
    </div>
  </div>
  
  <!-- 備考 -->
  <div class="bg-white shadow rounded-lg p-6">
    <h2 class="text-lg font-medium text-gray-900 mb-4">備考</h2>
    
    <TextArea
      bind:value={formData.notes}
      placeholder="必要に応じて備考を入力..."
      rows={4}
    />
  </div>
  
  <!-- 送信ボタン -->
  <div class="flex justify-end space-x-4">
    <Button
      type="button"
      variant="secondary"
      on:click={() => dispatch('cancel')}
    >
      キャンセル
    </Button>
    <Button
      type="submit"
      {loading}
    >
      {mode === 'create' ? '作成' : '更新'}
    </Button>
  </div>
</form>
```

#### 成果物

- 文書登録・編集フォーム
- 文書詳細表示画面
- バリデーション機能
- ファイル操作UI

---

### TASK-027: 組織管理画面

- **説明**: 部署・人員管理画面
- **優先度**: Medium
- **見積工数**: 16h
- **状態**: 未着手
- **依存関係**: TASK-026

#### 実装内容

1. 部署一覧・編集画面
2. 社員一覧・詳細画面
3. 組織階層表示
4. 部署配属履歴管理

#### 成果物

- 組織管理UI
- 社員管理機能
- 部署階層表示
- 配属履歴管理

---

### TASK-028: 通知システム実装

- **説明**: Email/Teams通知機能
- **優先度**: High
- **見積工数**: 16h
- **状態**: 未着手
- **依存関係**: TASK-013

#### 実装内容

1. Email通知機能
2. Teams Webhook通知
3. 通知テンプレート管理
4. 通知履歴管理

#### Email通知実装

```rust
// src/services/notification_service.rs
use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

pub struct EmailService {
    smtp_transport: SmtpTransport,
    from_address: Mailbox,
}

impl EmailService {
    pub fn new(
        smtp_server: &str,
        smtp_port: u16,
        username: &str,
        password: &str,
        from_address: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let creds = Credentials::new(username.to_string(), password.to_string());
        
        let transport = SmtpTransport::relay(smtp_server)?
            .port(smtp_port)
            .credentials(creds)
            .build();
        
        let from = from_address.parse()?;
        
        Ok(Self {
            smtp_transport: transport,
            from_address: from,
        })
    }
    
    pub async fn send_file_missing_notification(
        &self,
        to_addresses: Vec<String>,
        document: &Document,
        missing_type: FileMissingType,
    ) -> Result<(), NotificationError> {
        let subject = format!("【ファイル不存在】文書番号: {}", document.number);
        
        let body = match missing_type {
            FileMissingType::Folder => {
                format!(
                    "文書「{}」（文書番号: {}）のフォルダが存在しません。\n\n\
                     ネットワークパス: {}\n\n\
                     確認日時: {}\n\n\
                     対応をお願いします。",
                    document.title,
                    document.number,
                    document.network_path.as_ref().unwrap_or(&"不明".to_string()),
                    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
                )
            },
            FileMissingType::Approval => {
                format!(
                    "文書「{}」（文書番号: {}）の審査承認書が存在しません。\n\n\
                     ネットワークパス: {}\n\
                     必要ファイル: {}-審査承認.pdf\n\n\
                     確認日時: {}\n\n\
                     対応をお願いします。",
                    document.title,
                    document.number,
                    document.network_path.as_ref().unwrap_or(&"不明".to_string()),
                    document.number,
                    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
                )
            }
        };
        
        for to_address in to_addresses {
            let email = Message::builder()
                .from(self.from_address.clone())
                .to(to_address.parse()?)
                .subject(&subject)
                .header(ContentType::TEXT_PLAIN)
                .body(body.clone())?;
            
            self.smtp_transport.send(&email)?;
        }
        
        Ok(())
    }
}
```

#### Teams通知実装

```rust
// src/services/teams_service.rs
use serde_json::json;

pub struct TeamsService {
    webhook_url: String,
    client: reqwest::Client,
}

impl TeamsService {
    pub fn new(webhook_url: String) -> Self {
        Self {
            webhook_url,
            client: reqwest::Client::new(),
        }
    }
    
    pub async fn send_file_check_summary(
        &self,
        total_documents: usize,
        missing_folders: usize,
        missing_approvals: usize,
    ) -> Result<(), NotificationError> {
        let message = json!({
            "@type": "MessageCard",
            "@context": "http://schema.org/extensions",
            "themeColor": if missing_folders > 0 || missing_approvals > 0 { "FF0000" } else { "00FF00" },
            "summary": "ファイル存在確認結果",
            "sections": [{
                "activityTitle": "📁 月次ファイル存在確認結果",
                "activitySubtitle": format!("確認日時: {}", chrono::Utc::now().format("%Y-%m-%d %H:%M UTC")),
                "facts": [
                    {
                        "name": "確認対象文書数",
                        "value": total_documents.to_string()
                    },
                    {
                        "name": "フォルダ不存在",
                        "value": format!("{} 件", missing_folders)
                    },
                    {
                        "name": "承認書不存在",
                        "value": format!("{} 件", missing_approvals)
                    }
                ],
                "markdown": true
            }],
            "potentialAction": [{
                "@type": "OpenUri",
                "name": "詳細を確認",
                "targets": [{
                    "os": "default",
                    "uri": "https://docman.corp.local/reports/file-check"
                }]
            }]
        });
        
        let response = self.client
            .post(&self.webhook_url)
            .json(&message)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(NotificationError::TeamsError(
                format!("Teams notification failed: {}", response.status())
            ));
        }
        
        Ok(())
    }
}
```

#### 成果物

- Email通知システム
- Teams通知機能
- 通知テンプレート
- 通知履歴管理

---

### TASK-029: ダッシュボード

- **説明**: 統計・状況表示画面
- **優先度**: Medium
- **見積工数**: 12h
- **状態**: 未着手
- **依存関係**: TASK-027

#### 実装内容

1. 文書統計ダッシュボード
2. ファイル確認状況表示
3. システム稼働状況
4. ユーザーアクティビティ

#### 成果物

- 統計ダッシュボード
- システム状況表示
- ユーザー向け情報表示

---

### TASK-030: レスポンシブ対応

- **説明**: モバイル・タブレット対応
- **優先度**: Low
- **見積工数**: 8h
- **状態**: 未着手
- **依存関係**: TASK-029

#### 実装内容

1. モバイル最適化
2. タブレット対応
3. タッチ操作対応
4. 画面サイズ適応

#### 成果物

- レスポンシブUI
- モバイル対応
- タッチ操作サポート

---

### TASK-031: UI/UXテスト

- **説明**: ユーザビリティテスト
- **優先度**: Medium
- **見積工数**: 6h
- **状態**: 未着手
- **依存関係**: TASK-030

#### 実装内容

1. ユーザビリティテスト
2. アクセシビリティテスト
3. パフォーマンステスト
4. ブラウザ互換性テスト

#### 成果物

- UI/UXテスト結果
- 改善提案
- 品質保証レポート

## フェーズ完了基準

### 必須条件

- [ ] 文書検索・登録・編集画面が動作する
- [ ] 認証機能が統合されている
- [ ] Email・Teams通知が送信される
- [ ] レスポンシブデザインが実装されている
- [ ] ユーザビリティテストが完了している

### 検証方法

```bash
# フロントエンド起動
cd ui && npm run dev

# 機能テスト
npm run test:e2e

# アクセシビリティテスト
npm run test:accessibility
```

## 次フェーズへの引き継ぎ事項

- 完全なWebユーザーインターフェース
- 通知システム稼働
- データ移行機能実装準備
- 本格運用準備

## リスク・課題

- **UI複雑性**: 多機能による画面の複雑化
- **通知設定**: 本番環境での通知設定
- **ブラウザ互換性**: 古いブラウザでの動作

## 対応策

- ユーザーテストによる改善
- 段階的通知機能展開
- ブラウザサポート方針明確化
