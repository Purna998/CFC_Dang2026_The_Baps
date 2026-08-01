'use client';

import { useEffect, useState } from 'react';
import Link from 'next/link';
import confetti from 'canvas-confetti';
import { 
  CheckCircle2, 
  ShieldCheck, 
  KeyRound, 
  Download, 
  Copy, 
  BarChart3, 
  ArrowRight,
  ExternalLink,
  QrCode
} from 'lucide-react';
import { motion } from 'framer-motion';

export default function VoteSubmittedReceiptPage() {
  const [copied, setCopied] = useState(false);
  const receiptCode = 'SPR-8821-X9Y0-2024-NEP';
  const blockHeight = 1842904;
  const timestamp = '2026-08-01 14:25:30 NDT';

  useEffect(() => {
    // Fire confetti celebration on page load
    try {
      confetti({
        particleCount: 80,
        spread: 70,
        origin: { y: 0.6 }
      });
    } catch (e) {
      console.log('Confetti loaded');
    }
  }, []);

  const handleCopy = () => {
    navigator.clipboard.writeText(receiptCode);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="bg-surface-bright min-h-screen py-12">
      <div className="max-w-3xl mx-auto px-4 sm:px-6 lg:px-8 space-y-8">
        
        {/* Top Success Badge */}
        <motion.div 
          initial={{ opacity: 0, scale: 0.95 }}
          animate={{ opacity: 1, scale: 1 }}
          className="text-center space-y-3"
        >
          <div className="w-16 h-16 rounded-3xl bg-emerald-green text-white flex items-center justify-center mx-auto shadow-xl shadow-emerald-green/30">
            <CheckCircle2 className="w-10 h-10" />
          </div>

          <h1 className="font-display font-extrabold text-3xl sm:text-4xl text-deep-navy">
            Encrypted Ballot Successfully Submitted!
          </h1>

          <p className="text-sm text-slate-600 max-w-lg mx-auto">
            Your vote has been encrypted using Paillier homomorphic cryptography and sealed into the sovereign election ledger of Nepal.
          </p>
        </motion.div>

        {/* Cryptographic Receipt Card */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.15 }}
          className="bg-white border-2 border-border-gray rounded-3xl p-6 sm:p-8 shadow-xl space-y-6"
        >
          <div className="flex items-center justify-between border-b border-slate-100 pb-4">
            <div>
              <span className="text-[10px] font-bold text-slate-400 uppercase tracking-widest block">
                Official Cryptographic Receipt Code
              </span>
              <div className="font-mono text-2xl sm:text-3xl font-extrabold text-deep-navy tracking-wider pt-1">
                {receiptCode}
              </div>
            </div>

            <button
              onClick={handleCopy}
              className="bg-slate-100 hover:bg-slate-200 text-deep-navy text-xs font-bold px-4 py-2.5 rounded-xl transition-colors flex items-center gap-1.5"
            >
              <Copy className="w-4 h-4 text-secondary-crimson" />
              <span>{copied ? 'Copied!' : 'Copy Code'}</span>
            </button>
          </div>

          {/* QR Code & Technical Details Grid */}
          <div className="grid grid-cols-1 sm:grid-cols-12 gap-6 items-center">
            
            {/* QR Mockup */}
            <div className="sm:col-span-4 bg-slate-50 border border-slate-200 p-4 rounded-2xl text-center space-y-2">
              <div className="w-32 h-32 bg-deep-navy rounded-xl mx-auto flex items-center justify-center text-white p-2">
                <QrCode className="w-24 h-24 text-emerald-green" />
              </div>
              <span className="text-[10px] font-mono text-slate-500 block">
                SCAN TO VERIFY LEDGER
              </span>
            </div>

            {/* Audit Details */}
            <div className="sm:col-span-8 space-y-3 text-xs">
              <div className="bg-soft-blue-bg/60 p-3 rounded-xl border border-slate-200 space-y-1">
                <div className="text-[10px] font-bold text-slate-400 uppercase">Election Title</div>
                <div className="font-bold text-deep-navy text-sm">2024 Federal Representative Elections</div>
              </div>

              <div className="grid grid-cols-2 gap-3">
                <div className="bg-slate-50 p-3 rounded-xl border border-slate-200">
                  <div className="text-[10px] text-slate-400">Block Height</div>
                  <div className="font-mono font-bold text-deep-navy text-sm">#{blockHeight}</div>
                </div>
                <div className="bg-slate-50 p-3 rounded-xl border border-slate-200">
                  <div className="text-[10px] text-slate-400">Timestamp (NDT)</div>
                  <div className="font-mono font-bold text-deep-navy text-xs">{timestamp}</div>
                </div>
              </div>

              <div className="bg-slate-900 text-white p-3 rounded-xl font-mono text-[11px] space-y-1">
                <div className="text-emerald-green flex items-center gap-1 font-bold">
                  <ShieldCheck className="w-3.5 h-3.5" /> ZERO-KNOWLEDGE PROOF VALID
                </div>
                <div className="text-slate-400 truncate">
                  MERKLE_ROOT: 0x8a92f03b41c998de4410a887b02921c5
                </div>
              </div>
            </div>

          </div>

          {/* Action Buttons */}
          <div className="pt-4 border-t border-slate-100 flex flex-col sm:flex-row gap-3">
            <button
              onClick={() => {
                alert(`Downloading official PDF receipt for ${receiptCode}...`);
              }}
              className="flex-1 py-3 bg-slate-900 hover:bg-slate-800 text-white text-xs font-bold rounded-xl shadow transition-colors flex items-center justify-center gap-2"
            >
              <Download className="w-4 h-4 text-emerald-green" />
              <span>Download Official PDF Receipt</span>
            </button>

            <Link
              href="/results"
              className="flex-1 py-3 bg-secondary-crimson hover:bg-dark-red text-white text-xs font-bold rounded-xl shadow transition-colors flex items-center justify-center gap-2"
            >
              <BarChart3 className="w-4 h-4" />
              <span>View Live Election Results</span>
              <ArrowRight className="w-3.5 h-3.5" />
            </Link>
          </div>

        </motion.div>

        {/* Info Box */}
        <div className="bg-emerald-green/10 border border-emerald-green/30 rounded-2xl p-4 text-xs text-slate-700 flex items-start gap-3">
          <ShieldCheck className="w-5 h-5 text-emerald-green shrink-0 mt-0.5" />
          <div>
            <span className="font-bold text-deep-navy block">Notice on Voter Privacy:</span>
            Your choice remains strictly confidential. DIGIMAT receipt codes verify ballot inclusion on the ledger without revealing which candidate or party was selected.
          </div>
        </div>

      </div>
    </div>
  );
}
