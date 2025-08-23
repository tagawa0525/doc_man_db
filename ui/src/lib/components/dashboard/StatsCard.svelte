<script lang="ts">
  export let title: string;
  export let value: string | number;
  export let change: string = "";
  export let trend: "up" | "down" | "neutral" = "neutral";
  export let icon: string = "default";
  export let description: string = "";
  export let color: "blue" | "green" | "yellow" | "red" | "purple" | "gray" =
    "blue";

  // アイコンパス取得
  function getIconPath(iconName: string): string {
    const icons: Record<string, string> = {
      documents:
        "M19.5 14.25v-2.625a3.375 3.375 0 00-3.375-3.375h-1.5A1.125 1.125 0 0113.5 7.125v-1.5a3.375 3.375 0 00-3.375-3.375H8.25m0 12.75h7.5m-7.5 3H12M10.5 2.25H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 00-9-9z",
      users:
        "M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z",
      new: "M12 4.5v15m7.5-7.5h-15",
      warning:
        "M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z",
      check: "M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
      trending:
        "M2.25 18L9 11.25l4.306 4.307a11.95 11.95 0 015.814-5.519l2.74-1.22m0 0l-5.94-2.28m5.94 2.28l-2.28 5.941",
      storage:
        "M20.25 6.375c0 2.278-3.694 4.125-8.25 4.125S3.75 8.653 3.75 6.375m16.5 0c0-2.278-3.694-4.125-8.25-4.125S3.75 4.097 3.75 6.375m16.5 0v11.25c0 2.278-3.694 4.125-8.25 4.125s-8.25-1.847-8.25-4.125V6.375m16.5 0v3.75m-16.5-3.75v3.75m16.5 0v3.75C20.25 16.153 16.556 18 12 18s-8.25-1.847-8.25-4.125v-3.75m16.5 0c0 2.278-3.694 4.125-8.25 4.125s-8.25-1.847-8.25-4.125",
      calendar:
        "M6.75 3v2.25M17.25 3v2.25M3 9.75h18M12 15.75h.007v.008H12V15.75z",
      clock: "M12 6v6h4.5m4.5 0a9 9 0 11-18 0 9 9 0 0118 0z",
      default:
        "M3.75 3v11.25A2.25 2.25 0 006 16.5h2.25M3.75 3h-1.5m1.5 0h16.5m0 0h1.5m-1.5 0v11.25A2.25 2.25 0 0118 16.5h-2.25m-7.5 0h7.5m-7.5 0l-1 3m8.5-3l1 3m0 0l.5 1.5m-.5-1.5h-9.5m0 0l-.5 1.5m.75-9l3-3 2.148 2.148A12.061 12.061 0 0116.5 7.605",
    };
    return icons[iconName] || icons.default;
  }

  // 色のクラス取得
  function getColorClasses(colorName: string): string {
    const colors: Record<string, string> = {
      blue: "bg-blue-500",
      green: "bg-green-500",
      yellow: "bg-yellow-500",
      red: "bg-red-500",
      purple: "bg-purple-500",
      gray: "bg-gray-500",
    };
    return colors[colorName] || colors.blue;
  }

  // トレンド色取得
  function getTrendColor(trendDirection: string): string {
    const colors: Record<string, string> = {
      up: "text-green-600",
      down: "text-red-600",
      neutral: "text-gray-600",
    };
    return colors[trendDirection] || colors.neutral;
  }
</script>

<div
  class="relative overflow-hidden rounded-lg bg-white px-4 py-5 shadow sm:px-6 sm:py-6 hover:shadow-md transition-shadow"
>
  <dt>
    <div class="absolute rounded-md {getColorClasses(color)} p-3">
      <svg
        class="h-6 w-6 text-white"
        fill="none"
        viewBox="0 0 24 24"
        stroke-width="1.5"
        stroke="currentColor"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          d={getIconPath(icon)}
        />
      </svg>
    </div>
    <p class="ml-16 truncate text-sm font-medium text-gray-500">{title}</p>
    {#if description}
      <p class="ml-16 mt-1 text-xs text-gray-400">{description}</p>
    {/if}
  </dt>
  <dd class="ml-16 flex items-baseline pb-6 sm:pb-7">
    <p class="text-2xl font-semibold text-gray-900">{value}</p>
    {#if change}
      <p
        class="ml-2 flex items-baseline text-sm font-semibold {getTrendColor(
          trend,
        )}"
      >
        {#if trend !== "neutral"}
          <svg
            class="h-4 w-4 flex-shrink-0 self-center {trend === 'down'
              ? 'rotate-180'
              : ''}"
            fill="none"
            viewBox="0 0 24 24"
            stroke-width="1.5"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              d="M4.5 15.75l6-6 3.75 3.75 6-6"
            />
          </svg>
        {/if}
        <span class="ml-1">{change}</span>
      </p>
    {/if}
  </dd>
</div>
