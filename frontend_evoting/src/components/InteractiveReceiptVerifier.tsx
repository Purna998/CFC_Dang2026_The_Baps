'use client';

import { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { KeyRound, Search, CheckCircle2, ShieldCheck, Copy, Download, RefreshCw } from 'lucide-react';

export default function InteractiveReceiptVerifier() {
  const [receiptInput, setReceiptInput] = useState('');
  const [isVerifying, setIsVerifying] = useState(false);
  const [verificationResult, setVerificationResult] = useState<any>(null);

  const sampleReceipts = [
    'SPR-8821-X9Y0-2024-NEP',
    'NEA-3341-B992-2026-NEP',
    'LMC-9910-K442-2026-NEP'
  ];

  const handleVerify = (codeToTest?: string) => {
    const code = codeToTest || receiptInput;
    if (!code.trim()) return;

    setIsVerifying(true);
    setVerificationResult(null);

    setTimeout(() => {
      setIsVerifying(false);
      setVerificationResult({
        receiptCode: code.toUpperCase(),
        status: 'VERIFIED',
        election: '2024 Federal Representative Elections (Kathmandu No. 1)',
        blockHeight: 1842904,
        timestamp: '2026-08-01 14:22:09 NDT',
        merkleRoot: '0x8a92f03b41c998de4410a887b02921c5',
        zkProofStatus: 'Zero-Knowledge Range Proof Validated',
        paillierStatus: 'Encrypted Tally Sum Confirmed',
        nodeSignature: 'SIG_GOV_NEPAL_NODE_01_SEC256'
      });
    }, 1000);
  };

  return (
    <section className="py-20 bg-white border-b border-border-gray">
      <div className="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8">
        
        <div className="bg-gradient-to-br from-deep-navy to-primary-container text-white rounded-3xl p-8 sm:p-12 shadow-2xl border border-slate-700 relative overflow-hidden">
          
          {/* Subtle Background Accent */}
          <div className="absolute top-0 right-0 w-96 h-96 bg-secondary-crimson/10 rounded-full blur-3xl pointer-events-none" />

          <div className="relative z-10 text-center max-w-2xl mx-auto space-y-4">
            <div className="inline-flex items-center gap-2 text-xs font-bold text-emerald-green bg-emerald-green/10 border border-emerald-green/20 px-3.5 py-1 rounded-full">
              <KeyRound className="w-4 h-4" />
              <span>PUBLIC RECEIPT VERIFIER</span>
            </div>

            <h2 className="font-display font-extrabold text-2xl sm:text-3xl lg:text-4xl text-white">
              Verify Your Cryptographic Ballot Receipt
            </h2>

            <p className="text-xs sm:text-sm text-slate-300">
              Input your unique receipt code below to independently verify that your encrypted vote was included in the election ledger without breaking anonymity.
            </p>

            {/* Input Bar */}
            <div className="pt-4 flex flex-col sm:flex-row gap-3 max-w-xl mx-auto">
              <div className="relative flex-1">
                <Search className="w-5 h-5 text-slate-400 absolute left-4 top-1/2 -translate-y-1/2" />
                <input
                  type="text"
                  value={receiptInput}
                  onChange={(e) => setReceiptInput(e.target.value)}
                  placeholder="e.g. SPR-8821-X9Y0-2024-NEP"
                  className="w-full bg-slate-900/90 border border-slate-700 focus:border-emerald-green text-white font-mono text-sm pl-12 pr-4 py-3.5 rounded-xl outline-none placeholder:text-slate-500"
                />
              </div>

              <button
                onClick={() => handleVerify()}
                disabled={isVerifying}
                className="bg-secondary-crimson hover:bg-dark-red text-white font-display text-xs font-bold px-6 py-3.5 rounded-xl shadow-lg transition-all flex items-center justify-center gap-2 shrink-0 disabled:opacity-50"
              >
                {isVerifying ? (
                  <>
                    <RefreshCw className="w-4 h-4 animate-spin" />
                    <span>Verifying Ledger...</span>
                  </>
                ) : (
                  <>
                    <ShieldCheck className="w-4 h-4" />
                    <span>Verify Receipt</span>
                  </>
                )}
              </button>
            </div>

            {/* Sample Quick Click Tokens */}
            <div className="flex flex-wrap items-center justify-center gap-2 pt-2">
              <span className="text-[11px] text-slate-400">Try sample tokens:</span>
              {sampleReceipts.map((code, idx) => (
                <button
                  key={idx}
                  onClick={() => {
                    setReceiptInput(code);
                    handleVerify(code);
                  }}
                  className="text-[10px] font-mono font-semibold bg-slate-800/80 hover:bg-slate-700 text-slate-200 px-2.5 py-1 rounded-md border border-slate-700 transition-colors"
                >
                  {code}
                </button>
              ))}
            </div>

          </div>

          {/* Verification Result Modal / Card */}
          <AnimatePresence>
            {verificationResult && (
              <motion.div
                initial={{ opacity: 0, y: 20, scale: 0.98 }}
                animate={{ opacity: 1, y: 0, scale: 1 }}
                exit={{ opacity: 0, y: 20 }}
                className="mt-8 bg-slate-900 border border-emerald-green/40 rounded-2xl p-6 sm:p-8 space-y-6 text-left relative"
              >
                <div className="flex flex-wrap items-center justify-between gap-4 border-b border-slate-800 pb-4">
                  <div className="flex items-center gap-3">
                    <div className="w-10 h-10 rounded-xl bg-emerald-green/20 text-emerald-green flex items-center justify-center border border-emerald-green/40">
                      <CheckCircle2 className="w-6 h-6" />
                    </div>
                    <div>
                      <div className="text-xs font-bold text-emerald-green uppercase tracking-wider">
                        STATUS: VERIFIED ON PUBLIC LEDGER
                      </div>
                      <div className="font-mono text-base font-extrabold text-white">
                        {verificationResult.receiptCode}
                      </div>
                    </div>
                  </div>

                  <div className="text-right">
                    <div className="text-[10px] text-slate-400 uppercase">Block Height</div>
                    <div className="font-mono text-sm font-bold text-white">#{verificationResult.blockHeight}</div>
                  </div>
                </div>

                <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 text-xs font-sans">
                  <div className="bg-slate-950 p-3.5 rounded-xl border border-slate-800">
                    <span className="text-slate-400 block text-[10px]">Election Context</span>
                    <span className="font-semibold text-slate-200">{verificationResult.election}</span>
                  </div>
                  <div className="bg-slate-950 p-3.5 rounded-xl border border-slate-800">
                    <span className="text-slate-400 block text-[10px]">Timestamp (NDT)</span>
                    <span className="font-mono font-semibold text-slate-200">{verificationResult.timestamp}</span>
                  </div>
                  <div className="bg-slate-950 p-3.5 rounded-xl border border-slate-800">
                    <span className="text-slate-400 block text-[10px]">Zero-Knowledge Proof</span>
                    <span className="font-semibold text-emerald-green">{verificationResult.zkProofStatus}</span>
                  </div>
                  <div className="bg-slate-950 p-3.5 rounded-xl border border-slate-800">
                    <span className="text-slate-400 block text-[10px]">Homomorphic Tally</span>
                    <span className="font-semibold text-emerald-green">{verificationResult.paillierStatus}</span>
                  </div>
                </div>

                <div className="bg-slate-950 p-3.5 rounded-xl border border-slate-800 font-mono text-[11px] space-y-1">
                  <div className="text-slate-400 flex justify-between">
                    <span>Merkle Root Hash:</span>
                    <span className="text-slate-300">{verificationResult.merkleRoot}</span>
                  </div>
                  <div className="text-slate-400 flex justify-between">
                    <span>Node Signature:</span>
                    <span className="text-slate-300">{verificationResult.nodeSignature}</span>
                  </div>
                </div>

                <div className="flex justify-end gap-3 pt-2">
                  <button
                    onClick={() => {
                      navigator.clipboard.writeText(JSON.stringify(verificationResult, null, 2));
                      alert('Cryptographic proof JSON copied to clipboard!');
                    }}
                    className="text-xs font-bold text-slate-300 hover:text-white px-4 py-2 bg-slate-800 rounded-lg flex items-center gap-1.5"
                  >
                    <Copy className="w-3.5 h-3.5" />
                    Copy Proof JSON
                  </button>
                </div>
              </motion.div>
            )}
          </AnimatePresence>

        </div>

      </div>
    </section>
  );
}
