'use client';

import Link from 'next/link';
import Image from 'next/image';
import { Globe, Share2 } from 'lucide-react';
import { useLanguage } from '@/context/LanguageContext';

export default function Footer() {
  const { t, tx } = useLanguage();

  return (
    <footer className="bg-slate-100 text-slate-700 border-t border-border-gray text-xs">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-14">
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-10">
          
          {/* Col 1 & 2: Brand Info */}
          <div className="lg:col-span-2 space-y-4">
            <div className="flex items-center gap-2.5">
              <Image 
                src="/logo.png" 
                alt="DIGI-मत Logo" 
                width={220} 
                height={65} 
                className="h-13 sm:h-[50px] w-auto object-contain" 
              />
            </div>

            <p className="text-xs text-slate-500 leading-relaxed max-w-sm">
              {t('footer.brand_text')}
            </p>

            <div className="flex items-center gap-4 text-slate-500 pt-2">
              <Globe className="w-4 h-4 hover:text-deep-navy cursor-pointer" />
              <Share2 className="w-4 h-4 hover:text-deep-navy cursor-pointer" />
            </div>
          </div>

          {/* Col 3: PRODUCT */}
          <div className="space-y-3">
            <h4 className="font-display font-bold text-xs uppercase tracking-wider text-deep-navy">
              {tx('PRODUCT', 'उत्पादन')}
            </h4>
            <ul className="space-y-2 text-xs">
              <li><Link href="/#features" className="hover:text-deep-navy">{tx('Features', 'विशेषताहरू')}</Link></li>
              <li><Link href="/categories" className="hover:text-deep-navy">{tx('Pricing', 'मूल्य निर्धारण')}</Link></li>
              <li><Link href="/security" className="hover:text-deep-navy">{tx('Security Details', 'सुरक्षा विवरण')}</Link></li>
              <li><Link href="/security" className="hover:text-deep-navy">{tx('API Support', 'API सहायता')}</Link></li>
            </ul>
          </div>

          {/* Col 4: RESOURCES */}
          <div className="space-y-3">
            <h4 className="font-display font-bold text-xs uppercase tracking-wider text-deep-navy">
              {tx('RESOURCES', 'स्रोत सामग्री')}
            </h4>
            <ul className="space-y-2 text-xs">
              <li><Link href="/results" className="hover:text-deep-navy">{tx('Case Studies', 'अध्ययन रिपोर्टहरू')}</Link></li>
              <li><Link href="/security" className="hover:text-deep-navy">{tx('Documentation', 'कागजात')}</Link></li>
              <li><Link href="/security" className="hover:text-deep-navy">{tx('Legal Whitepapers', 'कानुनी दस्तावेज')}</Link></li>
              <li><Link href="/security" className="hover:text-deep-navy">{tx('Help Center', 'सहायता केन्द्र')}</Link></li>
            </ul>
          </div>

          {/* Col 5: COMPANY */}
          <div className="space-y-3">
            <h4 className="font-display font-bold text-xs uppercase tracking-wider text-deep-navy">
              {tx('COMPANY', 'कम्पनी')}
            </h4>
            <ul className="space-y-2 text-xs">
              <li><Link href="/security" className="hover:text-deep-navy">{tx('About', 'हाम्रो बारेमा')}</Link></li>
              <li><Link href="/security" className="hover:text-deep-navy">{tx('Privacy Policy', 'गोपनीयता नीति')}</Link></li>
              <li><Link href="/security" className="hover:text-deep-navy">{tx('Terms of Service', 'सेवाका शर्तहरू')}</Link></li>
              <li><Link href="/security" className="hover:text-deep-navy">{tx('Contact Us', 'सम्पर्क गर्नुहोस्')}</Link></li>
            </ul>
          </div>

        </div>

        {/* Bottom Bar */}
        <div className="mt-12 pt-6 border-t border-slate-200 flex flex-col sm:flex-row items-center justify-between text-[11px] text-slate-500 gap-4">
          <div>
            {t('footer.copyright')}
          </div>
          <div className="font-mono text-[10px] text-slate-400">
            v4.2.0-STABLE | ENCRYPTED CHANNEL 128-bit
          </div>
        </div>

      </div>
    </footer>
  );
}
