'use client';

export default function AdminControlSection() {
  return (
    <section className="py-20 bg-surface-bright border-b border-border-gray">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="grid grid-cols-1 lg:grid-cols-12 gap-12 items-center">
          
          {/* Left Column: Browser Window Mockup */}
          <div className="lg:col-span-6">
            <div className="bg-white border border-border-gray rounded-2xl shadow-xl overflow-hidden">
              {/* Window Bar */}
              <div className="bg-slate-100 px-4 py-3 border-b border-border-gray flex items-center gap-2">
                <span className="w-3 h-3 rounded-full bg-red-400" />
                <span className="w-3 h-3 rounded-full bg-amber-400" />
                <span className="w-3 h-3 rounded-full bg-green-400" />
              </div>

              {/* Inner Dashboard Content */}
              <div className="p-6 space-y-6">
                <div className="space-y-2">
                  <div className="flex justify-between items-baseline">
                    <h3 className="font-display font-bold text-lg text-deep-navy">
                      Kathmandu Metropolitan Election
                    </h3>
                    <span className="font-mono font-extrabold text-deep-navy text-sm">78.4%</span>
                  </div>
                  <div className="text-[11px] text-slate-500 font-sans">Turnout Progress</div>

                  {/* Red Turnout Bar */}
                  <div className="w-full bg-slate-100 rounded-full h-3 overflow-hidden">
                    <div className="bg-secondary-crimson h-full rounded-full w-[78.4%]" />
                  </div>
                </div>

                {/* Internal Mockup Dashboard Preview */}
                <div className="bg-slate-900 rounded-xl p-4 text-white space-y-3">
                  <div className="flex justify-between text-[11px] font-mono text-slate-400 border-b border-slate-800 pb-2">
                    <span>VOTES CAST: 1,483,024</span>
                    <span>STATUS: ACTIVE</span>
                  </div>
                  <div className="h-24 flex items-end justify-between gap-1.5 pt-2">
                    {[40, 60, 85, 70, 95, 65, 80, 100, 75, 90].map((val, idx) => (
                      <div key={idx} className="w-full bg-slate-800 rounded-t h-full flex items-end">
                        <div 
                          className="w-full bg-slate-300 rounded-t"
                          style={{ height: `${val}%` }}
                        />
                      </div>
                    ))}
                  </div>
                </div>
              </div>

            </div>
          </div>

          {/* Right Column: Text Content */}
          <div className="lg:col-span-6 space-y-6">
            <h2 className="font-display font-extrabold text-3xl sm:text-4xl text-deep-navy tracking-tight">
              Total Control for Administrators
            </h2>

            <p className="text-xs sm:text-sm text-slate-600 leading-relaxed font-sans">
              Our dashboard provides real-time oversight of your election's health without compromising individual voter anonymity. Monitor turnout, verify server integrity, and export results with one click.
            </p>

            <div className="space-y-5 pt-2">
              <div className="flex items-start gap-3">
                <span className="w-2.5 h-2.5 rounded-full bg-secondary-crimson shrink-0 mt-1.5" />
                <div>
                  <h4 className="font-display font-bold text-sm text-deep-navy">Real-time Turnout Monitoring</h4>
                  <p className="text-xs text-slate-500 font-sans">Live updates on how many ballots have been securely cast.</p>
                </div>
              </div>

              <div className="flex items-start gap-3">
                <span className="w-2.5 h-2.5 rounded-full bg-secondary-crimson shrink-0 mt-1.5" />
                <div>
                  <h4 className="font-display font-bold text-sm text-deep-navy">Automated Voter Roll Management</h4>
                  <p className="text-xs text-slate-500 font-sans">Securely import and manage voter lists with encrypted identifiers.</p>
                </div>
              </div>

              <div className="flex items-start gap-3">
                <span className="w-2.5 h-2.5 rounded-full bg-secondary-crimson shrink-0 mt-1.5" />
                <div>
                  <h4 className="font-display font-bold text-sm text-deep-navy">Immutable Results Ledger</h4>
                  <p className="text-xs text-slate-500 font-sans">Results are written to a secure blockchain-inspired ledger for auditing.</p>
                </div>
              </div>
            </div>

          </div>

        </div>
      </div>
    </section>
  );
}
