# Design Variations: Low-Poly Valley Theme

Inspired by Monument Valley's low-poly aesthetic with a valley landscape forming the decorative backdrop.

## Core Concept

**Background**: Purely decorative low-poly valley landscape forming V-shape
- Sky/clouds at top
- Valley slopes descending on sides
- Valley ridges (foreground) at bottom

**UI Elements**: Float above background with proper contrast
- Light mode: Dark text (`#374151` gray-700) on light backgrounds
- Dark mode: Light text (`#F3F4F6` gray-100) on dark backgrounds

## Color Palettes

### Variation A: Classic Valley

**Light Mode Valley Background**
- Sky: `#87CEEB` → `#B0E0E6` (sky blue gradient)
- Clouds/Mist: `#E8F4F8` (light blue-gray, floating wisps)
- Valley Far Slopes: `#10B981` (emerald-500)
- Valley Mid Slopes: `#059669` (emerald-600)
- Valley Near Ridges: `#475569` (slate-600)
- Valley Foreground Ridges: `#334155` (slate-700)

**Light Mode UI Colors**
- Card Background: `#FFFFFF` (white)
- Panel Background: `#F8FAFC` (slate-50)
- Text: `#374151` (gray-700)
- Primary Button: `#10B981` (emerald-500) + white text
- Border: `#E2E8F0` (slate-200)

**Dark Mode Valley Background**
- Sky: `#1E3A5F` → `#2D5A7B` (deep to medium blue)
- Sunset Gradient: `#9333EA` (purple-600) → `#FB923C` (orange-400)
- Valley Far Slopes: `#64748B` (slate-500)
- Valley Mid Slopes: `#475569` (slate-600)
- Valley Near Ridges: `#334155` (slate-700)
- Valley Foreground Ridges: `#1E293B` (slate-800)

**Dark Mode UI Colors**
- Card Background: `#1E293B` (slate-800)
- Panel Background: `#0F172A` (slate-900)
- Text: `#F3F4F6` (gray-100)
- Primary Button: `#10B981` (emerald-500) + white text
- Border: `#334155` (slate-700)

### Variation B: Vibrant Valley

**Light Mode Valley Background**
- Sky: `#7DD3FC` → `#BAE6FD` (bright sky blue)
- Clouds: `#F0F9FF` (sky-50, floating)
- Valley Far Slopes: `#34D399` (emerald-400)
- Valley Mid Slopes: `#10B981` (emerald-500)
- Valley Near Ridges: `#64748B` (slate-500)
- Valley Foreground Ridges: `#475569` (slate-600)

**Light Mode UI Colors**
- Card Background: `#FFFFFF` (white)
- Panel Background: `#FAFAFA` (neutral-50)
- Text: `#374151` (gray-700)
- Primary Button: `#06B6D4` (cyan-500) + white text
- Border: `#D1D5DB` (gray-300)

**Dark Mode Valley Background**
- Sky: `#0C4A6E` → `#075985` (sky-900 to sky-800)
- Sunset: `#A855F7` → `#F59E0B` → `#FB923C` (purple to amber to orange)
- Valley Far Slopes: `#64748B` (slate-500)
- Valley Mid Slopes: `#475569` (slate-600)
- Valley Near Ridges: `#374151` (gray-700)
- Valley Foreground Ridges: `#1F2937` (gray-800)

**Dark Mode UI Colors**
- Card Background: `#1F2937` (gray-800)
- Panel Background: `#111827` (gray-900)
- Text: `#F3F4F6` (gray-100)
- Primary Button: `#06B6D4` (cyan-500) + white text
- Border: `#374151` (gray-700)

### Variation C: Soft Pastel Valley

**Light Mode Valley Background**
- Sky: `#DBEAFE` → `#E0F2FE` (blue-100 to sky-100)
- Clouds: `#F0FDFA` (teal-50, soft wisps)
- Valley Far Slopes: `#6EE7B7` (emerald-300)
- Valley Mid Slopes: `#34D399` (emerald-400)
- Valley Near Ridges: `#94A3B8` (slate-400)
- Valley Foreground Ridges: `#64748B` (slate-500)

**Light Mode UI Colors**
- Card Background: `#FFFFFF` (white)
- Panel Background: `#F8FAFC` (slate-50)
- Text: `#374151` (gray-700)
- Primary Button: `#14B8A6` (teal-500) + white text
- Border: `#CBD5E1` (slate-300)

