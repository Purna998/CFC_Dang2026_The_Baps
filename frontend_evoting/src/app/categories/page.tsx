'use client';

import Link from 'next/link';
import { CATEGORIES } from '@/lib/data';
import { 
  Landmark, 
  Award, 
  GraduationCap, 
  Building2, 
  Coins, 
  Home,
  ArrowRight,
  ShieldCheck,
  CheckCircle2,
  Users,
  Heart,
  Sparkles,
  UserCheck
} from 'lucide-react';
import { motion } from 'framer-motion';
import { useLanguage } from '@/context/LanguageContext';

export default function CategoriesPage() {
  const { t, tx } = useLanguage();

  const iconMap: Record<string, any> = {
    Landmark,
    Award,
    GraduationCap,
    Building2,
    Coins,
    Home
  };

  const humanizedCategories = [
    {
      id: 'gov',
      title: tx('Government & Sovereign', 'सरकारी तथा स्थानीय निकाय'),
      subtitle: tx('Federal & Municipal Elections', 'संघीय तथा नगरपालिका निर्वाचन'),
      description: tx(
        'Empowering citizens in Kathmandu, Pokhara, Lalitpur and across all 77 districts to shape their local leadership with absolute trust.',
        'काठमाडौं, पोखरा, ललितपुर र सबै ७७ जिल्लाका नागरिकहरूलाई आफ्नो स्थानीय नेतृत्व चयन गर्न सक्षम बनाउँदै।'
      ),
      icon: Landmark,
      count: tx('14 Active Elections', '१४ सक्रिय निर्वाचन'),
      voters: tx('1.2M+ Voters Active', '१२ लाख भन्दा बढी सक्रिय मतदाता'),
      tag: tx('Public Trust', 'सार्वजनिक विश्वास'),
    },
    {
      id: 'prof',
      title: tx('Professional Associations', 'व्यावसायिक संघ-संस्थाहरू'),
      subtitle: tx('NEA, NBA & Medical Federations', 'नेपाल इन्जिनियर्स एशोसिएसन, बार एशोसिएसन र मेडिकल काउन्सिल'),
      description: tx(
        'Seamless voting for Nepal Engineers Association (NEA), Nepal Bar Association (NBA), and chartered societies nationwide.',
        'नेपाल इन्जिनियर्स एशोसिएसन (NEA), नेपाल बार एशोसिएसन (NBA) र देशभरका व्यावसायिक संस्थाहरूका लागि सहज मतदान।'
      ),
      icon: Award,
      count: tx('32 Active Elections', '३२ सक्रिय निर्वाचन'),
      voters: tx('45,000+ Professionals', '४५,०००+ व्यवसायीहरू'),
      tag: tx('Verified Identity', 'प्रमाणित पहिचान'),
    },
    {
      id: 'edu',
      title: tx('Education & Universities', 'शिक्षा तथा विश्वविद्यालयहरू'),
      subtitle: tx('Student Unions & Faculty Senate', 'विद्यार्थी युनियन तथा प्राध्यापक सिनेट'),
      description: tx(
        'Bringing modern digital voting to Tribhuvan University, Kathmandu University, and student union councils across Nepal.',
        'त्रिभुवन विश्वविद्यालय, काठमाडौं विश्वविद्यालय र देशभरका विद्यार्थी युनियनहरूमा आधुनिक डिजिटल मतदान।'
      ),
      icon: GraduationCap,
      count: tx('8 Active Elections', '८ सक्रिय निर्वाचन'),
      voters: tx('120,000+ Students', '१,२०,०००+ विद्यार्थीहरू'),
      tag: tx('Youth Engagement', 'युवा सहभागिता'),
    },
    {
      id: 'corp',
      title: tx('Corporate & Board Proxy', 'कर्पोरेट तथा शेयरधनी संस्था'),
      subtitle: tx('AGM Voting & Director Appointments', 'साधारण सभा मतदान र सञ्चालक समिति'),
      description: tx(
        'Transparent shareholder voting, Board of Directors appointments, and Annual General Meeting (AGM) resolutions.',
        'पारदर्शी शेयरधनी मतदान, सञ्चालक समिति चयन र वार्षिक साधारण सभा (AGM) प्रस्तावहरू।'
      ),
      icon: Building2,
      count: tx('19 Active Elections', '१९ सक्रिय निर्वाचन'),
      voters: tx('85,000+ Shareholders', '८५,०००+ शेयरधनीहरू'),
      tag: tx('Secure Governance', 'सुरक्षित सुशासन'),
    },
    {
      id: 'coop',
      title: tx('Cooperatives & Credit Unions', 'सहकारी तथा बचत समूह'),
      subtitle: tx('Democratic Financial Governance', 'लोकतान्त्रिक वित्तीय सुशासन'),
      description: tx(
        'One-member-one-vote digital democracy for Nepal’s largest credit cooperatives and financial societies.',
        'नेपालका प्रमुख बचत तथा ऋण सहकारी संस्थाहरूका लागि एक-सदस्य-एक-मत डिजिटल लोकतन्त्र।'
      ),
      icon: Coins,
      count: tx('45 Active Elections', '४५ सक्रिय निर्वाचन'),
      voters: tx('350,000+ Members', '३,५०,०००+ सदस्यहरू'),
      tag: tx('Community Driven', 'समुदाय केन्द्रित'),
    },
    {
      id: 'hoa',
      title: tx('Condos & Housing Societies', 'आवास तथा हाउजिङ सोसाइटी'),
      subtitle: tx('Local Neighborhood Boards', 'स्थानीय समुदाय बोर्ड'),
      description: tx(
        'Effortless neighborhood decision-making, budget approvals, and committee elections for housing societies in Nepal.',
        'नेपालका आवास तथा हाउजिङ सोसाइटीहरूका लागि सहज सामुदायिक निर्णय, बजेट स्वीकृति र समिति निर्वाचन।'
      ),
      icon: Home,
      count: tx('60 Active Elections', '६० सक्रिय निर्वाचन'),
      voters: tx('28,000+ Residents', '२८,०००+ बासिन्दाहरू'),
      tag: tx('Direct Democracy', 'प्रत्यक्ष लोकतन्त्र'),
    }
  ];

  return (
    <div className="bg-surface-bright min-h-screen py-12">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 space-y-12">
        
        {/* Header with Human Touch */}
        <motion.div 
          initial={{ opacity: 0, y: -15 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5 }}
          className="text-center space-y-4 max-w-3xl mx-auto"
        >
          <div className="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full bg-slate-100 border border-slate-200 text-deep-navy text-xs font-bold shadow-sm">
            <Heart className="w-3.5 h-3.5 text-secondary-crimson fill-secondary-crimson" />
            <span>{tx('Democracy Built Around People', 'मानिसहरूका लागि निर्मित लोकतन्त्र')}</span>
          </div>

          <h1 className="font-display font-extrabold text-3xl sm:text-4xl lg:text-5xl text-deep-navy tracking-tight">
            {tx('Explore Election Categories', 'निर्वाचन वर्गहरू हेर्नुहोस्')}
          </h1>

          <p className="text-sm sm:text-base text-slate-600 leading-relaxed">
            {tx(
              'Connecting communities across Nepal with accessible, secure, and transparent e-voting tailored for every institution.',
              'हरेक संस्थाका लागि पहुँचयोग्य, सुरक्षित र पारदर्शी इ-भोटिङ मार्फत नेपालभरका समुदायहरूलाई जोड्दै।'
            )}
          </p>
        </motion.div>

        {/* Humanized Statistics Bar */}
        <motion.div 
          initial={{ opacity: 0, scale: 0.98 }}
          animate={{ opacity: 1, scale: 1 }}
          transition={{ duration: 0.5, delay: 0.15 }}
          className="grid grid-cols-2 md:grid-cols-4 gap-4 bg-white border border-border-gray rounded-3xl p-6 shadow-sm"
        >
          <div className="space-y-1 text-center border-r border-slate-100 last:border-none">
            <div className="text-xl sm:text-2xl font-extrabold text-deep-navy font-display">1.8M+</div>
            <div className="text-xs text-slate-500 font-medium">{tx('Verified Voters', 'प्रमाणित मतदाताहरू')}</div>
          </div>
          <div className="space-y-1 text-center border-r border-slate-100 last:border-none">
            <div className="text-xl sm:text-2xl font-extrabold text-secondary-crimson font-display">77</div>
            <div className="text-xs text-slate-500 font-medium">{tx('Districts Reached', 'समेटिएका जिल्लाहरू')}</div>
          </div>
          <div className="space-y-1 text-center border-r border-slate-100 last:border-none">
            <div className="text-xl sm:text-2xl font-extrabold text-deep-navy font-display">178+</div>
            <div className="text-xs text-slate-500 font-medium">{tx('Active Polls', 'सक्रिय निर्वाचनहरू')}</div>
          </div>
          <div className="space-y-1 text-center">
            <div className="text-xl sm:text-2xl font-extrabold text-deep-navy font-display">100%</div>
            <div className="text-xs text-slate-500 font-medium">{tx('Encrypted & Audit-Verifiable', 'सुरक्षित तथा परीक्षणयोग्य')}</div>
          </div>
        </motion.div>

        {/* Categories Grid with Smooth Motion & Refined Design */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
          {humanizedCategories.map((cat, idx) => {
            const Icon = cat.icon;
            return (
              <motion.div
                key={cat.id}
                initial={{ opacity: 0, y: 30 }}
                animate={{ opacity: 1, y: 0 }}
                whileHover={{ 
                  y: -6,
                  transition: { duration: 0.25 }
                }}
                transition={{ duration: 0.45, delay: idx * 0.08 }}
                className="bg-white border border-border-gray rounded-3xl p-6 sm:p-7 shadow-sm hover:shadow-xl hover:border-slate-300 transition-all group flex flex-col justify-between relative overflow-hidden"
              >
                {/* Subtle top accent bar */}
                <div className="absolute top-0 left-0 right-0 h-1 bg-gradient-to-r from-deep-navy via-slate-700 to-secondary-crimson opacity-0 group-hover:opacity-100 transition-opacity duration-300" />

                <div className="space-y-5">
                  
                  {/* Top Bar: Icon + Clean Site-Matching Pill (No Green Gradient) */}
                  <div className="flex items-center justify-between">
                    <div className="w-13 h-13 rounded-2xl bg-deep-navy text-white flex items-center justify-center p-3 shadow-md group-hover:bg-slate-800 transition-colors">
                      <Icon className="w-6 h-6 text-white" />
                    </div>

                    <div className="flex items-center gap-2 bg-slate-100 border border-slate-200 px-3 py-1 rounded-full shadow-2xs">
                      <span className="w-2 h-2 rounded-full bg-secondary-crimson animate-pulse" />
                      <span className="text-[11px] font-bold text-deep-navy font-mono">
                        {cat.count}
                      </span>
                    </div>
                  </div>

                  {/* Title & Subtitle */}
                  <div>
                    <div className="text-[11px] font-bold uppercase tracking-wider text-secondary-crimson">
                      {cat.tag}
                    </div>
                    <h3 className="font-display font-extrabold text-xl text-deep-navy tracking-tight pt-0.5">
                      {cat.title}
                    </h3>
                  </div>

                  {/* Description */}
                  <p className="text-xs sm:text-sm text-slate-600 leading-relaxed font-sans">
                    {cat.description}
                  </p>
                </div>

                {/* Footer Link & Voter Reach */}
                <div className="pt-6 mt-6 border-t border-slate-100 space-y-3">
                  <div className="flex items-center justify-between text-[11px] text-slate-500 font-medium">
                    <span className="flex items-center gap-1.5">
                      <UserCheck className="w-3.5 h-3.5 text-deep-navy" />
                      <span>{cat.voters}</span>
                    </span>
                  </div>

                  <Link
                    href="/dashboard"
                    className="w-full py-2.5 px-4 bg-slate-50 group-hover:bg-deep-navy text-deep-navy group-hover:text-white font-display text-xs font-bold rounded-xl transition-all flex items-center justify-between border border-slate-200 group-hover:border-deep-navy shadow-2xs"
                  >
                    <span>{tx('View Elections', 'निर्वाचनहरू हेर्नुहोस्')}</span>
                    <ArrowRight className="w-4 h-4 text-secondary-crimson group-hover:translate-x-1 transition-transform" />
                  </Link>
                </div>
              </motion.div>
            );
          })}
        </div>

        {/* Humanized Trust Banner */}
        <motion.div 
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5, delay: 0.4 }}
          className="bg-deep-navy text-white rounded-3xl p-8 sm:p-12 shadow-2xl pattern-overlay grid grid-cols-1 lg:grid-cols-12 gap-8 items-center"
        >
          <div className="lg:col-span-7 space-y-4">
            <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-white/10 border border-white/20 text-xs font-bold text-slate-200">
              <Sparkles className="w-3.5 h-3.5 text-secondary-crimson" />
              <span>{tx('EQUAL ACCESS FOR ALL CITIZENS', 'सबै नागरिकका लागि समान पहुँच')}</span>
            </div>
            
            <h2 className="font-display font-extrabold text-2xl sm:text-3xl text-white leading-tight">
              {tx('Designed for Every Voter Across Nepal', 'नेपालभरका प्रत्येक मतदाताका लागि डिजाइन गरिएको')}
            </h2>
            
            <p className="text-xs sm:text-sm text-slate-300 leading-relaxed">
              {tx(
                'Whether casting a vote from a smartphone in Kathmandu or via an offline community kiosk in rural wards, DIGIMAT ensures every citizen has a voice.',
                'काठमाडौँमा स्मार्टफोन वा ग्रामीण वडामा अफलाइन सामुदायिक किओस्क मार्फत मतदान गर्दा पनि DIGIMAT ले हरेक नागरिकको आवाज सुरक्षित गर्दछ।'
              )}
            </p>
          </div>

          <div className="lg:col-span-5 bg-slate-900/90 border border-slate-700 rounded-2xl p-6 space-y-3 shadow-inner">
            <div className="text-xs font-bold text-white flex items-center gap-2.5">
              <CheckCircle2 className="w-4 h-4 text-secondary-crimson shrink-0" />
              <span>{tx('Mobile & Web Accessible', 'मोबाइल तथा वेब पहुँचयोग्य')}</span>
            </div>
            <div className="text-xs font-bold text-white flex items-center gap-2.5">
              <CheckCircle2 className="w-4 h-4 text-secondary-crimson shrink-0" />
              <span>{tx('National ID & Biometric Verification', 'राष्ट्रिय परिचयपत्र र बायोमेट्रिक प्रमाणीकरण')}</span>
            </div>
            <div className="text-xs font-bold text-white flex items-center gap-2.5">
              <CheckCircle2 className="w-4 h-4 text-secondary-crimson shrink-0" />
              <span>{tx('100% Verifiable Encrypted Receipt', '१००% परीक्षणयोग्य इन्क्रिप्टेड रसीद')}</span>
            </div>
          </div>
        </motion.div>

      </div>
    </div>
  );
}
