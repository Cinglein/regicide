import type { JsCard } from '@/bindings/JsCard';
import type { Phase } from '@/bindings/Phase';

export function isValidCombo(cards: JsCard[]): boolean {
  const len = cards.length;

  if (len === 0) return false;

  if (len === 1) return true;

  if (len === 2) {
    const [one, two] = cards;
    const v1 = one.value;
    const v2 = two.value;

    if ((v1 === 1 && v2 !== 0) || (v2 === 1 && v1 !== 0)) {
      return true;
    }

    if (v1 === v2 && v1 >= 2 && v1 <= 5) {
      return true;
    }

    return false;
  }

  if (len === 3) {
    const values = cards.map((c) => c.value);
    const allSame = values.every((v) => v === values[0]);
    return allSame && (values[0] === 2 || values[0] === 3);
  }

  if (len === 4) {
    const values = cards.map((c) => c.value);
    return values.every((v) => v === 2);
  }

  return false;
}

export function canPlayCombo(cards: JsCard[], phase: Phase, isMyTurn: boolean): boolean {
  if (!isMyTurn) return false;
  if (typeof phase === 'string') return false;
  if (!('Play' in phase)) return false;
  return isValidCombo(cards);
}

export function canDiscard(phase: Phase, isMyTurn: boolean): boolean {
  if (!isMyTurn) return false;
  if (typeof phase === 'string') return false;
  if (!('Defend' in phase)) return false;
  return true;
}
