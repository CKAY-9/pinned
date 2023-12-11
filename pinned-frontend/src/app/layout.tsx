import type { Metadata } from "next"
import "./globals.scss"

export const metadata: Metadata = {
  title: "Pinned",
  description: "A platform for creatives and creators to share their work.",
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>
        <div id="notifications_main" className="notifications"></div>
        {children}
      </body>
    </html>
  )
}
