'use client';

import { 
  Shield, 
  Lock, 
  UserCheck, 
  FileText, 
  Activity, 
  EyeOff, 
  CheckCircle2, 
  FileCheck2
} from 'lucide-react';
import { motion } from 'framer-motion';
import { useLanguage } from '@/context/LanguageContext';

export default function SecurityTrustSection() {
  const { t, tx } = useLanguage();

  const features = [
    {
      id: 'feature-1',
      title: tx('Protected Data Transmission', 'सुरक्षित डेटा ट्रान्समिसन'),
      description: tx(
        'Voting sessions and sensitive information are transmitted through secure connections designed to reduce interception and unauthorized access.',
        'मतदान सेसन र संवेदनशील सूचनाहरू अनधिकृत पहुँच र हस्तक्षेप रोक्नका लागि डिजाइन गरिएका सुरक्षित च्यानलहरू मार्फत पठाउने गरिन्छ।'
      ),
      label: tx('Secure communication', 'सुरक्षित सञ्चार'),
      icon: Shield,
    },
    {
      id: 'feature-2',
      title: tx('Controlled System Access', 'नियन्त्रित प्रणाली पहुँच'),
      description: tx(
        'Role-based permissions separate voter, election officer, auditor, and administrator responsibilities across the platform.',
        'भूमिकामा आधारित अनुमतिहरूले प्लेटफर्मभर मतदाता, निर्वाचन अधिकारी, लेखापरीक्षक र प्रशासकको जिम्मेवारी अलग गर्दछ।'
      ),
      label: tx('Role-based authorization', 'भूमिकामा आधारित अनुमति'),
      icon: UserCheck,
    },
    {
      id: 'feature-3',
      title: tx('Traceable Administrative Activity', 'अनुगमनयोग्य प्रशासकीय गतिविधि'),
      description: tx(
        'Important administrative actions can be recorded to support accountability, investigation, and authorized post-election review.',
        'जवाफदेही, अनुसन्धान र निर्वाचन पश्चात्को आधिकारिक समीक्षाका लागि महत्त्वपूर्ण प्रशासकीय कार्यहरू अभिलेख गरिन्छ।'
      ),
      label: tx('Audit visibility', 'अडिट दृश्यता'),
      icon: FileText,
    },
    {
      id: 'feature-4',
      title: tx('Operational Reliability', 'सञ्चालन विश्वसनीयता'),
      description: tx(
        'Monitoring, backups, recovery procedures, and controlled deployment practices help maintain system availability during an election.',
        'निगरानी, ब्याकअप, रिकभरी प्रक्रिया र नियन्त्रित प्रणाली अभ्यासहरूले निर्वाचनको समयमा प्रणालीको उपलब्धता कायम राख्न मद्दत गर्दछ।'
      ),
      label: tx('Continuity planning', 'निरन्तरता योजना'),
      icon: Activity,
    },
  ];

  const workflowSteps = [
    {
      step: '1',
      title: tx('Voter Authentication', 'मतदाता प्रमाणीकरण'),
      desc: tx('Eligible voters securely access the correct election.', 'योग्य मतदाताहरूले सही निर्वाचनमा सुरक्षित पहुँच प्राप्त गर्छन्।'),
      icon: UserCheck,
      status: tx('Verified', 'प्रमाणित'),
    },
    {
      step: '2',
      title: tx('Ballot Privacy', 'मतपत्र गोपनीयता'),
      desc: tx('Voter identity and submitted ballot information are handled through separated responsibilities.', 'मतदाताको पहिचान र पेश गरिएको मतपत्र अलग जिम्मेवारी मार्फत व्यवस्थापन गरिन्छ।'),
      icon: EyeOff,
      status: tx('Protected', 'संरक्षित'),
    },
    {
      step: '3',
      title: tx('Submission Control', 'पेश गर्ने नियन्त्रण'),
      desc: tx('Election rules prevent unauthorized or duplicate submissions.', 'निर्वाचन नियमहरूले अनधिकृत वा दोहोरो मत पेश गर्नबाट रोक्छन्।'),
      icon: CheckCircle2,
      status: tx('Enforced', 'लागू गरिएको'),
    },
    {
      step: '4',
      title: tx('Verification & Audit', 'प्रमाणीकरण तथा अडिट'),
      desc: tx('Receipts, system records, and authorized logs support election review.', 'रसीद, प्रणाली अभिलेख र आधिकारिक लगहरूले निर्वाचन समीक्षालाई सहयोग गर्छन्।'),
      icon: FileCheck2,
      status: tx('Auditable', 'परीक्षणयोग्य'),
    },
  ];

  return (
    <section className="py-20 lg:py-24 bg-white text-slate-900 border-b border-border-gray relative">
      <div className="max-w-[1240px] mx-auto px-4 sm:px-6 lg:px-8 relative z-10 space-y-12">
        
        {/* Header Section */}
        <div className="space-y-4 max-w-3xl">
          <div className="inline-flex items-center gap-2 text-xs font-bold uppercase tracking-widest text-deep-navy bg-slate-100 border border-slate-200 px-3.5 py-1.5 rounded-full shadow-2xs">
            <span className="w-2 h-2 rounded-full bg-secondary-crimson" />
            <span>{tx('SECURITY & TRUST', 'सुरक्षा र विश्वास')}</span>
          </div>

          <h2 className="font-display font-extrabold text-3xl sm:text-4xl lg:text-[48px] text-deep-navy tracking-tight leading-[1.1]">
            {tx('Security Designed Around Every Vote', 'हरेक मतको सुरक्षाका लागि निर्मित संरचना')}
          </h2>

          <p className="text-base sm:text-lg text-slate-600 leading-relaxed font-sans">
            {tx(
              'DIGIMAT combines responsible access controls, ballot privacy, audit visibility, and dependable system operations to help organizations conduct trustworthy digital elections.',
              'DIGIMAT ले जिम्मेवार पहुँच नियन्त्रण, मतपत्र गोपनीयता, अडिट दृश्यता र भरपर्दो प्रणाली सञ्चालनलाई संयोजन गरी संस्थाहरूलाई विश्वसनीय डिजिटल निर्वाचन सञ्चालन गर्न मद्दत गर्दछ।'
            )}
          </p>

          <p className="text-xs sm:text-sm text-slate-500 font-medium italic pt-1 border-l-2 border-secondary-crimson pl-3">
            {tx(
              'Security capabilities must always match the deployed infrastructure, configured election rules, and completed technical audits.',
              'सुरक्षा क्षमताहरू सधैं लागू गरिएको पूर्वाधार, तय गरिएका निर्वाचन नियमहरू र पूरा भएका प्राविधिक अडिटहरूसँग मेल खानुपर्छ।'
            )}
          </p>
        </div>

        {/* Desktop Layout: 2 Columns */}
        <div className="grid grid-cols-1 lg:grid-cols-12 gap-10 lg:gap-16 items-start">
          
          {/* LEFT COLUMN (~52%): 4 Security Features */}
          <div className="lg:col-span-6 space-y-4">
            {features.map((item, idx) => {
              const Icon = item.icon;
              return (
                <motion.div
                  key={item.id}
                  initial={{ opacity: 0, y: 15 }}
                  whileInView={{ opacity: 1, y: 0 }}
                  viewport={{ once: true }}
                  transition={{ duration: 0.35, delay: idx * 0.08 }}
                  className="bg-white border border-slate-200 hover:border-slate-300 rounded-2xl p-5 sm:p-5 transition-all duration-200 hover:-translate-y-[2px] shadow-sm hover:shadow-md group relative"
                >
                  <div className="flex items-start gap-4">
                    
                    {/* Icon Container with subtle red detail */}
                    <div className="w-11 h-11 sm:w-12 sm:h-12 rounded-xl bg-deep-navy text-white flex items-center justify-center shrink-0 relative overflow-hidden group-hover:bg-slate-800 transition-colors shadow-sm">
                      <div className="absolute top-0 left-0 bottom-0 w-1 bg-secondary-crimson" />
                      <Icon className="w-5 h-5 text-white ml-0.5" />
                    </div>

                    <div className="space-y-1.5 flex-1 min-w-0">
                      <div className="flex flex-wrap items-center justify-between gap-2">
                        <h3 className="font-display font-bold text-base sm:text-lg text-deep-navy">
                          {item.title}
                        </h3>

                        <span className="text-[10px] font-bold uppercase tracking-wider text-slate-600 bg-slate-100 border border-slate-200 px-2.5 py-0.5 rounded-full">
                          {item.label}
                        </span>
                      </div>

                      <p className="text-xs sm:text-sm text-slate-600 leading-relaxed font-sans">
                        {item.description}
                      </p>
                    </div>

                  </div>
                </motion.div>
              );
            })}
          </div>

          {/* RIGHT COLUMN (~48%): Visual Security Architecture Panel */}
          <div className="lg:col-span-6">
            <motion.div 
              initial={{ opacity: 0, scale: 0.98 }}
              whileInView={{ opacity: 1, scale: 1 }}
              viewport={{ once: true }}
              transition={{ duration: 0.45 }}
              className="bg-slate-50/90 border border-slate-200 rounded-[28px] p-6 sm:p-8 relative shadow-sm space-y-6 overflow-hidden"
            >
              {/* Panel Heading */}
              <div className="space-y-1.5 border-b border-slate-200 pb-5">
                <h3 className="font-display font-extrabold text-xl sm:text-2xl text-deep-navy">
                  {tx('How DIGIMAT Protects the Voting Process', 'DIGIMAT ले कसरी मतदान प्रक्रियालाई सुरक्षित गर्दछ')}
                </h3>
                <p className="text-xs sm:text-sm text-slate-600">
                  {tx(
                    'Security is applied across identity, ballot submission, administrative access, and verification.',
                    'पहचान, मतपत्र पेश, प्रशासकीय पहुँच र प्रमाणीकरणमा सुरक्षा लागू गरिन्छ।'
                  )}
                </p>
              </div>

              {/* 4-Layer Connected Visual Process Flow */}
              <div className="relative pl-6 space-y-5">
                
                {/* Vertical Process Connecting Line */}
                <div className="absolute left-[13px] top-[14px] bottom-[14px] w-0.5 bg-gradient-to-b from-secondary-crimson via-slate-300 to-emerald-600" />

                {workflowSteps.map((step) => {
                  const StepIcon = step.icon;
                  return (
                    <div key={step.step} className="relative flex items-start gap-4 group">
                      
                      {/* Step Circle Marker */}
                      <div className="w-7 h-7 rounded-full bg-white border-2 border-slate-300 group-hover:border-secondary-crimson text-deep-navy font-mono text-xs font-extrabold flex items-center justify-center shrink-0 z-10 transition-colors shadow-sm -ml-[27px]">
                        {step.step}
                      </div>

                      <div className="bg-white border border-slate-200 group-hover:border-slate-300 rounded-xl p-4 flex-1 transition-all space-y-1 shadow-2xs">
                        <div className="flex items-center justify-between gap-2">
                          <div className="flex items-center gap-2">
                            <StepIcon className="w-4 h-4 text-deep-navy" />
                            <span className="font-display font-bold text-sm text-deep-navy">
                              {step.title}
                            </span>
                          </div>

                          <span className="text-[10px] font-bold text-emerald-700 bg-emerald-50 border border-emerald-200 px-2 py-0.5 rounded-full flex items-center gap-1">
                            <span className="w-1.5 h-1.5 rounded-full bg-emerald-500" />
                            <span>{step.status}</span>
                          </span>
                        </div>

                        <p className="text-xs text-slate-600 leading-relaxed font-sans">
                          {step.desc}
                        </p>
                      </div>

                    </div>
                  );
                })}

              </div>

              {/* Status Preview Strip at Bottom */}
              <div className="mt-6 pt-5 border-t border-slate-200 space-y-2.5">
                <div className="text-[11px] font-mono uppercase text-slate-500 font-bold tracking-wider">
                  {tx('Example Workflow Verification States', 'उदाहरण कार्यप्रवाह प्रमाणीकरण अवस्था')}
                </div>

                <div className="grid grid-cols-2 gap-2 text-xs font-mono text-deep-navy">
                  <div className="bg-white border border-slate-200 px-3 py-2 rounded-lg flex items-center gap-2 shadow-2xs">
                    <CheckCircle2 className="w-3.5 h-3.5 text-emerald-600 shrink-0" />
                    <span className="truncate">{tx('Secure session', 'सुरक्षित सेसन')}</span>
                  </div>

                  <div className="bg-white border border-slate-200 px-3 py-2 rounded-lg flex items-center gap-2 shadow-2xs">
                    <CheckCircle2 className="w-3.5 h-3.5 text-emerald-600 shrink-0" />
                    <span className="truncate">{tx('Ballot submitted', 'मत पेश गरियो')}</span>
                  </div>

                  <div className="bg-white border border-slate-200 px-3 py-2 rounded-lg flex items-center gap-2 shadow-2xs">
                    <CheckCircle2 className="w-3.5 h-3.5 text-emerald-600 shrink-0" />
                    <span className="truncate">{tx('Receipt generated', 'रसीद सिर्जना भयो')}</span>
                  </div>

                  <div className="bg-white border border-slate-200 px-3 py-2 rounded-lg flex items-center gap-2 shadow-2xs">
                    <CheckCircle2 className="w-3.5 h-3.5 text-emerald-600 shrink-0" />
                    <span className="truncate">{tx('Audit record created', 'अडिट लग सिर्जना भयो')}</span>
                  </div>
                </div>
              </div>

            </motion.div>
          </div>

        </div>

      </div>
    </section>
  );
}