**Dark Mode Valley Background**
- Sky: `#1E1B4B` → `#312E81` (indigo-950 to indigo-900)
- Sunset: `#C084FC` → `#FBBF24` (purple-400 to amber-400)
- Valley Far Slopes: `#6B7280` (gray-500)
- Valley Mid Slopes: `#4B5563` (gray-600)
- Valley Near Ridges: `#374151` (gray-700)
- Valley Foreground Ridges: `#111827` (gray-900)

**Dark Mode UI Colors**
- Card Background: `#1F2937` (gray-800)
- Panel Background: `#0F172A` (slate-900)
- Text: `#F3F4F6` (gray-100)
- Primary Button: `#14B8A6` (teal-500) + white text
- Border: `#374151` (gray-700)

### Variation D: Dramatic Sunset

**Light Mode Valley Background**
- Sky: `#93C5FD` → `#BFDBFE` (blue-300 to blue-200)
- Clouds: `#FFFFFF` (white wisps)
- Valley Far Slopes: `#22C55E` (green-500)
- Valley Mid Slopes: `#16A34A` (green-600)
- Valley Near Ridges: `#57534E` (stone-600)
- Valley Foreground Ridges: `#44403C` (stone-700)

**Light Mode UI Colors**
- Card Background: `#FAFAF9` (stone-50)
- Panel Background: `#F5F5F4` (stone-100)
- Text: `#292524` (stone-800)
- Primary Button: `#16A34A` (green-600) + white text
- Border: `#D6D3D1` (stone-300)

**Dark Mode Valley Background**
- Sky: `#18181B` → `#27272A` (zinc-900 to zinc-800)
- Sunset: `#7C3AED` → `#F97316` (violet-600 to orange-500)
- Valley Far Slopes: `#52525B` (zinc-600)
- Valley Mid Slopes: `#3F3F46` (zinc-700)
- Valley Near Ridges: `#27272A` (zinc-800)
- Valley Foreground Ridges: `#09090B` (zinc-950)

**Dark Mode UI Colors**
- Card Background: `#27272A` (zinc-800)
- Panel Background: `#18181B` (zinc-900)
- Text: `#FAFAFA` (zinc-50)
- Primary Button: `#8B5CF6` (violet-500) + white text
- Border: `#3F3F46` (zinc-700)

## Valley Implementation

### SVG Background (Recommended)

Layered polygons for clean low-poly aesthetic:

```svg
<svg class="absolute inset-0 -z-10 w-full h-full"
     viewBox="0 0 100 100"
     preserveAspectRatio="none">

  <!-- Sky gradient -->
  <defs>
    <linearGradient id="skyGradient" x1="0%" y1="0%" x2="0%" y2="100%">
      <stop offset="0%" class="[stop-color:#87CEEB]" />
      <stop offset="100%" class="[stop-color:#B0E0E6]" />
    </linearGradient>
  </defs>
  <rect width="100" height="100" fill="url(#skyGradient)" />

  <!-- Clouds (optional) -->
  <ellipse cx="20" cy="12" rx="12" ry="6" fill="#E8F4F8" opacity="0.7" />
  <ellipse cx="75" cy="18" rx="15" ry="7" fill="#E8F4F8" opacity="0.6" />

  <!-- Far valley slopes -->
  <polygon points="0,25 35,100 0,100" fill="#10B981" />
  <polygon points="100,25 65,100 100,100" fill="#10B981" />

  <!-- Mid valley slopes -->
  <polygon points="0,45 25,100 0,100" fill="#059669" />
  <polygon points="100,45 75,100 100,100" fill="#059669" />

  <!-- Near valley ridges -->
  <polygon points="0,65 18,100 0,100" fill="#475569" />
  <polygon points="100,65 82,100 100,100" fill="#475569" />

  <!-- Foreground valley ridges -->
  <polygon points="0,82 10,100 0,100" fill="#334155" />
  <polygon points="100,82 90,100 100,100" fill="#334155" />
</svg>
```

## UI Component Specifications

### Card Back Component (for Library, Discard, Resolving)

**Visual Design:**
```
┌─────────┐
│ ╔═════╗ │
│ ║     ║ │
│ ║ 24  ║ │  ← Number centered
│ ║     ║ │
│ ╚═════╝ │
└─────────┘
```

