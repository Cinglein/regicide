import type { Suit } from '@/bindings/Suit';

export function getSuitSymbol(suit: Suit | null): string {
  if (suit === null) return '🃏';

  switch (suit) {
    case 'Heart':
      return '♥';
    case 'Spade':
      return '♠';
    case 'Diamond':
      return '♦';
    case 'Club':
      return '♣';
  }
}

export function getSuitColor(suit: Suit | null): string {
  if (suit === null) return 'text-purple-600 dark:text-purple-400';

  switch (suit) {
    case 'Heart':
    case 'Diamond':
      return 'text-red-600 dark:text-red-400';
    case 'Spade':
    case 'Club':
      return 'text-gray-900 dark:text-gray-100';
  }
}
