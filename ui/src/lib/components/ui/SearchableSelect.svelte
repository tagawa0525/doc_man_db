<script lang="ts">
  import { createEventDispatcher, onMount } from "svelte";
  import { clickOutside } from "$lib/utils/clickOutside";

  interface Option {
    value: string | number | boolean | null;
    label: string;
    disabled?: boolean;
  }

  export let options: Option[] = [];
  export let value: string | number | boolean | null = null;
  export let placeholder = "検索して選択...";
  export let disabled = false;
  export let required = false;
  export let error = "";
  export let id = "";
  export let name = "";
  export let label = "";

  const dispatch = createEventDispatcher();

  let isOpen = false;
  let searchTerm = "";
  let filteredOptions: Option[] = [];
  let selectedOption: Option | null = null;
  let inputRef: HTMLInputElement;
  let dropdownRef: HTMLElement;
  let focusedIndex = -1;

  // 選択された値から対応するオプションを見つける
  function findSelectedOption() {
    selectedOption = options.find((opt) => opt.value === value) || null;
  }

  // オプションをフィルタリング
  function filterOptions() {
    if (!searchTerm.trim()) {
      filteredOptions = options;
    } else {
      filteredOptions = options.filter(
        (option) =>
          option.label.toLowerCase().includes(searchTerm.toLowerCase()) ||
          (option.value &&
            String(option.value)
              .toLowerCase()
              .includes(searchTerm.toLowerCase())),
      );
    }
    focusedIndex = -1;
  }

  // オプション選択
  function selectOption(option: Option) {
    value = option.value;
    selectedOption = option;
    searchTerm = option.label;
    isOpen = false;
    focusedIndex = -1;
    dispatch("change", { value: option.value, option });
  }

  // ドロップダウンを開く
  function openDropdown() {
    if (disabled) return;
    isOpen = true;
    searchTerm = "";
    filterOptions();
    setTimeout(() => inputRef?.focus(), 0);
  }

  // ドロップダウンを閉じる
  function closeDropdown() {
    isOpen = false;
    searchTerm = selectedOption ? selectedOption.label : "";
    focusedIndex = -1;
  }

  // 入力変更時の処理
  function handleInput() {
    if (!isOpen) {
      isOpen = true;
    }
    filterOptions();
  }

  // キーボード操作
  function handleKeydown(event: KeyboardEvent) {
    if (disabled) return;

    switch (event.key) {
      case "ArrowDown":
        event.preventDefault();
        if (!isOpen) {
          openDropdown();
        } else {
          focusedIndex = Math.min(focusedIndex + 1, filteredOptions.length - 1);
        }
        break;

      case "ArrowUp":
        event.preventDefault();
        if (isOpen) {
          focusedIndex = Math.max(focusedIndex - 1, -1);
        }
        break;

      case "Enter":
        event.preventDefault();
        if (isOpen && focusedIndex >= 0 && filteredOptions[focusedIndex]) {
          selectOption(filteredOptions[focusedIndex]);
        } else if (!isOpen) {
          openDropdown();
        }
        break;

      case "Escape":
        event.preventDefault();
        closeDropdown();
        break;

      case "Tab":
        closeDropdown();
        break;
    }
  }

  // クリックアウト時にドロップダウンを閉じる
  function handleClickOutside() {
    closeDropdown();
  }

  // 初期化とリアクティブ更新
  onMount(() => {
    findSelectedOption();
    searchTerm = selectedOption ? selectedOption.label : "";
    filterOptions();
  });

  $: {
    findSelectedOption();
    if (!isOpen) {
      searchTerm = selectedOption ? selectedOption.label : "";
    }
  }

  $: filterOptions(), searchTerm;
</script>

