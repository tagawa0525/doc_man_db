<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import Button from "$lib/components/ui/Button.svelte";
  import TextArea from "$lib/components/ui/TextArea.svelte";

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

  export let step: CirculationStep;

  const dispatch = createEventDispatcher();

  let showActionPanel = false;
  let selectedAction: "Approve" | "Reject" | "RequestChanges" | null = null;
  let comments = "";

  function handleAction(action: "Approve" | "Reject" | "RequestChanges") {
    selectedAction = action;
    showActionPanel = true;
  }

  function submitAction() {
    if (selectedAction) {
      dispatch("action", {
        stepId: step.id,
        action: selectedAction,
        comments: comments.trim() || undefined,
      });
      showActionPanel = false;
      selectedAction = null;
      comments = "";
    }
  }

  function cancelAction() {
    showActionPanel = false;
    selectedAction = null;
    comments = "";
  }

  function getActionRequiredText(action: string): string {
    switch (action) {
      case "Review":
        return "確認";
      case "Approve":
        return "承認";
      case "Acknowledge":
        return "受理";
      default:
        return action;
    }
  }

  function getActionButtonText(
    action: "Approve" | "Reject" | "RequestChanges",
  ): string {
    switch (action) {
      case "Approve":
        return "承認";
      case "Reject":
        return "却下";
      case "RequestChanges":
        return "修正依頼";
      default:
        return action;
    }
  }
</script>

<div class="bg-white shadow rounded-lg p-6 border-l-4 border-blue-500">
  <div class="flex justify-between items-start mb-4">
    <div>
      <h3 class="text-lg font-medium text-gray-900">
        ステップ {step.step_number} - {getActionRequiredText(
          step.action_required,
        )}
      </h3>
      <p class="text-sm text-gray-600 mt-1">
        回覧ID: {step.circulation_id} | ステップID: {step.id}
      </p>
      <p class="text-sm text-gray-500 mt-1">
        割り当て日時: {new Date(step.assigned_at).toLocaleString("ja-JP")}
      </p>
    </div>

    <span
      class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium
      {step.status === 'Pending'
        ? 'bg-yellow-100 text-yellow-800'
        : step.status === 'Completed'
          ? 'bg-green-100 text-green-800'
          : 'bg-gray-100 text-gray-800'}"
    >
      {step.status === "Pending"
        ? "保留中"
        : step.status === "Completed"
          ? "完了"
          : "スキップ"}
    </span>
  </div>

  {#if step.comments}
    <div class="mb-4 p-3 bg-gray-50 rounded-md">
      <p class="text-sm text-gray-700 font-medium mb-1">コメント:</p>
      <p class="text-sm text-gray-600">{step.comments}</p>
    </div>
  {/if}

  {#if step.status === "Pending"}
    {#if !showActionPanel}
      <div class="flex space-x-3">
        {#if step.action_required === "Review"}
          <Button variant="primary" on:click={() => handleAction("Approve")}>
            確認完了
          </Button>
          <Button
            variant="secondary"
            on:click={() => handleAction("RequestChanges")}
          >
            修正依頼
          </Button>
        {:else if step.action_required === "Approve"}
          <Button variant="primary" on:click={() => handleAction("Approve")}>
            承認
          </Button>
          <Button variant="danger" on:click={() => handleAction("Reject")}>
            却下
          </Button>
          <Button
            variant="secondary"
            on:click={() => handleAction("RequestChanges")}
          >
            修正依頼
          </Button>
        {:else if step.action_required === "Acknowledge"}
          <Button variant="primary" on:click={() => handleAction("Approve")}>
            受理
          </Button>
        {/if}
      </div>
    {:else}
      <div class="border-t pt-4">
        <h4 class="text-md font-medium text-gray-900 mb-3">
          {selectedAction ? getActionButtonText(selectedAction) : ""} - コメント入力
        </h4>

        <div class="mb-4">
          <TextArea
            bind:value={comments}
            placeholder="コメントを入力してください（任意）"
            rows={3}
          />
        </div>

        <div class="flex space-x-3">
          <Button variant="primary" on:click={submitAction}>
            {selectedAction ? getActionButtonText(selectedAction) : ""}実行
          </Button>
          <Button variant="secondary" on:click={cancelAction}>
            キャンセル
          </Button>
        </div>
      </div>
    {/if}
  {:else if step.status === "Completed" && step.completed_at}
    <p class="text-sm text-green-600">
      完了日時: {new Date(step.completed_at).toLocaleString("ja-JP")}
    </p>
  {/if}
</div>
