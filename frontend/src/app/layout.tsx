import type { Metadata } from "next";
import { Pixelify_Sans } from "next/font/google";
import "./globals.css";

const pixelifySans = Pixelify_Sans({ subsets: ["latin"] });

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
      <body className={pixelifySans.className}>{children}</body>
    </html>
  );
}
