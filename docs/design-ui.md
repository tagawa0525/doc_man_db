# UI/UX設計書

## 1. UI/UX概要

### 1.1 設計方針

- **直感的操作**: 最小限の研修で利用可能な分かりやすいUI
- **効率性重視**: 日常業務での頻繁な操作を最適化
- **レスポンシブ**: デスクトップ・タブレット対応
- **アクセシビリティ**: 色覚・視覚に配慮したデザイン
- **一貫性**: 企業システムらしい統一感のあるデザイン

### 1.2 技術スタック

```json
{
  "framework": "SvelteKit + TypeScript",
  "styling": "Tailwind CSS",
  "components": "カスタムコンポーネント",
  "icons": "Heroicons / Lucide",
  "desktop": "Tauri (Web+α機能)",
  "browser": "Progressive Web App"
}
```

### 1.3 カラーパレット

```css
:root {
  /* Primary Colors */
  --color-primary-50: #eff6ff;
  --color-primary-100: #dbeafe;
  --color-primary-500: #3b82f6;   /* メインブルー */
  --color-primary-600: #2563eb;
  --color-primary-700: #1d4ed8;
  
  /* Secondary Colors */
  --color-gray-50: #f9fafb;
  --color-gray-100: #f3f4f6;
  --color-gray-300: #d1d5db;
  --color-gray-500: #6b7280;
  --color-gray-700: #374151;
  --color-gray-900: #111827;
  
  /* Status Colors */
  --color-success: #10b981;      /* 成功・完了 */
  --color-warning: #f59e0b;      /* 警告・注意 */
  --color-error: #ef4444;        /* エラー・削除 */
  --color-info: #06b6d4;         /* 情報・案内 */
  
  /* Confidentiality Colors */
  --color-internal: #059669;     /* 社内文書 */
  --color-external: #dc2626;     /* 社外文書 */
  --color-class1: #7c2d12;       /* 情報クラスⅠ */
  --color-class2: #92400e;       /* 情報クラスⅡ */
}
```

## 2. レイアウト設計

### 2.1 基本レイアウト構造

```text
┌─────────────────────────────────────────────────────────┐
│ Header (Logo, Navigation, User Menu)                   │
├─────────────────────────────────────────────────────────┤
│ ┌─────────────┐ ┌───────────────────────────────────────┐ │
│ │             │ │                                       │ │
│ │   Sidebar   │ │         Main Content Area            │ │
│ │             │ │                                       │ │
│ │ - Navigation│ │ - Page Title                          │ │
│ │ - Quick     │ │ - Breadcrumb                          │ │
│ │   Actions   │ │ - Content                             │ │
│ │ - Favorites │ │                                       │ │
│ │             │ │                                       │ │
│ └─────────────┘ └───────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────┤
│ Footer (Status, Notifications)                         │
└─────────────────────────────────────────────────────────┘
```

### 2.2 レスポンシブブレイクポイント

```css
/* Tailwind CSS ブレイクポイント */
/* sm: 640px   - タブレット縦 */
/* md: 768px   - タブレット横 */
/* lg: 1024px  - デスクトップ小 */
/* xl: 1280px  - デスクトップ大 */
```

### 2.3 ナビゲーション構造

```text
メインナビゲーション:
├── 📊 ダッシュボード
├── 📄 文書管理
│   ├── 文書一覧
│   ├── 文書登録
│   └── 文書検索
├── 👥 組織管理
│   ├── 社員一覧
│   ├── 部署管理
│   └── 所属履歴
├── 💼 業務管理
│   ├── 業務一覧
│   ├── 従事者管理
│   └── 外部連絡先
├── ⚙️ システム管理
│   ├── ルール管理
│   ├── ファイル確認
│   └── ログ確認
└── 📤 データ管理
    ├── CSVインポート
    ├── データエクスポート
    └── バックアップ
```

## 3. ページ設計

### 3.1 ダッシュボード

