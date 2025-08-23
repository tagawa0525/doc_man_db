<script lang="ts">
  import { onMount, onDestroy } from "svelte";

  export let showMonitor = false;
  export let autoStart = true;

  interface PerformanceMetrics {
    fcp: number; // First Contentful Paint
    lcp: number; // Largest Contentful Paint
    fid: number; // First Input Delay
    cls: number; // Cumulative Layout Shift
    ttfb: number; // Time to First Byte
    memoryUsage: number;
    jsHeapSize: number;
    domNodes: number;
    renderTime: number;
  }

  let metrics: PerformanceMetrics = {
    fcp: 0,
    lcp: 0,
    fid: 0,
    cls: 0,
    ttfb: 0,
    memoryUsage: 0,
    jsHeapSize: 0,
    domNodes: 0,
    renderTime: 0,
  };

  let isMonitoring = false;
  let observerInstances: PerformanceObserver[] = [];
  let intervalId: number;

  // Start performance monitoring
  function startMonitoring() {
    isMonitoring = true;

    // Observe Core Web Vitals
    if ("PerformanceObserver" in window) {
      // First Contentful Paint
      const fcpObserver = new PerformanceObserver((list) => {
        const entries = list.getEntries();
        entries.forEach((entry) => {
          if (entry.name === "first-contentful-paint") {
            metrics.fcp = entry.startTime;
          }
        });
      });
      fcpObserver.observe({ entryTypes: ["paint"] });
      observerInstances.push(fcpObserver);

      // Largest Contentful Paint
      const lcpObserver = new PerformanceObserver((list) => {
        const entries = list.getEntries();
        const lastEntry = entries[entries.length - 1];
        metrics.lcp = lastEntry.startTime;
      });
      lcpObserver.observe({ entryTypes: ["largest-contentful-paint"] });
      observerInstances.push(lcpObserver);

      // First Input Delay
      const fidObserver = new PerformanceObserver((list) => {
        const entries = list.getEntries();
        entries.forEach((entry: any) => {
          metrics.fid = entry.processingStart - entry.startTime;
        });
      });
      fidObserver.observe({ entryTypes: ["first-input"] });
      observerInstances.push(fidObserver);

      // Cumulative Layout Shift
      let clsValue = 0;
      const clsObserver = new PerformanceObserver((list) => {
        for (const entry of list.getEntries()) {
          if (!(entry as any).hadRecentInput) {
            clsValue += (entry as any).value;
            metrics.cls = clsValue;
          }
        }
      });
      clsObserver.observe({ entryTypes: ["layout-shift"] });
      observerInstances.push(clsObserver);
    }

    // Navigation timing
    const navigation = performance.getEntriesByType(
      "navigation",
    )[0] as PerformanceNavigationTiming;
    if (navigation) {
      metrics.ttfb = navigation.responseStart - navigation.requestStart;
      metrics.renderTime =
        navigation.domContentLoadedEventEnd -
        navigation.domContentLoadedEventStart;
    }

    // Start periodic monitoring
    intervalId = setInterval(updateRuntimeMetrics, 1000);
    updateRuntimeMetrics();
  }

  // Stop performance monitoring
  function stopMonitoring() {
    isMonitoring = false;

    observerInstances.forEach((observer) => {
      observer.disconnect();
    });
    observerInstances = [];

    if (intervalId) {
      clearInterval(intervalId);
    }
  }

  // Update runtime metrics
  function updateRuntimeMetrics() {
    // Memory usage (Chrome only)
    if ("memory" in performance) {
      const memory = (performance as any).memory;
      metrics.memoryUsage = memory.usedJSHeapSize / 1048576; // Convert to MB
      metrics.jsHeapSize = memory.totalJSHeapSize / 1048576;
    }

    // DOM nodes count
    metrics.domNodes = document.querySelectorAll("*").length;

    // Trigger reactivity
    metrics = { ...metrics };
  }

  // Get performance score
  function getPerformanceScore(): number {
    let score = 100;

    // FCP scoring (Good: <1.8s, Needs Improvement: 1.8s-3s, Poor: >3s)
    if (metrics.fcp > 3000) score -= 20;
    else if (metrics.fcp > 1800) score -= 10;

    // LCP scoring (Good: <2.5s, Needs Improvement: 2.5s-4s, Poor: >4s)
    if (metrics.lcp > 4000) score -= 25;
    else if (metrics.lcp > 2500) score -= 15;

    // FID scoring (Good: <100ms, Needs Improvement: 100ms-300ms, Poor: >300ms)
    if (metrics.fid > 300) score -= 15;
    else if (metrics.fid > 100) score -= 8;

    // CLS scoring (Good: <0.1, Needs Improvement: 0.1-0.25, Poor: >0.25)
    if (metrics.cls > 0.25) score -= 20;
    else if (metrics.cls > 0.1) score -= 10;

    // Memory usage penalty
    if (metrics.memoryUsage > 50) score -= 10;
    else if (metrics.memoryUsage > 30) score -= 5;

    return Math.max(0, score);
  }

  // Format time
  function formatTime(ms: number): string {
    if (ms < 1000) return `${Math.round(ms)}ms`;
    return `${(ms / 1000).toFixed(2)}s`;
  }

  // Format memory
  function formatMemory(mb: number): string {
    return `${mb.toFixed(1)}MB`;
  }

  // Get metric status color
  function getMetricColor(metric: string, value: number): string {
    switch (metric) {
      case "fcp":
        return value > 3000
          ? "text-red-600"
          : value > 1800
            ? "text-yellow-600"
            : "text-green-600";
      case "lcp":
        return value > 4000
          ? "text-red-600"
          : value > 2500
            ? "text-yellow-600"
            : "text-green-600";
      case "fid":
        return value > 300
          ? "text-red-600"
          : value > 100
            ? "text-yellow-600"
            : "text-green-600";
      case "cls":
        return value > 0.25
          ? "text-red-600"
          : value > 0.1
            ? "text-yellow-600"
            : "text-green-600";
      case "memory":
        return value > 50
          ? "text-red-600"
          : value > 30
            ? "text-yellow-600"
            : "text-green-600";
      default:
        return "text-gray-600";
    }
  }

  onMount(() => {
    if (autoStart) {
      startMonitoring();
    }
  });

  onDestroy(() => {
    stopMonitoring();
  });
