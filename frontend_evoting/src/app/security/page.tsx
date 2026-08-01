'use client';

import { 
  ShieldCheck, 
  Lock, 
  KeyRound, 
  Cpu, 
  CheckCircle2, 
  FileText, 
  Download, 
  Globe,
  ArrowRight
} from 'lucide-react';
import Link from 'next/link';

export default function SecurityTrustPage() {
  return (
    <div className="bg-surface-bright min-h-screen py-12">
      <div className="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 space-y-12">
        
        {/* Header */}
        <div className="bg-deep-navy text-white rounded-3xl p-8 sm:p-12 shadow-2xl pattern-overlay text-center space-y-4">
          <div className="w-14 h-14 rounded-2xl bg-emerald-green text-white flex items-center justify-center mx-auto shadow-lg">
            <ShieldCheck className="w-8 h-8" />
          </div>

          <h1 className="font-display font-extrabold text-3xl sm:text-4xl text-white">
            Security, Cryptography & Sovereign Trust
          </h1>

          <p className="text-xs sm:text-sm text-slate-300 max-w-2xl mx-auto">
            Technical architecture breakdown of DIGIMAT’s End-to-End Verifiable (E2EV) digital election protocol designed for Nepal.
          </p>

          <div className="pt-2 flex flex-wrap justify-center gap-3 text-xs font-mono">
            <span className="bg-slate-900 border border-slate-700 px-3 py-1.5 rounded-lg text-emerald-green">
              PAILLIER HOMOMORPHIC SCHEME
            </span>
            <span className="bg-slate-900 border border-slate-700 px-3 py-1.5 rounded-lg text-emerald-green">
              NIZKP (NON-INTERACTIVE ZKP)
            </span>
            <span className="bg-slate-900 border border-slate-700 px-3 py-1.5 rounded-lg text-emerald-green">
              SHAMIR THRESHOLD KEY SPLIT
            </span>
          </div>
        </div>

        {/* 3 Main Cryptographic Guarantees */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div className="bg-white border border-border-gray rounded-3xl p-6 shadow-sm space-y-3">
            <div className="w-10 h-10 rounded-xl bg-secondary-crimson/10 text-secondary-crimson flex items-center justify-center">
              <Lock className="w-5 h-5 text-secondary-crimson" />
            </div>
            <h3 className="font-display font-bold text-lg text-deep-navy">Cast-as-Intended</h3>
            <p className="text-xs text-slate-600 leading-relaxed font-sans">
              Client-side zero-knowledge proof confirms your ballot matches your selected candidate choice on screen before encryption.
            </p>
          </div>

          <div className="bg-white border border-border-gray rounded-3xl p-6 shadow-sm space-y-3">
            <div className="w-10 h-10 rounded-xl bg-emerald-green/10 text-emerald-green flex items-center justify-center">
              <ShieldCheck className="w-5 h-5 text-emerald-green" />
            </div>
            <h3 className="font-display font-bold text-lg text-deep-navy">Recorded-as-Cast</h3>
            <p className="text-xs text-slate-600 leading-relaxed font-sans">
              Your cryptographic Receipt Code permits independent audit verification that your encrypted ballot entered the public election ledger.
            </p>
          </div>

          <div className="bg-white border border-border-gray rounded-3xl p-6 shadow-sm space-y-3">
            <div className="w-10 h-10 rounded-xl bg-deep-navy/10 text-deep-navy flex items-center justify-center">
              <Cpu className="w-5 h-5 text-deep-navy" />
            </div>
            <h3 className="font-display font-bold text-lg text-deep-navy">Tallied-as-Recorded</h3>
            <p className="text-xs text-slate-600 leading-relaxed font-sans">
              Homomorphic mathematical summation ensures all recorded ballots are included in the final certified turnout tally accurately.
            </p>
          </div>
        </div>

        {/* Distributed Key Generation Diagram */}
        <div className="bg-white border border-border-gray rounded-3xl p-8 shadow-md space-y-6">
          <div className="space-y-2">
            <h2 className="font-display font-bold text-xl text-deep-navy flex items-center gap-2">
              <KeyRound className="w-5 h-5 text-emerald-green" />
              <span>Multi-Trustee Key Share Distribution (Nepal)</span>
            </h2>
            <p className="text-xs text-slate-600">
              Decryption authority is split into 3 independent secret key shares using Shamir’s Threshold Scheme.
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-4 font-sans text-xs">
            <div className="bg-slate-50 border border-slate-200 rounded-2xl p-5 space-y-2">
              <div className="font-bold text-deep-navy text-sm">Key Share #1</div>
              <div className="text-secondary-crimson font-semibold">Election Commission of Nepal</div>
              <p className="text-slate-500 text-[11px]">Holds 1st threshold key piece. Stored inside hardware security module (HSM).</p>
            </div>

            <div className="bg-slate-50 border border-slate-200 rounded-2xl p-5 space-y-2">
              <div className="font-bold text-deep-navy text-sm">Key Share #2</div>
              <div className="text-secondary-crimson font-semibold">Supreme Court IT Auditor</div>
              <p className="text-slate-500 text-[11px]">Holds 2nd threshold key piece. Ensures judicial oversight of tally decryption.</p>
            </div>

            <div className="bg-slate-50 border border-slate-200 rounded-2xl p-5 space-y-2">
              <div className="font-bold text-deep-navy text-sm">Key Share #3</div>
              <div className="text-secondary-crimson font-semibold">Civil Society & NEA Observers</div>
              <p className="text-slate-500 text-[11px]">Holds 3rd threshold key piece. Represents independent voter advocacy groups.</p>
            </div>
          </div>
        </div>

        {/* Download Whitepaper Bar */}
        <div className="bg-slate-900 text-white rounded-3xl p-8 text-center space-y-4 shadow-xl">
          <h3 className="font-display font-extrabold text-2xl text-white">
            Download the Sovereign Technical Whitepaper
          </h3>
          <p className="text-xs text-slate-300 max-w-lg mx-auto">
            Comprehensive 42-page mathematical paper covering Paillier encryption benchmarks, zero-knowledge range proofs, and NID integration specs.
          </p>

          <button
            onClick={() => alert('Downloading DIGIMAT_Sovereign_Voting_Whitepaper_2026.pdf')}
            className="bg-emerald-green hover:bg-emerald-600 text-white font-display text-xs font-bold px-6 py-3.5 rounded-xl shadow-lg transition-all inline-flex items-center gap-2"
          >
            <Download className="w-4 h-4" />
            <span>Download Cryptographic Specification PDF</span>
          </button>
        </div>

      </div>
    </div>
  );
}
