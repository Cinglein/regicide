'use client';

import { Geist, Geist_Mono } from 'next/font/google';
import './globals.css';
import { ClientProvider } from '@/contexts/ClientContext';
import { GameProvider, useGame } from '@/contexts/GameContext';
import { WebSocketProvider } from '@/contexts/WebSocketContext';
import type { ServerMsg } from '@/bindings/ServerMsg';

const geistSans = Geist({
  variable: '--font-geist-sans',
  subsets: ['latin'],
});

const geistMono = Geist_Mono({
  variable: '--font-geist-mono',
  subsets: ['latin'],
});

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <head>
        <title>Regicide</title>
        <meta name="description" content="A cooperative card game" />
      </head>
      <body className={`${geistSans.variable} ${geistMono.variable} antialiased`}>
        <ClientProvider>
          <GameProvider>
            <WebSocketProviderWrapper>{children}</WebSocketProviderWrapper>
          </GameProvider>
        </ClientProvider>
      </body>
    </html>
  );
}

function WebSocketProviderWrapper({ children }: { children: React.ReactNode }) {
  const { setGameState } = useGame();

  const handleMessage = (msg: ServerMsg) => {
    setGameState(msg);
  };

  return <WebSocketProvider onMessage={handleMessage}>{children}</WebSocketProvider>;
}
