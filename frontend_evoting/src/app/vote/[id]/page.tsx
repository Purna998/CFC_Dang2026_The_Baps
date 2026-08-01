'use client';

import { useState } from 'react';
import { useRouter, useParams } from 'next/navigation';
import { MOCK_ELECTIONS, Candidate } from '@/lib/data';
import { 
  ShieldCheck, 
  Vote, 
  Lock, 
  CheckCircle2, 
  ArrowRight, 
  AlertCircle, 
  RefreshCw,
  KeyRound,
  FileText
} from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';

export default function ElectronicBallotPage() {
  const router = useRouter();
  const params = useParams();
  const electionId = (params?.id as string) || 'fed-rep-2024';

  const election = MOCK_ELECTIONS.find((e) => e.id === electionId) || MOCK_ELECTIONS[0];
  
  const [selectedCandidate, setSelectedCandidate] = useState<string | null>(null);
  const [isConfirmModalOpen, setIsConfirmModalOpen] = useState(false);
  const [otpInput, setOtpInput] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleCastVote = () => {
    if (!selectedCandidate) return;
    setIsConfirmModalOpen(true);
  };

  const handleFinalSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    setIsSubmitting(true);

    setTimeout(() => {
      setIsSubmitting(false);
      // Navigate to receipt with celebratory receipt code
      router.push('/receipt');
    }, 1500);
  };

  return (
    <div className="bg-surface-bright min-h-screen py-10">
      <div className="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 space-y-8">
        
        {/* Ballot Top Header Banner */}
        <div className="bg-deep-navy text-white rounded-3xl p-6 sm:p-8 shadow-xl border border-slate-800 pattern-overlay space-y-3">
          <div className="flex flex-wrap items-center justify-between gap-3">
            <span className="bg-secondary-crimson text-white text-[10px] font-bold px-3 py-1 rounded-full uppercase tracking-wider">
              OFFICIAL ELECTRONIC BALLOT
            </span>
            <span className="text-xs font-mono text-emerald-green font-bold flex items-center gap-1.5">
              <Lock className="w-3.5 h-3.5" /> PAILLIER CLIENT-SIDE ENCRYPTION ACTIVE
            </span>
          </div>

          <h1 className="font-display font-extrabold text-2xl sm:text-3xl text-white">
            {election.title}
          </h1>

          <p className="text-xs sm:text-sm text-slate-300">
            {election.description} • <span className="font-semibold text-white">{election.location}</span>
          </p>
        </div>

        {/* Candidate Selection List */}
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <h2 className="font-display font-bold text-lg text-deep-navy flex items-center gap-2">
              <Vote className="w-5 h-5 text-secondary-crimson" />
              <span>Select Your Nominated Candidate</span>
            </h2>
            <span className="text-xs text-slate-500 font-sans">
              Choose 1 candidate from the ballot list below
            </span>
          </div>

          <div className="grid grid-cols-1 gap-4">
            {election.candidates.map((cand: Candidate) => {
              const isSelected = selectedCandidate === cand.id;
              return (
                <motion.div
                  key={cand.id}
                  onClick={() => setSelectedCandidate(cand.id)}
                  whileHover={{ scale: 1.005 }}
                  className={`bg-white border-2 rounded-2xl p-6 shadow-md cursor-pointer transition-all ${
                    isSelected 
                      ? 'border-secondary-crimson ring-4 ring-secondary-crimson/10 bg-soft-blue-bg/30' 
                      : 'border-border-gray hover:border-slate-300'
                  }`}
                >
                  <div className="flex flex-col sm:flex-row items-start sm:items-center justify-between gap-6">
                    
                    {/* Candidate Info */}
                    <div className="flex items-center gap-4">
                      <div className="relative">
                        <img
                          src={cand.avatar}
                          alt={cand.name}
                          className="w-16 h-16 rounded-full object-cover border-2 border-slate-200"
                        />
                        <div className="absolute -bottom-1 -right-1 w-7 h-7 rounded-full bg-deep-navy text-white text-xs flex items-center justify-center border border-white">
                          {cand.partySymbol}
                        </div>
                      </div>

                      <div>
                        <h3 className="font-display font-bold text-lg text-deep-navy">
                          {cand.name}
                        </h3>
                        <div className="text-xs font-semibold text-secondary-crimson">
                          {cand.party}
                        </div>
                        <div className="text-xs text-slate-500">
                          {cand.position}
                        </div>
                      </div>
                    </div>

                    {/* Radio Selector */}
                    <div className="flex items-center gap-3">
                      <div className={`w-6 h-6 rounded-full border-2 flex items-center justify-center transition-colors ${
                        isSelected ? 'border-secondary-crimson bg-secondary-crimson text-white' : 'border-slate-300'
                      }`}>
                        {isSelected && <CheckCircle2 className="w-4 h-4 fill-white text-secondary-crimson" />}
                      </div>
                      <span className="text-xs font-bold text-deep-navy">
                        {isSelected ? 'SELECTED' : 'SELECT'}
                      </span>
                    </div>

                  </div>

                  {/* Manifesto highlights */}
                  <div className="mt-4 pt-4 border-t border-slate-100 space-y-1.5">
                    <div className="text-[11px] font-bold text-slate-400 uppercase tracking-wider">
                      Key Manifesto Commitments:
                    </div>
                    <div className="flex flex-wrap gap-2">
                      {cand.manifesto.map((item, idx) => (
                        <span key={idx} className="bg-slate-100 text-slate-700 text-[11px] font-medium px-2.5 py-1 rounded-md">
                          • {item}
                        </span>
                      ))}
                    </div>
                  </div>
                </motion.div>
              );
            })}
          </div>
        </div>

        {/* Bottom Cast Vote Bar */}
        <div className="sticky bottom-6 bg-deep-navy text-white rounded-2xl p-4 sm:p-5 shadow-2xl border border-slate-700 flex flex-col sm:flex-row items-center justify-between gap-4 z-40">
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 rounded-xl bg-emerald-green/20 text-emerald-green flex items-center justify-center border border-emerald-green/30">
              <ShieldCheck className="w-6 h-6" />
            </div>
            <div>
              <div className="text-xs text-slate-300">Selected Candidate:</div>
              <div className="font-display font-bold text-white text-sm">
                {selectedCandidate 
                  ? election.candidates.find(c => c.id === selectedCandidate)?.name 
                  : 'No candidate selected yet'}
              </div>
            </div>
          </div>

          <button
            onClick={handleCastVote}
            disabled={!selectedCandidate}
            className="w-full sm:w-auto bg-secondary-crimson hover:bg-dark-red disabled:opacity-40 disabled:hover:bg-secondary-crimson text-white font-display text-xs font-bold px-8 py-3.5 rounded-xl shadow-lg transition-all flex items-center justify-center gap-2"
          >
            <span>Proceed to Digital Signature</span>
            <ArrowRight className="w-4 h-4" />
          </button>
        </div>

      </div>

      {/* Confirmation & OTP Modal */}
      <AnimatePresence>
        {isConfirmModalOpen && (
          <div className="fixed inset-0 z-50 bg-black/80 backdrop-blur-md flex items-center justify-center p-4">
            <motion.div
              initial={{ opacity: 0, scale: 0.95 }}
              animate={{ opacity: 1, scale: 1 }}
              exit={{ opacity: 0, scale: 0.95 }}
              className="bg-slate-900 border border-slate-700 rounded-3xl max-w-lg w-full p-6 sm:p-8 space-y-6 text-white"
            >
              <div className="flex items-center justify-between border-b border-slate-800 pb-4">
                <div className="flex items-center gap-2">
                  <KeyRound className="w-5 h-5 text-emerald-green" />
                  <span className="font-display font-extrabold text-lg text-white">
                    Confirm Ballot & Digital Signature
                  </span>
                </div>
                <button
                  onClick={() => setIsConfirmModalOpen(false)}
                  className="text-slate-400 hover:text-white text-sm"
                >
                  ✕
                </button>
              </div>

              <div className="bg-slate-950 p-4 rounded-xl border border-slate-800 space-y-2 text-xs">
                <div className="text-slate-400">Selected Choice:</div>
                <div className="font-display font-bold text-emerald-green text-base">
                  {election.candidates.find(c => c.id === selectedCandidate)?.name} ({election.candidates.find(c => c.id === selectedCandidate)?.party})
                </div>
                <div className="text-[11px] text-slate-500 font-mono pt-1 border-t border-slate-900">
                  ENC_SCHEME: Paillier ZK-Proof • NID: NEP-8842-1994-01
                </div>
              </div>

              {/* Form with OTP */}
              <form onSubmit={handleFinalSubmit} className="space-y-4">
                <div className="space-y-1.5">
                  <label className="text-xs font-semibold text-slate-300">
                    Enter Citizen Biometric OTP Code (Sent to registered mobile)
                  </label>
                  <input
                    type="text"
                    required
                    maxLength={6}
                    value={otpInput}
                    onChange={(e) => setOtpInput(e.target.value)}
                    placeholder="e.g. 884201"
                    className="w-full bg-slate-950 border border-slate-700 focus:border-emerald-green text-white font-mono text-center text-lg tracking-widest py-3 rounded-xl outline-none"
                  />
                  <span className="text-[10px] text-slate-400 block text-right">
                    Demo OTP: Enter any 6 digits (e.g. 884201)
                  </span>
                </div>

                <div className="pt-2 flex items-center justify-end gap-3">
                  <button
                    type="button"
                    onClick={() => setIsConfirmModalOpen(false)}
                    className="px-5 py-3 bg-slate-800 hover:bg-slate-700 text-xs font-bold rounded-xl text-slate-300"
                  >
                    Cancel
                  </button>

                  <button
                    type="submit"
                    disabled={isSubmitting || otpInput.length < 4}
                    className="px-6 py-3 bg-secondary-crimson hover:bg-dark-red disabled:opacity-50 text-white font-display text-xs font-bold rounded-xl shadow-lg transition-all flex items-center gap-2"
                  >
                    {isSubmitting ? (
                      <>
                        <RefreshCw className="w-4 h-4 animate-spin" />
                        <span>Sealing & Encrypting Vote...</span>
                      </>
                    ) : (
                      <>
                        <Vote className="w-4 h-4" />
                        <span>Seal & Submit Encrypted Ballot</span>
                      </>
                    )}
                  </button>
                </div>
              </form>

            </motion.div>
          </div>
        )}
      </AnimatePresence>

    </div>
  );
}
