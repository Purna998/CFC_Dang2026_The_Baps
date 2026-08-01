'use client';

import { useState } from 'react';
import Link from 'next/link';
import { MOCK_ELECTIONS } from '@/lib/data';
import { 
  UserCheck, 
  ShieldCheck, 
  Vote, 
  KeyRound, 
  CheckCircle2, 
  Clock, 
  MapPin, 
  ArrowRight, 
  Download, 
  ExternalLink,
  Lock
} from 'lucide-react';
import { motion } from 'framer-motion';

export default function VoterDashboardPage() {
  const [activeTab, setActiveTab] = useState<'active' | 'receipts'>('active');

  const voterProfile = {
    name: 'Aayush Shrestha',
    nid: 'NEP-8842-1994-01',
    constituency: 'Kathmandu Constituency No. 1',
    municipality: 'Kathmandu Metropolitan City Ward 4',
    status: 'Verified Citizen Voter',
    keyStatus: 'Active RSA-4096 / ZK Pair'
  };

  const pastReceipts = [
    {
      id: 'REC-9910-2025-FED',
      election: '2025 Provincial Representative Council',
      date: 'Nov 12, 2025',
      receiptCode: 'SPR-8821-X9Y0-2025-NEP',
      status: 'AUDITED & CONFIRMED'
    },
    {
      id: 'REC-4412-2025-LMC',
      election: 'Lalitpur Heritage Advisory Board',
      date: 'May 04, 2025',
      receiptCode: 'LMC-9910-K442-2025-NEP',
      status: 'AUDITED & CONFIRMED'
    },
    {
      id: 'REC-1092-2024-NEA',
      election: '33rd Executive Council NEA Election',
      date: 'Aug 20, 2024',
      receiptCode: 'NEA-3341-B992-2024-NEP',
      status: 'AUDITED & CONFIRMED'
    }
  ];

  return (
    <div className="bg-surface-bright min-h-screen py-10">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 space-y-8">
        
        {/* Top Header Card */}
        <motion.div
          initial={{ opacity: 0, y: 15 }}
          animate={{ opacity: 1, y: 0 }}
          className="bg-deep-navy text-white rounded-3xl p-6 sm:p-8 shadow-xl border border-slate-800 pattern-overlay flex flex-col md:flex-row md:items-center justify-between gap-6"
        >
          <div className="space-y-2">
            <div className="flex items-center gap-2">
              <span className="bg-emerald-green/20 text-emerald-green border border-emerald-green/40 text-[10px] font-bold px-2.5 py-0.5 rounded-full flex items-center gap-1">
                <CheckCircle2 className="w-3 h-3" /> NID VERIFIED CITIZEN
              </span>
              <span className="text-xs text-slate-400 font-mono">ID: {voterProfile.nid}</span>
            </div>

            <h1 className="font-display font-extrabold text-2xl sm:text-3xl text-white">
              Namaste, {voterProfile.name}
            </h1>

            <p className="text-xs sm:text-sm text-slate-300 flex items-center gap-1.5">
              <MapPin className="w-4 h-4 text-secondary-crimson shrink-0" />
              <span>{voterProfile.constituency} • {voterProfile.municipality}</span>
            </p>
          </div>

          {/* Quick Identity Credentials */}
          <div className="bg-slate-900/90 border border-slate-700 rounded-2xl p-4 flex items-center gap-4 text-xs">
            <div className="w-10 h-10 rounded-xl bg-emerald-green/20 text-emerald-green flex items-center justify-center border border-emerald-green/30">
              <Lock className="w-5 h-5" />
            </div>
            <div>
              <div className="font-bold text-white">Cryptographic Voting Key</div>
              <div className="text-[11px] text-emerald-green font-mono">{voterProfile.keyStatus}</div>
            </div>
          </div>
        </motion.div>

        {/* Stats Row */}
        <div className="grid grid-cols-1 sm:grid-cols-3 gap-6">
          <div className="bg-white border border-border-gray rounded-2xl p-5 shadow-sm space-y-1">
            <div className="text-xs font-semibold text-slate-500">Active Eligible Elections</div>
            <div className="text-2xl font-extrabold text-deep-navy font-display flex items-center gap-2">
              <span>{MOCK_ELECTIONS.length} Active</span>
              <span className="w-2 h-2 rounded-full bg-emerald-green animate-ping" />
            </div>
            <div className="text-[11px] text-slate-400">Ready for instant voting</div>
          </div>

          <div className="bg-white border border-border-gray rounded-2xl p-5 shadow-sm space-y-1">
            <div className="text-xs font-semibold text-slate-500">Verified Ballots Cast</div>
            <div className="text-2xl font-extrabold text-deep-navy font-display">12 Total</div>
            <div className="text-[11px] text-emerald-green font-bold">100% Inclusion Verified</div>
          </div>

          <div className="bg-white border border-border-gray rounded-2xl p-5 shadow-sm space-y-1">
            <div className="text-xs font-semibold text-slate-500">Security & Privacy Score</div>
            <div className="text-2xl font-extrabold text-emerald-green font-display">OPTIMAL (100%)</div>
            <div className="text-[11px] text-slate-400">Zero-Knowledge Encrypted</div>
          </div>
        </div>

        {/* Main Content Area */}
        <div className="space-y-6">
          
          {/* Navigation Tabs */}
          <div className="flex border-b border-border-gray gap-8 font-display text-sm font-bold text-slate-600">
            <button
              onClick={() => setActiveTab('active')}
              className={`pb-3 border-b-2 transition-colors flex items-center gap-2 ${
                activeTab === 'active' 
                  ? 'border-secondary-crimson text-deep-navy font-extrabold' 
                  : 'border-transparent hover:text-deep-navy'
              }`}
            >
              <Vote className="w-4 h-4 text-secondary-crimson" />
              Active Elections Available
            </button>

            <button
              onClick={() => setActiveTab('receipts')}
              className={`pb-3 border-b-2 transition-colors flex items-center gap-2 ${
                activeTab === 'receipts' 
                  ? 'border-secondary-crimson text-deep-navy font-extrabold' 
                  : 'border-transparent hover:text-deep-navy'
              }`}
            >
              <KeyRound className="w-4 h-4 text-emerald-green" />
              Past Receipts & Verification History
            </button>
          </div>

          {/* Tab 1: Active Elections Cards */}
          {activeTab === 'active' && (
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
              {MOCK_ELECTIONS.map((elec) => (
                <div
                  key={elec.id}
                  className="bg-white border border-border-gray rounded-2xl p-6 shadow-md hover:shadow-xl transition-all space-y-4 flex flex-col justify-between"
                >
                  <div className="space-y-3">
                    <div className="flex items-center justify-between">
                      <span className="text-[10px] font-bold bg-slate-100 text-deep-navy px-2.5 py-1 rounded-md border border-slate-200 uppercase">
                        {elec.category}
                      </span>
                      <span className="text-[11px] font-bold text-emerald-green flex items-center gap-1">
                        <span className="w-2 h-2 rounded-full bg-emerald-green animate-pulse" />
                        POLLS OPEN
                      </span>
                    </div>

                    <h3 className="font-display font-bold text-lg text-deep-navy leading-snug">
                      {elec.title}
                    </h3>

                    <p className="text-xs text-slate-600 line-clamp-2">
                      {elec.description}
                    </p>

                    <div className="pt-2 space-y-1.5 text-xs text-slate-500 font-sans">
                      <div className="flex justify-between">
                        <span>Constituency:</span>
                        <span className="font-semibold text-slate-700">{elec.location}</span>
                      </div>
                      <div className="flex justify-between">
                        <span>Poll Closing:</span>
                        <span className="font-mono text-secondary-crimson font-bold">{elec.endDate}</span>
                      </div>
                    </div>
                  </div>

                  <div className="pt-4 border-t border-slate-100">
                    <Link
                      href={`/vote/${elec.id}`}
                      className="w-full py-3 bg-secondary-crimson hover:bg-dark-red text-white text-xs font-bold rounded-xl shadow-md transition-colors flex items-center justify-center gap-2"
                    >
                      <Vote className="w-4 h-4" />
                      <span>Cast Ballot Now</span>
                      <ArrowRight className="w-3.5 h-3.5" />
                    </Link>
                  </div>
                </div>
              ))}
            </div>
          )}

          {/* Tab 2: Past Receipts Table */}
          {activeTab === 'receipts' && (
            <div className="bg-white border border-border-gray rounded-2xl shadow-sm overflow-hidden">
              <div className="p-5 bg-slate-50 border-b border-border-gray flex justify-between items-center">
                <span className="font-display font-bold text-sm text-deep-navy">
                  Verified Cryptographic Receipts & Receipts Log
                </span>
                <span className="text-xs text-slate-500 font-mono">
                  TOTAL RECORDED: {pastReceipts.length}
                </span>
              </div>

              <div className="overflow-x-auto">
                <table className="w-full text-left text-xs">
                  <thead className="bg-slate-100 font-bold text-slate-600 border-b border-slate-200">
                    <tr>
                      <th className="p-4">Election Name</th>
                      <th className="p-4">Date Cast</th>
                      <th className="p-4">Cryptographic Receipt Code</th>
                      <th className="p-4">Verification Status</th>
                      <th className="p-4 text-right">Action</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-slate-100">
                    {pastReceipts.map((rec, idx) => (
                      <tr key={idx} className="hover:bg-slate-50 transition-colors">
                        <td className="p-4 font-bold text-deep-navy">{rec.election}</td>
                        <td className="p-4 text-slate-600">{rec.date}</td>
                        <td className="p-4 font-mono font-bold text-secondary-crimson">{rec.receiptCode}</td>
                        <td className="p-4">
                          <span className="bg-emerald-green/10 text-emerald-green border border-emerald-green/30 px-2.5 py-0.5 rounded font-bold text-[10px]">
                            ✓ {rec.status}
                          </span>
                        </td>
                        <td className="p-4 text-right">
                          <Link
                            href="/receipt"
                            className="inline-flex items-center gap-1 text-xs font-bold text-deep-navy hover:text-secondary-crimson"
                          >
                            <span>Verify Ledger</span>
                            <ExternalLink className="w-3.5 h-3.5" />
                          </Link>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          )}

        </div>

      </div>
    </div>
  );
}
