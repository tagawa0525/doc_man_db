// タッチ操作用ユーティリティ

export interface SwipeDirection {
  direction: 'left' | 'right' | 'up' | 'down' | null;
  distance: number;
}

export interface TouchPoint {
  x: number;
  y: number;
}

export class TouchHandler {
  private startTouch: TouchPoint | null = null;
  private currentTouch: TouchPoint | null = null;
  private minSwipeDistance = 30;
  
  constructor(
    private element: HTMLElement,
    private options: {
      onSwipe?: (direction: SwipeDirection) => void;
      onTap?: (point: TouchPoint) => void;
      onLongPress?: (point: TouchPoint) => void;
      longPressDelay?: number;
    } = {}
  ) {
    this.setupEventListeners();
  }
  
  private setupEventListeners() {
    let longPressTimer: NodeJS.Timeout;
    
    this.element.addEventListener('touchstart', (e) => {
      const touch = e.touches[0];
      this.startTouch = { x: touch.clientX, y: touch.clientY };
      this.currentTouch = this.startTouch;
      
      // ロングプレス判定開始
      if (this.options.onLongPress) {
        longPressTimer = setTimeout(() => {
          if (this.startTouch && this.options.onLongPress) {
            this.options.onLongPress(this.startTouch);
          }
        }, this.options.longPressDelay || 500);
      }
    }, { passive: true });
    
    this.element.addEventListener('touchmove', (e) => {
      if (!this.startTouch) return;
      
      const touch = e.touches[0];
      this.currentTouch = { x: touch.clientX, y: touch.clientY };
      
      // 移動が検出されたらロングプレス判定をキャンセル
      if (longPressTimer) {
        clearTimeout(longPressTimer);
      }
    }, { passive: true });
    
    this.element.addEventListener('touchend', (e) => {
      if (!this.startTouch || !this.currentTouch) return;
      
      // ロングプレス判定をキャンセル
      if (longPressTimer) {
        clearTimeout(longPressTimer);
      }
      
      const deltaX = this.currentTouch.x - this.startTouch.x;
      const deltaY = this.currentTouch.y - this.startTouch.y;
      const distance = Math.sqrt(deltaX * deltaX + deltaY * deltaY);
      
      if (distance < this.minSwipeDistance) {
        // タップ判定
        if (this.options.onTap) {
          this.options.onTap(this.startTouch);
        }
      } else {
        // スワイプ判定
        const absDeltaX = Math.abs(deltaX);
        const absDeltaY = Math.abs(deltaY);
        
        let direction: 'left' | 'right' | 'up' | 'down';
        
        if (absDeltaX > absDeltaY) {
          direction = deltaX > 0 ? 'right' : 'left';
        } else {
          direction = deltaY > 0 ? 'down' : 'up';
        }
        
        if (this.options.onSwipe) {
          this.options.onSwipe({ direction, distance });
        }
      }
      
      this.startTouch = null;
      this.currentTouch = null;
    }, { passive: true });
  }
  
  destroy() {
    // イベントリスナーを削除
    this.element.removeEventListener('touchstart', () => {});
    this.element.removeEventListener('touchmove', () => {});
    this.element.removeEventListener('touchend', () => {});
  }
}

// スワイプ操作用のSvelteアクション
export function swipe(
  element: HTMLElement,
  options: {
    onSwipe?: (direction: SwipeDirection) => void;
    onTap?: (point: TouchPoint) => void;
    onLongPress?: (point: TouchPoint) => void;
  }
) {
  const handler = new TouchHandler(element, options);
  
  return {
    destroy() {
      handler.destroy();
    }
  };
}

// タッチフレンドリーなクリック処理
export function enhanceTouch(element: HTMLElement) {
  // タッチデバイスでのクリック遅延を防ぐ
  element.style.touchAction = 'manipulation';
  
  // タップ時のハイライト色を設定
  element.style.webkitTapHighlightColor = 'rgba(59, 130, 246, 0.1)';
  
  return {
    destroy() {
      element.style.touchAction = '';
      element.style.webkitTapHighlightColor = '';
    }
  };
}

// ビューポートの検出
export function getViewportInfo() {
  return {
    width: window.innerWidth,
    height: window.innerHeight,
    isMobile: window.innerWidth < 768,
    isTablet: window.innerWidth >= 768 && window.innerWidth < 1024,
    isDesktop: window.innerWidth >= 1024,
    isTouchDevice: 'ontouchstart' in window || navigator.maxTouchPoints > 0
  };
}

// スクロール位置の管理
export class ScrollManager {
  private scrollPositions = new Map<string, number>();
  
  saveScrollPosition(key: string) {
    this.scrollPositions.set(key, window.scrollY);
  }
  
  restoreScrollPosition(key: string) {
    const position = this.scrollPositions.get(key);
    if (position !== undefined) {
      window.scrollTo(0, position);
    }
  }
  
  clearScrollPosition(key: string) {
    this.scrollPositions.delete(key);
  }
}