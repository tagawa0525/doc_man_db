<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { slide } from "svelte/transition";

  interface ToastNotification {
    id: string;
    type: "info" | "success" | "warning" | "error";
    title: string;
    message?: string;
    duration?: number;
    actions?: Array<{
      label: string;
      action: () => void;
    }>;
  }

  export let notification: ToastNotification;
  export let autoClose = true;

  const dispatch = createEventDispatcher();

  // デフォルトの表示時間（ミリ秒）
  const defaultDuration = {
    info: 4000,
    success: 3000,
    warning: 5000,
    error: 6000,
  };

  let timeoutId: NodeJS.Timeout;

  // 自動クローズタイマー設定
  function setAutoCloseTimer() {
    if (autoClose) {
      const duration =
        notification.duration || defaultDuration[notification.type];
      timeoutId = setTimeout(() => {
        closeToast();
      }, duration);
    }
  }

  // トースト閉じる
  function closeToast() {
    if (timeoutId) {
      clearTimeout(timeoutId);
    }
    dispatch("close", notification.id);
  }

  // マウスオーバーでタイマー一時停止
  function pauseAutoClose() {
    if (timeoutId) {
      clearTimeout(timeoutId);
    }
  }

  // マウスアウトでタイマー再開
  function resumeAutoClose() {
    setAutoCloseTimer();
  }

  // 通知タイプ別のスタイル
  function getToastStyle(type: string): string {
    const styles: Record<string, string> = {
      info: "bg-blue-50 border-blue-200",
      success: "bg-green-50 border-green-200",
      warning: "bg-yellow-50 border-yellow-200",
      error: "bg-red-50 border-red-200",
    };
    return styles[type] || styles.info;
  }

  // 通知タイプ別のアイコン色
  function getIconColor(type: string): string {
    const colors: Record<string, string> = {
      info: "text-blue-400",
      success: "text-green-400",
      warning: "text-yellow-400",
      error: "text-red-400",
    };
    return colors[type] || colors.info;
  }

  // 通知タイプ別のテキスト色
  function getTextColor(type: string): string {
    const colors: Record<string, string> = {
      info: "text-blue-800",
      success: "text-green-800",
      warning: "text-yellow-800",
      error: "text-red-800",
    };
    return colors[type] || colors.info;
  }

  // 通知タイプ別のアイコン
  function getIcon(type: string): string {
    const icons: Record<string, string> = {
      info: "M11.25 11.25l.041-.02a.75.75 0 011.063.852l-.708 2.836a.75.75 0 001.063.853l.041-.021M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9-3.75h.008v.008H12V8.25z",
      success: "M9 12.75L11.25 15 15 9.75M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
      warning:
        "M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z",
      error:
        "M12 9v3.75m9-.75a9 9 0 11-18 0 9 9 0 0118 0zm-9 3.75h.008v.008H12v-.008z",
    };
    return icons[type] || icons.info;
  }

  // コンポーネント初期化時にタイマー設定
  setAutoCloseTimer();
</script>

<div
  class="max-w-sm w-full border rounded-lg shadow-lg pointer-events-auto {getToastStyle(
    notification.type,
  )}"
  transition:slide={{ duration: 300 }}
  on:mouseenter={pauseAutoClose}
  on:mouseleave={resumeAutoClose}
  role="alert"
  aria-live="assertive"
  aria-atomic="true"
>
  <div class="p-4">
    <div class="flex">
      <!-- アイコン -->
      <div class="flex-shrink-0">
        <svg
          class="h-5 w-5 {getIconColor(notification.type)}"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d={getIcon(notification.type)}
          />
        </svg>
      </div>

      <!-- コンテンツ -->
      <div class="ml-3 flex-1">
        <p class="text-sm font-medium {getTextColor(notification.type)}">
          {notification.title}
        </p>

        {#if notification.message}
          <p class="mt-1 text-sm text-gray-600">
            {notification.message}
          </p>
        {/if}

        <!-- アクションボタン -->
        {#if notification.actions && notification.actions.length > 0}
          <div class="mt-3 flex space-x-3">
            {#each notification.actions as action}
              <button
                type="button"
                class="text-sm font-medium {getTextColor(
                  notification.type,
                )} hover:opacity-75"
                on:click={action.action}
              >
                {action.label}
              </button>
            {/each}
          </div>
        {/if}
      </div>

      <!-- 閉じるボタン -->
      <div class="ml-4 flex-shrink-0">
        <button
          type="button"
          class="inline-flex text-gray-400 hover:text-gray-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
          on:click={closeToast}
        >
          <span class="sr-only">閉じる</span>
          <svg
            class="h-4 w-4"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M6 18L18 6M6 6l12 12"
            />
          </svg>
        </button>
      </div>
    </div>
  </div>
</div>
