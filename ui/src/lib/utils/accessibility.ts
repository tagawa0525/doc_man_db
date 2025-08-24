/**
 * Accessibility utilities for testing and improving UI accessibility
 */

export interface AccessibilityReport {
  score: number;
  issues: AccessibilityIssue[];
  recommendations: string[];
}

export interface AccessibilityIssue {
  severity: 'error' | 'warning' | 'notice';
  element: string;
  issue: string;
  solution: string;
}

// ARIA role validation
export const validAriaRoles = [
  'alert', 'alertdialog', 'application', 'article', 'banner', 'button',
  'cell', 'checkbox', 'columnheader', 'combobox', 'complementary',
  'contentinfo', 'definition', 'dialog', 'directory', 'document',
  'feed', 'figure', 'form', 'grid', 'gridcell', 'group', 'heading',
  'img', 'link', 'list', 'listbox', 'listitem', 'log', 'main',
  'marquee', 'math', 'menu', 'menubar', 'menuitem', 'menuitemcheckbox',
  'menuitemradio', 'navigation', 'none', 'note', 'option', 'presentation',
  'progressbar', 'radio', 'radiogroup', 'region', 'row', 'rowgroup',
  'rowheader', 'scrollbar', 'search', 'separator', 'slider', 'spinbutton',
  'status', 'switch', 'tab', 'table', 'tablist', 'tabpanel', 'term',
  'textbox', 'timer', 'toolbar', 'tooltip', 'tree', 'treegrid', 'treeitem'
];

// Color contrast testing
export function checkColorContrast(foreground: string, background: string): {
  ratio: number;
  wcagAA: boolean;
  wcagAAA: boolean;
} {
  const getLuminance = (color: string) => {
    // Simplified luminance calculation
    const rgb = color.match(/\d+/g);
    if (!rgb) return 0;

    const [r, g, b] = rgb.map(c => {
      const val = parseInt(c) / 255;
      return val <= 0.03928 ? val / 12.92 : Math.pow((val + 0.055) / 1.055, 2.4);
    });

    return 0.2126 * r + 0.7152 * g + 0.0722 * b;
  };

  const l1 = getLuminance(foreground);
  const l2 = getLuminance(background);
  const ratio = (Math.max(l1, l2) + 0.05) / (Math.min(l1, l2) + 0.05);

  return {
    ratio,
    wcagAA: ratio >= 4.5,
    wcagAAA: ratio >= 7
  };
}

// Keyboard navigation testing
export function checkKeyboardNavigation(element: HTMLElement): AccessibilityIssue[] {
  const issues: AccessibilityIssue[] = [];

  // Check for focusable elements without visible focus
  const focusableElements = element.querySelectorAll(
    'a, button, input, select, textarea, [tabindex]:not([tabindex="-1"])'
  );

  focusableElements.forEach((el, index) => {
    const htmlEl = el as HTMLElement;

    // Check if element can receive focus
    if (htmlEl.tabIndex < 0 && !htmlEl.hasAttribute('tabindex')) {
      issues.push({
        severity: 'warning',
        element: htmlEl.tagName.toLowerCase(),
        issue: 'Element may not be keyboard accessible',
        solution: 'Add tabindex="0" or ensure element is naturally focusable'
      });
    }

    // Check for skip links on first focusable element
    if (index === 0 && !htmlEl.textContent?.includes('スキップ') && !htmlEl.textContent?.includes('Skip')) {
      issues.push({
        severity: 'notice',
        element: htmlEl.tagName.toLowerCase(),
        issue: 'Consider adding skip link for keyboard navigation',
        solution: 'Add a skip link as the first focusable element'
      });
    }
  });

  return issues;
}

