<script lang="ts">
  import { onMount } from "svelte";
  import CirculationCard from "$lib/components/circulation/CirculationCard.svelte";
  import CreateCirculationModal from "$lib/components/circulation/CreateCirculationModal.svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import PlaceholderBanner from "$lib/components/ui/PlaceholderBanner.svelte";

  interface CirculationStep {
    id: number;
    circulation_id: number;
    step_number: number;
    assignee_id: number;
    action_required: "Review" | "Approve" | "Acknowledge";
    status: "Pending" | "Completed" | "Skipped";
    assigned_at: string;
    completed_at?: string;
    comments?: string;
  }

  interface DocumentCirculation {
    id: number;
    document_id: number;
    workflow_id: number;
    initiated_by: number;
    current_step: number;
    status: "Active" | "Completed" | "Cancelled";
    started_at: string;
    completed_at?: string;
    notes?: string;
  }

  let activeTab = "pending";
  let showCreateModal = false;
  let pendingCirculations: CirculationStep[] = [];
  let completedCirculations: DocumentCirculation[] = [];
  let loading = true;

  onMount(async () => {
    await loadCirculations();
  });

  async function loadCirculations() {
    try {
      loading = true;
      // Load pending circulations for current user
      const pendingResponse = await fetch(
        "/api/circulations/pending?user_id=1",
      );
      if (pendingResponse.ok) {
        const pendingData = await pendingResponse.json();
        pendingCirculations = pendingData.data || [];
      }

      // Load completed circulations
      const completedResponse = await fetch("/api/circulations/completed");
      if (completedResponse.ok) {
        const completedData = await completedResponse.json();
        completedCirculations = completedData.data || [];
      }
    } catch (error) {
      console.error("Failed to load circulations:", error);
    } finally {
      loading = false;
    }
  }

  async function handleCreateCirculation(event: CustomEvent) {
    const { document_id, workflow_id, notes } = event.detail;
    try {
      const response = await fetch("/api/circulations", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ document_id, workflow_id, notes }),
      });

      if (response.ok) {
        showCreateModal = false;
        await loadCirculations();
      } else {
        console.error("Failed to create circulation");
      }
    } catch (error) {
      console.error("Error creating circulation:", error);
    }
  }

  async function handleStepAction(event: CustomEvent) {
    const { stepId, action, comments } = event.detail;
    try {
      const response = await fetch("/api/circulations/complete-step", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          circulation_id: 0, // Will be determined by step_id
          step_id: stepId,
          action,
          comments,
        }),
      });

      if (response.ok) {
        await loadCirculations();
      } else {
        console.error("Failed to complete step");
      }
    } catch (error) {
      console.error("Error completing step:", error);
    }
  }
</script>

<svelte:head>
  <title>文書回覧管理</title>
</svelte:head>

<div class="min-h-screen bg-gray-50">
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <!-- Placeholder Banner -->
    <PlaceholderBanner featureKey="circulation" class="mb-6" />

    <!-- Header -->
    <div class="mb-8">
      <div class="flex justify-between items-center">
        <div>
          <h1 class="text-3xl font-bold text-gray-900">文書回覧管理</h1>
          <p class="mt-2 text-gray-600">
            文書の回覧状況を確認し、承認手続きを行います。
          </p>
        </div>
        <Button on:click={() => (showCreateModal = true)} variant="primary">
          新規回覧開始
        </Button>
      </div>
    </div>

    <!-- Tabs -->
    <div class="border-b border-gray-200 mb-6">
      <nav class="-mb-px flex space-x-8">
        <button
          class="py-2 px-1 border-b-2 font-medium text-sm {activeTab ===
          'pending'
            ? 'border-blue-500 text-blue-600'
            : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}"
          on:click={() => (activeTab = "pending")}
        >
          承認待ち ({pendingCirculations.length})
        </button>
        <button
          class="py-2 px-1 border-b-2 font-medium text-sm {activeTab ===
          'completed'
            ? 'border-blue-500 text-blue-600'
            : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'}"
          on:click={() => (activeTab = "completed")}
        >
          完了済み ({completedCirculations.length})
        </button>
      </nav>
    </div>

    <!-- Content -->
    {#if loading}
      <div class="flex justify-center items-center py-12">
        <div
          class="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600"
        ></div>
        <span class="ml-2 text-gray-600">読み込み中...</span>
      </div>
    {:else if activeTab === "pending"}
      <div class="space-y-4">
        {#each pendingCirculations as step (step.id)}
          <CirculationCard {step} on:action={handleStepAction} />
        {:else}
          <div class="text-center py-12">
            <div class="text-gray-400 text-lg mb-2">
              承認待ちの回覧はありません
            </div>
            <p class="text-gray-500">
              新しい回覧が開始されると、ここに表示されます。
            </p>
          </div>
        {/each}
      </div>
    {:else if activeTab === "completed"}
      <div class="space-y-4">
        {#each completedCirculations as circulation (circulation.id)}
          <div class="bg-white shadow rounded-lg p-6">
            <div class="flex justify-between items-start">
              <div>
                <h3 class="text-lg font-medium text-gray-900">
                  回覧ID: {circulation.id}
                </h3>
                <p class="text-sm text-gray-600 mt-1">
                  文書ID: {circulation.document_id} | ワークフローID: {circulation.workflow_id}
                </p>
                <p class="text-sm text-gray-500 mt-2">
                  開始: {new Date(circulation.started_at).toLocaleDateString(
                    "ja-JP",
                  )}
                  {#if circulation.completed_at}
                    | 完了: {new Date(
                      circulation.completed_at,
                    ).toLocaleDateString("ja-JP")}
                  {/if}
                </p>
              </div>
              <span
                class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium
                {circulation.status === 'Completed'
                  ? 'bg-green-100 text-green-800'
                  : circulation.status === 'Cancelled'
                    ? 'bg-red-100 text-red-800'
                    : 'bg-yellow-100 text-yellow-800'}"
              >
                {circulation.status === "Completed"
                  ? "完了"
                  : circulation.status === "Cancelled"
                    ? "キャンセル"
                    : "アクティブ"}
              </span>
            </div>
            {#if circulation.notes}
              <div class="mt-4 p-3 bg-gray-50 rounded-md">
                <p class="text-sm text-gray-700">{circulation.notes}</p>
              </div>
            {/if}
          </div>
        {:else}
          <div class="text-center py-12">
            <div class="text-gray-400 text-lg mb-2">
              完了済みの回覧はありません
            </div>
          </div>
        {/each}
      </div>
    {/if}
  </div>
</div>

<!-- Create Circulation Modal -->
{#if showCreateModal}
  <CreateCirculationModal
    on:create={handleCreateCirculation}
    on:cancel={() => (showCreateModal = false)}
  />
{/if}
