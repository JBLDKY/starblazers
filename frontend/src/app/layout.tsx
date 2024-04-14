import type { Metadata } from "next";
import { Changa } from "next/font/google";
import "./globals.css";

const changa = Changa({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Starblaze.rs",
  description: "Light the stars ablaze together with your friends.",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={changa.className}>{children}</body>
    </html>
  );
}