**Styling:**
- Light mode: `bg-white border-2 border-gray-300 text-gray-700`
- Dark mode: `bg-slate-800 border-2 border-slate-600 text-gray-100`
- Card back pattern: Subtle gradient or decorative border
- Number: Large, centered, bold font
- Size: Same as regular card
- Hover: Scale up slightly, cursor pointer (for clickable ones)

**Usage:**
- Library: Shows library count, not clickable
- Discard: Shows discard count, not clickable
- Resolving: Shows resolving count, clickable → opens modal

### Resolving Cards Modal

**When clicked:**
```
┌────────────────────────────────────┐
│  Resolving Cards           [×]     │
├────────────────────────────────────┤
│                                    │
│  Combo 1:                          │
│  ┌─────┐ ┌─────┐                  │
│  │ ♥ 5 │ │ ♦ 5 │                  │
│  └─────┘ └─────┘                  │
│                                    │
│  Combo 2:                          │
│  ┌─────┐ ┌─────┐ ┌─────┐          │
│  │ ♠ A │ │ ♠ 2 │ │ ♠ 3 │          │
│  └─────┘ └─────┘ └─────┘          │
│                                    │
└────────────────────────────────────┘
```

**Structure:**
- Modal overlay: Semi-transparent dark background
- Modal panel: Glass-morphism effect
  - Light: `bg-white/95 backdrop-blur-md`
  - Dark: `bg-slate-900/95 backdrop-blur-md`
- Each combo (Card[]) shown as a row of cards
- Close button (X) in top-right
- Click outside to close

### Card Component

**Regular Card (in hand, in modal):**
```
┌─────────┐
│ ♥       │
│         │
│    A    │  ← Rank centered
│         │
│       ♥ │
└─────────┘
```

**Styling:**
- Light mode: `bg-white border border-gray-300 text-gray-700`
- Dark mode: `bg-slate-800 border border-slate-600 text-gray-100`
- Selected: `ring-4 ring-emerald-500`
- Hover: `shadow-lg transform -translate-y-1`
- Suit colors:
  - Hearts/Diamonds: `text-red-600 dark:text-red-400`
  - Spades/Clubs: `text-gray-900 dark:text-gray-100`
  - Joker: `text-purple-600 dark:text-purple-400`

### Enemy Card Display

**Large, prominent card:**
```
┌───────────────┐
│   ♠           │
│               │
│      K        │  ← King, larger
│               │
│           ♠   │
├───────────────┤
│ Damage: 15/20 │  ← Damage bar
└───────────────┘
```

**Styling:**
- Larger than regular cards (1.5x)
- Shadow: `shadow-2xl`
- Elevated: `transform -translate-y-2`
- Damage bar below card:
  - Progress bar showing damage / (enemy_value * 2)
  - Light: `bg-gray-200` filled with `bg-emerald-500`
  - Dark: `bg-slate-700` filled with `bg-emerald-500`

### Game Stats Display

**Layout:**
```
┌─────────┐  ┌─────────┐
│ ╔═════╗ │  │ ╔═════╗ │
│ ║ 24  ║ │  │ ║  8  ║ │
│ ╚═════╝ │  │ ╚═════╝ │
│ Library │  │ Discard │
└─────────┘  └─────────┘
```

Both displayed as card backs with labels underneath.

### Player List

**Mobile (Vertical):**
```
┌──────────────────────┐
│ → Alice (You): 6 ♠   │ ← Arrow = current turn
│   Bob: 5 ♠           │   Card back icon
│   Carol: 7 ♠         │   Clickable in Jester phase
└──────────────────────┘
```

**Desktop (Vertical sidebar):**
```
┌────────────────┐
│ Players        │
├────────────────┤
│ → Alice (You)  │
│   ┌─────┐      │
│   │ ╔═╗ │      │
│   │ ║6║ │      │  ← Card back showing count
│   │ ╚═╝ │      │
│   └─────┘      │
│                │
│   Bob          │
│   ┌─────┐      │
│   │ ╔═╗ │      │
│   │ ║5║ │      │
│   │ ╚═╝ │      │
│   └─────┘      │
└────────────────┘
```

**Styling:**
- Light: `bg-white border border-gray-200`
- Dark: `bg-slate-800 border border-slate-700`
- Current turn: Highlighted with left border `border-l-4 border-emerald-500`
- Clickable (Jester): `hover:bg-gray-100 dark:hover:bg-slate-700 cursor-pointer`

### Panels & Containers

**Glass-morphism effect:**
```css
bg-white/90 dark:bg-slate-900/90
backdrop-blur-sm
border border-gray-200 dark:border-slate-700
shadow-xl
rounded-lg
```

