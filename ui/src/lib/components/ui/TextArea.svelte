<script lang="ts">
  import { createEventDispatcher } from "svelte";

  export let value = "";
  export let placeholder = "";
  export let disabled = false;
  export let readonly = false;
  export let required = false;
  export let rows = 3;
  export let error = "";
  export let id = "";
  export let name = "";
  export let maxlength: number | undefined = undefined;

  const dispatch = createEventDispatcher();

  function handleInput(event: Event) {
    const target = event.target as HTMLTextAreaElement;
    value = target.value;
    dispatch("input", { value });
  }

  function handleChange(event: Event) {
    dispatch("change", event);
  }

  function handleFocus(event: FocusEvent) {
    dispatch("focus", event);
  }

  function handleBlur(event: FocusEvent) {
    dispatch("blur", event);
  }
</script>

<div class="relative">
  <textarea
    {id}
    {name}
    {placeholder}
    {disabled}
    {readonly}
    {required}
    {rows}
    {maxlength}
    {value}
    class="block w-full rounded-md border-0 py-1.5 px-3 text-gray-900 shadow-sm ring-1 ring-inset
           {error
      ? 'ring-red-300 focus:ring-red-500'
      : 'ring-gray-300 focus:ring-blue-500'} 
           placeholder:text-gray-400 focus:ring-2 focus:ring-inset sm:text-sm sm:leading-6
           disabled:bg-gray-50 disabled:text-gray-500 disabled:cursor-not-allowed
           resize-none"
    on:input={handleInput}
    on:change={handleChange}
    on:focus={handleFocus}
    on:blur={handleBlur}
  ></textarea>

  {#if error}
    <div
      class="absolute top-2 right-0 pr-3 flex items-center pointer-events-none"
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

<div class="flex justify-between mt-1">
  {#if error}
    <p class="text-sm text-red-600">{error}</p>
  {:else}
    <div></div>
  {/if}

  {#if maxlength}
    <p class="text-sm text-gray-500">
      {value.length}/{maxlength}
    </p>
  {/if}
</div>
