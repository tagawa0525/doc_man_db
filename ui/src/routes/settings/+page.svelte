<script lang="ts">
  import { onMount } from "svelte";
  import PlaceholderBanner from "$lib/components/ui/PlaceholderBanner.svelte";

  let activeTab = "general";
  let settings = {
    general: {
      systemName: "文書管理システム",
      defaultLanguage: "ja",
      timezone: "Asia/Tokyo",
      dateFormat: "YYYY-MM-DD",
      sessionTimeout: 60,
    },
    document: {
      maxFileSize: 50,
      allowedExtensions: ["pdf", "docx", "xlsx", "pptx", "txt"],
      autoNumbering: true,
      numberingPrefix: "CTA",
      approvalRequired: true,
      versionControl: true,
    },
    storage: {
      basePath: "\\\\server\\documents",
      backupEnabled: true,
      backupInterval: "daily",
      retentionPeriod: 365,
      storageQuota: 1000,
    },
    notification: {
      emailEnabled: true,
      smtpServer: "smtp.company.com",
      smtpPort: 587,
      smtpUsername: "system@company.com",
      teamsEnabled: false,
      teamsWebhook: "",
      inAppEnabled: true,
    },
    security: {
      adIntegration: true,
      adServer: "dc.company.com",
      passwordExpiry: 90,
      sessionSecurity: "high",
      auditLogging: true,
      ipRestriction: false,
    },
  };

  let originalSettings = JSON.parse(JSON.stringify(settings));
  let hasChanges = false;
  let isLoading = true;
  let isSaving = false;

  const tabs = [
    { id: "general", name: "基本設定", icon: "⚙️" },
    { id: "document", name: "文書設定", icon: "📄" },
    { id: "storage", name: "ストレージ設定", icon: "💾" },
    { id: "notification", name: "通知設定", icon: "📧" },
    { id: "security", name: "セキュリティ設定", icon: "🔐" },
  ];

  onMount(() => {
    setTimeout(() => {
      isLoading = false;
    }, 500);
  });

  $: {
    hasChanges = JSON.stringify(settings) !== JSON.stringify(originalSettings);
  }

  const handleSave = async () => {
    isSaving = true;
    try {
      // TODO: 実際のAPI呼び出し
      await new Promise((resolve) => setTimeout(resolve, 1000));
      originalSettings = JSON.parse(JSON.stringify(settings));
      console.log("設定を保存しました:", settings);
    } catch (error) {
      console.error("設定保存エラー:", error);
    } finally {
      isSaving = false;
    }
  };

  const handleReset = () => {
    settings = JSON.parse(JSON.stringify(originalSettings));
  };

  const handleTestConnection = async (type: string) => {
    console.log(`${type}接続テストを実行中...`);
    // TODO: 実際の接続テスト
  };
</script>

<svelte:head>
  <title>システム設定 - 文書管理システム</title>
</svelte:head>

