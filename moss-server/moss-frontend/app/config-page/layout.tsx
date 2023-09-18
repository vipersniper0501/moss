import '../styles/globals.scss'
import type { Metadata } from 'next'
import { Inter } from 'next/font/google'
import {AdminNavbar} from '../components/navbar'

const inter = Inter({ subsets: ['latin'] })

export const metadata: Metadata = {
  title: 'Moss Dashboard',
  description: 'Dashboard for the Moss program.',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
      <div>
          <AdminNavbar />
          {children}
      </div>
  )
}
