<script lang="ts">
  import { createEventDispatcher } from "svelte";

  interface Option {
    value: string | number | null;
    label: string;
    disabled?: boolean;
  }

  export let options: Option[] = [];
  export let value: string | number | null = "";
  export let placeholder = "選択してください...";
  export let disabled = false;
  export let required = false;
  export let error = "";
  export let id = "";
  export let name = "";
  export let label = "";

  const dispatch = createEventDispatcher();

  function handleChange(event: Event) {
    const target = event.target as HTMLSelectElement;
    let newValue: string | number | null = target.value;
    
    // 数値型の場合は変換
    if (newValue && !isNaN(Number(newValue))) {
      newValue = Number(newValue);
    }
    
    // "null" 文字列の場合はnullに変換
    if (newValue === "null") {
      newValue = null;
    }
    
    value = newValue;
    dispatch("change", { value });
  }
</script>

{#if label}
  <label for={id} class="block text-sm font-medium text-gray-700 mb-1">
    {label}
    {#if required}
      <span class="text-red-500">*</span>
    {/if}
  </label>
{/if}

<div class="relative">
  <select
    {id}
    {name}
    {disabled}
    {required}
    bind:value
    class="block w-full rounded-md border-0 py-1.5 pl-3 pr-10 text-gray-900 shadow-sm ring-1 ring-inset
           {error
      ? 'ring-red-300 focus:ring-red-500'
      : 'ring-gray-300 focus:ring-blue-500'} 
           focus:ring-2 focus:ring-inset sm:text-sm sm:leading-6
           disabled:bg-gray-50 disabled:text-gray-500 disabled:cursor-not-allowed"
    on:change={handleChange}
  >
    {#if placeholder}
      <option value="" disabled selected={!value}>{placeholder}</option>
    {/if}

    {#each options as option}
      <option
        value={option.value === null ? "null" : option.value}
        disabled={option.disabled}
        selected={value === option.value}
      >
        {option.label}
      </option>
    {/each}
  </select>

  <!-- Dropdown arrow -->
  <div
    class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-2"
  >
    <svg class="h-5 w-5 text-gray-400" viewBox="0 0 20 20" fill="currentColor">
      <path
        fill-rule="evenodd"
        d="M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z"
        clip-rule="evenodd"
      />
    </svg>
  </div>

  {#if error}
    <div
      class="absolute inset-y-0 right-8 pr-3 flex items-center pointer-events-none"
    >
      <svg class="h-5 w-5 text-red-500" viewBox="0 0 20 20" fill="currentColor">
        <path
          fill-rule="evenodd"
          d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-3a1 1 0 00-.867.5 1 1 0 11-1.731-1A3 3 0 0113 8a3.001 3.001 0 01-2 2.83V11a1 1 0 11-2 0v-1a1 1 0 011-1 1 1 0 100-2zm0 8a1 1 0 100-2 1 1 0 000 2z"
          clip-rule="evenodd"
        />
      </svg>
    </div>
  {/if}
</div>

{#if error}
  <p class="mt-2 text-sm text-red-600">{error}</p>
{/if}
