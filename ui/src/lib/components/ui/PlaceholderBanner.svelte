<script lang="ts">
  export let feature: string;
  export let description: string = "";
  export let roadmapPhase: string = "";
  export let estimatedCompletion: string = "";
  export let contactInfo: string = "システム管理者にお問い合わせください";

  let className: string = "";
  export { className as class };
</script>

<div class="bg-blue-50 border border-blue-200 rounded-md p-4 mb-6 {className}">
  <div class="flex items-start space-x-3">
    <div class="flex-shrink-0">
      <svg
        class="w-5 h-5 text-blue-400"
        fill="currentColor"
        viewBox="0 0 20 20"
      >
        <path
          fill-rule="evenodd"
          d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z"
          clip-rule="evenodd"
        />
      </svg>
    </div>
    <div class="flex-1">
      <h3 class="text-sm font-medium text-blue-800">
        {feature}機能は準備中です
      </h3>
      {#if description}
        <p class="mt-1 text-sm text-blue-700">{description}</p>
      {/if}
      <div class="mt-2 space-y-1 text-xs text-blue-600">
        {#if roadmapPhase}
          <p><strong>実装予定:</strong> {roadmapPhase}</p>
        {/if}
        {#if estimatedCompletion}
          <p><strong>完了予定:</strong> {estimatedCompletion}</p>
        {/if}
        <p><strong>お問い合わせ:</strong> {contactInfo}</p>
      </div>
    </div>
  </div>
</div>
