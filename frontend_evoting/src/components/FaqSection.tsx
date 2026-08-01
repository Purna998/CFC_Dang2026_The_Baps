'use client';

import { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { FAQS } from '@/lib/data';
import { ChevronDown, HelpCircle, ArrowRight, ShieldCheck } from 'lucide-react';
import Link from 'next/link';

export default function FaqSection() {
  const [openIdx, setOpenIdx] = useState<number | null>(0);

  const toggleFaq = (idx: number) => {
    setOpenIdx(openIdx === idx ? null : idx);
  };

  return (
    <section className="py-24 bg-white border-b border-border-gray">
      <div className="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 space-y-16">
        
        {/* Header */}
        <div className="text-center space-y-3">
          <span className="text-xs font-extrabold uppercase tracking-widest text-secondary-crimson bg-secondary-crimson/10 px-3.5 py-1 rounded-full">
            TRANSPARENCY & GUIDANCE
          </span>
          <h2 className="font-display font-extrabold text-3xl sm:text-4xl text-deep-navy tracking-tight">
            Frequently Asked Questions
          </h2>
          <p className="text-sm text-slate-600">
            Learn more about DIGIMAT security, verifiability, and compliance standards.
          </p>
        </div>

        {/* Accordion */}
        <div className="space-y-4">
          {FAQS.map((faq, idx) => {
            const isOpen = openIdx === idx;
            return (
              <div
                key={idx}
                className="bg-surface-bright border border-border-gray rounded-2xl overflow-hidden transition-colors"
              >
                <button
                  onClick={() => toggleFaq(idx)}
                  className="w-full text-left p-6 flex items-center justify-between gap-4 font-display font-bold text-base text-deep-navy hover:text-secondary-crimson transition-colors"
                >
                  <span className="flex items-center gap-3">
                    <HelpCircle className="w-5 h-5 text-secondary-crimson shrink-0" />
                    <span>{faq.q}</span>
                  </span>
                  <ChevronDown className={`w-5 h-5 text-slate-400 transition-transform duration-300 ${isOpen ? 'rotate-180 text-secondary-crimson' : ''}`} />
                </button>

                <AnimatePresence>
                  {isOpen && (
                    <motion.div
                      initial={{ height: 0, opacity: 0 }}
                      animate={{ height: 'auto', opacity: 1 }}
                      exit={{ height: 0, opacity: 0 }}
                      transition={{ duration: 0.25 }}
                    >
                      <div className="px-6 pb-6 pt-0 text-xs sm:text-sm text-slate-600 leading-relaxed font-sans border-t border-slate-200/60 pt-4">
                        {faq.a}
                      </div>
                    </motion.div>
                  )}
                </AnimatePresence>
              </div>
            );
          })}
        </div>

        {/* Ready to Modernize CTA Banner */}
        <div className="bg-deep-navy text-white rounded-3xl p-8 sm:p-10 text-center space-y-6 shadow-2xl pattern-overlay relative overflow-hidden">
          <div className="relative z-10 space-y-3 max-w-xl mx-auto">
            <ShieldCheck className="w-10 h-10 text-emerald-green mx-auto" />
            <h3 className="font-display font-extrabold text-2xl sm:text-3xl text-white">
              Ready to Modernize Your Electoral Process?
            </h3>
            <p className="text-xs sm:text-sm text-slate-300">
              Deploy secure online voting for your government agency, professional association, cooperative, or enterprise today.
            </p>

            <div className="flex flex-wrap items-center justify-center gap-4 pt-4">
              <Link
                href="/vote/fed-rep-2024"
                className="bg-secondary-crimson hover:bg-dark-red text-white font-display text-xs font-bold px-6 py-3.5 rounded-xl shadow-lg transition-all flex items-center gap-2"
              >
                <span>Try Live Demo Voting Booth</span>
                <ArrowRight className="w-4 h-4" />
              </Link>
              
              <Link
                href="/security"
                className="bg-slate-800 hover:bg-slate-700 text-white font-display text-xs font-bold px-6 py-3.5 rounded-xl border border-slate-700 transition-all"
              >
                <span>Download Technical Audit Whitepaper</span>
              </Link>
            </div>
          </div>
        </div>

      </div>
    </section>
  );
}
