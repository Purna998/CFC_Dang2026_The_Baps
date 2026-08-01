import type { Metadata } from 'next';
import './globals.css';
import Navbar from '@/components/Navbar';
import Footer from '@/components/Footer';
import { LanguageProvider } from '@/context/LanguageContext';

export const metadata: Metadata = {
  title: 'DIGIMAT | Secure Electoral Infrastructure of Nepal',
  description: 'Sovereign digital voting platform powered by Zero-Knowledge Cryptography, end-to-end verifiability, and client-side homomorphic encryption for Nepal federal and institutional elections.',
  keywords: ['DIGIMAT', 'Online Voting Nepal', 'E-voting System', 'Nepalese Elections', 'Cryptographic Voting', 'Zero Knowledge Voting'],
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className="light scroll-smooth">
      <body className="bg-surface-bright font-sans text-on-surface antialiased min-h-screen flex flex-col justify-between">
        <LanguageProvider>
          <Navbar />
          <main className="flex-grow">{children}</main>
          <Footer />
        </LanguageProvider>
      </body>
    </html>
  );
}
