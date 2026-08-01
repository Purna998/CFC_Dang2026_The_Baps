'use client';

import React, { createContext, useContext, useState, useEffect } from 'react';

export type Language = 'EN' | 'NE';

interface LanguageContextType {
  language: Language;
  setLanguage: (lang: Language) => void;
  t: (key: string) => string;
  tx: (en: string, ne: string) => string;
}

const LanguageContext = createContext<LanguageContextType | undefined>(undefined);

export function LanguageProvider({ children }: { children: React.ReactNode }) {
  const [language, setLanguageState] = useState<Language>('EN');

  useEffect(() => {
    const savedLang = localStorage.getItem('digimat_lang') as Language;
    if (savedLang === 'EN' || savedLang === 'NE') {
      setLanguageState(savedLang);
    }
  }, []);

  const setLanguage = (lang: Language) => {
    setLanguageState(lang);
    localStorage.setItem('digimat_lang', lang);
  };

  // Helper to pick text based on language
  const tx = (en: string, ne: string) => {
    return language === 'NE' ? ne : en;
  };

  // Translation key lookup table
  const t = (key: string): string => {
    if (language === 'NE' && translationsNe[key]) {
      return translationsNe[key];
    }
    return translationsEn[key] || key;
  };

  return (
    <LanguageContext.Provider value={{ language, setLanguage, t, tx }}>
      {children}
    </LanguageContext.Provider>
  );
}

export function useLanguage() {
  const context = useContext(LanguageContext);
  if (!context) {
    throw new Error('useLanguage must be used within a LanguageProvider');
  }
  return context;
}

const translationsEn: Record<string, string> = {
  // Nav
  'nav.home': 'HOME',
  'nav.features': 'FEATURES',
  'nav.categories': 'CATEGORIES',
  'nav.results': 'RESULTS',
  'nav.help': 'HELP',
  'nav.signin': 'SIGN IN',
  'nav.demo': 'REQUEST DEMO',
  'nav.dashboard': 'VOTER DASHBOARD',
  
  // Hero
  'hero.status': 'LIVE VOTING STATUS: 1,240 ACTIVE POLLS',
  'hero.title1': 'Trusted Online Voting',
  'hero.title2': 'for Every Organization',
  'hero.subtitle': 'Empowering Nepal’s institutions with secure, end-to-end encrypted voting infrastructure. Transparent results, verified by cryptography, built for national trust.',
  'hero.start_btn': 'Start an Election',
  'hero.works_btn': 'See How It Works',
  'hero.badge': 'Sovereign Trust Architecture',

  // Nepal Solutions
  'nepal.badge': 'BUILT FOR NEPAL',
  'nepal.title': 'Digital Electoral Infrastructure Tailored for Nepal',
  'nepal.subtitle': 'From Federal Parliamentary polls to Local Metropolitan, NEA, and Credit Cooperatives nationwide.',
  
  // Workflow
  'workflow.badge': 'HOW IT WORKS',
  'workflow.title': 'Four Steps to Transparent Digital Democracy',
  'workflow.subtitle': 'Cryptographically enforced integrity from voter identity verification to audited tally publish.',
  
  // Security
  'security.badge': 'CRYPTOGRAPHIC COMPLIANCE',
  'security.title': 'Zero-Trust Security & Verifiability Standard',
  'security.subtitle': 'Built with Paillier Homomorphic Encryption, ECC 256-bit key splits, and immutable ledger audit logging.',

  // CTA
  'cta.title': 'Make Your Next Election Simple, Secure, and Accessible',
  'cta.subtitle': 'Join hundreds of organizations across Nepal transitioning to the future of digital democracy.',
  'cta.start': 'Get Started Today',
  'cta.demo': 'Schedule a Demo',

  // Footer
  'footer.brand_text': 'Secure electoral infrastructure for a modern Nepal.',
  'footer.copyright': '© 2024 DIGIMAT. Secure Electoral Infrastructure of Nepal.',
};

const translationsNe: Record<string, string> = {
  // Nav
  'nav.home': 'गृह',
  'nav.features': 'विशेषताहरू',
  'nav.categories': 'वर्गहरू',
  'nav.results': 'परिणामहरू',
  'nav.help': 'मद्दत',
  'nav.signin': 'साइन इन',
  'nav.demo': 'डेमो अनुरोध',
  'nav.dashboard': 'मतदाता ड्यासबोर्ड',

  // Hero
  'hero.status': 'प्रत्यक्ष मतदान स्थिति: १,२४० सक्रिय मतदानहरू',
  'hero.title1': 'विश्वसनीय अनलाइन मतदान',
  'hero.title2': 'हरेक संस्थाका लागि',
  'hero.subtitle': 'नेपालका संस्थाहरूलाई सुरक्षित, इन्ड-टु-इन्ड इन्क्रिप्टेड मतदान पूर्वाधार मार्फत सशक्त बनाउँदै। पारदर्शी परिणाम, क्रिप्टोग्राफीद्वारा प्रमाणित।',
  'hero.start_btn': 'निर्वाचन सुरु गर्नुहोस्',
  'hero.works_btn': 'कसरी काम गर्छ हेर्नुहोस्',
  'hero.badge': 'सार्वभौम विश्वास संरचना',

  // Nepal Solutions
  'nepal.badge': 'नेपालका लागि निर्मित',
  'nepal.title': 'नेपालका लागि अनुकूलित डिजिटल चुनावी पूर्वाधार',
  'nepal.subtitle': 'संघीय संसदीय निर्वाचनदेखि स्थानीय महानगर, एनइए र बचत तथा ऋण सहकारीहरूसम्म।',

  // Workflow
  'workflow.badge': 'कसरी काम गर्छ',
  'workflow.title': 'पारदर्शी डिजिटल लोकतन्त्रका चार चरण',
  'workflow.subtitle': 'मतदाता पहिचान प्रमाणीकरणदेखि परीक्षित मत गणना प्रकाशनसम्म क्रिप्टोग्राफीद्वारा सुरक्षित।',

  // Security
  'security.badge': 'क्रिप्टोग्राफिक पालना',
  'security.title': 'शून्य-विश्वास सुरक्षा र प्रमाणीकरण मानक',
  'security.subtitle': 'Paillier Homomorphic इन्क्रिप्सन, ECC 256-बिट कुञ्जी र अपरिवर्तनीय लेजर अडिट लगिङद्वारा निर्मित।',

  // CTA
  'cta.title': 'तपाईंको आगामी निर्वाचनलाई सरल, सुरक्षित र पहुँचयोग्य बनाउनुहोस्',
  'cta.subtitle': 'डिजिटल लोकतन्त्रको भविष्यतर्फ अघि बढ्दै गरेका नेपालभरका सयौं संस्थाहरूमा सामेल हुनुहोस्।',
  'cta.start': 'आजै सुरु गर्नुहोस्',
  'cta.demo': 'डेमो तय गर्नुहोस्',

  // Footer
  'footer.brand_text': 'आधुनिक नेपालका लागि सुरक्षित चुनावी पूर्वाधार।',
  'footer.copyright': '© २०२४ DIGIMAT। नेपालको सुरक्षित चुनावी पूर्वाधार।',
};