<div class="relative" use:clickOutside={handleClickOutside}>
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
      bind:this={inputRef}
      {id}
      {name}
      {disabled}
      {required}
      {placeholder}
      bind:value={searchTerm}
      class="block w-full rounded-md border-0 py-1.5 pl-3 pr-10 text-gray-900 shadow-sm ring-1 ring-inset
             {error
        ? 'ring-red-300 focus:ring-red-500'
        : 'ring-gray-300 focus:ring-blue-500'} 
             focus:ring-2 focus:ring-inset sm:text-sm sm:leading-6
             disabled:bg-gray-50 disabled:text-gray-500 disabled:cursor-not-allowed"
      autocomplete="off"
      on:input={handleInput}
      on:keydown={handleKeydown}
      on:focus={openDropdown}
    />

    <!-- Dropdown arrow -->
    <button
      type="button"
      class="absolute inset-y-0 right-0 flex items-center pr-2"
      class:pointer-events-none={disabled}
      on:click={openDropdown}
    >
      <svg
        class="h-5 w-5 text-gray-400 transition-transform duration-200"
        class:rotate-180={isOpen}
        viewBox="0 0 20 20"
        fill="currentColor"
      >
        <path
          fill-rule="evenodd"
          d="M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z"
          clip-rule="evenodd"
        />
      </svg>
    </button>

    <!-- Error icon -->
    {#if error}
      <div
        class="absolute inset-y-0 right-8 pr-3 flex items-center pointer-events-none"
      >
        <svg
          class="h-5 w-5 text-red-500"
          viewBox="0 0 20 20"
          fill="currentColor"
        >
          <path
            fill-rule="evenodd"
            d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-3a1 1 0 00-.867.5 1 1 0 11-1.731-1A3 3 0 0113 8a3.001 3.001 0 01-2 2.83V11a1 1 0 11-2 0v-1a1 1 0 011-1 1 1 0 100-2zm0 8a1 1 0 100-2 1 1 0 000 2z"
            clip-rule="evenodd"
          />
        </svg>
      </div>
    {/if}
  </div>

  <!-- Dropdown -->
  {#if isOpen}
    <div
      bind:this={dropdownRef}
      class="absolute z-10 mt-1 w-full bg-white shadow-lg max-h-60 rounded-md py-1 text-base ring-1 ring-black ring-opacity-5 overflow-auto focus:outline-none sm:text-sm"
    >
      {#if filteredOptions.length === 0}
        <div
          class="relative cursor-default select-none py-2 pl-3 pr-9 text-gray-500"
        >
          該当する項目が見つかりません
        </div>
      {:else}
        {#each filteredOptions as option, index}
          <button
            type="button"
            class="relative w-full cursor-pointer select-none py-2 pl-3 pr-9 text-left
                   {focusedIndex === index
              ? 'bg-blue-600 text-white'
              : 'text-gray-900 hover:bg-gray-50'}
                   {option.disabled ? 'opacity-50 cursor-not-allowed' : ''}"
            disabled={option.disabled}
            on:click={() => selectOption(option)}
            on:mouseenter={() => (focusedIndex = index)}
          >
            <span
              class="block truncate {value === option.value
                ? 'font-semibold'
                : 'font-normal'}"
            >
              {option.label}
            </span>

            {#if value === option.value}
              <span
                class="absolute inset-y-0 right-0 flex items-center pr-4
                           {focusedIndex === index
                  ? 'text-white'
                  : 'text-blue-600'}"
              >
                <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                  <path
                    fill-rule="evenodd"
                    d="M16.704 4.153a.75.75 0 01.143 1.052l-8 10.5a.75.75 0 01-1.127.075l-4.5-4.5a.75.75 0 011.06-1.06l3.894 3.893 7.48-9.817a.75.75 0 011.05-.143z"
                    clip-rule="evenodd"
                  />
                </svg>
              </span>
            {/if}
          </button>
        {/each}
      {/if}
    </div>
  {/if}
</div>

{#if error}
  <p class="mt-2 text-sm text-red-600">{error}</p>
{/if}
