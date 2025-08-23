<script lang="ts">
  import { onMount } from "svelte";
  import {
    auditAccessibility,
    type AccessibilityReport,
  } from "$lib/utils/accessibility";

  export let targetElement: HTMLElement | null = null;
  export let showReport = false;

  let accessibilityReport: AccessibilityReport | null = null;
  let isRunning = false;
  let performanceMetrics = {
    loadTime: 0,
    interactionTime: 0,
    renderTime: 0,
  };

  // User interaction tracking
  let interactionCount = 0;
  let errorCount = 0;
  let completionTime = 0;
  let startTime = 0;

  // Task tracking
  interface Task {
    id: string;
    description: string;
    completed: boolean;
    timeToComplete?: number;
    errors: string[];
  }

  let tasks: Task[] = [
    {
      id: "search",
      description: "文書を検索して詳細を確認する",
      completed: false,
      errors: [],
    },
    {
      id: "create",
      description: "新規文書を作成する",
      completed: false,
      errors: [],
    },
    {
      id: "navigate",
      description: "ナビゲーションメニューから組織管理画面に移動する",
      completed: false,
      errors: [],
    },
    {
      id: "filter",
      description: "フィルター機能を使用して検索結果を絞り込む",
      completed: false,
      errors: [],
    },
  ];

  // Performance measurement
  function measurePerformance() {
    const navigation = performance.getEntriesByType(
      "navigation",
    )[0] as PerformanceNavigationTiming;

    performanceMetrics = {
      loadTime: navigation.loadEventEnd - navigation.loadEventStart,
      interactionTime: navigation.domInteractive - navigation.domLoading,
      renderTime:
        navigation.domContentLoadedEventEnd -
        navigation.domContentLoadedEventStart,
    };
  }

  // Start usability test
  function startTest() {
    isRunning = true;
    startTime = Date.now();
    interactionCount = 0;
    errorCount = 0;

    // Reset tasks
    tasks = tasks.map((task) => ({
      ...task,
      completed: false,
      timeToComplete: undefined,
      errors: [],
    }));

    // Track interactions
    document.addEventListener("click", trackInteraction);
    document.addEventListener("keydown", trackKeyboardInteraction);

    measurePerformance();
  }

  // Stop usability test
  function stopTest() {
    isRunning = false;
    completionTime = Date.now() - startTime;

    document.removeEventListener("click", trackInteraction);
    document.removeEventListener("keydown", trackKeyboardInteraction);

    runAccessibilityAudit();
  }

  // Track user interactions
  function trackInteraction(event: MouseEvent) {
    interactionCount++;

    // Check if interaction target is accessible
    const target = event.target as HTMLElement;
    if (
      target.tagName === "DIV" &&
      !target.hasAttribute("role") &&
      !target.onclick
    ) {
      errorCount++;
      console.warn("Clicked on non-interactive element:", target);
    }
  }

  function trackKeyboardInteraction(event: KeyboardEvent) {
    // Track keyboard navigation attempts
    if (event.key === "Tab") {
      interactionCount++;
    }

    // Check for keyboard traps
    const activeElement = document.activeElement;
    if (
      activeElement &&
      activeElement.tagName === "BUTTON" &&
      event.key === "Enter"
    ) {
      // Simulate button activation tracking
      interactionCount++;
    }
  }

  // Run accessibility audit
  function runAccessibilityAudit() {
    if (targetElement) {
      accessibilityReport = auditAccessibility(targetElement);
    } else {
      accessibilityReport = auditAccessibility(document.body);
    }
  }

  // Mark task as completed
  function completeTask(taskId: string) {
    const taskIndex = tasks.findIndex((t) => t.id === taskId);
    if (taskIndex !== -1 && isRunning) {
      tasks[taskIndex] = {
        ...tasks[taskIndex],
        completed: true,
        timeToComplete: Date.now() - startTime,
      };
      tasks = [...tasks];
    }
  }

  // Calculate usability score
  function calculateUsabilityScore(): number {
    const completedTasks = tasks.filter((t) => t.completed).length;
    const totalTasks = tasks.length;
    const taskCompletionRate = (completedTasks / totalTasks) * 100;

    const avgTimePerTask = completionTime / totalTasks;
    const timeScore = Math.max(0, 100 - avgTimePerTask / 1000); // Penalty for slow completion

    const errorRate = (errorCount / Math.max(1, interactionCount)) * 100;
    const errorScore = Math.max(0, 100 - errorRate);

    const accessibilityScore = accessibilityReport?.score || 0;

    return Math.round(
      taskCompletionRate * 0.4 +
        timeScore * 0.2 +
        errorScore * 0.2 +
        accessibilityScore * 0.2,
    );
  }

  // Format time for display
  function formatTime(ms: number): string {
    return `${(ms / 1000).toFixed(1)}秒`;
  }

  onMount(() => {
    measurePerformance();
  });
</script>

<div
  class="fixed bottom-4 right-4 w-80 bg-white shadow-lg rounded-lg border border-gray-200 z-50"
