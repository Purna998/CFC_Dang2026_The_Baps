'use client';

import { Lock, KeyRound, MonitorSmartphone } from 'lucide-react';

export default function CoreInfrastructureSection() {
  const features = [
    {
      title: 'Double Encryption',
      desc: 'Ballots are encrypted at the device and again during transmission to ensure absolute privacy.',
      icon: Lock
    },
    {
      title: 'Voter Verifiability',
      desc: 'Every voter receives a cryptographically signed receipt to verify their vote was recorded as intended.',
      icon: KeyRound
    },
    {
      title: 'Device Neutral',
      desc: 'Optimized for smartphones, tablets, and desktops to ensure high participation across all demographics.',
      icon: MonitorSmartphone
    }
  ];

  return (
    <section className="py-20 bg-surface-bright border-b border-border-gray">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        
        {/* Title */}
        <div className="text-center max-w-2xl mx-auto mb-14">
          <h2 className="font-display font-extrabold text-3xl sm:text-4xl text-deep-navy tracking-tight">
            Core Infrastructure Features
          </h2>
        </div>

        {/* 3 Cards */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          {features.map((feat, idx) => {
            const Icon = feat.icon;
            return (
              <div
                key={idx}
                className="bg-white border border-border-gray rounded-2xl p-8 space-y-4 shadow-sm hover:shadow-md transition-shadow"
              >
                <div className="w-10 h-10 rounded-xl bg-soft-blue-bg text-deep-navy flex items-center justify-center border border-slate-200">
                  <Icon className="w-5 h-5 text-deep-navy" />
                </div>

                <h3 className="font-display font-bold text-lg text-deep-navy">
                  {feat.title}
                </h3>

                <p className="text-xs text-slate-600 leading-relaxed font-sans">
                  {feat.desc}
                </p>
              </div>
            );
          })}
        </div>

      </div>
    </section>
  );
}
