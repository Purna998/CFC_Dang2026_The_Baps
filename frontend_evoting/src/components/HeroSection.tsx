'use client';

import Link from 'next/link';
import { ArrowRight, Lock, ShieldCheck, CheckCircle2, Zap } from 'lucide-react';
import { useLanguage } from '@/context/LanguageContext';

export default function HeroSection() {
  const { t } = useLanguage();

  return (
    <section className="relative bg-deep-navy text-white overflow-hidden">
      
      {/* Background Image with Dark Vignette Overlay */}
      <div className="absolute inset-0 z-0">
        <img
          src="https://images.unsplash.com/photo-1577962917302-cd874c4e31d2?auto=format&fit=crop&w=2000&q=80"
          alt="Nepalese Collaboration Online Voting"
          className="w-full h-full object-cover object-center opacity-30"
        />
        <div className="absolute inset-0 bg-gradient-to-r from-deep-navy via-deep-navy/90 to-deep-navy/70" />
      </div>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 relative z-10 pt-16 pb-20 lg:pt-24 lg:pb-28">
        <div className="max-w-2xl space-y-6">
          
          {/* Status Pill Badge */}
          <div className="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full bg-slate-900/80 border border-slate-700 text-[11px] font-bold text-slate-200 uppercase tracking-wider backdrop-blur-md">
            <span className="w-2 h-2 rounded-full bg-emerald-green animate-ping" />
            <span>{t('hero.status')}</span>
          </div>

          {/* Main Headline */}
          <h1 className="font-display font-extrabold text-4xl sm:text-5xl lg:text-6xl tracking-tight leading-tight">
            {t('hero.title1')} <br />
            <span className="text-secondary-crimson">
              {t('hero.title2')}
            </span>
          </h1>

          {/* Subtitle */}
          <p className="text-sm sm:text-base text-slate-300 leading-relaxed font-sans max-w-xl">
            {t('hero.subtitle')}
          </p>

          {/* Action Buttons */}
          <div className="flex flex-wrap items-center gap-4 pt-2">
            <Link
              href="/vote/fed-rep-2024"
              className="bg-secondary-crimson hover:bg-dark-red text-white font-display text-xs font-bold px-7 py-4 rounded-xl shadow-lg transition-colors flex items-center gap-2"
            >
              <span>{t('hero.start_btn')}</span>
              <ArrowRight className="w-4 h-4" />
            </Link>

            <Link
              href="/#features"
              className="bg-slate-900/90 hover:bg-slate-800 text-white font-display text-xs font-bold px-7 py-4 rounded-xl border border-slate-600 transition-colors"
            >
              {t('hero.works_btn')}
            </Link>
          </div>

          {/* Sovereign Trust Architecture Tag */}
          <div className="pt-4 text-[11px] font-mono font-bold text-slate-400 tracking-widest flex items-center gap-2">
            <span className="w-8 h-[2px] bg-secondary-crimson inline-block" />
            <span>SOVEREIGN TRUST ARCHITECTURE</span>
          </div>

        </div>
      </div>

      {/* Red Accent Strip across section bottom */}
      <div className="h-1 bg-secondary-crimson w-full" />

      {/* Bottom Ticker Bar */}
      <div className="bg-primary-dark text-white border-t border-slate-800 py-4 px-4">
        <div className="max-w-7xl mx-auto grid grid-cols-2 md:grid-cols-4 gap-4 text-center md:text-left text-xs font-bold tracking-wider">
          <div className="flex items-center justify-center md:justify-start gap-2 text-slate-200">
            <span className="w-2 h-2 rounded-full bg-secondary-crimson" />
            <span>Secure Authentication</span>
          </div>

          <div className="flex items-center justify-center md:justify-start gap-2 text-slate-200">
            <span className="w-2 h-2 rounded-full bg-secondary-crimson" />
            <span>Private Ballots</span>
          </div>

          <div className="flex items-center justify-center md:justify-start gap-2 text-slate-200">
            <span className="w-2 h-2 rounded-full bg-secondary-crimson" />
            <span>Controlled Access</span>
          </div>

          <div className="flex items-center justify-center md:justify-start gap-2 text-slate-200">
            <span className="w-2 h-2 rounded-full bg-secondary-crimson" />
            <span>Instant Tabulation</span>
          </div>
        </div>
      </div>

    </section>
  );
}
