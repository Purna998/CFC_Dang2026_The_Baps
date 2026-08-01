'use client';

import { PlusCircle, ShieldCheck, Vote, CheckCircle2 } from 'lucide-react';

export default function WorkflowSection() {
  const steps = [
    {
      title: 'Create',
      desc: 'Define candidates, set ballot parameters, and configure voting dates.',
      icon: PlusCircle,
    },
    {
      title: 'Authenticate',
      desc: 'Voters are verified via citizenship data or organizational ID tokens.',
      icon: ShieldCheck,
    },
    {
      title: 'Vote',
      desc: 'Cast ballots through a simple, end-to-end encrypted interface.',
      icon: Vote,
    },
    {
      title: 'Publish',
      desc: 'Automated results calculation with full cryptographic audit trails.',
      icon: CheckCircle2,
    }
  ];

  return (
    <section id="features" className="py-20 bg-surface-bright border-b border-border-gray">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        
        {/* Section Header */}
        <div className="text-center max-w-2xl mx-auto space-y-3 mb-16">
          <h2 className="font-display font-extrabold text-3xl sm:text-4xl text-deep-navy tracking-tight">
            Seamless 4–Step Workflow
          </h2>
          <p className="text-xs sm:text-sm text-slate-600 font-sans">
            Digitizing democracy doesn't have to be complex. Our workflow is designed for institutional clarity and absolute security.
          </p>
        </div>

        {/* Steps Grid */}
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-6">
          {steps.map((step, idx) => {
            const Icon = step.icon;
            return (
              <div
                key={idx}
                className="bg-white border border-border-gray rounded-2xl p-8 text-center space-y-4 shadow-sm hover:shadow-md transition-shadow"
              >
                <div className="w-12 h-12 rounded-xl bg-soft-blue-bg text-deep-navy flex items-center justify-center mx-auto border border-slate-200">
                  <Icon className="w-6 h-6 text-deep-navy" />
                </div>

                <h3 className="font-display font-bold text-lg text-deep-navy">
                  {step.title}
                </h3>

                <p className="text-xs text-slate-600 leading-relaxed font-sans">
                  {step.desc}
                </p>
              </div>
            );
          })}
        </div>

      </div>
    </section>
  );
}
