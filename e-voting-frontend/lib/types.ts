export interface User {
  id: string;
  email: string;
  full_name: string;
  role: 'admin' | 'voter';
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface UserResponse {
  id: string;
  email: string;
  full_name: string;
  role: 'admin' | 'voter';
}

export interface LoginRequest {
  email: string;
  password: string;
}

export interface LoginResponse {
  token: string;
  user: UserResponse;
}

export interface Voter {
  id: string;
  user_id: string;
  voter_id: string;
  national_id: string;
  date_of_birth: string;
  address?: string;
  phone?: string;
  is_verified: boolean;
  created_at: string;
  updated_at: string;
}

export interface RegisterVoterRequest {
  email: string;
  password: string;
  full_name: string;
  voter_id: string;
  national_id: string;
  date_of_birth: string;
  address?: string;
  phone?: string;
}

export type ElectionStatus = 'draft' | 'open' | 'closed' | 'archived';

export interface Election {
  id: string;
  title: string;
  description?: string;
  election_type: string;
  status: ElectionStatus;
  start_time: string;
  end_time: string;
  created_by: string;
  created_at: string;
  updated_at: string;
}

export interface CreateElectionRequest {
  title: string;
  description?: string;
  election_type: string;
  start_time: string;
  end_time: string;
}

export interface UpdateElectionRequest {
  title?: string;
  description?: string;
  election_type?: string;
  start_time?: string;
  end_time?: string;
}

export interface Candidate {
  id: string;
  election_id: string;
  party_id?: string;
  full_name: string;
  photo_url?: string;
  biography?: string;
  position_number: number;
  created_at: string;
  updated_at: string;
}

export interface CreateCandidateRequest {
  election_id: string;
  party_id?: string;
  full_name: string;
  photo_url?: string;
  biography?: string;
  position_number: number;
}

export interface UpdateCandidateRequest {
  full_name?: string;
  photo_url?: string;
  biography?: string;
  position_number?: number;
}

export interface CastVoteRequest {
  election_id: string;
  candidate_id: string;
}

export interface VoteResponse {
  verification_code: string;
  message: string;
}

export interface VoteStatusResponse {
  has_voted: boolean;
}

export interface Ballot {
  id: string;
  election_id: string;
  voter_id: string;
  candidate_id: string;
  verification_code_hash: string;
  cast_at: string;
}

export interface ElectionResult {
  id: string;
  election_id: string;
  candidate_id: string;
  candidate_name: string;
  party_name?: string;
  vote_count?: number;
  vote_percentage?: number;
  created_at: string;
  updated_at: string;
}

export interface ChangePasswordRequest {
  old_password: string;
  new_password: string;
}

export interface ApiResponse<T> {
  success: boolean;
  data: T;
}

export interface MessageResponse {
  message: string;
}

export interface DashboardStats {
  total_elections: number;
  active_elections: number;
  total_voters: number;
  verified_voters: number;
  total_votes_cast: number;
}
