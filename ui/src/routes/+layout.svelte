<script lang="ts">
  import '../app.css';
  import Header from '$lib/components/layout/Header.svelte';
  import Navigation from '$lib/components/layout/Navigation.svelte';
  import MobileHeader from '$lib/components/mobile/MobileHeader.svelte';
  import MobileNavigation from '$lib/components/mobile/MobileNavigation.svelte';
  import ToastContainer from '$lib/components/notifications/ToastContainer.svelte';
  
  // TODO: 実際の認証システムと連携
  let user = {
    name: 'テストユーザー',
    department: '情報システム部'
  };
  
  let showMobileNav = false;
  
  // ナビゲーション項目
  const navigationItems = [
    {
      label: 'ダッシュボード',
      href: '/',
      icon: 'dashboard'
    },
    {
      label: '文書管理',
      href: '/documents',
      icon: 'documents',
      children: [
        { label: '文書検索', href: '/documents', icon: 'search' },
        { label: '新規作成', href: '/documents/new', icon: 'plus' },
        { label: '一括処理', href: '/documents/bulk', icon: 'batch' }
      ]
    },
    {
      label: '組織管理',
      href: '/organization',
      icon: 'organization',
      children: [
        { label: '部署管理', href: '/organization/departments', icon: 'department' },
        { label: '社員管理', href: '/organization/employees', icon: 'employees' }
      ]
    },
    {
      label: 'レポート',
      href: '/reports',
      icon: 'reports',
      children: [
        { label: 'ファイル確認結果', href: '/reports/file-check', icon: 'file-check' },
        { label: 'システム稼働状況', href: '/reports/system-status', icon: 'system' },
        { label: '利用統計', href: '/reports/usage', icon: 'analytics' }
      ]
    },
    {
      label: '通知',
      href: '/notifications',
      icon: 'notifications',
      children: [
        { label: '通知履歴', href: '/notifications', icon: 'history' },
        { label: 'テンプレート管理', href: '/notifications/templates', icon: 'template' }
      ]
    },
    {
      label: '設定',
      href: '/settings',
      icon: 'settings'
    }
  ];
  
  function handleToggleNav() {
    showMobileNav = !showMobileNav;
  }
</script>

<div class="min-h-screen bg-gray-50">
  <!-- モバイルナビゲーション -->
  <MobileNavigation {navigationItems} bind:showMobileNav />
  
  <!-- デスクトップレイアウト -->
  <div class="hidden lg:fixed lg:inset-y-0 lg:z-50 lg:flex lg:w-72 lg:flex-col">
    <Navigation />
  </div>
  
  <div class="lg:pl-72">
    <!-- モバイル/タブレットヘッダー -->
    <div class="lg:hidden">
      <MobileHeader {user} on:toggle-nav={handleToggleNav} />
    </div>
    
    <!-- デスクトップヘッダー -->
    <div class="hidden lg:block">
      <Header {user} />
    </div>
    
    <!-- メインコンテンツ -->
    <main class="p-4 sm:p-6 lg:p-8">
      <slot />
    </main>
  </div>
  
  <!-- Toast Notifications -->
  <ToastContainer />
</div>