'use client';

import { useLanguage } from '@/context/LanguageContext';

export default function NepalSolutionsSection() {
  const { t, tx } = useLanguage();

  const solutions = [
    {
      title: tx('Municipal Government', 'स्थानीय तथा महानगर सरकार'),
      desc: tx('Localized voting for city councils, ward representatives, and public consultations.', 'नगरपालिका परिषद्, वडा प्रतिनिधि तथा सार्वजनिक परामर्शका लागि स्थानीयकृत मतदान।'),
      image: 'https://images.unsplash.com/photo-1544735716-392fe2489ffa?auto=format&fit=crop&w=600&q=80',
    },
    {
      title: tx('Academic Institutions', 'शैक्षिक संघ-संस्था'),
      desc: tx('Student union elections and faculty board voting with robust authentication.', 'मजबुत प्रमाणीकरण सहित विद्यार्थी युनियन निर्वाचन र प्राध्यापक बोर्ड मतदान।'),
      image: 'https://images.unsplash.com/photo-1523240795612-9a054b0db644?auto=format&fit=crop&w=600&q=80',
    },
    {
      title: tx('Cooperatives & NGOs', 'सहकारी तथा गैर-सरकारी संस्था'),
      desc: tx('Membership voting, policy updates, and leadership appointments for social organizations.', 'सामाजिक संस्थाहरूका लागि सदस्य मतदान, नीति अद्यावधिक र नेतृत्व चयन।'),
      image: 'https://images.unsplash.com/photo-1600880292203-757bb62b4baf?auto=format&fit=crop&w=600&q=80',
    }
  ];

  return (
    <section className="py-20 bg-surface-bright border-b border-border-gray">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        
        {/* Section Header */}
        <div className="text-center max-w-2xl mx-auto mb-14">
          <h2 className="font-display font-extrabold text-3xl sm:text-4xl text-deep-navy tracking-tight">
            {t('nepal.title')}
          </h2>
        </div>

        {/* 3 Grid items */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
          {solutions.map((item, idx) => (
            <div key={idx} className="space-y-3">
              <div className="relative h-48 w-full rounded-2xl overflow-hidden shadow-md">
                <img
                  src={item.image}
                  alt={item.title}
                  className="w-full h-full object-cover"
                />
                <div className="absolute inset-0 bg-black/40 flex items-end p-4">
                  <h3 className="font-display font-bold text-white text-base">
                    {item.title}
                  </h3>
                </div>
              </div>

              <p className="text-xs text-slate-600 leading-relaxed font-sans px-1">
                {item.desc}
              </p>
            </div>
          ))}
        </div>

      </div>
    </section>
  );
}
