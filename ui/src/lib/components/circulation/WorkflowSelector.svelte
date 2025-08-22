<script lang="ts">
  import { onMount } from 'svelte';
  import Select from '$lib/components/ui/Select.svelte';
  
  interface WorkflowStep {
    step_number: number;
    assignee_role: string;
    action_required: 'Review' | 'Approve' | 'Acknowledge';
    is_optional: boolean;
    timeout_hours?: number;
  }
  
  interface CirculationWorkflow {
    id: number;
    name: string;
    description?: string;
    steps: WorkflowStep[];
    is_active: boolean;
    created_by: number;
    created_at: string;
  }
  
  export let selectedWorkflowId: number | null = null;
  
  let workflows: CirculationWorkflow[] = [];
  let loading = true;
  
  $: selectedWorkflow = workflows.find(w => w.id === selectedWorkflowId);
  
  onMount(async () => {
    await loadWorkflows();
  });
  
  async function loadWorkflows() {
    try {
      loading = true;
      const response = await fetch('/api/workflows');
      if (response.ok) {
        const data = await response.json();
        workflows = data.data || [];
      }
    } catch (error) {
      console.error('Failed to load workflows:', error);
    } finally {
      loading = false;
    }
  }
  
  function handleWorkflowChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    selectedWorkflowId = target.value ? parseInt(target.value) : null;
  }
  
  function getActionRequiredText(action: string): string {
    switch (action) {
      case 'Review': return '確認';
      case 'Approve': return '承認';
      case 'Acknowledge': return '受理';
      default: return action;
    }
  }
</script>

<div class="space-y-4">
  <div>
    <label for="workflow-select" class="block text-sm font-medium text-gray-700 mb-1">
      ワークフローを選択
    </label>
    {#if loading}
      <div class="animate-pulse h-10 bg-gray-200 rounded-md"></div>
    {:else}
      <select
        id="workflow-select"
        value={selectedWorkflowId || ''}
        on:change={handleWorkflowChange}
        class="w-full border border-gray-300 rounded-md px-3 py-2 bg-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
      >
        <option value="">ワークフローを選択してください</option>
        {#each workflows as workflow (workflow.id)}
          <option value={workflow.id} disabled={!workflow.is_active}>
            {workflow.name} {!workflow.is_active ? '(無効)' : ''}
          </option>
        {/each}
      </select>
    {/if}
  </div>
  
  {#if selectedWorkflow}
    <div class="border border-gray-200 rounded-lg p-4 bg-gray-50">
      <h3 class="text-lg font-medium text-gray-900 mb-2">{selectedWorkflow.name}</h3>
      
      {#if selectedWorkflow.description}
        <p class="text-sm text-gray-600 mb-4">{selectedWorkflow.description}</p>
      {/if}
      
      <div class="space-y-3">
        <h4 class="text-md font-medium text-gray-800">ワークフロー手順:</h4>
        
        {#each selectedWorkflow.steps as step, index (step.step_number)}
          <div class="flex items-center space-x-3 p-3 bg-white rounded-md border">
            <div class="flex-shrink-0">
              <span class="inline-flex items-center justify-center w-8 h-8 rounded-full bg-blue-100 text-blue-800 text-sm font-medium">
                {step.step_number}
              </span>
            </div>
            
            <div class="flex-1 min-w-0">
              <div class="flex items-center space-x-2">
                <p class="text-sm font-medium text-gray-900">
                  {step.assignee_role}
                </p>
                <span class="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-gray-100 text-gray-800">
                  {getActionRequiredText(step.action_required)}
                </span>
                {#if step.is_optional}
                  <span class="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800">
                    任意
                  </span>
                {/if}
              </div>
              
              {#if step.timeout_hours}
                <p class="text-xs text-gray-500 mt-1">
                  タイムアウト: {step.timeout_hours}時間
                </p>
              {/if}
            </div>
          </div>
        {/each}
      </div>
      
      <div class="mt-4 text-xs text-gray-500">
        作成日時: {new Date(selectedWorkflow.created_at).toLocaleString('ja-JP')}
      </div>
    </div>
  {/if}
</div>