<div class="max-w-6xl mx-auto p-6">
  <!-- Placeholder Banner -->
  <PlaceholderBanner featureKey="settings" class="mb-6" />

  <div class="mb-6">
    <h1 class="text-3xl font-bold text-gray-900">システム設定</h1>
    <p class="text-gray-600 mt-2">システムの動作を設定・管理</p>
  </div>

  {#if isLoading}
    <div class="bg-white rounded-lg shadow p-8 text-center">
      <div
        class="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500 mx-auto"
      ></div>
      <p class="mt-4 text-gray-600">設定を読み込み中...</p>
    </div>
  {:else}
    <div class="flex">
      <!-- サイドバー -->
      <div class="w-64 flex-shrink-0 mr-6">
        <nav class="space-y-1">
          {#each tabs as tab}
            <button
              on:click={() => (activeTab = tab.id)}
              class="w-full flex items-center px-3 py-2 text-left rounded-md text-sm font-medium transition-colors {activeTab ===
              tab.id
                ? 'bg-blue-100 text-blue-700'
                : 'text-gray-600 hover:bg-gray-100'}"
            >
              <span class="mr-3">{tab.icon}</span>
              {tab.name}
            </button>
          {/each}
        </nav>

        {#if hasChanges}
          <div
            class="mt-6 p-4 bg-yellow-50 border border-yellow-200 rounded-lg"
          >
            <div class="flex">
              <svg
                class="w-5 h-5 text-yellow-400 mt-0.5 mr-2"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"
                ></path>
              </svg>
              <div>
                <p class="text-sm font-medium text-yellow-800">未保存の変更</p>
                <p class="text-sm text-yellow-700 mt-1">
                  設定に変更があります。保存を忘れずに！
                </p>
              </div>
            </div>
          </div>
        {/if}
      </div>

      <!-- メインコンテンツ -->
      <div class="flex-1">
        <div class="bg-white rounded-lg shadow">
          <!-- 基本設定 -->
          {#if activeTab === "general"}
            <div class="p-6 border-b border-gray-200">
              <h2 class="text-lg font-semibold text-gray-900">基本設定</h2>
              <p class="text-gray-600 mt-1">システム全体の基本的な設定</p>
            </div>
            <div class="p-6 space-y-6">
              <div>
                <label
                  for="systemName"
                  class="block text-sm font-medium text-gray-700 mb-2"
                  >システム名</label
                >
                <input
                  id="systemName"
                  type="text"
                  bind:value={settings.general.systemName}
                  class="input w-full"
                />
              </div>
              <div class="grid grid-cols-2 gap-4">
                <div>
                  <label
                    for="defaultLanguage"
                    class="block text-sm font-medium text-gray-700 mb-2"
                    >言語設定</label
                  >
                  <select
                    id="defaultLanguage"
                    bind:value={settings.general.defaultLanguage}
                    class="input w-full"
                  >
                    <option value="ja">日本語</option>
                    <option value="en">English</option>
                  </select>
                </div>
                <div>
                  <label
                    for="timezone"
                    class="block text-sm font-medium text-gray-700 mb-2"
                    >タイムゾーン</label
                  >
                  <select
                    id="timezone"
                    bind:value={settings.general.timezone}
                    class="input w-full"
                  >
                    <option value="Asia/Tokyo">Asia/Tokyo (JST)</option>
                    <option value="UTC">UTC</option>
                  </select>
                </div>
              </div>
              <div class="grid grid-cols-2 gap-4">
                <div>
                  <label
                    for="dateFormat"
                    class="block text-sm font-medium text-gray-700 mb-2"
                    >日付フォーマット</label
                  >
                  <select
                    id="dateFormat"
                    bind:value={settings.general.dateFormat}
                    class="input w-full"
                  >
                    <option value="YYYY-MM-DD">YYYY-MM-DD</option>
                    <option value="MM/DD/YYYY">MM/DD/YYYY</option>
                    <option value="DD/MM/YYYY">DD/MM/YYYY</option>
                  </select>
                </div>
                <div>
                  <label
                    for="sessionTimeout"
                    class="block text-sm font-medium text-gray-700 mb-2"
                    >セッションタイムアウト (分)</label
                  >
                  <input
                    id="sessionTimeout"
                    type="number"
                    bind:value={settings.general.sessionTimeout}
                    min="5"
                    max="480"
                    class="input w-full"
                  />
                </div>
              </div>
            </div>
          {/if}

          <!-- 文書設定 -->
          {#if activeTab === "document"}
            <div class="p-6 border-b border-gray-200">
              <h2 class="text-lg font-semibold text-gray-900">文書設定</h2>
              <p class="text-gray-600 mt-1">文書管理に関する設定</p>
            </div>
            <div class="p-6 space-y-6">
              <div class="grid grid-cols-2 gap-4">
                <div>
                  <label
                    for="maxFileSize"
                    class="block text-sm font-medium text-gray-700 mb-2"
                    >最大ファイルサイズ (MB)</label
                  >
                  <input
                    id="maxFileSize"
                    type="number"
                    bind:value={settings.document.maxFileSize}
                    min="1"
                    max="1000"
                    class="input w-full"
                  />
                </div>
                <div>
                  <label
                    for="numberingPrefix"
                    class="block text-sm font-medium text-gray-700 mb-2"
                    >文書番号プレフィックス</label
                  >
                  <input
                    id="numberingPrefix"
                    type="text"
                    bind:value={settings.document.numberingPrefix}
                    class="input w-full"
                  />
                </div>
              </div>
              <div>
                <label
                  for="allowed-extensions"
                  class="block text-sm font-medium text-gray-700 mb-2"
                  >許可ファイル形式</label
                >
                <div id="allowed-extensions" class="flex flex-wrap gap-2 mt-2">
                  {#each settings.document.allowedExtensions as ext}
                    <span
                      class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800"
                    >
                      .{ext}
                    </span>
                  {/each}
                </div>
              </div>
              <div class="space-y-4">
                <div class="flex items-center">
                  <input
                    id="autoNumbering"
                    type="checkbox"
                    bind:checked={settings.document.autoNumbering}
                    class="mr-3"
                  />
                  <label
                    for="autoNumbering"
                    class="text-sm font-medium text-gray-700"
                    >自動文書番号生成</label
                  >
                </div>
                <div class="flex items-center">
                  <input
                    id="approvalRequired"
                    type="checkbox"
                    bind:checked={settings.document.approvalRequired}
                    class="mr-3"
                  />
                  <label
                    for="approvalRequired"
                    class="text-sm font-medium text-gray-700"
                    >承認プロセス必須</label
                  >
                </div>
                <div class="flex items-center">
                  <input
                    id="versionControl"
                    type="checkbox"
                    bind:checked={settings.document.versionControl}
                    class="mr-3"
                  />
                  <label
                    for="versionControl"
                    class="text-sm font-medium text-gray-700"
                    >バージョン管理有効</label
                  >
                </div>
              </div>
            </div>
          {/if}

          <!-- ストレージ設定 -->
          {#if activeTab === "storage"}
            <div class="p-6 border-b border-gray-200">
              <h2 class="text-lg font-semibold text-gray-900">
                ストレージ設定
              </h2>
              <p class="text-gray-600 mt-1">ファイル保存とバックアップの設定</p>
            </div>
            <div class="p-6 space-y-6">
              <div>
                <label
                  for="basePath"
                  class="block text-sm font-medium text-gray-700 mb-2"
                  >ベースパス</label
                >
                <input
                  id="basePath"
                  type="text"
                  bind:value={settings.storage.basePath}
                  class="input w-full"
                />
              </div>
              <div class="grid grid-cols-2 gap-4">
                <div>
                  <label
                    for="backupInterval"
                    class="block text-sm font-medium text-gray-700 mb-2"
                    >バックアップ間隔</label
                  >
                  <select
                    id="backupInterval"
                    bind:value={settings.storage.backupInterval}
                    class="input w-full"
                  >
                    <option value="hourly">毎時</option>
                    <option value="daily">毎日</option>
                    <option value="weekly">毎週</option>
                  </select>
                </div>
                <div>
                  <label
                    for="retentionPeriod"
                    class="block text-sm font-medium text-gray-700 mb-2"
                    >保持期間 (日)</label
                  >
                  <input
                    id="retentionPeriod"
                    type="number"
                    bind:value={settings.storage.retentionPeriod}
                    min="1"
                    max="3650"
                    class="input w-full"
                  />
                </div>
              </div>
              <div>
                <label
                  for="storageQuota"
                  class="block text-sm font-medium text-gray-700 mb-2"
                  >ストレージクォータ (GB)</label
                >
                <input
                  id="storageQuota"
                  type="number"
                  bind:value={settings.storage.storageQuota}
                  min="10"
                  max="10000"
                  class="input w-full"
                />
              </div>
              <div class="flex items-center">
                <input
                  id="backupEnabled"
                  type="checkbox"
                  bind:checked={settings.storage.backupEnabled}
                  class="mr-3"
                />
                <label
                  for="backupEnabled"
                  class="text-sm font-medium text-gray-700"
                  >自動バックアップ有効</label
                >
              </div>
            </div>
          {/if}

          <!-- 通知設定 -->
          {#if activeTab === "notification"}
            <div class="p-6 border-b border-gray-200">
              <h2 class="text-lg font-semibold text-gray-900">通知設定</h2>
              <p class="text-gray-600 mt-1">
                メール・Teams・アプリ内通知の設定
              </p>
            </div>
            <div class="p-6 space-y-6">
              <div class="border border-gray-200 rounded-lg p-4">
                <div class="flex items-center justify-between mb-4">
                  <div>
                    <h3 class="text-sm font-medium text-gray-900">
                      メール通知
                    </h3>
                    <p class="text-sm text-gray-500">SMTP経由でのメール送信</p>
                  </div>
                  <input
                    id="emailEnabled"
                    type="checkbox"
                    bind:checked={settings.notification.emailEnabled}
                    class="ml-3"
                  />
                </div>
                {#if settings.notification.emailEnabled}
                  <div class="grid grid-cols-2 gap-4">
                    <div>
                      <label
                        for="smtpServer"
                        class="block text-sm font-medium text-gray-700 mb-1"
                        >SMTPサーバー</label
                      >
                      <input
                        id="smtpServer"
                        type="text"
                        bind:value={settings.notification.smtpServer}
                        class="input w-full"
                      />
                    </div>
                    <div>
                      <label
                        for="smtpPort"
                        class="block text-sm font-medium text-gray-700 mb-1"
                        >ポート</label
                      >
                      <input
                        id="smtpPort"
                        type="number"
                        bind:value={settings.notification.smtpPort}
                        class="input w-full"
                      />
                    </div>
                  </div>
                  <div class="mt-2">
                    <label
                      for="smtpUsername"
                      class="block text-sm font-medium text-gray-700 mb-1"
                      >ユーザー名</label
                    >
                    <input
                      id="smtpUsername"
                      type="text"
                      bind:value={settings.notification.smtpUsername}
                      class="input w-full"
                    />
                  </div>
                  <div class="mt-2">
                    <button
                      on:click={() => handleTestConnection("SMTP")}
                      class="btn btn-secondary"
                    >
                      接続テスト
                    </button>
                  </div>
                {/if}
              </div>

              <div class="border border-gray-200 rounded-lg p-4">
                <div class="flex items-center justify-between mb-4">
                  <div>
                    <h3 class="text-sm font-medium text-gray-900">Teams通知</h3>
                    <p class="text-sm text-gray-500">Microsoft Teams連携</p>
                  </div>
                  <input
                    id="teamsEnabled"
                    type="checkbox"
                    bind:checked={settings.notification.teamsEnabled}
                    class="ml-3"
                  />
                </div>
                {#if settings.notification.teamsEnabled}
                  <div>
                    <label
                      for="teamsWebhook"
                      class="block text-sm font-medium text-gray-700 mb-1"
                      >Webhook URL</label
                    >
                    <input
                      id="teamsWebhook"
                      type="url"
                      bind:value={settings.notification.teamsWebhook}
                      class="input w-full"
                      placeholder="https://outlook.office.com/webhook/..."
                    />
                  </div>
                  <div class="mt-2">
                    <button
                      on:click={() => handleTestConnection("Teams")}
                      class="btn btn-secondary"
                    >
                      接続テスト
                    </button>
                  </div>
                {/if}
              </div>

              <div class="flex items-center">
                <input
                  id="inAppEnabled"
                  type="checkbox"
                  bind:checked={settings.notification.inAppEnabled}
                  class="mr-3"
                />
                <label
                  for="inAppEnabled"
                  class="text-sm font-medium text-gray-700"
                  >アプリ内通知有効</label
                >
              </div>
            </div>
          {/if}

          <!-- セキュリティ設定 -->
          {#if activeTab === "security"}
            <div class="p-6 border-b border-gray-200">
              <h2 class="text-lg font-semibold text-gray-900">
                セキュリティ設定
              </h2>
              <p class="text-gray-600 mt-1">認証・セキュリティポリシーの設定</p>
            </div>
            <div class="p-6 space-y-6">
              <div class="border border-gray-200 rounded-lg p-4">
                <div class="flex items-center justify-between mb-4">
                  <div>
                    <h3 class="text-sm font-medium text-gray-900">
                      Active Directory連携
                    </h3>
                    <p class="text-sm text-gray-500">Windows AD認証</p>
                  </div>
                  <input
                    id="adIntegration"
                    type="checkbox"
                    bind:checked={settings.security.adIntegration}
                    class="ml-3"
                  />
                </div>
                {#if settings.security.adIntegration}
                  <div>
                    <label
                      for="adServer"
                      class="block text-sm font-medium text-gray-700 mb-1"
                      >ADサーバー</label
                    >
                    <input
                      id="adServer"
                      type="text"
                      bind:value={settings.security.adServer}
                      class="input w-full"
                    />
                  </div>
                  <div class="mt-2">
                    <button
                      on:click={() => handleTestConnection("AD")}
                      class="btn btn-secondary"
                    >
                      接続テスト
                    </button>
                  </div>
                {/if}
              </div>

              <div class="grid grid-cols-2 gap-4">
                <div>
                  <label
                    for="passwordExpiry"
                    class="block text-sm font-medium text-gray-700 mb-2"
                    >パスワード有効期限 (日)</label
                  >
                  <input
                    id="passwordExpiry"
                    type="number"
                    bind:value={settings.security.passwordExpiry}
                    min="1"
                    max="365"
                    class="input w-full"
                  />
                </div>
                <div>
                  <label
                    for="sessionSecurity"
                    class="block text-sm font-medium text-gray-700 mb-2"
                    >セッションセキュリティ</label
                  >
                  <select
                    id="sessionSecurity"
                    bind:value={settings.security.sessionSecurity}
                    class="input w-full"
                  >
                    <option value="low">低</option>
                    <option value="medium">中</option>
                    <option value="high">高</option>
                  </select>
                </div>
              </div>

              <div class="space-y-4">
                <div class="flex items-center">
                  <input
                    id="auditLogging"
                    type="checkbox"
                    bind:checked={settings.security.auditLogging}
                    class="mr-3"
                  />
                  <label
                    for="auditLogging"
                    class="text-sm font-medium text-gray-700"
                    >監査ログ有効</label
                  >
                </div>
                <div class="flex items-center">
                  <input
                    id="ipRestriction"
                    type="checkbox"
                    bind:checked={settings.security.ipRestriction}
                    class="mr-3"
                  />
                  <label
                    for="ipRestriction"
                    class="text-sm font-medium text-gray-700">IP制限有効</label
                  >
                </div>
              </div>
            </div>
          {/if}

          <!-- フッター -->
          <div class="p-6 border-t border-gray-200 flex justify-end space-x-3">
            <button
              on:click={handleReset}
              disabled={!hasChanges}
              class="btn btn-secondary disabled:opacity-50"
            >
              リセット
            </button>
            <button
              on:click={handleSave}
              disabled={!hasChanges || isSaving}
              class="btn btn-primary disabled:opacity-50"
            >
              {#if isSaving}
                <div
                  class="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"
                ></div>
                保存中...
              {:else}
                保存
              {/if}
            </button>
          </div>
        </div>
      </div>
    </div>
  {/if}
</div>
