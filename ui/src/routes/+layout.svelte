<script lang="ts">
  import '../app.css';
  import Header from '$lib/components/layout/Header.svelte';
  import Navigation from '$lib/components/layout/Navigation.svelte';
  import ToastContainer from '$lib/components/notifications/ToastContainer.svelte';
  
  // TODO: 実際の認証システムと連携
  let user = {
    name: 'テストユーザー',
    department: '情報システム部'
  };
</script>

<div class="min-h-screen bg-gray-50">
  <Header {user} />
  
  <div class="flex">
    <Navigation />
    
    <main class="flex-1 p-6">
      <slot />
    </main>
  </div>
  
  <!-- Toast Notifications -->
  <ToastContainer />
</div>