<script lang="ts">
  import { createEventDispatcher } from "svelte";

  type InputType =
    | "text"
    | "email"
    | "password"
    | "number"
    | "tel"
    | "url"
    | "date"
    | "datetime-local"
    | "time";

  export let type: InputType = "text";
  export let value = "";
  export let placeholder = "";
  export let disabled = false;
  export let readonly = false;
  export let required = false;
  export let error = "";
  export let id = "";
  export let name = "";
  export let autocomplete: string | undefined = undefined;
  export let label = "";
  export let maxlength: number | undefined = undefined;

  let className = "";
  export { className as class };

  const dispatch = createEventDispatcher();

  function handleInput(event: Event) {
    const target = event.target as HTMLInputElement;
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

{#if label}
  <label for={id} class="block text-sm font-medium text-gray-700 mb-1">
    {label}
    {#if required}
      <span class="text-red-500">*</span>
    {/if}
  </label>
{/if}

<div class="relative">
  <input
    {type}
    {id}
    {name}
    {placeholder}
    {disabled}
    {readonly}
    {required}
    {maxlength}
    {autocomplete}
    {value}
    class="block w-full rounded-md border-0 py-1.5 px-3 text-gray-900 shadow-sm ring-1 ring-inset
           {error
      ? 'ring-red-300 focus:ring-red-500'
      : 'ring-gray-300 focus:ring-blue-500'} 
           placeholder:text-gray-400 focus:ring-2 focus:ring-inset sm:text-sm sm:leading-6
           disabled:bg-gray-50 disabled:text-gray-500 disabled:cursor-not-allowed {className}"
    on:input={handleInput}
    on:change={handleChange}
    on:focus={handleFocus}
    on:blur={handleBlur}
  />

  {#if error}
    <div
      class="absolute inset-y-0 right-0 pr-3 flex items-center pointer-events-none"
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
