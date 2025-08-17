<script lang="ts">
  import { page } from '$app/stores';
  import { onMount } from 'svelte';
  import Button from '$lib/components/ui/Button.svelte';
  
  // パラメータからドキュメントIDを取得
  $: documentId = $page.params.id;
  
  // 状態管理
  let document: any = null;
  let isLoading = true;
  let error = '';
  let showEditDialog = false;
  let showDeleteDialog = false;
  
  // 仮の文書データ
  const mockDocuments: Record<string, any> = {
    '1': {
      id: 1,
      number: 'CTA-2508001',
      title: 'システム設計書 v2.1',
      documentType: '技術文書',
      businessNumber: 'PJ2024-001',
      createdDate: '2024-08-15',
      createdBy: '山田太郎',
      department: '情報システム部',
      confidentiality: {
        internalExternal: 'internal',
        importanceClass: 'class1',
        personalInfo: 'none'
      },
      networkPath: '\\\\server\\documents\\2024\\08\\CTA-2508001',
      hasFile: true,
      hasApproval: true,
      approvalDate: '2024-08-16',
      approvalBy: '田中部長',
      notes: 'システムリニューアルに伴う設計書です。\n機密性の高い内容が含まれているため、取り扱いにご注意ください。',
      fileSize: '2.5MB',
      lastModified: '2024-08-15 14:30:00',
      version: '2.1',
      history: [
        { version: '2.1', date: '2024-08-15', user: '山田太郎', changes: '承認プロセスの追加' },
        { version: '2.0', date: '2024-08-10', user: '山田太郎', changes: 'セキュリティ要件の更新' },
        { version: '1.0', date: '2024-08-01', user: '山田太郎', changes: '初版作成' }
      ]
    }
  };
  
  // 文書データ読み込み
  async function loadDocument() {
    isLoading = true;
    error = '';
    
    try {
      // TODO: 実際のAPI呼び出しに置き換え
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      const doc = mockDocuments[documentId];
      if (!doc) {
        error = '指定された文書が見つかりません。';
        return;
      }
      
      document = doc;
    } catch (err) {
      error = '文書の読み込みに失敗しました。';
      console.error('Failed to load document:', err);
    } finally {
      isLoading = false;
    }
  }
  
  // ファイルを開く
  function openFile() {
    if (!document?.hasFile) {
      alert('ファイルが存在しません。');
      return;
    }
    
    // TODO: 実際のファイルオープン処理
    window.open(`/api/documents/${documentId}/file`, '_blank');
  }
  
  // 承認書を開く
  function openApproval() {
    if (!document?.hasApproval) {
      alert('承認書が存在しません。');
      return;
    }
    
    // TODO: 実際の承認書オープン処理
    window.open(`/api/documents/${documentId}/approval`, '_blank');
  }
  
  // ネットワークパスをコピー
  function copyNetworkPath() {
    if (document?.networkPath) {
      navigator.clipboard.writeText(document.networkPath);
      alert('ネットワークパスをクリップボードにコピーしました。');
    }
  }
  
  // 編集画面に遷移
  function editDocument() {
    window.location.href = `/documents/${documentId}/edit`;
  }
  
  // 文書削除
  async function deleteDocument() {
    try {
      // TODO: 実際のAPI呼び出しに置き換え
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      alert('文書を削除しました。');
      window.location.href = '/documents';
    } catch (error) {
      alert('文書の削除に失敗しました。');
      console.error('Failed to delete document:', error);
    }
  }
  
  // 機密レベル表示
  function getConfidentialityLabel(confidentiality: any): string {
    const parts = [];
    if (confidentiality.internalExternal === 'external') parts.push('社外');
    if (confidentiality.importanceClass === 'class1') parts.push('重要');
    if (confidentiality.personalInfo === 'present') parts.push('個人情報');
    return parts.length > 0 ? parts.join('・') : '通常';
  }
  
  function getConfidentialityColor(confidentiality: any): string {
    if (confidentiality.importanceClass === 'class1' || confidentiality.personalInfo === 'present') {
      return 'bg-red-100 text-red-800 border-red-200';
    }
    if (confidentiality.internalExternal === 'external') {
      return 'bg-yellow-100 text-yellow-800 border-yellow-200';
    }
    return 'bg-gray-100 text-gray-800 border-gray-200';
  }
  
  // 初期読み込み
  onMount(() => {
    loadDocument();
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
        <Button variant="primary" on:click={loadDocument}>
          再読み込み
        </Button>
      </div>
    </div>
  {:else if document}
    <!-- 文書詳細 -->
    <!-- ページヘッダー -->
    <div class="md:flex md:items-center md:justify-between">
      <div class="min-w-0 flex-1">
        <div class="flex items-center">
          <h1 class="text-2xl font-bold leading-7 text-gray-900 sm:truncate sm:text-3xl sm:tracking-tight">
            {document.title}
          </h1>
          <span class="ml-3 inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border {getConfidentialityColor(document.confidentiality)}">
            {getConfidentialityLabel(document.confidentiality)}
          </span>
        </div>
        <div class="mt-1 flex flex-col sm:flex-row sm:flex-wrap sm:space-x-6">
          <div class="mt-2 flex items-center text-sm text-gray-500">
            <svg class="mr-1.5 h-4 w-4 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
            文書番号: {document.number}
          </div>
          <div class="mt-2 flex items-center text-sm text-gray-500">
            <svg class="mr-1.5 h-4 w-4 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3a2 2 0 012-2h4a2 2 0 012 2v4m-6 0V7h6V3a2 2 0 00-2-2H10a2 2 0 00-2 2v4z" />
            </svg>
            {document.documentType}
          </div>
          {#if document.businessNumber}
            <div class="mt-2 flex items-center text-sm text-gray-500">
              <svg class="mr-1.5 h-4 w-4 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4" />
              </svg>
              業務番号: {document.businessNumber}
            </div>
          {/if}
        </div>
      </div>
      
      <!-- アクションボタン -->
      <div class="mt-4 flex space-x-3 md:ml-4 md:mt-0">
        <Button variant="secondary" size="sm" on:click={editDocument}>
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
      </div>
    </div>

    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- メイン情報 -->
      <div class="lg:col-span-2 space-y-6">
        <!-- 基本情報 -->
        <div class="bg-white shadow rounded-lg p-6">
          <h3 class="text-lg font-medium text-gray-900 mb-4">基本情報</h3>
          
          <dl class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <div>
              <dt class="text-sm font-medium text-gray-500">作成者</dt>
              <dd class="mt-1 text-sm text-gray-900">{document.createdBy}</dd>
            </div>
            <div>
              <dt class="text-sm font-medium text-gray-500">所属部署</dt>
              <dd class="mt-1 text-sm text-gray-900">{document.department}</dd>
            </div>
            <div>
              <dt class="text-sm font-medium text-gray-500">作成日</dt>
              <dd class="mt-1 text-sm text-gray-900">{document.createdDate}</dd>
            </div>
            <div>
              <dt class="text-sm font-medium text-gray-500">最終更新</dt>
              <dd class="mt-1 text-sm text-gray-900">{document.lastModified}</dd>
            </div>
            <div>
              <dt class="text-sm font-medium text-gray-500">バージョン</dt>
              <dd class="mt-1 text-sm text-gray-900">v{document.version}</dd>
            </div>
            <div>
              <dt class="text-sm font-medium text-gray-500">ファイルサイズ</dt>
              <dd class="mt-1 text-sm text-gray-900">{document.fileSize}</dd>
            </div>
          </dl>
          
          {#if document.notes}
            <div class="mt-6">
              <dt class="text-sm font-medium text-gray-500">備考</dt>
              <dd class="mt-2 text-sm text-gray-900 whitespace-pre-wrap">{document.notes}</dd>
            </div>
          {/if}
        </div>
        
        <!-- ファイル操作 -->
        <div class="bg-white shadow rounded-lg p-6">
          <h3 class="text-lg font-medium text-gray-900 mb-4">ファイル操作</h3>
          
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <div class="border rounded-lg p-4">
              <div class="flex items-center justify-between mb-2">
                <h4 class="text-sm font-medium text-gray-900">文書ファイル</h4>
                <div class="flex items-center">
                  <div class="h-2 w-2 rounded-full {document.hasFile ? 'bg-green-400' : 'bg-red-400'} mr-2"></div>
                  <span class="text-xs text-gray-600">{document.hasFile ? '存在' : '不存在'}</span>
                </div>
              </div>
              <p class="text-xs text-gray-500 mb-3">メインの文書ファイルです</p>
              <Button
                variant="primary"
                size="sm"
                disabled={!document.hasFile}
                on:click={openFile}
              >
                ファイルを開く
              </Button>
            </div>
            
            <div class="border rounded-lg p-4">
              <div class="flex items-center justify-between mb-2">
                <h4 class="text-sm font-medium text-gray-900">審査承認書</h4>
                <div class="flex items-center">
                  <div class="h-2 w-2 rounded-full {document.hasApproval ? 'bg-green-400' : 'bg-red-400'} mr-2"></div>
                  <span class="text-xs text-gray-600">{document.hasApproval ? '存在' : '不存在'}</span>
                </div>
              </div>
              <p class="text-xs text-gray-500 mb-3">
                {document.hasApproval ? `承認日: ${document.approvalDate}` : '承認書が見つかりません'}
              </p>
              <Button
                variant="secondary"
                size="sm"
                disabled={!document.hasApproval}
                on:click={openApproval}
              >
                承認書を開く
              </Button>
            </div>
          </div>
          
          <div class="mt-4 p-3 bg-gray-50 rounded-md">
            <div class="flex items-center justify-between">
              <div>
                <p class="text-sm font-medium text-gray-700">ネットワークパス</p>
                <p class="text-xs text-gray-500 font-mono break-all">{document.networkPath}</p>
              </div>
              <Button variant="secondary" size="sm" on:click={copyNetworkPath}>
                コピー
              </Button>
            </div>
          </div>
        </div>
        
        <!-- 更新履歴 -->
        <div class="bg-white shadow rounded-lg p-6">
          <h3 class="text-lg font-medium text-gray-900 mb-4">更新履歴</h3>
          
          <div class="flow-root">
            <ul role="list" class="-mb-8">
              {#each document.history as item, index}
                <li>
                  <div class="relative pb-8">
                    {#if index !== document.history.length - 1}
                      <span class="absolute top-4 left-4 -ml-px h-full w-0.5 bg-gray-200" aria-hidden="true"></span>
                    {/if}
                    <div class="relative flex space-x-3">
                      <div>
                        <span class="h-8 w-8 rounded-full bg-blue-500 flex items-center justify-center ring-8 ring-white">
                          <svg class="h-4 w-4 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
                          </svg>
                        </span>
                      </div>
                      <div class="min-w-0 flex-1 pt-1.5 flex justify-between space-x-4">
                        <div>
                          <p class="text-sm text-gray-500">
                            バージョン <span class="font-medium text-gray-900">{item.version}</span> by {item.user}
                          </p>
                          <p class="text-sm text-gray-900 mt-1">{item.changes}</p>
                        </div>
                        <div class="text-right text-sm whitespace-nowrap text-gray-500">
                          {item.date}
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
            <Button variant="primary" size="sm" on:click={openFile} disabled={!document.hasFile}>
              <svg class="mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" d="M2.036 12.322a1.012 1.012 0 010-.639C3.423 7.51 7.36 4.5 12 4.5c4.639 0 8.573 3.007 9.963 7.178.07.207.07.431 0 .639C20.577 16.49 16.64 19.5 12 19.5c-4.639 0-8.573-3.007-9.963-7.178z" />
                <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
              ファイルを開く
            </Button>
            
            <Button variant="secondary" size="sm" on:click={editDocument}>
              <svg class="mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" d="M16.862 4.487l1.687-1.688a1.875 1.875 0 112.652 2.652L6.832 19.82a4.5 4.5 0 01-1.897 1.13l-2.685.8.8-2.685a4.5 4.5 0 011.13-1.897L16.863 4.487zm0 0L19.5 7.125" />
              </svg>
              文書を編集
            </Button>
            
            <Button variant="secondary" size="sm" on:click={copyNetworkPath}>
              <svg class="mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" d="M15.666 3.888A2.25 2.25 0 0013.5 2.25h-3c-1.03 0-1.9.693-2.166 1.638m7.332 0c.055.194.084.4.084.612v0a.75.75 0 01-.75.75H9a.75.75 0 01-.75-.75v0c0-.212.03-.418.084-.612m7.332 0c.646.049 1.288.11 1.927.184 1.1.128 1.907 1.077 1.907 2.185V19.5a2.25 2.25 0 01-2.25 2.25H6.75A2.25 2.25 0 014.5 19.5V6.257c0-1.108.806-2.057 1.907-2.185a48.208 48.208 0 011.927-.184" />
              </svg>
              パスをコピー
            </Button>
            
            <hr class="my-4">
            
            <Button variant="danger" size="sm" on:click={() => showDeleteDialog = true}>
              <svg class="mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" d="M14.74 9l-.346 9m-4.788 0L9.26 9m9.968-3.21c.342.052.682.107 1.022.166m-1.022-.165L18.16 19.673a2.25 2.25 0 01-2.244 2.077H8.084a2.25 2.25 0 01-2.244-2.077L4.772 5.79m14.456 0a48.108 48.108 0 00-3.478-.397m-12 .562c.34-.059.68-.114 1.022-.165m0 0a48.11 48.11 0 013.478-.397m7.5 0v-.916c0-1.18-.91-2.164-2.09-2.201a51.964 51.964 0 00-3.32 0c-1.18.037-2.09 1.022-2.09 2.201v.916m7.5 0a48.667 48.667 0 00-7.5 0" />
              </svg>
              文書を削除
            </Button>
          </div>
        </div>
        
        <!-- 承認情報 -->
        {#if document.hasApproval}
          <div class="mt-6 bg-white shadow rounded-lg p-6">
            <h3 class="text-lg font-medium text-gray-900 mb-4">承認情報</h3>
            
            <div class="space-y-3">
              <div class="flex items-center text-green-600">
                <svg class="mr-2 h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                <span class="text-sm font-medium">承認済み</span>
              </div>
              
              <div class="text-sm text-gray-600">
                <p>承認者: {document.approvalBy}</p>
                <p>承認日: {document.approvalDate}</p>
              </div>
            </div>
          </div>
        {/if}
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
          <h3 class="text-lg font-medium text-gray-900 mt-4">文書を削除しますか？</h3>
          <div class="mt-2 px-7 py-3">
            <p class="text-sm text-gray-500">
              この操作は取り消すことができません。文書「{document?.title}」を完全に削除します。
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
                on:click={deleteDocument}
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