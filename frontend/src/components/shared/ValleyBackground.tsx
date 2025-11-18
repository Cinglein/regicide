'use client';

export function ValleyBackground() {
  return (
    <svg
      className="absolute inset-0 -z-10 h-full w-full"
      viewBox="0 0 100 100"
      preserveAspectRatio="none"
      aria-hidden="true"
    >
      <defs>
        <linearGradient id="skyGradient" x1="0%" y1="0%" x2="0%" y2="100%">
          <stop offset="0%" className="[stop-color:#E0F2FE] dark:[stop-color:#1E1B4B]" />
          <stop offset="100%" className="[stop-color:#F0F9FF] dark:[stop-color:#312E81]" />
        </linearGradient>
        <linearGradient id="sunsetGradient" x1="0%" y1="0%" x2="100%" y2="0%">
          <stop offset="0%" className="dark:[stop-color:#DDD6FE]" stopOpacity="0" />
          <stop offset="50%" className="dark:[stop-color:#FDE68A]" stopOpacity="0.3" />
          <stop offset="100%" className="dark:[stop-color:#DDD6FE]" stopOpacity="0" />
        </linearGradient>
      </defs>

      <rect width="100" height="100" fill="url(#skyGradient)" />

      <rect width="100" height="30" y="10" fill="url(#sunsetGradient)" className="opacity-0 dark:opacity-100" />

      <ellipse
        cx="20"
        cy="12"
        rx="12"
        ry="6"
        className="fill-[#FAFAFA] opacity-60 dark:opacity-0"
      />
      <ellipse
        cx="75"
        cy="18"
        rx="15"
        ry="7"
        className="fill-[#FAFAFA] opacity-50 dark:opacity-0"
      />

      <polygon
        points="0,25 35,100 0,100"
        className="fill-[#86EFAC] dark:fill-[#6B7280]"
      />
      <polygon
        points="100,25 65,100 100,100"
        className="fill-[#86EFAC] dark:fill-[#6B7280]"
      />

      <polygon
        points="0,45 25,100 0,100"
        className="fill-[#6EE7B7] dark:fill-[#4B5563]"
      />
      <polygon
        points="100,45 75,100 100,100"
        className="fill-[#6EE7B7] dark:fill-[#4B5563]"
      />

      <polygon
        points="0,65 18,100 0,100"
        className="fill-[#CBD5E1] dark:fill-[#374151]"
      />
      <polygon
        points="100,65 82,100 100,100"
        className="fill-[#CBD5E1] dark:fill-[#374151]"
      />

      <polygon
        points="0,82 10,100 0,100"
        className="fill-[#94A3B8] dark:fill-[#1F2937]"
      />
      <polygon
        points="100,82 90,100 100,100"
        className="fill-[#94A3B8] dark:fill-[#1F2937]"
      />
    </svg>
  );
}
