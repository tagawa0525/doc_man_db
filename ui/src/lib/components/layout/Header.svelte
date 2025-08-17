<script lang="ts">
  import { page } from '$app/stores';
  import Button from '../ui/Button.svelte';
  import NotificationCenter from '../notifications/NotificationCenter.svelte';
  
  export let user: { name: string; department?: string } | null = null;
  
  let showUserMenu = false;
  
  function toggleUserMenu() {
    showUserMenu = !showUserMenu;
  }
  
  function handleLogout() {
    // TODO: ログアウト処理の実装
    console.log('Logout requested');
  }
</script>

<header class="bg-white shadow-sm border-b border-gray-200">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <div class="flex justify-between items-center h-16">
      <!-- Logo and Title -->
      <div class="flex items-center">
        <div class="flex-shrink-0 flex items-center">
          <svg class="h-8 w-8 text-blue-600" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
            <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z" />
          </svg>
          <h1 class="ml-3 text-xl font-semibold text-gray-900">文書管理システム</h1>
        </div>
      </div>
      
      <!-- Notifications and User Menu -->
      <div class="flex items-center space-x-4">
        <!-- Notification Center -->
        <NotificationCenter />
        
        <!-- User Menu -->
        <div class="relative">
        {#if user}
          <button
            type="button"
            class="flex items-center text-sm rounded-full bg-white focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
            on:click={toggleUserMenu}
          >
            <span class="sr-only">ユーザーメニューを開く</span>
            <div class="h-8 w-8 rounded-full bg-blue-600 flex items-center justify-center">
              <span class="text-sm font-medium text-white">
                {user.name.charAt(0)}
              </span>
            </div>
            <span class="ml-3 text-gray-700 text-sm font-medium hidden md:block">
              {user.name}
            </span>
            <svg class="ml-2 h-4 w-4 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
            </svg>
          </button>
          
          {#if showUserMenu}
            <div class="origin-top-right absolute right-0 mt-2 w-56 rounded-md shadow-lg bg-white ring-1 ring-black ring-opacity-5 focus:outline-none z-50">
              <div class="py-1">
                <div class="px-4 py-2 text-sm text-gray-700 border-b border-gray-200">
                  <div class="font-medium">{user.name}</div>
                  {#if user.department}
                    <div class="text-gray-500">{user.department}</div>
                  {/if}
                </div>
                
                <a href="/profile" class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100">
                  プロフィール
                </a>
                
                <a href="/settings" class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100">
                  設定
                </a>
                
                <button
                  type="button"
                  class="block w-full text-left px-4 py-2 text-sm text-gray-700 hover:bg-gray-100"
                  on:click={handleLogout}
                >
                  ログアウト
                </button>
              </div>
            </div>
          {/if}
        {:else}
          <Button variant="primary">
            ログイン
          </Button>
        {/if}
      </div>
    </div>
  </div>
</header>

<!-- クリック外でメニューを閉じる -->
{#if showUserMenu}
  <!-- svelte-ignore a11y-click-events-have-key-events -->
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div 
    class="fixed inset-0 z-40" 
    on:click={() => showUserMenu = false}
  ></div>
{/if}