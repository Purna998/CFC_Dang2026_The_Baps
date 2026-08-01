'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { Shield, Lock, ArrowRight, CheckCircle2, KeyRound, Smartphone, User } from 'lucide-react';
import { motion } from 'framer-motion';

export default function VoterLoginPage() {
  const router = useRouter();
  const [citizenId, setCitizenId] = useState('NEP-8842-1994-01');
  const [mobileNum, setMobileNum] = useState('+977 9841234567');
  const [isLoading, setIsLoading] = useState(false);

  const handleLogin = (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);

    setTimeout(() => {
      setIsLoading(false);
      router.push('/dashboard');
    }, 1200);
  };

  return (
    <div className="bg-surface-bright min-h-[85vh] flex items-center justify-center py-12 px-4">
      <div className="max-w-4xl w-full bg-white border border-border-gray rounded-3xl shadow-2xl overflow-hidden grid grid-cols-1 md:grid-cols-12">
        
        {/* Left Dark Banner */}
        <div className="md:col-span-5 bg-deep-navy text-white p-8 sm:p-10 pattern-overlay flex flex-col justify-between space-y-8">
          <div className="space-y-4">
            <div className="w-12 h-12 rounded-2xl bg-secondary-crimson flex items-center justify-center text-white font-bold shadow-lg">
              <Shield className="w-7 h-7" />
            </div>

            <div className="space-y-2">
              <span className="text-[10px] font-bold uppercase tracking-widest text-emerald-green bg-emerald-green/10 border border-emerald-green/20 px-2.5 py-0.5 rounded-full">
                CITIZEN AUTHENTICATION
              </span>
              <h2 className="font-display font-extrabold text-2xl sm:text-3xl text-white leading-tight">
                Your Vote. Your Voice. Your Digital Matdan.
              </h2>
            </div>

            <p className="text-xs text-slate-300 leading-relaxed font-sans">
              Access your sovereign voter profile, check active elections in your constituency, and cast end-to-end encrypted digital ballots.
            </p>
          </div>

          <div className="space-y-2 pt-4 border-t border-slate-800 text-xs text-slate-400 font-sans">
            <div className="flex items-center gap-2 text-slate-300 font-semibold">
              <Lock className="w-3.5 h-3.5 text-emerald-green" /> National Identity Card (NID) Enabled
            </div>
            <div className="flex items-center gap-2 text-slate-300 font-semibold">
              <CheckCircle2 className="w-3.5 h-3.5 text-secondary-crimson" /> Biometric OTP Authentication
            </div>
          </div>
        </div>

        {/* Right Form Container */}
        <div className="md:col-span-7 p-8 sm:p-10 flex flex-col justify-center space-y-6">
          <div className="space-y-1">
            <h3 className="font-display font-bold text-xl text-deep-navy">
              Voter Portal Sign In
            </h3>
            <p className="text-xs text-slate-500">
              Enter your National ID (NID) or registered mobile number to proceed.
            </p>
          </div>

          <form onSubmit={handleLogin} className="space-y-4">
            <div className="space-y-1.5">
              <label className="text-xs font-bold text-slate-700 block">
                National Identity Card (NID) Number
              </label>
              <div className="relative">
                <User className="w-4 h-4 text-slate-400 absolute left-3.5 top-1/2 -translate-y-1/2" />
                <input
                  type="text"
                  required
                  value={citizenId}
                  onChange={(e) => setCitizenId(e.target.value)}
                  placeholder="NEP-8842-1994-01"
                  className="w-full bg-slate-50 border border-slate-300 focus:border-deep-navy font-mono text-xs pl-10 pr-4 py-3 rounded-xl outline-none text-slate-900"
                />
              </div>
            </div>

            <div className="space-y-1.5">
              <label className="text-xs font-bold text-slate-700 block">
                Registered Mobile Phone (+977)
              </label>
              <div className="relative">
                <Smartphone className="w-4 h-4 text-slate-400 absolute left-3.5 top-1/2 -translate-y-1/2" />
                <input
                  type="text"
                  required
                  value={mobileNum}
                  onChange={(e) => setMobileNum(e.target.value)}
                  placeholder="+977 9841234567"
                  className="w-full bg-slate-50 border border-slate-300 focus:border-deep-navy font-mono text-xs pl-10 pr-4 py-3 rounded-xl outline-none text-slate-900"
                />
              </div>
            </div>

            <button
              type="submit"
              disabled={isLoading}
              className="w-full py-3.5 bg-deep-navy hover:bg-primary-container text-white font-display text-xs font-bold rounded-xl shadow-lg transition-all flex items-center justify-center gap-2 group"
            >
              {isLoading ? (
                <span>Authenticating Credentials...</span>
              ) : (
                <>
                  <span>Authenticate & Enter Secure Portal</span>
                  <ArrowRight className="w-4 h-4 text-secondary-crimson group-hover:translate-x-1 transition-transform" />
                </>
              )}
            </button>
          </form>

          <div className="pt-4 border-t border-slate-100 text-center">
            <button
              onClick={() => router.push('/dashboard')}
              className="text-xs font-bold text-secondary-crimson hover:underline"
            >
              Demo Mode: Skip Authentication & Enter Dashboard Directly →
            </button>
          </div>
        </div>

      </div>
    </div>
  );
}
