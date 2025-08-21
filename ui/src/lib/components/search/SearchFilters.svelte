<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import Input from '$lib/components/ui/Input.svelte';
  import Select from '$lib/components/ui/Select.svelte';
  
  interface SearchFilters {
    title?: string;
    document_type_id?: number;
    created_by?: number;
    created_date_from?: string;
    created_date_to?: string;
    department_id?: number;
    business_id?: number;
    content?: string;
    file_exists?: boolean;
    confidentiality_level?: string;
    limit: number;
    offset: number;
  }
  
  export let filters: SearchFilters;
  
  const dispatch = createEventDispatcher();
  
  let documentTypes: any[] = [];
  let departments: any[] = [];
  let businesses: any[] = [];
  let users: any[] = [];
  
  onMount(async () => {
    await Promise.all([
      loadDocumentTypes(),
      loadDepartments(),
      loadBusinesses(),
      loadUsers()
    ]);
  });
  
  async function loadDocumentTypes() {
    try {
      const response = await fetch('/api/document-types');
      if (response.ok) {
        const data = await response.json();
        documentTypes = data.data || [];
      }
    } catch (error) {
      console.error('Failed to load document types:', error);
    }
  }
  
  async function loadDepartments() {
    try {
      const response = await fetch('/api/departments');
      if (response.ok) {
        const data = await response.json();
        departments = data.data || [];
      }
    } catch (error) {
      console.error('Failed to load departments:', error);
    }
  }
  
  async function loadBusinesses() {
    try {
      const response = await fetch('/api/businesses');
      if (response.ok) {
        const data = await response.json();
        businesses = data.data || [];
      }
    } catch (error) {
      console.error('Failed to load businesses:', error);
    }
  }
  
  async function loadUsers() {
    try {
      const response = await fetch('/api/users');
      if (response.ok) {
        const data = await response.json();
        users = data.data || [];
      }
    } catch (error) {
      console.error('Failed to load users:', error);
    }
  }
  
  function handleFilterChange() {
    dispatch('change', filters);
  }
  
  $: if (filters) {
    handleFilterChange();
  }
</script>

<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
  <!-- Document Type -->
  <div>
    <label class="block text-sm font-medium text-gray-700 mb-1">
      文書種別
    </label>
    <select
      bind:value={filters.document_type_id}
      class="w-full border border-gray-300 rounded-md px-3 py-2 bg-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
    >
      <option value="">すべての種別</option>
      {#each documentTypes as type (type.id)}
        <option value={type.id}>{type.name}</option>
      {/each}
    </select>
  </div>
  
  <!-- Department -->
  <div>
    <label class="block text-sm font-medium text-gray-700 mb-1">
      部署
    </label>
    <select
      bind:value={filters.department_id}
      class="w-full border border-gray-300 rounded-md px-3 py-2 bg-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
    >
      <option value="">すべての部署</option>
      {#each departments as dept (dept.id)}
        <option value={dept.id}>{dept.name}</option>
      {/each}
    </select>
  </div>
  
  <!-- Business Unit -->
  <div>
    <label class="block text-sm font-medium text-gray-700 mb-1">
      事業部
    </label>
    <select
      bind:value={filters.business_id}
      class="w-full border border-gray-300 rounded-md px-3 py-2 bg-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
    >
      <option value="">すべての事業部</option>
      {#each businesses as business (business.id)}
        <option value={business.id}>{business.name}</option>
      {/each}
    </select>
  </div>
  
  <!-- Created By -->
  <div>
    <label class="block text-sm font-medium text-gray-700 mb-1">
      作成者
    </label>
    <select
      bind:value={filters.created_by}
      class="w-full border border-gray-300 rounded-md px-3 py-2 bg-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
    >
      <option value="">すべてのユーザー</option>
      {#each users as user (user.id)}
        <option value={user.id}>{user.name} ({user.username})</option>
      {/each}
    </select>
  </div>
  
  <!-- Date From -->
  <div>
    <label class="block text-sm font-medium text-gray-700 mb-1">
      作成日 (開始)
    </label>
    <Input
      type="date"
      bind:value={filters.created_date_from}
    />
  </div>
  
  <!-- Date To -->
  <div>
    <label class="block text-sm font-medium text-gray-700 mb-1">
      作成日 (終了)
    </label>
    <Input
      type="date"
      bind:value={filters.created_date_to}
    />
  </div>
  
  <!-- Confidentiality Level -->
  <div>
    <label class="block text-sm font-medium text-gray-700 mb-1">
      機密度
    </label>
    <select
      bind:value={filters.confidentiality_level}
      class="w-full border border-gray-300 rounded-md px-3 py-2 bg-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
    >
      <option value="">すべてのレベル</option>
      <option value="public">公開</option>
      <option value="internal">社内</option>
      <option value="confidential">機密</option>
    </select>
  </div>
  
  <!-- File Exists -->
  <div>
    <label class="block text-sm font-medium text-gray-700 mb-1">
      ファイル存在
    </label>
    <select
      bind:value={filters.file_exists}
      class="w-full border border-gray-300 rounded-md px-3 py-2 bg-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
    >
      <option value="">すべて</option>
      <option value={true}>存在する</option>
      <option value={false}>存在しない</option>
    </select>
  </div>
  
  <!-- Results Per Page -->
  <div>
    <label class="block text-sm font-medium text-gray-700 mb-1">
      表示件数
    </label>
    <select
      bind:value={filters.limit}
      class="w-full border border-gray-300 rounded-md px-3 py-2 bg-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
    >
      <option value={10}>10件</option>
      <option value={20}>20件</option>
      <option value={50}>50件</option>
      <option value={100}>100件</option>
    </select>
  </div>
</div>