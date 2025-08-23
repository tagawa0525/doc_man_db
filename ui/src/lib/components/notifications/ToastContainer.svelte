<script lang="ts">
  import { toastNotifications, removeToast } from "$lib/stores/notifications";
  import NotificationToast from "./NotificationToast.svelte";

  function handleToastClose(event: CustomEvent<string>) {
    removeToast(event.detail);
  }
</script>

<!-- トースト通知コンテナ -->
<div class="fixed top-4 right-4 z-50 space-y-2">
  {#each $toastNotifications as notification (notification.id)}
    <NotificationToast {notification} on:close={handleToastClose} />
  {/each}
</div>