```text
┌─────────────────────────────────────────────────────────┐
│ 📊 ダッシュボード                    🔔 通知: 3件       │
├─────────────────────────────────────────────────────────┤
│ ┌─────────────┐ ┌─────────────┐ ┌─────────────────────┐ │
│ │📈 文書統計   │ │👥 組織状況   │ │⚠️ 要確認事項        │ │
│ │             │ │             │ │                     │ │
│ │総数: 5,234  │ │部署: 12     │ │• ファイル未確認: 15 │ │
│ │今月: 125    │ │社員: 45     │ │• 承認書不足: 3      │ │
│ │承認待ち: 8  │ │異動: 2      │ │• パス不正: 1        │ │
│ └─────────────┘ └─────────────┘ └─────────────────────┘ │
├─────────────────────────────────────────────────────────┤
│ 📋 最近の文書                                           │
│ ┌─────────────────────────────────────────────────────┐ │
│ │ CTA-2508001 | 月次会議議事録      | 2024/12/15 | ✅ │ │
│ │ CTA-2508002 | システム提案書      | 2024/12/14 | 📋 │ │
│ │ CTA-2508003 | 顧客要件定義書      | 2024/12/13 | ⏳ │ │
│ └─────────────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────┤
│ 🔍 クイック検索                     📁 よく使う機能     │
│ ┌─────────────────────┐             ┌─────────────────┐ │
│ │ [検索ボックス]  [🔍] │             │• 文書登録       │ │
│ └─────────────────────┘             │• CSV取込み      │ │
│                                     │• ファイル確認   │ │
│                                     └─────────────────┘ │
└─────────────────────────────────────────────────────────┘
```

### 3.2 文書一覧ページ

```text
┌─────────────────────────────────────────────────────────┐
│ 📄 文書一覧                                             │
├─────────────────────────────────────────────────────────┤
│ 🔍 検索フィルタ                        [+ 新規登録]     │
│ ┌─────────────────────────────────────────────────────┐ │
│ │ 文書番号: [_______] タイトル: [__________]           │ │
│ │ 種別: [選択▼] 作成者: [_____] 期間: [____] - [____] │ │
│ │ 機密: 社内外[▼] 重要度[▼] 個人情報[▼] [🔍検索][🔄] │ │
│ └─────────────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────┤
│ 📊 検索結果: 1,234件 (10件表示)        📥 エクスポート │
│ ┌─────────────────────────────────────────────────────┐ │
│ │☐│文書番号      │タイトル    │種別│作成日  │状態│操作│ │
│ │─┼─────────────┼──────────┼───┼───────┼───┼───│ │
│ │☐│CTA-2508001  │月次会議議事録│A  │12/15  │✅ │📁📝│ │
│ │☐│CTA-2508002  │提案書      │B  │12/14  │📋 │📁📝│ │
│ │☐│CTA-2508003  │要件定義書   │B  │12/13  │⏳ │📁📝│ │
│ │─┼─────────────┼──────────┼───┼───────┼───┼───│ │
│ │  [一括操作▼] 選択: 0件                              │ │
│ └─────────────────────────────────────────────────────┘ │
│ ◀ 1 2 3 ... 124 ▶                                     │
└─────────────────────────────────────────────────────────┘
```

### 3.3 文書登録ページ

```text
┌─────────────────────────────────────────────────────────┐
│ ➕ 文書登録                              [キャンセル]   │
├─────────────────────────────────────────────────────────┤
│ 📝 基本情報                                             │
│ ┌─────────────────────────────────────────────────────┐ │
│ │ タイトル*: [________________________________]       │ │
│ │ 文書種別*: [報告書        ▼] 作成日*: [2024/12/15] │ │
│ │ 業務番号:  [JOB-2024-001] 作成者*: [田中太郎(自動)] │ │
│ └─────────────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────┤
│ 🔐 機密レベル                                           │
│ ┌─────────────────────────────────────────────────────┐ │
│ │ 社内外*: (●)社内 ( )社外                           │ │
│ │ 重要度*: (●)情報クラスⅡ ( )情報クラスⅠ            │ │
│ │ 個人情報*: (●)なし ( )あり                         │ │
│ └─────────────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────┤
│ 📂 ファイル情報                                         │
│ ┌─────────────────────────────────────────────────────┐ │
│ │ 文書番号: [CTA-2508004] (自動生成) [🔄 再生成]     │ │
│ │ パス: \\server01\docs\2024\技術部\報告書\CTA-2508004 │ │
│ │ 承認書: [×] 不要 [📁] フォルダ確認                   │ │
│ └─────────────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────┤
│ 📋 備考・メモ                                           │
│ ┌─────────────────────────────────────────────────────┐ │
│ │ [_________________________________________________] │ │
│ │                                                     │ │
│ └─────────────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────┤
│                              [登録] [下書き保存]       │
└─────────────────────────────────────────────────────────┘
```

