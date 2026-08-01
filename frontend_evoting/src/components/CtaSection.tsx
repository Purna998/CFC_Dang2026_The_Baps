'use client';

import Link from 'next/link';
import { useLanguage } from '@/context/LanguageContext';

export default function CtaSection() {
  const { t, tx } = useLanguage();

  return (
    <section className="py-24 bg-surface-bright border-b border-border-gray text-center">
      <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 space-y-6">
        
        <h2 className="font-display font-extrabold text-3xl sm:text-4xl text-deep-navy tracking-tight">
          {t('cta.title')}
        </h2>

        <p className="text-xs sm:text-sm text-slate-600 font-sans max-w-xl mx-auto">
          {t('cta.subtitle')}
        </p>

        {/* Buttons */}
        <div className="flex flex-wrap items-center justify-center gap-4 pt-2">
          <Link
            href="/vote/fed-rep-2024"
            className="bg-deep-navy hover:bg-slate-800 text-white font-display text-xs font-bold px-8 py-3.5 rounded-xl shadow transition-colors"
          >
            {t('cta.start')}
          </Link>

          <Link
            href="/security"
            className="bg-white hover:bg-slate-50 text-deep-navy border border-slate-300 font-display text-xs font-bold px-8 py-3.5 rounded-xl transition-colors"
          >
            {t('cta.demo')}
          </Link>
        </div>

        <p className="text-[11px] text-slate-400 font-sans pt-2">
          {tx('Trusted by the Election Commission of Nepal for research & development.', 'अनुसन्धान र विकासका लागि नेपाल निर्वाचन आयोगद्वारा समर्थित।')}
        </p>

      </div>
    </section>
  );
}