### Buttons

**Primary Action:**
```css
bg-emerald-500 hover:bg-emerald-600
text-white font-semibold
px-6 py-2 rounded-lg
shadow-md hover:shadow-lg
transition-all
```

**Disabled:**
```css
bg-gray-300 dark:bg-slate-700
text-gray-500 dark:text-slate-500
cursor-not-allowed
```

## Layout Structures

### Mobile Layout (< 768px)

```
┌─────────────────────────┐
│    VALLEY BACKGROUND    │
│         (SKY)           │
│                         │
│    ┌─────────────┐      │
│    │   ENEMY     │      │  ← Floats above valley
│    │  Damage Bar │      │
│    └─────────────┘      │
│  [Library] [Discard]    │
│  [Resolving]            │  ← Card backs
│                         │
│  /                   \  │  ← Valley slopes (decorative)
│ /   ┌────────────┐    \ │
│/    │  Players   │     \│
│     │   List     │      │
│     └────────────┘      │
│                         │
│  ┌──────────────────┐   │  ← UI panels on top
│  │   Your Hand      │   │
│  │ [C] [C] [C] [C] │   │
│  └──────────────────┘   │
│  Phase: Your Turn       │
│  [Start] [Play] [Disc]  │
│  ● Connected            │
└─────────────────────────┘
```

### Desktop Layout (≥ 768px)

```
┌──────────────────────────────────────────┐
│         VALLEY BACKGROUND (SKY)          │
│   /                                  \   │
│  /  ┌────────┐  ┌──────────┐  ●     \  │
│ /   │Players │  │  ENEMY   │ Conn    \ │
│/    │        │  │ Damage   │          \│
│     │→ Alice │  └──────────┘           │
│     │  Bob   │  [Lib][Disc][Res]       │  ← Valley slopes frame
│     │  Carol │                          │     game board
│     └────────┘                          │
│                                         │
│  ┌───────────────────────────────────┐  │  ← UI on top of valley
│  │       Your Hand                   │  │
│  │  [Card] [Card] [Card] [Card]     │  │
│  └───────────────────────────────────┘  │
│  Phase: Your Turn - Play                │
│  [Start Game] [Play Cards] [Discard]    │
└──────────────────────────────────────────┘
```

## Reconnection UI

```
┌─────────────────────────────────────┐
│      VALLEY BACKGROUND              │
│                                     │
│    ┌──────────────────────┐        │
│    │   ⟳ Reconnecting...  │        │  ← Glass panel
│    │                      │        │
│    │   Attempt 3         │        │
│    │   Retrying in 4s... │        │
│    │                      │        │
│    │  [Reconnect Now]     │        │
│    │  [Cancel]            │        │
│    └──────────────────────┘        │
│                                     │
└─────────────────────────────────────┘
```

**Panel styling:**
- Light: `bg-white/95 backdrop-blur-md border border-gray-300`
- Dark: `bg-slate-900/95 backdrop-blur-md border border-slate-700`
- Manual reconnect button always available
- Shows current attempt number
- Shows countdown to next retry
- Cancel returns to landing page

## Responsive Breakpoints

```tsx
/* Mobile: Vertical stack */
className="flex flex-col md:hidden"

/* Desktop: Centered board with sidebar */
className="hidden md:grid md:grid-cols-[250px_1fr]"

/* Large desktop: More spacing */
className="lg:grid-cols-[300px_1fr] lg:gap-8"
```

## Final Design: Soft Pastel Valley

**Refined Variation C with Parchment UI**

### Light Mode Colors

**Valley Background (Decorative)**
- Sky: `#E0F2FE` → `#F0F9FF` (sky-200 to sky-50, soft gradient)
- Clouds: `#FAFAFA` (neutral-50, subtle wisps)
- Valley Far Slopes: `#86EFAC` (green-300)
- Valley Mid Slopes: `#6EE7B7` (emerald-300)
- Valley Near Ridges: `#CBD5E1` (slate-300)
- Valley Foreground Ridges: `#94A3B8` (slate-400)

**UI Components (Opaque)**
- Card Background: `#FFFBF0` (parchment/cream)
- Panel Background: `#FAF9F6` (warm gray-50)
- Text: `#374151` (gray-700)
- Secondary Text: `#6B7280` (gray-500)
- Primary Button: `#34D399` (emerald-400) + `#1F2937` (gray-800 text)
- Border: `#E5E7EB` (gray-200)
- Shadow: Soft, subtle shadows with low opacity