### 3.4 文書詳細ページ

```text
┌─────────────────────────────────────────────────────────┐
│ 📄 CTA-2508001: 月次会議議事録           [編集] [削除]  │
├─────────────────────────────────────────────────────────┤
│ ℹ️ 基本情報                                             │
│ ┌───────────────┬───────────────┬─────────────────────┐ │
│ │ 文書番号        │ 作成日        │ 最終更新           │ │
│ │ CTA-2508001    │ 2024/12/15   │ 2024/12/15 10:30  │ │
│ ├───────────────┼───────────────┼─────────────────────┤ │
│ │ 文書種別        │ 業務番号      │ 作成者             │ │
│ │ A(報告書)      │ JOB-2024-001 │ 田中太郎           │ │
│ └───────────────┴───────────────┴─────────────────────┘ │
├─────────────────────────────────────────────────────────┤
│ 🔐 機密レベル: 🟢社内 🟡クラスⅡ ⚪個人情報なし          │
├─────────────────────────────────────────────────────────┤
│ 📂 ファイル情報                                         │
│ ┌─────────────────────────────────────────────────────┐ │
│ │ パス: \\server01\docs\2024\技術部\報告書\CTA-2508001  │ │
│ │ ステータス: ✅ フォルダ確認済み (2024/12/15 02:00)   │ │
│ │ 承認書: ✅ 存在確認済み (CTA-2508001-審査承認.pdf)   │ │
│ │ 操作: [📁 フォルダを開く] [🔍 ファイル確認]         │ │
│ └─────────────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────────────┤
│ 📋 備考・履歴                                           │
│ ┌─────────────────────────────────────────────────────┐ │
│ │ 備考: 2024年12月定例会議の議事録                    │ │
│ │                                                     │ │
│ │ 📋 更新履歴:                                       │ │
│ │ • 2024/12/15 10:30 - 田中太郎: 文書登録             │ │
│ │ • 2024/12/15 11:00 - システム: ファイル確認完了     │ │
│ └─────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────┘
```

## 4. コンポーネント設計

### 4.1 共通コンポーネント

#### 4.1.1 Button Component

