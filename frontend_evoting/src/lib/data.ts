export interface Candidate {
  id: string;
  name: string;
  party: string;
  partySymbol: string;
  avatar: string;
  position: string;
  votes: number;
  percentage: number;
  bio: string;
  manifesto: string[];
}

export interface Election {
  id: string;
  title: string;
  category: string;
  status: 'active' | 'upcoming' | 'completed';
  startDate: string;
  endDate: string;
  totalVoters: number;
  votesCast: number;
  location: string;
  description: string;
  candidates: Candidate[];
}

export const MOCK_ELECTIONS: Election[] = [
  {
    id: 'fed-rep-2024',
    title: '2024 Federal Representative Elections',
    category: 'Governmental & Sovereign',
    status: 'active',
    startDate: '2026-08-01 06:00 NDT',
    endDate: '2026-08-01 18:00 NDT',
    totalVoters: 17890400,
    votesCast: 12435201,
    location: 'Kathmandu Constituency No. 1, Nepal',
    description: 'National vote for representatives to the Federal Parliament with zero-knowledge cryptographic verification.',
    candidates: [
      {
        id: 'cand-1',
        name: 'Ramesh Bahadur Giri',
        party: 'Democratic Reform Alliance',
        partySymbol: '🌟',
        avatar: 'https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?auto=format&fit=crop&w=300&q=80',
        position: 'Member of Parliament',
        votes: 4852301,
        percentage: 39.0,
        bio: 'Senior advocate committed to digital governance, transparency, and youth employment in Nepal.',
        manifesto: ['Universal high-speed digital infrastructure', 'Zero-tolerance anti-corruption legislative framework', 'Youth entrepreneurship grants & tech hubs']
      },
      {
        id: 'cand-2',
        name: 'Sita Maya Tamang',
        party: 'National Progressive Coalition',
        partySymbol: '☀️',
        avatar: 'https://images.unsplash.com/photo-1573496359142-b8d87734a5a2?auto=format&fit=crop&w=300&q=80',
        position: 'Member of Parliament',
        votes: 4120150,
        percentage: 33.1,
        bio: 'Environmental economist advocating for renewable energy transition and decentralised provincial development.',
        manifesto: ['Clean Himalayan green energy initiative', 'Provincial digital health and education card', 'Equal gender representation in local councils']
      },
      {
        id: 'cand-3',
        name: 'Prakash Sharma',
        party: 'Independent Citizens Front',
        partySymbol: '⚖️',
        avatar: 'https://images.unsplash.com/photo-1500648767791-00dcc994a43e?auto=format&fit=crop&w=300&q=80',
        position: 'Member of Parliament',
        votes: 3462750,
        percentage: 27.9,
        bio: 'Civic engineer focused on urban mobility, smart Kathmandu transit, and open government data APIs.',
        manifesto: ['Open-source government financial auditing', 'Kathmandu smart transit system', 'De-bureaucratization of small business licensing']
      }
    ]
  },
  {
    id: 'lalitpur-council-2026',
    title: 'Lalitpur Metropolitan Council Appointments',
    category: 'Municipal & Local Government',
    status: 'active',
    startDate: '2026-08-01 08:00 NDT',
    endDate: '2026-08-02 17:00 NDT',
    totalVoters: 245000,
    votesCast: 168400,
    location: 'Lalitpur Metropolitan City',
    description: 'Selection of municipal committee chairs for heritage preservation and urban infrastructure.',
    candidates: [
      {
        id: 'cand-4',
        name: 'Aayush Shrestha',
        party: 'Heritage Preservation Collective',
        partySymbol: '🏛️',
        avatar: 'https://images.unsplash.com/photo-1472099645785-5658abf4ff4e?auto=format&fit=crop&w=300&q=80',
        position: 'Committee Chair',
        votes: 94500,
        percentage: 56.1,
        bio: 'Heritage architect dedicated to restoring Patan Durbar square architecture and sustainable tourism.',
        manifesto: ['Patan cultural corridor restoration', 'Electric public shuttle system', 'Pedestrian-first heritage zone']
      },
      {
        id: 'cand-5',
        name: 'Bina Maharjan',
        party: 'Lalitpur Civic Vision',
        partySymbol: '🌿',
        avatar: 'https://images.unsplash.com/photo-1580489944761-15a19d654956?auto=format&fit=crop&w=300&q=80',
        position: 'Committee Chair',
        votes: 73900,
        percentage: 43.9,
        bio: 'Community organiser focusing on green spaces, waste management, and rainwater harvesting.',
        manifesto: ['Zero-waste municipal recycling program', 'Solar rooftop subsidies for residences', 'Community art & youth centers']
      }
    ]
  },
  {
    id: 'nea-presidential-2026',
    title: 'Nepal Engineers Association Executive Election',
    category: 'Professional Associations',
    status: 'active',
    startDate: '2026-07-30 00:00 NDT',
    endDate: '2026-08-05 23:59 NDT',
    totalVoters: 32000,
    votesCast: 24110,
    location: 'Nationwide (Digital Ballot)',
    description: 'Annual election for the 34th Executive Committee of the Nepal Engineers Association.',
    candidates: [
      {
        id: 'cand-6',
        name: 'Er. Dipendra Adhikari',
        party: 'Engineers for Innovation Panel',
        partySymbol: '⚙️',
        avatar: 'https://images.unsplash.com/photo-1534528741775-53994a69daeb?auto=format&fit=crop&w=300&q=80',
        position: 'NEA President',
        votes: 14200,
        percentage: 58.9,
        bio: 'Structural engineer specializing in earthquake resilience and digital BIM standards.',
        manifesto: ['Global engineering license reciprocity', 'BIM adoption standard for public works', 'Young Engineers Fellowship Program']
      },
      {
        id: 'cand-7',
        name: 'Er. Sabina Thapa',
        party: 'Professional Excellence Forum',
        partySymbol: '📐',
        avatar: 'https://images.unsplash.com/photo-1544005313-94ddf0286df2?auto=format&fit=crop&w=300&q=80',
        position: 'NEA President',
        votes: 9910,
        percentage: 41.1,
        bio: 'Hydropower & environmental engineer advocating for sustainable mountain infrastructure.',
        manifesto: ['Hydropower safety and environmental auditing', 'Continuing engineering education mandatory credits', 'Engineers welfare & insurance pool']
      }
    ]
  }
];

