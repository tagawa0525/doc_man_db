<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import NotificationCenter from '../notifications/NotificationCenter.svelte';
  
  export let user: { name: string; department?: string } | null = null;
  
  const dispatch = createEventDispatcher();
  
  let showUserMenu = false;
  
  function toggleUserMenu() {
    showUserMenu = !showUserMenu;
  }
  
  function toggleMobileNav() {
    dispatch('toggle-nav');
  }
  
  function handleLogout() {
    // TODO: ログアウト処理の実装
    console.log('Logout requested');
  }
</script>

<header class="sticky top-0 z-40 flex h-16 shrink-0 items-center gap-x-4 border-b border-gray-200 bg-white px-4 shadow-sm sm:gap-x-6 sm:px-6 lg:px-8">
  <!-- モバイルメニューボタン -->
  <button
    type="button"
    class="-m-2.5 p-2.5 text-gray-700 lg:hidden"
    on:click={toggleMobileNav}
  >
    <span class="sr-only">Open sidebar</span>
    <svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
      <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5" />
    </svg>
  </button>
  
  <!-- セパレーター -->
  <div class="h-6 w-px bg-gray-200 lg:hidden" aria-hidden="true"></div>
  
  <!-- ロゴ（モバイル時のみ表示） -->
  <div class="flex flex-1 items-center gap-x-4 self-stretch lg:gap-x-6">
    <div class="flex lg:hidden items-center">
      <svg class="h-6 w-6 text-blue-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
        <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z" />
      </svg>
      <h1 class="ml-2 text-lg font-semibold text-gray-900">文書管理</h1>
    </div>
    
    <!-- 検索バー（デスクトップ時） -->
    <form class="relative hidden lg:flex flex-1" action="#" method="GET">
      <label for="search-field" class="sr-only">Search</label>
      <svg class="pointer-events-none absolute inset-y-0 left-0 h-full w-5 text-gray-400" viewBox="0 0 20 20" fill="currentColor">
        <path fill-rule="evenodd" d="M9 3.5a5.5 5.5 0 100 11 5.5 5.5 0 000-11zM2 9a7 7 0 1112.452 4.391l3.328 3.329a.75.75 0 11-1.06 1.06l-3.329-3.328A7 7 0 012 9z" clip-rule="evenodd" />
      </svg>
      <input
        id="search-field"
        class="block h-full w-full border-0 py-0 pl-8 pr-0 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm"
        placeholder="文書を検索..."
        type="search"
        name="search"
      />
    </form>
  </div>
  
  <div class="flex items-center gap-x-4 lg:gap-x-6">
    <!-- 通知 -->
    <NotificationCenter />
    
    <!-- セパレーター -->
    <div class="hidden lg:block lg:h-6 lg:w-px lg:bg-gray-200" aria-hidden="true"></div>
    
    <!-- プロフィールドロップダウン -->
    <div class="relative">
      {#if user}
        <button
          type="button"
          class="-m-1.5 flex items-center p-1.5"
          on:click={toggleUserMenu}
        >
          <span class="sr-only">Open user menu</span>
          <div class="h-8 w-8 rounded-full bg-blue-600 flex items-center justify-center">
            <span class="text-sm font-medium text-white">
              {user.name.charAt(0)}
            </span>
          </div>
          <span class="hidden lg:flex lg:items-center">
            <span class="ml-4 text-sm font-semibold leading-6 text-gray-900">{user.name}</span>
            <svg class="ml-2 h-5 w-5 text-gray-400" viewBox="0 0 20 20" fill="currentColor">
              <path fill-rule="evenodd" d="M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z" clip-rule="evenodd" />
            </svg>
          </span>
        </button>
        
        {#if showUserMenu}
          <div class="absolute right-0 z-10 mt-2.5 w-32 origin-top-right rounded-md bg-white py-2 shadow-lg ring-1 ring-gray-900/5 focus:outline-none">
            <div class="px-3 py-2 text-sm text-gray-700 border-b border-gray-200 lg:hidden">
              <div class="font-medium">{user.name}</div>
              {#if user.department}
                <div class="text-gray-500 text-xs">{user.department}</div>
              {/if}
            </div>
            
            <a href="/profile" class="block px-3 py-1 text-sm leading-6 text-gray-900 hover:bg-gray-50">
              プロフィール
            </a>
            <a href="/settings" class="block px-3 py-1 text-sm leading-6 text-gray-900 hover:bg-gray-50">
              設定
            </a>
            <button
              type="button"
              class="block w-full text-left px-3 py-1 text-sm leading-6 text-gray-900 hover:bg-gray-50"
              on:click={handleLogout}
            >
              ログアウト
            </button>
          </div>
        {/if}
      {/if}
    </div>
  </div>
</header>

<!-- クリック外でメニューを閉じる -->
{#if showUserMenu}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div 
    class="fixed inset-0 z-30" 
    on:click={() => showUserMenu = false}
  ></div>
{/if}