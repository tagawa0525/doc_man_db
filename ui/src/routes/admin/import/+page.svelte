<script lang="ts">
  import { onMount } from 'svelte';
  import Button from '$lib/components/ui/Button.svelte';
  
  interface ImportResult {
    import_id: string;
    file_name: string;
    total_records: number;
    successful_imports: number;
    failed_imports: number;
    errors: ImportError[];
    start_time: string;
    end_time: string;
    duration_seconds: number;
  }
  
  interface ImportError {
    row_number: number;
    field?: string;
    message: string;
    raw_data: string;
  }
  
  interface ValidationResult {
    file_name: string;
    is_valid: boolean;
    record_count: number;
    headers: string[];
    sample_records: Record<string, string>[];
    errors: any[];
  }
  
  let files: FileList;
  let isImporting = false;
  let isValidating = false;
  let importResult: ImportResult | null = null;
  let validationResult: ValidationResult | null = null;
  let importErrors: ImportError[] = [];
  let showAdvancedOptions = false;
  
  // インポートオプション
  let skipDuplicates = true;
  let validateReferences = true;
  let autoCreateReferences = false;
  
  // インポート実行履歴
  let importHistory: any[] = [];
  
  async function validateCsv() {
    if (!files || files.length === 0) return;
    
    isValidating = true;
    validationResult = null;
    
    try {
      const formData = new FormData();
      formData.append('file', files[0]);
      
      const response = await fetch('/api/admin/csv/validate', {
        method: 'POST',
        body: formData
      });
      
      if (response.ok) {
        validationResult = await response.json();
      } else {
        throw new Error('Validation failed');
      }
    } catch (error) {
      console.error('Validation error:', error);
      alert('CSVファイルの検証に失敗しました');
    } finally {
      isValidating = false;
    }
  }
  
  async function handleImport() {
    if (!files || files.length === 0) return;
    
    isImporting = true;
    importResult = null;
    importErrors = [];
    
    try {
      const formData = new FormData();
      formData.append('file', files[0]);
      formData.append('skip_duplicates', skipDuplicates.toString());
      formData.append('validate_references', validateReferences.toString());
      formData.append('auto_create_references', autoCreateReferences.toString());
      
      const response = await fetch('/api/admin/csv/import', {
        method: 'POST',
        body: formData
      });
      
      if (response.ok) {
        const result = await response.json();
        importResult = result;
        importErrors = result.errors || [];
        await loadImportHistory(); // 履歴を更新
      } else {
        const errorData = await response.json();
        throw new Error(errorData.error || 'Import failed');
      }
    } catch (error) {
      console.error('Import error:', error);
      alert(`インポートに失敗しました: ${error.message}`);
    } finally {
      isImporting = false;
    }
  }
  
  async function downloadTemplate() {
    try {
      const response = await fetch('/api/admin/csv/template');
      if (response.ok) {
        const blob = await response.blob();
        const url = window.URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = 'document_import_template.csv';
        document.body.appendChild(a);
        a.click();
        window.URL.revokeObjectURL(url);
        document.body.removeChild(a);
      }
    } catch (error) {
      console.error('Template download error:', error);
      alert('テンプレートのダウンロードに失敗しました');
    }
  }
  
  async function loadImportHistory() {
    try {
      const response = await fetch('/api/admin/csv/executions');
      if (response.ok) {
        const data = await response.json();
        importHistory = data.executions || [];
      }
    } catch (error) {
      console.error('Failed to load import history:', error);
    }
  }
  
  function formatDate(dateString: string): string {
    return new Date(dateString).toLocaleString('ja-JP');
  }
  
  function formatDuration(seconds: number): string {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins}分${secs}秒`;
  }
  
  onMount(() => {
    loadImportHistory();
  });
</script>

<div class="container mx-auto px-4 py-8">
  <div class="max-w-6xl mx-auto">
    <h1 class="text-3xl font-bold text-gray-900 mb-8">CSVデータインポート</h1>
    
    <!-- テンプレートダウンロード -->
    <div class="mb-8 bg-blue-50 border border-blue-200 rounded-lg p-6">
      <h2 class="text-lg font-medium text-blue-900 mb-4">📝 CSVテンプレート</h2>
      <p class="text-sm text-blue-700 mb-4">
        インポート用のCSVファイル形式を確認するには、テンプレートをダウンロードしてください。
      </p>
      <Button variant="secondary" on:click={downloadTemplate}>
        <svg class="mr-2 h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
        </svg>
        テンプレートダウンロード
      </Button>
    </div>
    
    <!-- ファイル選択とオプション -->
    <div class="bg-white shadow rounded-lg p-6 mb-8">
      <h2 class="text-lg font-medium text-gray-900 mb-4">ファイル選択</h2>
      
      <div class="mb-6">
        <label for="csvFile" class="block text-sm font-medium text-gray-700 mb-2">
          CSVファイルを選択
        </label>
        <input
          id="csvFile"
          type="file"
          accept=".csv"
          bind:files
          on:change={() => { validationResult = null; importResult = null; }}
          class="block w-full text-sm text-gray-500 file:mr-4 file:py-2 file:px-4 file:rounded-full file:border-0 file:text-sm file:font-semibold file:bg-blue-50 file:text-blue-700 hover:file:bg-blue-100"
        />
      </div>
      
      <!-- 詳細オプション -->
      <div class="mb-6">
        <button
          type="button"
          on:click={() => (showAdvancedOptions = !showAdvancedOptions)}
          class="text-blue-600 hover:text-blue-700 text-sm font-medium"
        >
          {showAdvancedOptions ? '詳細オプションを閉じる' : '詳細オプション'}
        </button>
        
        {#if showAdvancedOptions}
          <div class="mt-4 p-4 bg-gray-50 rounded-lg space-y-3">
            <label class="flex items-center">
              <input
                type="checkbox"
                bind:checked={skipDuplicates}
                class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
              />
              <span class="ml-2 text-sm text-gray-700">重複データをスキップ</span>
            </label>
            
            <label class="flex items-center">
              <input
                type="checkbox"
                bind:checked={validateReferences}
                class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
              />
              <span class="ml-2 text-sm text-gray-700">参照データの存在確認</span>
            </label>
            
            <label class="flex items-center">
              <input
                type="checkbox"
                bind:checked={autoCreateReferences}
                class="rounded border-gray-300 text-blue-600 focus:ring-blue-500"
              />
              <span class="ml-2 text-sm text-gray-700">不存在参照データの自動作成</span>
            </label>
          </div>
        {/if}
      </div>
      
      <div class="flex space-x-4">
        <Button
          on:click={validateCsv}
          disabled={!files || files.length === 0 || isValidating}
          loading={isValidating}
          variant="secondary"
        >
          検証のみ実行
        </Button>
        
        <Button
          on:click={handleImport}
          disabled={!files || files.length === 0 || isImporting}
          loading={isImporting}
        >
          インポート実行
        </Button>
      </div>
    </div>
    
    <!-- 検証結果 -->
    {#if validationResult}
      <div class="bg-white shadow rounded-lg p-6 mb-8">
        <h2 class="text-lg font-medium text-gray-900 mb-4">検証結果</h2>
        
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
          <div class="text-center p-4 bg-gray-50 rounded">
            <div class="text-2xl font-bold text-gray-900">{validationResult.record_count}</div>
            <div class="text-sm text-gray-600">レコード数</div>
          </div>
          <div class="text-center p-4 {validationResult.is_valid ? 'bg-green-50' : 'bg-red-50'} rounded">
            <div class="text-2xl font-bold {validationResult.is_valid ? 'text-green-600' : 'text-red-600'}">
              {validationResult.is_valid ? '✓' : '✗'}
            </div>
            <div class="text-sm text-gray-600">形式チェック</div>
          </div>
          <div class="text-center p-4 bg-blue-50 rounded">
            <div class="text-2xl font-bold text-blue-600">{validationResult.headers.length}</div>
            <div class="text-sm text-gray-600">列数</div>
          </div>
          <div class="text-center p-4 bg-purple-50 rounded">
            <div class="text-2xl font-bold text-purple-600">{validationResult.errors.length}</div>
            <div class="text-sm text-gray-600">エラー数</div>
          </div>
        </div>
        
        <!-- ヘッダー情報 -->
        <div class="mb-4">
          <h3 class="text-md font-medium text-gray-900 mb-2">CSVヘッダー</h3>
          <div class="flex flex-wrap gap-2">
            {#each validationResult.headers as header}
              <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                {header}
              </span>
            {/each}
          </div>
        </div>
        
        <!-- サンプルデータ -->
        {#if validationResult.sample_records.length > 0}
          <div class="mb-4">
            <h3 class="text-md font-medium text-gray-900 mb-2">サンプルデータ（先頭5件）</h3>
            <div class="overflow-x-auto">
              <table class="min-w-full divide-y divide-gray-200">
                <thead class="bg-gray-50">
                  <tr>
                    {#each validationResult.headers as header}
                      <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                        {header}
                      </th>
                    {/each}
                  </tr>
                </thead>
                <tbody class="bg-white divide-y divide-gray-200">
                  {#each validationResult.sample_records as record}
                    <tr>
                      {#each validationResult.headers as header}
                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                          {record[header] || '-'}
                        </td>
                      {/each}
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          </div>
        {/if}
        
        <!-- エラー一覧 -->
        {#if validationResult.errors.length > 0}
          <div>
            <h3 class="text-md font-medium text-red-900 mb-2">検証エラー</h3>
            <div class="bg-red-50 border border-red-200 rounded-lg p-4">
              {#each validationResult.errors as error}
                <div class="mb-2">
                  <span class="font-medium text-red-800">行 {error.row}:</span>
                  <span class="text-red-700">{error.error}</span>
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    {/if}
    
    <!-- インポート結果 -->
    {#if importResult}
      <div class="bg-white shadow rounded-lg p-6 mb-8">
        <h2 class="text-lg font-medium text-gray-900 mb-4">インポート結果</h2>
        
        <div class="grid grid-cols-2 md:grid-cols-4 gap-4 mb-6">
          <div class="text-center p-4 bg-gray-50 rounded">
            <div class="text-2xl font-bold text-gray-900">{importResult.total_records}</div>
            <div class="text-sm text-gray-600">総レコード数</div>
          </div>
          <div class="text-center p-4 bg-green-50 rounded">
            <div class="text-2xl font-bold text-green-600">{importResult.successful_imports}</div>
            <div class="text-sm text-gray-600">成功</div>
          </div>
          <div class="text-center p-4 bg-red-50 rounded">
            <div class="text-2xl font-bold text-red-600">{importResult.failed_imports}</div>
            <div class="text-sm text-gray-600">失敗</div>
          </div>
          <div class="text-center p-4 bg-blue-50 rounded">
            <div class="text-2xl font-bold text-blue-600">{formatDuration(importResult.duration_seconds)}</div>
            <div class="text-sm text-gray-600">実行時間</div>
          </div>
        </div>
        
        <div class="mb-4">
          <p class="text-sm text-gray-600">
            インポートID: <span class="font-mono text-gray-900">{importResult.import_id}</span>
          </p>
          <p class="text-sm text-gray-600">
            開始時刻: {formatDate(importResult.start_time)}
          </p>
          <p class="text-sm text-gray-600">
            終了時刻: {formatDate(importResult.end_time)}
          </p>
        </div>
        
        {#if importErrors.length > 0}
          <div>
            <h3 class="text-md font-medium text-red-900 mb-2">エラー詳細</h3>
            <div class="max-h-64 overflow-y-auto bg-red-50 border border-red-200 rounded-lg p-4">
              {#each importErrors as error}
                <div class="mb-3 pb-3 border-b border-red-200 last:border-b-0">
                  <div class="font-medium text-red-800">
                    行 {error.row_number}
                    {#if error.field}
                      - フィールド: {error.field}
                    {/if}
                  </div>
                  <div class="text-sm text-red-700 mt-1">{error.message}</div>
                  {#if error.raw_data}
                    <div class="text-xs text-red-600 mt-1 font-mono bg-red-100 p-1 rounded">
                      データ: {error.raw_data}
                    </div>
                  {/if}
                </div>
              {/each}
            </div>
          </div>
        {/if}
      </div>
    {/if}
    
    <!-- インポート履歴 -->
    {#if importHistory.length > 0}
      <div class="bg-white shadow rounded-lg overflow-hidden">
        <div class="px-6 py-4 border-b border-gray-200">
          <h2 class="text-lg font-medium text-gray-900">インポート履歴</h2>
        </div>
        
        <div class="overflow-x-auto">
          <table class="min-w-full divide-y divide-gray-200">
            <thead class="bg-gray-50">
              <tr>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  ファイル名
                </th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  レコード数
                </th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  成功/失敗
                </th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  実行時刻
                </th>
                <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  ステータス
                </th>
              </tr>
            </thead>
            <tbody class="bg-white divide-y divide-gray-200">
              {#each importHistory as execution}
                <tr>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                    {execution.file_name}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                    {execution.total_records}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                    <span class="text-green-600">{execution.successful_imports}</span>
                    /
                    <span class="text-red-600">{execution.failed_imports}</span>
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
                    {formatDate(execution.started_at)}
                  </td>
                  <td class="px-6 py-4 whitespace-nowrap">
                    <span class="inline-flex px-2 py-1 text-xs font-semibold rounded-full {
                      execution.status === 'completed' ? 'bg-green-100 text-green-800' :
                      execution.status === 'running' ? 'bg-yellow-100 text-yellow-800' :
                      execution.status === 'failed' ? 'bg-red-100 text-red-800' :
                      'bg-gray-100 text-gray-800'
                    }">
                      {execution.status}
                    </span>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      </div>
    {/if}
  </div>
</div>