export const CATEGORIES = [
  {
    id: 'gov',
    title: 'Government & Sovereign',
    description: 'Federal elections, municipal councils, and parliamentary referendums with sovereign cryptographic compliance.',
    icon: 'Landmark',
    count: '14 Active Elections',
    status: 'ACTIVE'
  },
  {
    id: 'prof',
    title: 'Professional Associations & Societies',
    description: 'Bar associations, engineering councils, medical federations, and charter institutes.',
    icon: 'Award',
    count: '32 Active Elections',
    status: 'ACTIVE'
  },
  {
    id: 'edu',
    title: 'Education & Universities',
    description: 'Student union elections, senate appointments, and faculty governance boards.',
    icon: 'GraduationCap',
    count: '8 Active Elections',
    status: 'ACTIVE'
  },
  {
    id: 'corp',
    title: 'Corporate & Shareholder Proxy',
    description: 'Board of Directors voting, shareholder resolutions, and annual general meeting (AGM) ballots.',
    icon: 'Building2',
    count: '19 Active Elections',
    status: 'ACTIVE'
  },
  {
    id: 'coop',
    title: 'Cooperatives & Credit Unions',
    description: 'Democratic financial cooperative voting with weighted vote allocation and member verification.',
    icon: 'Coins',
    count: '45 Active Elections',
    status: 'ACTIVE'
  },
  {
    id: 'hoa',
    title: 'Condos & Housing Societies (HOA)',
    description: 'Community board elections, maintenance budget approvals, and amenity voting.',
    icon: 'Home',
    count: '60 Active Elections',
    status: 'ACTIVE'
  }
];

export const SECURITY_FEATURES = [
  {
    title: 'Usable Decryption & Verifiability',
    description: 'Voters receive a mathematical proof token confirming their encrypted vote was tallied into the final result without revealing their identity.',
    icon: 'ShieldCheck',
    badge: 'Zero-Knowledge Proofs'
  },
  {
    title: 'Voter Anonymity & Homomorphic Encryption',
    description: 'Individual ballots are encrypted at the client browser using Elliptic Curve Cryptography before transmission over TLS 1.3.',
    icon: 'Lock',
    badge: '256-bit ECC'
  },
  {
    title: 'Multi-Party Computation (MPC)',
    description: 'Decryption keys are split into cryptographic shares across distributed government audit keyholders so no single authority can inspect raw votes.',
    icon: 'KeyRound',
    badge: 'Threshold Cryptography'
  },
  {
    title: 'Immutable Ledger Audit Trail',
    description: 'Every vote submission appends a cryptographic hash block onto an immutable, public audit log accessible by third-party electoral observers.',
    icon: 'Cpu',
    badge: 'E2E Verifiable'
  }
];

export const FAQS = [
  {
    q: 'How does DIGIMAT guarantee that my vote remains private?',
    a: 'DIGIMAT uses Homomorphic Encryption (Paillier & ElGamal cryptographic schemes). When you cast your vote, your ballot is encrypted on your local device before being sent. The tallying process sums the encrypted ballots directly without ever decrypting individual votes.'
  },
  {
    q: 'How can I verify that my vote was counted correctly?',
    a: 'After voting, DIGIMAT generates a unique cryptographic Receipt Code (e.g. SPR-8821-X9Y0-2024-NEP). You can input this code into our public Receipt Verifier tool at any time to verify that your encrypted ballot is included in the election ledger.'
  },
  {
    q: 'Is DIGIMAT compliant with Nepalese Electoral Standards?',
    a: 'Yes! DIGIMAT is engineered according to the National Cyber Security Framework of Nepal, supporting National Identity Card (NID) authentication, biometric OTP verification, and multi-signature auditing by election commissioners.'
  },
  {
    q: 'Can an administrator tamper with the election results?',
    a: 'No. DIGIMAT operates on a zero-trust model. Neither platform administrators nor server hosting providers hold the complete decryption key. Decryption requires a threshold threshold consensus of independent trustees (e.g., Election Officers, Supreme Court Observers, and Civil Society Reps).'
  }
];
