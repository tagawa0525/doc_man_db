<script lang="ts">
  import { page } from '$app/stores';
  
  interface NavItem {
    label: string;
    href: string;
    icon: string;
    children?: NavItem[];
  }
  
  export let navigationItems: NavItem[];
  export let showMobileNav: boolean = false;
  
  let expandedItems: Set<string> = new Set();
  
  function toggleExpanded(href: string) {
    if (expandedItems.has(href)) {
      expandedItems.delete(href);
    } else {
      expandedItems.add(href);
    }
    expandedItems = expandedItems;
  }
  
  function getIcon(iconName: string): string {
    const icons: Record<string, string> = {
      dashboard: 'M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586l-2 2V5H5v14h7v2H4a1 1 0 01-1-1V4z',
      documents: 'M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z',
      search: 'm21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z',
      plus: 'M12 4.5v15m7.5-7.5h-15',
      organization: 'M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z',
      reports: 'M3 13.125C3 12.504 3.504 12 4.125 12h2.25c.621 0 1.125.504 1.125 1.125v6.75C7.5 20.496 6.996 21 6.375 21h-2.25A1.125 1.125 0 013 19.875v-6.75zM9.75 8.625c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125v11.25c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V8.625zM16.5 4.125c0-.621.504-1.125 1.125-1.125h2.25C20.496 3 21 3.504 21 4.125v15.75c0 .621-.504 1.125-1.125 1.125h-2.25a1.125 1.125 0 01-1.125-1.125V4.125z',
      notifications: 'M14.857 17.082a23.848 23.848 0 005.454-1.31A8.967 8.967 0 0118 9.75v-.7V9A6 6 0 006 9v.75a8.967 8.967 0 01-2.312 6.022c1.733.64 3.56 1.085 5.455 1.31m5.714 0a24.255 24.255 0 01-5.714 0m5.714 0a3 3 0 11-5.714 0',
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

<!-- モバイルナビゲーション -->
{#if showMobileNav}
  <div class="relative z-50 lg:hidden">
    <!-- オーバーレイ -->
    <div class="fixed inset-0 bg-gray-600 bg-opacity-75 transition-opacity" aria-hidden="true"></div>
    
    <div class="fixed inset-0 flex">
      <div class="relative mr-16 flex w-full max-w-xs flex-1">
        <div class="absolute left-full top-0 flex w-16 justify-center pt-5">
          <button type="button" class="-m-2.5 p-2.5" on:click={() => showMobileNav = false}>
            <span class="sr-only">Close sidebar</span>
            <svg class="h-6 w-6 text-white" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
        
        <!-- サイドバーコンテンツ -->
        <div class="flex grow flex-col gap-y-5 overflow-y-auto bg-gray-900 px-6 pb-4">
          <div class="flex h-16 shrink-0 items-center">
            <svg class="h-8 w-8 text-white" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z" />
            </svg>
            <span class="ml-3 text-white font-semibold">文書管理</span>
          </div>
          
          <nav class="flex flex-1 flex-col">
            <ul role="list" class="flex flex-1 flex-col gap-y-7">
              <li>
                <ul role="list" class="-mx-2 space-y-1">
                  {#each navigationItems as item}
                    <li>
                      {#if item.children}
                        <div>
                          <button
                            type="button"
                            class="w-full flex items-center gap-x-3 rounded-md p-2 text-left text-sm leading-6 font-semibold text-gray-300 hover:text-white hover:bg-gray-800 
                                   {isActive(item.href) || hasActiveChild(item) || expandedItems.has(item.href) ? 'bg-gray-800 text-white' : ''}"
                            on:click={() => toggleExpanded(item.href)}
                          >
                            <svg class="h-6 w-6 shrink-0" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                              <path stroke-linecap="round" stroke-linejoin="round" d={getIcon(item.icon)} />
                            </svg>
                            {item.label}
                            <svg
                              class="ml-auto h-5 w-5 shrink-0 transition-transform duration-200 {expandedItems.has(item.href) ? 'rotate-90' : ''}"
                              fill="none"
                              viewBox="0 0 24 24"
                              stroke-width="1.5"
                              stroke="currentColor"
                            >
                              <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 4.5l7.5 7.5-7.5 7.5" />
                            </svg>
                          </button>
                          
                          {#if expandedItems.has(item.href)}
                            <ul class="mt-1 px-2">
                              {#each item.children as child}
                                <li>
                                  <a
                                    href={child.href}
                                    class="flex items-center gap-x-3 rounded-md py-2 pl-9 pr-2 text-sm leading-6 text-gray-300 hover:text-white hover:bg-gray-800
                                           {isActive(child.href) ? 'bg-blue-600 text-white' : ''}"
                                    on:click={() => showMobileNav = false}
                                  >
                                    {child.label}
                                  </a>
                                </li>
                              {/each}
                            </ul>
                          {/if}
                        </div>
                      {:else}
                        <a
                          href={item.href}
                          class="flex items-center gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold text-gray-300 hover:text-white hover:bg-gray-800
                                 {isActive(item.href) ? 'bg-blue-600 text-white' : ''}"
                          on:click={() => showMobileNav = false}
                        >
                          <svg class="h-6 w-6 shrink-0" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" d={getIcon(item.icon)} />
                          </svg>
                          {item.label}
                        </a>
                      {/if}
                    </li>
                  {/each}
                </ul>
              </li>
            </ul>
          </nav>
        </div>
      </div>
    </div>
  </div>
{/if}