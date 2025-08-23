<script lang="ts">
  import { onMount } from "svelte";
  import {
    auditAccessibility,
    checkColorContrast,
    createFocusTracker,
    type AccessibilityReport,
  } from "$lib/utils/accessibility";

  export let autoRun = false;
  export let showDetailedReport = false;

  let report: AccessibilityReport | null = null;
  let isRunning = false;
  let focusTracker = createFocusTracker();
  let colorContrastResults: any[] = [];

  // Common color combinations to test
  const commonColors = [
    {
      name: "プライマリボタン",
      fg: "rgb(255, 255, 255)",
      bg: "rgb(37, 99, 235)",
    },
    {
      name: "セカンダリボタン",
      fg: "rgb(55, 65, 81)",
      bg: "rgb(243, 244, 246)",
    },
    {
      name: "成功メッセージ",
      fg: "rgb(255, 255, 255)",
      bg: "rgb(34, 197, 94)",
    },
    {
      name: "エラーメッセージ",
      fg: "rgb(255, 255, 255)",
      bg: "rgb(239, 68, 68)",
    },
    {
      name: "警告メッセージ",
      fg: "rgb(146, 64, 14)",
      bg: "rgb(254, 240, 138)",
    },
    { name: "通常テキスト", fg: "rgb(17, 24, 39)", bg: "rgb(255, 255, 255)" },
    {
      name: "サブテキスト",
      fg: "rgb(107, 114, 128)",
      bg: "rgb(255, 255, 255)",
    },
  ];

  // Run accessibility audit
  async function runAudit() {
    isRunning = true;

    try {
      // Wait a bit for any pending renders
      await new Promise((resolve) => setTimeout(resolve, 500));

      // Run main audit
      report = auditAccessibility(document.body);

      // Test color contrast
      colorContrastResults = commonColors.map((color) => ({
        ...color,
        result: checkColorContrast(color.fg, color.bg),
      }));

      // Test focus tracking
      focusTracker.startTracking();

      console.log("アクセシビリティ監査完了:", report);
    } catch (error) {
      console.error("アクセシビリティ監査エラー:", error);
    } finally {
      isRunning = false;
    }
  }

  // Get severity color
  function getSeverityColor(severity: string): string {
    switch (severity) {
      case "error":
        return "text-red-600 bg-red-50";
      case "warning":
        return "text-yellow-600 bg-yellow-50";
      case "notice":
        return "text-blue-600 bg-blue-50";
      default:
        return "text-gray-600 bg-gray-50";
    }
  }

  // Get score color
  function getScoreColor(score: number): string {
    if (score >= 90) return "text-green-600";
    if (score >= 80) return "text-yellow-600";
    if (score >= 70) return "text-orange-600";
    return "text-red-600";
  }

  // Format contrast ratio
  function formatContrastRatio(ratio: number): string {
    return `${ratio.toFixed(2)}:1`;
  }

  // Test keyboard navigation
  function testKeyboardNavigation() {
    const focusableElements = document.querySelectorAll(
      'a, button, input, select, textarea, [tabindex]:not([tabindex="-1"])',
    );

    if (focusableElements.length > 0) {
      (focusableElements[0] as HTMLElement).focus();

      let currentIndex = 0;
      const testInterval = setInterval(() => {
        currentIndex++;
        if (currentIndex < focusableElements.length) {
          (focusableElements[currentIndex] as HTMLElement).focus();
        } else {
          clearInterval(testInterval);
        }
      }, 500);
    }
  }

  onMount(() => {
    if (autoRun) {
      runAudit();
    }
  });
</script>

<div
  class="fixed top-4 right-4 w-96 bg-white shadow-lg rounded-lg border border-gray-200 z-50"
