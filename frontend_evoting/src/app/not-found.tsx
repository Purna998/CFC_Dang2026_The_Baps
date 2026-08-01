import Link from 'next/link';
import { ShieldAlert, Home } from 'lucide-react';

export default function NotFound() {
  return (
    <div className="bg-surface-bright min-h-[70vh] flex items-center justify-center p-6 text-center">
      <div className="max-w-md space-y-4 bg-white border border-border-gray p-8 rounded-3xl shadow-xl">
        <div className="w-14 h-14 rounded-2xl bg-secondary-crimson/10 text-secondary-crimson flex items-center justify-center mx-auto">
          <ShieldAlert className="w-8 h-8" />
        </div>
        <h2 className="font-display font-extrabold text-2xl text-deep-navy">Page Not Found</h2>
        <p className="text-xs text-slate-600">
          The requested page or electoral resource does not exist on the DIGIMAT network.
        </p>
        <Link
          href="/"
          className="inline-flex items-center gap-2 bg-deep-navy text-white text-xs font-bold px-6 py-3 rounded-xl shadow hover:bg-primary-container transition-colors"
        >
          <Home className="w-4 h-4" />
          <span>Return to Homepage</span>
        </Link>
      </div>
    </div>
  );
}
