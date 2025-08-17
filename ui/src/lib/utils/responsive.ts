import { writable } from 'svelte/store';

// Responsive breakpoints following Tailwind CSS
export const breakpoints = {
  sm: 640,
  md: 768,
  lg: 1024,
  xl: 1280,
  '2xl': 1536
} as const;

export type Breakpoint = keyof typeof breakpoints;

// Store for current screen size
export const screenSize = writable({
  width: 0,
  height: 0,
  isMobile: true,
  isTablet: false,
  isDesktop: false
});

// Utility function to check if screen is at or above a breakpoint
export function isBreakpoint(bp: Breakpoint, width?: number): boolean {
  const screenWidth = width ?? (typeof window !== 'undefined' ? window.innerWidth : 0);
  return screenWidth >= breakpoints[bp];
}

// Utility function to get current breakpoint
export function getCurrentBreakpoint(width?: number): Breakpoint {
  const screenWidth = width ?? (typeof window !== 'undefined' ? window.innerWidth : 0);
  
  if (screenWidth >= breakpoints['2xl']) return '2xl';
  if (screenWidth >= breakpoints.xl) return 'xl';
  if (screenWidth >= breakpoints.lg) return 'lg';
  if (screenWidth >= breakpoints.md) return 'md';
  if (screenWidth >= breakpoints.sm) return 'sm';
  
  return 'sm'; // Default to smallest breakpoint
}

// Initialize responsive utilities
export function initResponsive() {
  if (typeof window === 'undefined') return;
  
  const updateScreenSize = () => {
    const width = window.innerWidth;
    const height = window.innerHeight;
    
    screenSize.set({
      width,
      height,
      isMobile: width < breakpoints.md,
      isTablet: width >= breakpoints.md && width < breakpoints.lg,
      isDesktop: width >= breakpoints.lg
    });
  };
  
  // Initial update
  updateScreenSize();
  
  // Listen for resize events
  window.addEventListener('resize', updateScreenSize);
  
  // Return cleanup function
  return () => {
    window.removeEventListener('resize', updateScreenSize);
  };
}

// Responsive design utilities
export const responsive = {
  isBreakpoint,
  getCurrentBreakpoint,
  breakpoints,
  
  // Quick checks
  isMobile: (width?: number) => !isBreakpoint('md', width),
  isTablet: (width?: number) => isBreakpoint('md', width) && !isBreakpoint('lg', width),
  isDesktop: (width?: number) => isBreakpoint('lg', width),
  
  // Classes helper for conditional styling
  classes: {
    showOnMobile: 'block md:hidden',
    hideOnMobile: 'hidden md:block',
    showOnTablet: 'hidden md:block lg:hidden',
    hideOnTablet: 'block md:hidden lg:block',
    showOnDesktop: 'hidden lg:block',
    hideOnDesktop: 'block lg:hidden'
  }
};