>
  <div class="p-4 border-b border-gray-200">
    <div class="flex items-center justify-between">
      <h3 class="text-lg font-medium text-gray-900">
        アクセシビリティチェッカー
      </h3>
      <button
        class="text-gray-400 hover:text-gray-600"
        on:click={() => (showDetailedReport = !showDetailedReport)}
      >
        {showDetailedReport ? "簡易表示" : "詳細表示"}
      </button>
    </div>
  </div>

  <div class="p-4">
    <!-- Control Buttons -->
    <div class="flex space-x-2 mb-4">
      <button
        class="flex-1 bg-blue-600 text-white px-3 py-2 rounded-md text-sm hover:bg-blue-700 disabled:opacity-50"
        on:click={runAudit}
        disabled={isRunning}
      >
        {isRunning ? "監査中..." : "監査実行"}
      </button>
      <button
        class="flex-1 bg-gray-600 text-white px-3 py-2 rounded-md text-sm hover:bg-gray-700"
        on:click={testKeyboardNavigation}
      >
        キーボードテスト
      </button>
    </div>

    {#if report}
      <!-- Overall Score -->
      <div class="mb-4 p-3 bg-gray-50 rounded-md">
        <div class="flex items-center justify-between">
          <span class="text-sm font-medium text-gray-700">総合スコア</span>
          <span class="text-2xl font-bold {getScoreColor(report.score)}">
            {report.score}/100
          </span>
        </div>

        <div class="mt-2">
          <div class="flex justify-between text-xs text-gray-600">
            <span
              >エラー: {report.issues.filter((i) => i.severity === "error")
                .length}</span
            >
            <span
              >警告: {report.issues.filter((i) => i.severity === "warning")
                .length}</span
            >
            <span
              >注意: {report.issues.filter((i) => i.severity === "notice")
                .length}</span
            >
          </div>
        </div>
      </div>

      {#if showDetailedReport}
        <!-- Detailed Issues -->
        <div class="mb-4 max-h-48 overflow-y-auto">
          <h4 class="text-sm font-medium text-gray-900 mb-2">検出された問題</h4>
          {#if report.issues.length === 0}
            <p class="text-sm text-green-600">問題は検出されませんでした！</p>
          {:else}
            <div class="space-y-2">
              {#each report.issues as issue}
                <div
                  class="p-2 rounded-md text-xs {getSeverityColor(
                    issue.severity,
                  )}"
                >
                  <div class="font-medium mb-1">{issue.issue}</div>
                  <div class="text-gray-600">{issue.element}</div>
                  <div class="mt-1 text-gray-700">{issue.solution}</div>
                </div>
              {/each}
            </div>
          {/if}
        </div>

        <!-- Color Contrast Results -->
        <div class="mb-4">
          <h4 class="text-sm font-medium text-gray-900 mb-2">
            色彩コントラスト
          </h4>
          <div class="space-y-1">
            {#each colorContrastResults as color}
              <div class="flex items-center justify-between text-xs">
                <span class="text-gray-700">{color.name}</span>
                <div class="flex items-center space-x-2">
                  <span class="font-mono"
                    >{formatContrastRatio(color.result.ratio)}</span
                  >
                  <div class="flex space-x-1">
                    <span
                      class="px-1 rounded text-xs {color.result.wcagAA
                        ? 'bg-green-100 text-green-700'
                        : 'bg-red-100 text-red-700'}"
                    >
                      AA
                    </span>
                    <span
                      class="px-1 rounded text-xs {color.result.wcagAAA
                        ? 'bg-green-100 text-green-700'
                        : 'bg-red-100 text-red-700'}"
                    >
                      AAA
                    </span>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        </div>

        <!-- Recommendations -->
        <div class="mb-4">
          <h4 class="text-sm font-medium text-gray-900 mb-2">推奨改善事項</h4>
          <ul class="space-y-1">
            {#each report.recommendations.slice(0, 5) as recommendation}
              <li class="text-xs text-gray-600 flex items-start">
                <span class="text-blue-500 mr-1">•</span>
                {recommendation}
              </li>
            {/each}
          </ul>
        </div>
      {:else}
        <!-- Quick Summary -->
        <div class="space-y-2 text-sm">
          {#if report.issues.filter((i) => i.severity === "error").length > 0}
            <div class="text-red-600">
              🚨 {report.issues.filter((i) => i.severity === "error").length} 件の重大な問題
            </div>
          {/if}
          {#if report.issues.filter((i) => i.severity === "warning").length > 0}
            <div class="text-yellow-600">
              ⚠️ {report.issues.filter((i) => i.severity === "warning").length} 件の警告
            </div>
          {/if}
          {#if report.issues.length === 0}
            <div class="text-green-600">
              ✅ 大きな問題は見つかりませんでした
            </div>
          {/if}
        </div>
      {/if}
    {/if}

    {#if !report && !isRunning}
      <div class="text-center text-gray-500 text-sm">
        「監査実行」ボタンをクリックして<br
        />アクセシビリティをチェックしてください
      </div>
    {/if}
  </div>
</div>