>
  <div class="p-4 border-b border-gray-200">
    <div class="flex items-center justify-between">
      <h3 class="text-lg font-medium text-gray-900">ユーザビリティテスト</h3>
      <button
        class="text-gray-400 hover:text-gray-600"
        on:click={() => (showReport = !showReport)}
      >
        {showReport ? "閉じる" : "展開"}
      </button>
    </div>
  </div>

  {#if showReport}
    <div class="p-4 max-h-96 overflow-y-auto">
      <!-- Test Controls -->
      <div class="mb-4">
        {#if !isRunning}
          <button
            class="w-full bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700"
            on:click={startTest}
          >
            テスト開始
          </button>
        {:else}
          <button
            class="w-full bg-red-600 text-white px-4 py-2 rounded-md hover:bg-red-700"
            on:click={stopTest}
          >
            テスト終了
          </button>
        {/if}
      </div>

      {#if isRunning}
        <!-- Live Stats -->
        <div class="mb-4 p-3 bg-blue-50 rounded-md">
          <div class="text-sm font-medium text-blue-900 mb-2">テスト実行中</div>
          <div class="space-y-1 text-xs text-blue-700">
            <div>経過時間: {formatTime(Date.now() - startTime)}</div>
            <div>インタラクション数: {interactionCount}</div>
            <div>エラー数: {errorCount}</div>
          </div>
        </div>

        <!-- Task List -->
        <div class="mb-4">
          <h4 class="text-sm font-medium text-gray-900 mb-2">タスク一覧</h4>
          <div class="space-y-2">
            {#each tasks as task}
              <div class="flex items-start space-x-2">
                <input
                  type="checkbox"
                  checked={task.completed}
                  on:change={() => completeTask(task.id)}
                  class="mt-1"
                />
                <div class="flex-1">
                  <div
                    class="text-xs {task.completed
                      ? 'text-green-600 line-through'
                      : 'text-gray-700'}"
                  >
                    {task.description}
                  </div>
                  {#if task.timeToComplete}
                    <div class="text-xs text-gray-500">
                      完了時間: {formatTime(task.timeToComplete)}
                    </div>
                  {/if}
                  {#if task.errors.length > 0}
                    <div class="text-xs text-red-600">
                      エラー: {task.errors.length}件
                    </div>
                  {/if}
                </div>
              </div>
            {/each}
          </div>
        </div>
      {/if}

      <!-- Performance Metrics -->
      <div class="mb-4">
        <h4 class="text-sm font-medium text-gray-900 mb-2">パフォーマンス</h4>
        <div class="space-y-1 text-xs text-gray-600">
          <div>読み込み時間: {formatTime(performanceMetrics.loadTime)}</div>
          <div>
            インタラクション時間: {formatTime(
              performanceMetrics.interactionTime,
            )}
          </div>
          <div>
            レンダリング時間: {formatTime(performanceMetrics.renderTime)}
          </div>
        </div>
      </div>

      <!-- Accessibility Report -->
      {#if accessibilityReport}
        <div class="mb-4">
          <h4 class="text-sm font-medium text-gray-900 mb-2">
            アクセシビリティ
          </h4>
          <div class="text-xs text-gray-600 mb-2">
            スコア: <span
              class="font-medium {accessibilityReport.score >= 80
                ? 'text-green-600'
                : accessibilityReport.score >= 60
                  ? 'text-yellow-600'
                  : 'text-red-600'}"
            >
              {accessibilityReport.score}/100
            </span>
          </div>

          {#if accessibilityReport.issues.length > 0}
            <div class="space-y-1">
              {#each accessibilityReport.issues.slice(0, 3) as issue}
                <div
                  class="text-xs p-2 rounded {issue.severity === 'error'
                    ? 'bg-red-50 text-red-700'
                    : issue.severity === 'warning'
                      ? 'bg-yellow-50 text-yellow-700'
                      : 'bg-blue-50 text-blue-700'}"
                >
                  <div class="font-medium">{issue.issue}</div>
                  <div class="mt-1">{issue.solution}</div>
                </div>
              {/each}
              {#if accessibilityReport.issues.length > 3}
                <div class="text-xs text-gray-500">
                  他 {accessibilityReport.issues.length - 3} 件の問題
                </div>
              {/if}
            </div>
          {/if}
        </div>
      {/if}

      <!-- Usability Score -->
      {#if !isRunning && completionTime > 0}
        <div class="p-3 bg-gray-50 rounded-md">
          <div class="text-sm font-medium text-gray-900 mb-2">総合スコア</div>
          <div
            class="text-2xl font-bold {calculateUsabilityScore() >= 80
              ? 'text-green-600'
              : calculateUsabilityScore() >= 60
                ? 'text-yellow-600'
                : 'text-red-600'}"
          >
            {calculateUsabilityScore()}/100
          </div>
          <div class="text-xs text-gray-600 mt-1">
            完了率: {Math.round(
              (tasks.filter((t) => t.completed).length / tasks.length) * 100,
            )}%
          </div>
        </div>
      {/if}
    </div>
  {/if}
</div>
