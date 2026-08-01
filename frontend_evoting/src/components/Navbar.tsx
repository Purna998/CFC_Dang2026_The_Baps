'use client';

import { useState } from 'react';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import Image from 'next/image';
import { ChevronDown, Menu, X } from 'lucide-react';
import { useLanguage } from '@/context/LanguageContext';

export default function Navbar() {
  const [categoriesOpen, setCategoriesOpen] = useState(false);
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
  const { language, setLanguage, t, tx } = useLanguage();
  const pathname = usePathname();

  const isActive = (path: string) => pathname === path;

  return (
    <header className="sticky top-0 z-50 bg-white border-b border-border-gray shadow-sm">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between items-center h-20">
          
          {/* Brand Logo */}
          <Link href="/" className="flex items-center gap-2.5">
            <Image 
              src="/logo.png" 
              alt="DIGI-मत Logo" 
              width={240} 
              height={70} 
              className="h-14 sm:h-[58px] w-auto object-contain py-1" 
              priority 
            />
          </Link>

          {/* Navigation Links */}
          <nav className="hidden md:flex items-center gap-8 font-display text-xs font-bold uppercase tracking-wider">
            <Link
              href="/"
              className={`py-2 border-b-2 transition-colors ${
                isActive('/') 
                  ? 'text-deep-navy border-secondary-crimson' 
                  : 'text-slate-600 border-transparent hover:text-deep-navy'
              }`}
            >
              {t('nav.home')}
            </Link>

            <Link
              href="/#features"
              className="text-slate-600 border-b-2 border-transparent hover:text-deep-navy transition-colors py-2"
            >
              {t('nav.features')}
            </Link>

            {/* Categories Dropdown */}
            <div 
              className="relative"
              onMouseEnter={() => setCategoriesOpen(true)}
              onMouseLeave={() => setCategoriesOpen(false)}
            >
              <Link
                href="/categories"
                className={`flex items-center gap-1 py-2 border-b-2 transition-colors ${
                  isActive('/categories') 
                    ? 'text-deep-navy border-secondary-crimson' 
                    : 'text-slate-600 border-transparent hover:text-deep-navy'
                }`}
              >
                {t('nav.categories')}
                <ChevronDown className="w-3.5 h-3.5" />
              </Link>

              {categoriesOpen && (
                <div className="absolute left-0 top-full w-64 bg-white rounded-xl shadow-xl border border-border-gray py-2 z-50 normal-case tracking-normal">
                  <Link href="/categories#gov" className="block px-4 py-2 text-xs text-slate-700 hover:bg-slate-50 hover:text-deep-navy">
                    {tx('Governmental & Municipal', 'सरकारी तथा स्थानीय')}
                  </Link>
                  <Link href="/categories#prof" className="block px-4 py-2 text-xs text-slate-700 hover:bg-slate-50 hover:text-deep-navy">
                    {tx('Professional Associations (NEA, NBA)', 'व्यावसायिक संघ (एनइए, एनबिए)')}
                  </Link>
                  <Link href="/categories#edu" className="block px-4 py-2 text-xs text-slate-700 hover:bg-slate-50 hover:text-deep-navy">
                    {tx('Education & Student Unions', 'शिक्षा तथा विद्यार्थी युनियन')}
                  </Link>
                  <Link href="/categories#coop" className="block px-4 py-2 text-xs text-slate-700 hover:bg-slate-50 hover:text-deep-navy">
                    {tx('Cooperatives & Credit Unions', 'सहकारी तथा बचत समूह')}
                  </Link>
                </div>
              )}
            </div>

            <Link
              href="/results"
              className={`py-2 border-b-2 transition-colors ${
                isActive('/results') 
                  ? 'text-deep-navy border-secondary-crimson' 
                  : 'text-slate-600 border-transparent hover:text-deep-navy'
              }`}
            >
              {t('nav.results')}
            </Link>

            <Link
              href="/security"
              className={`py-2 border-b-2 transition-colors ${
                isActive('/security') 
                  ? 'text-deep-navy border-secondary-crimson' 
                  : 'text-slate-600 border-transparent hover:text-deep-navy'
              }`}
            >
              {t('nav.help')}
            </Link>
          </nav>

          {/* Right Action Buttons */}
          <div className="hidden sm:flex items-center gap-4">
            
            {/* Language Switcher */}
            <div className="text-xs font-bold text-slate-600 flex items-center gap-1.5 cursor-pointer select-none border border-slate-200 bg-slate-50 px-2.5 py-1.5 rounded-lg shadow-sm">
              <span 
                onClick={() => setLanguage('EN')} 
                className={`transition-colors ${language === 'EN' ? 'text-deep-navy font-extrabold underline' : 'hover:text-deep-navy text-slate-500'}`}
              >
                EN
              </span>
              <span className="text-slate-300">|</span>
              <span 
                onClick={() => setLanguage('NE')} 
                className={`transition-colors ${language === 'NE' ? 'text-deep-navy font-extrabold underline' : 'hover:text-deep-navy text-slate-500'}`}
              >
                नेपाली
              </span>
            </div>

            {/* Sign In Button */}
            <Link
              href="/login"
              className="text-xs font-bold text-deep-navy border border-slate-300 hover:bg-slate-50 px-4 py-2.5 rounded-lg transition-colors"
            >
              {t('nav.signin')}
            </Link>

            {/* Request Demo Button */}
            <Link
              href="/vote/fed-rep-2024"
              className="bg-deep-navy hover:bg-slate-800 text-white text-xs font-bold px-4 py-2.5 rounded-lg uppercase tracking-wider transition-colors"
            >
              {t('nav.demo')}
            </Link>
          </div>

          {/* Mobile Menu Button */}
          <div className="md:hidden flex items-center gap-3">
            {/* Mobile Language Switcher */}
            <div className="text-xs font-bold text-slate-600 flex items-center gap-1 cursor-pointer select-none bg-slate-100 px-2 py-1 rounded">
              <span 
                onClick={() => setLanguage('EN')} 
                className={language === 'EN' ? 'text-deep-navy font-extrabold underline' : 'text-slate-500'}
              >
                EN
              </span>
              <span>|</span>
              <span 
                onClick={() => setLanguage('NE')} 
                className={language === 'NE' ? 'text-deep-navy font-extrabold underline' : 'text-slate-500'}
              >
                नेपाली
              </span>
            </div>

            <button
              onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
              className="p-2 text-slate-700 hover:bg-slate-100 rounded-lg"
            >
              {mobileMenuOpen ? <X className="w-6 h-6" /> : <Menu className="w-6 h-6" />}
            </button>
          </div>

        </div>
      </div>

      {/* Mobile Drawer */}
      {mobileMenuOpen && (
        <div className="md:hidden bg-white border-b border-border-gray px-4 py-4 space-y-3 font-display text-xs font-bold uppercase">
          <Link href="/" onClick={() => setMobileMenuOpen(false)} className="block py-2 text-deep-navy">{t('nav.home')}</Link>
          <Link href="/dashboard" onClick={() => setMobileMenuOpen(false)} className="block py-2 text-slate-600">{t('nav.dashboard')}</Link>
          <Link href="/categories" onClick={() => setMobileMenuOpen(false)} className="block py-2 text-slate-600">{t('nav.categories')}</Link>
          <Link href="/results" onClick={() => setMobileMenuOpen(false)} className="block py-2 text-slate-600">{t('nav.results')}</Link>
          <Link href="/security" onClick={() => setMobileMenuOpen(false)} className="block py-2 text-slate-600">{t('nav.help')}</Link>
          <div className="pt-2 flex flex-col gap-2">
            <Link href="/login" onClick={() => setMobileMenuOpen(false)} className="w-full text-center py-2.5 border border-slate-300 text-deep-navy rounded-lg">
              {t('nav.signin')}
            </Link>
            <Link href="/vote/fed-rep-2024" onClick={() => setMobileMenuOpen(false)} className="w-full text-center py-2.5 bg-deep-navy text-white rounded-lg">
              {t('nav.demo')}
            </Link>
          </div>
        </div>
      )}
    </header>
  );
}