### Dark Mode Colors

**Valley Background (Decorative)**
- Sky: `#1E1B4B` → `#312E81` (indigo-950 to indigo-900)
- Sunset: `#DDD6FE` → `#FDE68A` (violet-200 to amber-200, soft glow)
- Valley Far Slopes: `#6B7280` (gray-500)
- Valley Mid Slopes: `#4B5563` (gray-600)
- Valley Near Ridges: `#374151` (gray-700)
- Valley Foreground Ridges: `#1F2937` (gray-800)

**UI Components (Opaque)**
- Card Background: `#1F2937` (gray-800)
- Panel Background: `#111827` (gray-900)
- Text: `#F3F4F6` (gray-100)
- Secondary Text: `#D1D5DB` (gray-300)
- Primary Button: `#6EE7B7` (emerald-300) + `#1F2937` (gray-800 text)
- Border: `#374151` (gray-700)
- Shadow: Soft, subtle shadows

## Component Styling (No Glass-Morphism)

### Cards

**Regular Card:**
```css
/* Light */
bg-[#FFFBF0]           /* Parchment */
border border-gray-200
text-gray-700
shadow-sm

/* Dark */
bg-gray-800
border border-gray-700
text-gray-100
shadow-sm

/* Selected state */
ring-2 ring-emerald-400
shadow-md

/* Hover */
shadow-md
transform -translate-y-0.5
```

### Panels & Containers

**Opaque, solid colors:**
```css
/* Light */
bg-[#FAF9F6]           /* Warm off-white */
border border-gray-200
shadow-lg
rounded-lg

/* Dark */
bg-gray-900
border border-gray-700
shadow-lg
rounded-lg
```

### Buttons

**Primary Action:**
```css
/* Light */
bg-emerald-400
hover:bg-emerald-500
text-gray-800
font-medium
px-6 py-2.5
rounded-lg
shadow-sm
hover:shadow-md

/* Dark */
bg-emerald-300
hover:bg-emerald-400
text-gray-900
font-medium
px-6 py-2.5
rounded-lg
shadow-sm
hover:shadow-md
```

**Disabled:**
```css
/* Light */
bg-gray-200
text-gray-400
cursor-not-allowed

/* Dark */
bg-gray-700
text-gray-500
cursor-not-allowed
```

### Modal

**Resolving Cards Modal (Opaque):**
```css
/* Overlay */
bg-black/40

/* Modal Panel Light */
bg-[#FAF9F6]
border-2 border-gray-300
shadow-2xl
rounded-xl
p-6

/* Modal Panel Dark */
bg-gray-900
border-2 border-gray-700
shadow-2xl
rounded-xl
p-6
```

### Card Backs

**Library/Discard/Resolving:**
```css
/* Light */
bg-[#FFFBF0]
border-2 border-gray-300
text-gray-700

/* Pattern/decorative border */
border-double border-4 border-emerald-300

/* Dark */
bg-gray-800
border-double border-4 border-emerald-400/50
text-gray-100
```

### Shadows

All shadows soft and subtle:
```css
shadow-sm    /* Subtle: 0 1px 2px rgba(0,0,0,0.05) */
shadow-md    /* Medium: 0 4px 6px rgba(0,0,0,0.07) */
shadow-lg    /* Large: 0 10px 15px rgba(0,0,0,0.1) */
shadow-xl    /* Extra: 0 20px 25px rgba(0,0,0,0.1) */
shadow-2xl   /* Modal: 0 25px 50px rgba(0,0,0,0.15) */
```

## Implementation Details

**Parchment Color:**
- Light mode cards/inputs: `#FFFBF0` (warm cream)
- Light mode panels: `#FAF9F6` (warm off-white)
- Borders: `#E5E7EB` (subtle gray)

**Opaque Components:**
- No `backdrop-blur`
- No transparency (`/90`, `/80`, etc.)
- All components fully opaque with solid backgrounds
- Layering achieved through subtle shadows only

**Soft Aesthetic:**
- Rounded corners: `rounded-lg` (0.5rem)
- Gentle shadows
- Pastel accent colors (emerald-300/400)
- Muted text colors (not pure black/white)

## Next Steps

1. Confirm parchment colors and pastel palette
2. Proceed with implementation plan and filesystem structure
3. Build component library with opaque, soft styling
