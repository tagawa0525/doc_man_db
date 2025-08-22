<script lang="ts">
  import { page } from '$app/stores';
  
  interface NavItem {
    label: string;
    href: string;
    icon: string;
    children?: NavItem[];
  }
  
  const navigationItems: NavItem[] = [
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
        { label: '高度検索', href: '/search', icon: 'advanced-search' },
        { label: '新規作成', href: '/documents/new', icon: 'plus' },
        { label: '一括処理', href: '/documents/bulk', icon: 'batch' }
      ]
    },
    {
      label: '文書回覧',
      href: '/circulations',
      icon: 'circulation',
      children: [
        { label: '承認待ち', href: '/circulations', icon: 'pending' },
        { label: '回覧履歴', href: '/circulations?tab=completed', icon: 'history' },
        { label: 'ワークフロー', href: '/circulations/workflows', icon: 'workflow' }
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
  
  let expandedItems: Set<string> = new Set();
  
  function toggleExpanded(href: string) {
    if (expandedItems.has(href)) {
      expandedItems.delete(href);
    } else {
      expandedItems.add(href);
    }
    expandedItems = expandedItems; // Trigger reactivity
  }
  
  function getIcon(iconName: string): string {
    const icons: Record<string, string> = {
      dashboard: 'M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586l-2 2V5H5v14h7v2H4a1 1 0 01-1-1V4z',
      documents: 'M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z',
      search: 'm21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z',
      'advanced-search': 'M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.324.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 011.37.49l1.296 2.247a1.125 1.125 0 01-.26 1.431l-1.003.827c-.293.24-.438.613-.431.992a6.759 6.759 0 010 .255c-.007.378.138.75.43.99l1.005.828c.424.35.534.954.26 1.43l-1.298 2.247a1.125 1.125 0 01-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.57 6.57 0 01-.22.128c-.331.183-.581.495-.644.869l-.213 1.28c-.09.543-.56.941-1.11.941h-2.594c-.55 0-1.02-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 01-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 01-1.369-.49l-1.297-2.247a1.125 1.125 0 01.26-1.431l1.004-.827c.292-.24.437-.613.43-.992a6.932 6.932 0 010-.255c.007-.378-.138-.75-.43-.99l-1.004-.828a1.125 1.125 0 01-.26-1.43l1.297-2.247a1.125 1.125 0 011.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.087.22-.128.332-.183.582-.495.644-.869l.214-1.281z M15 12a3 3 0 11-6 0 3 3 0 016 0z',
      plus: 'M12 4.5v15m7.5-7.5h-15',
      circulation: 'M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99',
      pending: 'M12 6v6h4.5m4.5 0a9 9 0 11-18 0 9 9 0 0118 0z',
      workflow: 'M3.75 6A2.25 2.25 0 016 3.75h2.25A2.25 2.25 0 0110.5 6v2.25a2.25 2.25 0 01-2.25 2.25H6a2.25 2.25 0 01-2.25-2.25V6zM3.75 15.75A2.25 2.25 0 016 13.5h2.25a2.25 2.25 0 012.25 2.25V18a2.25 2.25 0 01-2.25 2.25H6A2.25 2.25 0 013.75 18v-2.25zM13.5 6a2.25 2.25 0 012.25-2.25H18A2.25 2.25 0 0120.25 6v2.25A2.25 2.25 0 0118 10.5h-2.25a2.25 2.25 0 01-2.25-2.25V6zM13.5 15.75a2.25 2.25 0 012.25-2.25H18a2.25 2.25 0 012.25 2.25V18A2.25 2.25 0 0118 20.25h-2.25A2.25 2.25 0 0113.5 18v-2.25z',
      batch: 'M12 4.5v15m0 0l6.75-6.75M12 19.5l-6.75-6.75',
      organization: 'M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z',
      department: 'M19 7.5v3m0 0v3m0-3h3m-3 0h-3m-2.25-4.125a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zM4 19.235v-.11a6.375 6.375 0 0112.75 0v.109A12.318 12.318 0 0110.374 21c-2.331 0-4.512-.645-6.374-1.766z',
      employees: 'M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z',
      reports: 'M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z',
      'file-check': 'M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z',
      system: 'M5.25 14.25h13.5m-13.5 0a3 3 0 01-3-3m3 3a3 3 0 100 6h13.5a3 3 0 100-6m-16.5-3a3 3 0 013-3h13.5a3 3 0 013 3m-19.5 0a4.5 4.5 0 01.9-2.7L5.737 5.1a3.375 3.375 0 012.7-1.35h7.126c1.062 0 2.062.5 2.7 1.35l2.587 3.45a4.5 4.5 0 01.9 2.7m0 0a3 3 0 01-3 3m0 0h.008v.008h-.008V12zm0 0h.008v.008h-.008V12zm2.25 2.25V12a2.25 2.25 0 00-2.25-2.25H12A2.25 2.25 0 009.75 12v2.25',
      analytics: 'M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z',
      notifications: 'M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0',
      history: 'M12 6v6h4.5m4.5 0a9 9 0 11-18 0 9 9 0 0118 0z',
      template: 'M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m2.25 0H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z',
      settings: 'M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.324.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 011.37.49l1.296 2.247a1.125 1.125 0 01-.26 1.431l-1.003.827c-.293.24-.438.613-.431.992a6.759 6.759 0 010 .255c-.007.378.138.75.43.99l1.005.828c.424.35.534.954.26 1.43l-1.298 2.247a1.125 1.125 0 01-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.57 6.57 0 01-.22.128c-.331.183-.581.495-.644.869l-.213 1.28c-.09.543-.56.941-1.11.941h-2.594c-.55 0-1.02-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 01-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 01-1.369-.49l-1.297-2.247a1.125 1.125 0 01.26-1.431l1.004-.827c.292-.24.437-.613.43-.992a6.932 6.932 0 010-.255c.007-.378-.138-.75-.43-.99l-1.004-.828a1.125 1.125 0 01-.26-1.43l1.297-2.247a1.125 1.125 0 011.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.087.22-.128.332-.183.582-.495.644-.869l.214-1.281z M15 12a3 3 0 11-6 0 3 3 0 016 0z'
    };
    return icons[iconName] || icons.dashboard;
  }
  
  $: currentPath = $page.url.pathname;
  
  function isActive(href: string): boolean {
    return currentPath === href || (href !== '/' && currentPath.startsWith(href));
  }
  
  function hasActiveChild(item: NavItem): boolean {
    return item.children?.some(child => isActive(child.href)) || false;
  }
</script>

<nav class="bg-gray-900 w-64 min-h-screen flex-shrink-0">
  <div class="p-4">
    <ul class="space-y-1">
      {#each navigationItems as item}
        <li>
          <div class="relative">
            {#if item.children}
              <button
                type="button"
                class="w-full flex items-center px-3 py-2 text-sm font-medium rounded-md transition-colors duration-200
                       {isActive(item.href) || hasActiveChild(item) || expandedItems.has(item.href)
                         ? 'bg-gray-800 text-white' 
                         : 'text-gray-300 hover:bg-gray-700 hover:text-white'}"
                on:click={() => toggleExpanded(item.href)}
              >
                <svg class="mr-3 h-5 w-5 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" d={getIcon(item.icon)} />
                </svg>
                <span class="flex-1 text-left">{item.label}</span>
                <svg 
                  class="ml-3 h-4 w-4 transition-transform duration-200 {expandedItems.has(item.href) ? 'rotate-90' : ''}"
                  fill="none" 
                  viewBox="0 0 24 24" 
                  stroke="currentColor"
                >
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
                </svg>
              </button>
              
              {#if expandedItems.has(item.href)}
                <ul class="mt-1 ml-8 space-y-1">
                  {#each item.children as child}
                    <li>
                      <a
                        href={child.href}
                        class="flex items-center px-3 py-2 text-sm font-medium rounded-md transition-colors duration-200
                               {isActive(child.href) 
                                 ? 'bg-blue-600 text-white' 
                                 : 'text-gray-300 hover:bg-gray-700 hover:text-white'}"
                      >
                        <svg class="mr-3 h-4 w-4 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                          <path stroke-linecap="round" stroke-linejoin="round" d={getIcon(child.icon)} />
                        </svg>
                        {child.label}
                      </a>
                    </li>
                  {/each}
                </ul>
              {/if}
            {:else}
              <a
                href={item.href}
                class="flex items-center px-3 py-2 text-sm font-medium rounded-md transition-colors duration-200
                       {isActive(item.href) 
                         ? 'bg-blue-600 text-white' 
                         : 'text-gray-300 hover:bg-gray-700 hover:text-white'}"
              >
                <svg class="mr-3 h-5 w-5 flex-shrink-0" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                  <path stroke-linecap="round" stroke-linejoin="round" d={getIcon(item.icon)} />
                </svg>
                {item.label}
              </a>
            {/if}
          </div>
        </li>
      {/each}
    </ul>
  </div>
</nav>