```svelte
<!-- Button.svelte -->
<script lang="ts">
  export let variant: 'primary' | 'secondary' | 'success' | 'warning' | 'error' = 'primary';
  export let size: 'sm' | 'md' | 'lg' = 'md';
  export let disabled = false;
  export let loading = false;
  export let icon: string | undefined = undefined;
  
  $: buttonClasses = `
    inline-flex items-center justify-center rounded-md font-medium
    transition-colors duration-200 focus:outline-none focus:ring-2
    ${sizeClasses[size]}
    ${variantClasses[variant]}
    ${disabled || loading ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'}
  `;
  
  const sizeClasses = {
    sm: 'px-3 py-1.5 text-sm',
    md: 'px-4 py-2 text-sm',
    lg: 'px-6 py-3 text-base'
  };
  
  const variantClasses = {
    primary: 'bg-primary-500 text-white hover:bg-primary-600 focus:ring-primary-300',
    secondary: 'bg-gray-200 text-gray-900 hover:bg-gray-300 focus:ring-gray-300',
    success: 'bg-green-500 text-white hover:bg-green-600 focus:ring-green-300',
    warning: 'bg-yellow-500 text-white hover:bg-yellow-600 focus:ring-yellow-300',
    error: 'bg-red-500 text-white hover:bg-red-600 focus:ring-red-300'
  };
</script>

<button
  class={buttonClasses}
  {disabled}
  on:click
  on:submit
>
  {#if loading}
    <svg class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" fill="none" viewBox="0 0 24 24">
      <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"/>
      <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"/>
    </svg>
  {/if}
  
  {#if icon && !loading}
    <Icon name={icon} class="mr-2 h-4 w-4" />
  {/if}
  
  <slot />
</button>
```

#### 4.1.2 DataTable Component

```svelte
<!-- DataTable.svelte -->
<script lang="ts">
  export let data: any[] = [];
  export let columns: TableColumn[] = [];
  export let pagination: PaginationInfo | undefined = undefined;
  export let selectable = false;
  export let loading = false;
  
  let selectedRows: Set<string> = new Set();
  
  interface TableColumn {
    key: string;
    label: string;
    sortable?: boolean;
    width?: string;
    align?: 'left' | 'center' | 'right';
    render?: (value: any, row: any) => string;
  }
  
  interface PaginationInfo {
    currentPage: number;
    totalPages: number;
    totalItems: number;
    itemsPerPage: number;
  }
</script>

<div class="overflow-hidden shadow ring-1 ring-black ring-opacity-5 md:rounded-lg">
  <!-- Table Header -->
  <table class="min-w-full divide-y divide-gray-300">
    <thead class="bg-gray-50">
      <tr>
        {#if selectable}
          <th class="w-12 px-6 py-3">
            <input
              type="checkbox"
              class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
              on:change={handleSelectAll}
            />
          </th>
        {/if}
        
        {#each columns as column}
          <th
            class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider"
            style={column.width ? `width: ${column.width}` : ''}
          >
            <div class="flex items-center space-x-1">
              <span>{column.label}</span>
              {#if column.sortable}
                <button class="text-gray-400 hover:text-gray-600">
                  <Icon name="chevron-up-down" class="h-4 w-4" />
                </button>
              {/if}
            </div>
          </th>
        {/each}
      </tr>
    </thead>
    
    <!-- Table Body -->
    <tbody class="bg-white divide-y divide-gray-200">
      {#if loading}
        <tr>
          <td colspan={selectable ? columns.length + 1 : columns.length} class="px-6 py-12 text-center">
            <div class="flex items-center justify-center">
              <Icon name="spinner" class="animate-spin h-6 w-6 text-gray-400 mr-2" />
              <span class="text-gray-500">読み込み中...</span>
            </div>
          </td>
        </tr>
      {:else if data.length === 0}
        <tr>
          <td colspan={selectable ? columns.length + 1 : columns.length} class="px-6 py-12 text-center">
            <span class="text-gray-500">データがありません</span>
          </td>
        </tr>
      {:else}
        {#each data as row, index}
          <tr class="hover:bg-gray-50 {selectedRows.has(row.id) ? 'bg-primary-50' : ''}">
            {#if selectable}
              <td class="px-6 py-4">
                <input
                  type="checkbox"
                  class="h-4 w-4 text-primary-600 focus:ring-primary-500 border-gray-300 rounded"
                  checked={selectedRows.has(row.id)}
                  on:change={(e) => handleRowSelect(row.id, e.target.checked)}
                />
              </td>
            {/if}
            
            {#each columns as column}
              <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                {#if column.render}
                  {@html column.render(row[column.key], row)}
                {:else}
                  {row[column.key] || '-'}
                {/if}
              </td>
            {/each}
          </tr>
        {/each}
      {/if}
    </tbody>
  </table>
  
  <!-- Pagination -->
  {#if pagination && data.length > 0}
    <div class="bg-white px-4 py-3 flex items-center justify-between border-t border-gray-200">
      <div class="flex-1 flex justify-between sm:hidden">
        <Button
          variant="secondary"
          disabled={pagination.currentPage <= 1}
          on:click={() => goToPage(pagination.currentPage - 1)}
        >
          前へ
        </Button>
        <Button
          variant="secondary"
          disabled={pagination.currentPage >= pagination.totalPages}
          on:click={() => goToPage(pagination.currentPage + 1)}
        >
          次へ
        </Button>
      </div>
      
      <div class="hidden sm:flex-1 sm:flex sm:items-center sm:justify-between">
        <div>
          <p class="text-sm text-gray-700">
            {pagination.totalItems}件中 
            {(pagination.currentPage - 1) * pagination.itemsPerPage + 1}
            - 
            {Math.min(pagination.currentPage * pagination.itemsPerPage, pagination.totalItems)}
            件を表示
          </p>
        </div>
        
        <div>
          <nav class="relative z-0 inline-flex rounded-md shadow-sm -space-x-px">
            <!-- Page numbers -->
            {#each Array.from({length: pagination.totalPages}, (_, i) => i + 1) as page}
              {#if page === 1 || page === pagination.totalPages || Math.abs(page - pagination.currentPage) <= 2}
                <button
                  class="relative inline-flex items-center px-4 py-2 border text-sm font-medium
                    {page === pagination.currentPage
                      ? 'z-10 bg-primary-50 border-primary-500 text-primary-600'
                      : 'bg-white border-gray-300 text-gray-500 hover:bg-gray-50'}"
                  on:click={() => goToPage(page)}
                >
                  {page}
                </button>
              {:else if Math.abs(page - pagination.currentPage) === 3}
                <span class="relative inline-flex items-center px-4 py-2 border border-gray-300 bg-white text-sm font-medium text-gray-700">
                  ...
                </span>
              {/if}
            {/each}
          </nav>
        </div>
      </div>
    </div>
  {/if}
</div>
```

#### 4.1.3 Modal Component

```svelte
<!-- Modal.svelte -->
<script lang="ts">
  export let isOpen = false;
  export let title: string;
  export let size: 'sm' | 'md' | 'lg' | 'xl' = 'md';
  export let closable = true;
  
  $: if (isOpen) {
    document.body.style.overflow = 'hidden';
  } else {
    document.body.style.overflow = '';
  }
  
  const sizeClasses = {
    sm: 'max-w-md',
    md: 'max-w-lg',
    lg: 'max-w-2xl',
    xl: 'max-w-4xl'
  };
  
  function closeModal() {
    if (closable) {
      isOpen = false;
    }
  }
  
  function handleKeydown(event: KeyboardEvent) {
    if (event.key === 'Escape' && closable) {
      closeModal();
    }
  }
</script>

{#if isOpen}
  <div
    class="fixed inset-0 z-50 overflow-y-auto"
    aria-labelledby="modal-title"
    role="dialog"
    aria-modal="true"
    on:keydown={handleKeydown}
  >
    <!-- Background overlay -->
    <div class="flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0">
      <div
        class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"
        on:click={closeModal}
      ></div>
      
      <!-- Modal panel -->
      <div class="inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle {sizeClasses[size]} sm:w-full">
        <!-- Header -->
        <div class="bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4">
          <div class="flex items-start">
            <div class="mt-3 text-center sm:mt-0 sm:text-left flex-1">
              <h3 class="text-lg leading-6 font-medium text-gray-900" id="modal-title">
                {title}
              </h3>
            </div>
            
            {#if closable}
              <div class="ml-3 h-7 flex items-center">
                <button
                  type="button"
                  class="bg-white rounded-md text-gray-400 hover:text-gray-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                  on:click={closeModal}
                >
                  <Icon name="x-mark" class="h-6 w-6" />
                </button>
              </div>
            {/if}
          </div>
        </div>
        
        <!-- Content -->
        <div class="bg-white px-4 pb-4 sm:p-6 sm:pt-0">
          <slot />
        </div>
        
        <!-- Footer -->
        <div class="bg-gray-50 px-4 py-3 sm:px-6 sm:flex sm:flex-row-reverse">
          <slot name="footer" />
        </div>
      </div>
    </div>
  </div>
{/if}
```

### 4.2 フォームコンポーネント

#### 4.2.1 FormInput Component

```svelte
<!-- FormInput.svelte -->
<script lang="ts">
  export let label: string;
  export let name: string;
  export let type: 'text' | 'email' | 'password' | 'number' | 'date' = 'text';
  export let value: string | number = '';
  export let placeholder = '';
  export let required = false;
  export let disabled = false;
  export let error: string | undefined = undefined;
  export let helpText: string | undefined = undefined;
  
  let inputElement: HTMLInputElement;
  
  $: hasError = !!error;
  $: inputClasses = `
    block w-full px-3 py-2 border rounded-md shadow-sm
    focus:outline-none focus:ring-2 sm:text-sm
    ${hasError
      ? 'border-red-300 focus:ring-red-500 focus:border-red-500'
      : 'border-gray-300 focus:ring-primary-500 focus:border-primary-500'}
    ${disabled ? 'bg-gray-50 text-gray-500 cursor-not-allowed' : 'bg-white'}
  `;
</script>

<div class="space-y-1">
  <label for={name} class="block text-sm font-medium text-gray-700">
    {label}
    {#if required}
      <span class="text-red-500">*</span>
    {/if}
  </label>
  
  <input
    bind:this={inputElement}
    bind:value
    {name}
    {type}
    {placeholder}
    {required}
    {disabled}
    id={name}
    class={inputClasses}
    on:input
    on:change
    on:focus
    on:blur
  />
  
  {#if error}
    <p class="text-sm text-red-600">{error}</p>
  {:else if helpText}
    <p class="text-sm text-gray-500">{helpText}</p>
  {/if}
</div>
```

## 5. 状態管理

### 5.1 Svelte Store設計

```typescript
// stores/auth.ts
import { writable, derived } from 'svelte/store';
import type { Employee, UserPermissions } from '../types';

interface AuthState {
  user: Employee | null;
  permissions: UserPermissions | null;
  isAuthenticated: boolean;
  isLoading: boolean;
}

const initialState: AuthState = {
  user: null,
  permissions: null,
  isAuthenticated: false,
  isLoading: true,
};

export const authStore = writable<AuthState>(initialState);

export const currentUser = derived(authStore, $auth => $auth.user);
export const userPermissions = derived(authStore, $auth => $auth.permissions);
export const isAuthenticated = derived(authStore, $auth => $auth.isAuthenticated);

// Actions
export const authActions = {
  async login() {
    authStore.update(state => ({ ...state, isLoading: true }));
    
    try {
      const response = await fetch('/api/auth/me');
      const data = await response.json();
      
      authStore.set({
        user: data.user,
        permissions: data.permissions,
        isAuthenticated: true,
        isLoading: false,
      });
    } catch (error) {
      authStore.set({
        ...initialState,
        isLoading: false,
      });
    }
  },
  
  async logout() {
    await fetch('/api/auth/logout', { method: 'POST' });
    authStore.set(initialState);
  },
};
```

```typescript
// stores/documents.ts
import { writable } from 'svelte/store';
import type { Document, DocumentSearchInput } from '../types';

interface DocumentState {
  documents: Document[];
  currentDocument: Document | null;
  searchFilters: DocumentSearchInput;
  isLoading: boolean;
  error: string | null;
}

export const documentStore = writable<DocumentState>({
  documents: [],
  currentDocument: null,
  searchFilters: {},
  isLoading: false,
  error: null,
});

export const documentActions = {
  async searchDocuments(filters: DocumentSearchInput) {
    documentStore.update(state => ({ ...state, isLoading: true, error: null }));
    
    try {
      const query = `
        query SearchDocuments($search: DocumentSearchInput) {
          documents(search: $search) {
            documents {
              id number title createdDate
              documentType { name }
              creator { name }
            }
            totalCount
          }
        }
      `;
      
      const response = await fetch('/graphql', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ query, variables: { search: filters } }),
      });
      
      const data = await response.json();
      
      documentStore.update(state => ({
        ...state,
        documents: data.data.documents.documents,
        searchFilters: filters,
        isLoading: false,
      }));
    } catch (error) {
      documentStore.update(state => ({
        ...state,
        error: error.message,
        isLoading: false,
      }));
    }
  },
};
```

## 6. アクセシビリティ

### 6.1 WCAG 2.1 対応

- **AA レベル準拠**: 色覚、視覚、聴覚、運動機能への配慮
- **キーボードナビゲーション**: Tab/Shift+Tab での完全操作
- **スクリーンリーダー対応**: 適切なARIA属性、セマンティックHTML
- **カラーコントラスト**: 4.5:1 以上のコントラスト比

### 6.2 実装例

```svelte
<!-- アクセシブルなボタン -->
<button
  type="button"
  aria-label="文書を削除"
  aria-describedby="delete-description"
  on:click={handleDelete}
>
  <Icon name="trash" aria-hidden="true" />
  削除
</button>
<div id="delete-description" class="sr-only">
  選択された文書を完全に削除します。この操作は取り消せません。
</div>

<!-- アクセシブルなフォーム -->
<form on:submit={handleSubmit} aria-labelledby="form-title">
  <h2 id="form-title">文書登録フォーム</h2>
  
  <fieldset>
    <legend>機密レベル設定</legend>
    <div role="group" aria-labelledby="confidentiality-label">
      <div id="confidentiality-label" class="text-sm font-medium text-gray-700">
        社内外区分 <span class="text-red-500">*</span>
      </div>
      <label class="inline-flex items-center">
        <input
          type="radio"
          name="internal_external"
          value="internal"
          bind:group={internalExternal}
          aria-describedby="internal-desc"
        />
        <span class="ml-2">社内</span>
      </label>
      <div id="internal-desc" class="sr-only">社内関係者のみ閲覧可能</div>
    </div>
  </fieldset>
</form>
```

## 7. パフォーマンス最適化

### 7.1 コード分割

```typescript
// ルーティングでの動的インポート
export const routes = {
  '/': () => import('./pages/Dashboard.svelte'),
  '/documents': () => import('./pages/DocumentList.svelte'),
  '/documents/new': () => import('./pages/DocumentCreate.svelte'),
  '/documents/:id': () => import('./pages/DocumentDetail.svelte'),
};
```

### 7.2 仮想スクロール

```svelte
<!-- 大量データ用仮想スクロール -->
<script>
  import VirtualList from '@sveltejs/svelte-virtual-list';
  
  export let documents = [];
  
  let viewport;
  let contents;
</script>

<div bind:this={viewport} class="h-96 overflow-auto">
  <VirtualList
    bind:this={contents}
    items={documents}
    let:item
  >
    <DocumentRow document={item} />
  </VirtualList>
</div>
```

## 8. テーマ・ダークモード

### 8.1 CSS変数によるテーマ切り替え

```css
/* themes.css */
:root {
  --color-background: #ffffff;
  --color-surface: #f9fafb;
  --color-text-primary: #111827;
  --color-text-secondary: #6b7280;
}

[data-theme="dark"] {
  --color-background: #111827;
  --color-surface: #1f2937;
  --color-text-primary: #f9fafb;
  --color-text-secondary: #d1d5db;
}

.bg-background { background-color: var(--color-background); }
.bg-surface { background-color: var(--color-surface); }
.text-primary { color: var(--color-text-primary); }
.text-secondary { color: var(--color-text-secondary); }
```

### 8.2 テーマストア

```typescript
// stores/theme.ts
import { writable } from 'svelte/store';
import { browser } from '$app/environment';

type Theme = 'light' | 'dark' | 'system';

const defaultTheme: Theme = 'system';

export const theme = writable<Theme>(defaultTheme);

export function initializeTheme() {
  if (browser) {
    const stored = localStorage.getItem('theme') as Theme;
    if (stored) {
      theme.set(stored);
    }
    
    theme.subscribe(value => {
      localStorage.setItem('theme', value);
      updateThemeClass(value);
    });
  }
}

function updateThemeClass(currentTheme: Theme) {
  const root = document.documentElement;
  
  if (currentTheme === 'system') {
    const systemDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
    root.setAttribute('data-theme', systemDark ? 'dark' : 'light');
  } else {
    root.setAttribute('data-theme', currentTheme);
  }
}
```

---

**最終更新**: 2024年12月  
**作成者**: 開発チーム  
**承認者**: プロジェクトマネージャー