// Screen reader testing
export function checkScreenReaderSupport(element: HTMLElement): AccessibilityIssue[] {
  const issues: AccessibilityIssue[] = [];

  // Check for images without alt text
  const images = element.querySelectorAll('img');
  images.forEach(img => {
    if (!img.hasAttribute('alt')) {
      issues.push({
        severity: 'error',
        element: 'img',
        issue: 'Image missing alt text',
        solution: 'Add alt attribute describing the image content'
      });
    }
  });

  // Check for form inputs without labels
  const inputs = element.querySelectorAll('input, select, textarea');
  inputs.forEach(input => {
    const hasLabel = input.hasAttribute('aria-label') ||
      input.hasAttribute('aria-labelledby') ||
      element.querySelector(`label[for="${input.id}"]`) ||
      input.closest('label');

    if (!hasLabel) {
      issues.push({
        severity: 'error',
        element: input.tagName.toLowerCase(),
        issue: 'Form input missing accessible label',
        solution: 'Add aria-label, aria-labelledby, or associate with a label element'
      });
    }
  });

  // Check for buttons without accessible names
  const buttons = element.querySelectorAll('button');
  buttons.forEach(button => {
    const hasAccessibleName = button.textContent?.trim() ||
      button.hasAttribute('aria-label') ||
      button.hasAttribute('aria-labelledby');

    if (!hasAccessibleName) {
      issues.push({
        severity: 'error',
        element: 'button',
        issue: 'Button missing accessible name',
        solution: 'Add text content, aria-label, or aria-labelledby'
      });
    }
  });

  return issues;
}

// Heading structure testing
export function checkHeadingStructure(element: HTMLElement): AccessibilityIssue[] {
  const issues: AccessibilityIssue[] = [];
  const headings = element.querySelectorAll('h1, h2, h3, h4, h5, h6');

  let previousLevel = 0;
  headings.forEach(heading => {
    const level = parseInt(heading.tagName.charAt(1));

    if (level - previousLevel > 1) {
      issues.push({
        severity: 'warning',
        element: heading.tagName.toLowerCase(),
        issue: `Heading level skipped from h${previousLevel} to h${level}`,
        solution: 'Use consecutive heading levels for proper document structure'
      });
    }

    previousLevel = level;
  });

  return issues;
}

// Full accessibility audit
export function auditAccessibility(element: HTMLElement): AccessibilityReport {
  const issues: AccessibilityIssue[] = [
    ...checkKeyboardNavigation(element),
    ...checkScreenReaderSupport(element),
    ...checkHeadingStructure(element)
  ];

  const errorCount = issues.filter(i => i.severity === 'error').length;
  const warningCount = issues.filter(i => i.severity === 'warning').length;
  const noticeCount = issues.filter(i => i.severity === 'notice').length;

  // Calculate score (100 - penalties)
  const score = Math.max(0, 100 - (errorCount * 15) - (warningCount * 5) - (noticeCount * 1));

  const recommendations = [
    'すべてのインタラクティブ要素がキーボードでアクセス可能であることを確認',
    '画像に適切な代替テキストを提供',
    'フォーム要素に明確なラベルを提供',
    '適切な見出し構造を使用',
    '十分な色のコントラストを確保',
    'フォーカス状態を視覚的に明確に表示',
    'エラーメッセージは明確で理解しやすく表示'
  ];

  return {
    score,
    issues,
    recommendations
  };
}

// Utility for testing focus management
export function createFocusTracker() {
  const focusHistory: HTMLElement[] = [];

  const trackFocus = (event: FocusEvent) => {
    if (event.target instanceof HTMLElement) {
      focusHistory.push(event.target);
    }
  };

  const startTracking = () => {
    document.addEventListener('focusin', trackFocus);
  };

  const stopTracking = () => {
    document.removeEventListener('focusin', trackFocus);
  };

  const getFocusHistory = () => [...focusHistory];

  const clearHistory = () => {
    focusHistory.length = 0;
  };

  return {
    startTracking,
    stopTracking,
    getFocusHistory,
    clearHistory
  };
}

// Utility for testing keyboard shortcuts
export function testKeyboardShortcuts(element: HTMLElement, shortcuts: Record<string, () => void>) {
  const handleKeyDown = (event: KeyboardEvent) => {
    const key = [
      event.ctrlKey && 'Ctrl',
      event.altKey && 'Alt',
      event.shiftKey && 'Shift',
      event.metaKey && 'Meta',
      event.key
    ].filter(Boolean).join('+');

    if (shortcuts[key]) {
      event.preventDefault();
      shortcuts[key]();
    }
  };

  element.addEventListener('keydown', handleKeyDown);

  return () => {
    element.removeEventListener('keydown', handleKeyDown);
  };
}
