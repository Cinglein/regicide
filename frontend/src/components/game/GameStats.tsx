import { CardBack } from '@/components/shared/CardBack';

interface GameStatsProps {
  librarySize: number;
  discardSize: number;
  resolvingSize: number;
  onResolvingClick: () => void;
}

export function GameStats({
  librarySize,
  discardSize,
  resolvingSize,
  onResolvingClick,
}: GameStatsProps) {
  return (
    <div className="flex justify-center gap-4 flex-wrap">
      <CardBack count={librarySize} label="Library" />
      <CardBack count={discardSize} label="Discard" />
      <CardBack
        count={resolvingSize}
        label="Resolving"
        onClick={resolvingSize > 0 ? onResolvingClick : undefined}
      />
    </div>
  );
}
