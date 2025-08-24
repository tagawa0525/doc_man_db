<script lang="ts">
  export let title: string = "";
  export let subtitle: string = "";
  export let actions: boolean = false;
  export let padding: "sm" | "md" | "lg" = "md";
  export let shadow: "none" | "sm" | "md" | "lg" = "md";
  export let rounded: "none" | "sm" | "md" | "lg" = "lg";
  export let clickable: boolean = false;

  function getPaddingClass(size: string): string {
    const classes: Record<string, string> = {
      sm: "p-3 sm:p-4",
      md: "p-4 sm:p-6",
      lg: "p-6 sm:p-8",
    };
    return classes[size] || classes.md;
  }

  function getShadowClass(size: string): string {
    const classes: Record<string, string> = {
      none: "",
      sm: "shadow-sm",
      md: "shadow",
      lg: "shadow-lg",
    };
    return classes[size] || classes.md;
  }

  function getRoundedClass(size: string): string {
    const classes: Record<string, string> = {
      none: "",
      sm: "rounded-sm",
      md: "rounded-md",
      lg: "rounded-lg",
    };
    return classes[size] || classes.lg;
  }

  $: cardClasses = [
    "bg-white",
    getPaddingClass(padding),
    getShadowClass(shadow),
    getRoundedClass(rounded),
    clickable ? "cursor-pointer hover:shadow-lg transition-shadow" : "",
  ]
    .filter(Boolean)
    .join(" ");
</script>

{#if clickable}
  <div class={cardClasses} on:click on:keydown role="button" tabindex="0">
    {#if title || subtitle || $$slots.header}
      <div
        class="mb-4 pb-4 border-b border-gray-200 last:mb-0 last:pb-0 last:border-b-0"
      >
        {#if $$slots.header}
          <slot name="header" />
        {:else}
          <div class="flex items-start justify-between">
            <div>
              {#if title}
                <h3 class="text-lg font-medium text-gray-900 leading-6">
                  {title}
                </h3>
              {/if}
              {#if subtitle}
                <p class="mt-1 text-sm text-gray-500">{subtitle}</p>
              {/if}
            </div>
            {#if actions && $$slots.actions}
              <div class="flex-shrink-0 ml-4">
                <slot name="actions" />
              </div>
            {/if}
          </div>
        {/if}
      </div>
    {/if}

    <div class="space-y-4">
      <slot />
    </div>

    {#if $$slots.footer}
      <div class="mt-4 pt-4 border-t border-gray-200">
        <slot name="footer" />
      </div>
    {/if}
  </div>
{:else}
  <div class={cardClasses}>
    {#if title || subtitle || $$slots.header}
      <div
        class="mb-4 pb-4 border-b border-gray-200 last:mb-0 last:pb-0 last:border-b-0"
      >
        {#if $$slots.header}
          <slot name="header" />
        {:else}
          <div class="flex items-start justify-between">
            <div>
              {#if title}
                <h3 class="text-lg font-medium text-gray-900 leading-6">
                  {title}
                </h3>
              {/if}
              {#if subtitle}
                <p class="mt-1 text-sm text-gray-500">{subtitle}</p>
              {/if}
            </div>
            {#if actions && $$slots.actions}
              <div class="flex-shrink-0 ml-4">
                <slot name="actions" />
              </div>
            {/if}
          </div>
        {/if}
      </div>
    {/if}

    <div class="space-y-4">
      <slot />
    </div>

    {#if $$slots.footer}
      <div class="mt-4 pt-4 border-t border-gray-200">
        <slot name="footer" />
      </div>
    {/if}
  </div>
{/if}