</script>

<div
  class="fixed bottom-4 left-4 w-80 bg-white shadow-lg rounded-lg border border-gray-200 z-50"
>
  <div class="p-4 border-b border-gray-200">
    <div class="flex items-center justify-between">
      <h3 class="text-lg font-medium text-gray-900">パフォーマンス監視</h3>
      <button
        class="text-gray-400 hover:text-gray-600"
        on:click={() => (showMonitor = !showMonitor)}
      >
        {showMonitor ? "閉じる" : "展開"}
      </button>
    </div>
  </div>

  {#if showMonitor}
    <div class="p-4">
      <!-- Control -->
      <div class="mb-4">
        {#if !isMonitoring}
          <button
            class="w-full bg-blue-600 text-white px-4 py-2 rounded-md hover:bg-blue-700"
            on:click={startMonitoring}
          >
            監視開始
          </button>
        {:else}
          <button
            class="w-full bg-red-600 text-white px-4 py-2 rounded-md hover:bg-red-700"
            on:click={stopMonitoring}
          >
            監視停止
          </button>
        {/if}
      </div>

      {#if isMonitoring}
        <!-- Performance Score -->
        <div class="mb-4 p-3 bg-gray-50 rounded-md">
          <div class="flex items-center justify-between">
            <span class="text-sm font-medium text-gray-700"
              >パフォーマンススコア</span
            >
            <span
              class="text-xl font-bold {getPerformanceScore() >= 80
                ? 'text-green-600'
                : getPerformanceScore() >= 60
                  ? 'text-yellow-600'
                  : 'text-red-600'}"
            >
              {getPerformanceScore()}/100
            </span>
          </div>
        </div>

        <!-- Core Web Vitals -->
        <div class="mb-4">
          <h4 class="text-sm font-medium text-gray-900 mb-2">
            Core Web Vitals
          </h4>
          <div class="space-y-2">
            <div class="flex justify-between items-center">
              <span class="text-xs text-gray-600"
                >FCP (First Contentful Paint)</span
              >
              <span
                class="text-xs font-medium {getMetricColor('fcp', metrics.fcp)}"
              >
                {formatTime(metrics.fcp)}
              </span>
            </div>
            <div class="flex justify-between items-center">
              <span class="text-xs text-gray-600"
                >LCP (Largest Contentful Paint)</span
              >
              <span
                class="text-xs font-medium {getMetricColor('lcp', metrics.lcp)}"
              >
                {formatTime(metrics.lcp)}
              </span>
            </div>
            <div class="flex justify-between items-center">
              <span class="text-xs text-gray-600">FID (First Input Delay)</span>
              <span
                class="text-xs font-medium {getMetricColor('fid', metrics.fid)}"
              >
                {formatTime(metrics.fid)}
              </span>
            </div>
            <div class="flex justify-between items-center">
              <span class="text-xs text-gray-600"
                >CLS (Cumulative Layout Shift)</span
              >
              <span
                class="text-xs font-medium {getMetricColor('cls', metrics.cls)}"
              >
                {metrics.cls.toFixed(3)}
              </span>
            </div>
          </div>
        </div>

        <!-- Runtime Metrics -->
        <div class="mb-4">
          <h4 class="text-sm font-medium text-gray-900 mb-2">ランタイム指標</h4>
          <div class="space-y-2">
            <div class="flex justify-between items-center">
              <span class="text-xs text-gray-600"
                >TTFB (Time to First Byte)</span
              >
              <span class="text-xs font-medium text-gray-900">
                {formatTime(metrics.ttfb)}
              </span>
            </div>
            <div class="flex justify-between items-center">
              <span class="text-xs text-gray-600">メモリ使用量</span>
              <span
                class="text-xs font-medium {getMetricColor(
                  'memory',
                  metrics.memoryUsage,
                )}"
              >
                {formatMemory(metrics.memoryUsage)}
              </span>
            </div>
            <div class="flex justify-between items-center">
              <span class="text-xs text-gray-600">JSヒープサイズ</span>
              <span class="text-xs font-medium text-gray-900">
                {formatMemory(metrics.jsHeapSize)}
              </span>
            </div>
            <div class="flex justify-between items-center">
              <span class="text-xs text-gray-600">DOMノード数</span>
              <span class="text-xs font-medium text-gray-900">
                {metrics.domNodes.toLocaleString()}
              </span>
            </div>
            <div class="flex justify-between items-center">
              <span class="text-xs text-gray-600">レンダリング時間</span>
              <span class="text-xs font-medium text-gray-900">
                {formatTime(metrics.renderTime)}
              </span>
            </div>
          </div>
        </div>

        <!-- Performance Tips -->
        <div class="text-xs text-gray-500">
          <div class="font-medium mb-1">最適化のヒント:</div>
          <ul class="space-y-1">
            {#if metrics.lcp > 2500}
              <li>• 画像の最適化とレイジーローディングを検討</li>
            {/if}
            {#if metrics.cls > 0.1}
              <li>• レイアウトシフトを減らすため要素サイズを指定</li>
            {/if}
            {#if metrics.memoryUsage > 30}
              <li>• メモリリークの確認とイベントリスナーのクリーンアップ</li>
            {/if}
            {#if metrics.domNodes > 1500}
              <li>• DOM構造の簡素化を検討</li>
            {/if}
          </ul>
        </div>
      {/if}
    </div>
  {/if}
</div>
