'use client';

import { UserResponse } from './types';

export const setAuth = (token: string, user: UserResponse) => {
  localStorage.setItem('token', token);
  localStorage.setItem('user', JSON.stringify(user));
};

export const getAuth = (): { token: string | null; user: UserResponse | null } => {
  if (typeof window === 'undefined') {
    return { token: null, user: null };
  }

  const token = localStorage.getItem('token');
  const userStr = localStorage.getItem('user');
  const user = userStr ? JSON.parse(userStr) : null;

  return { token, user };
};

export const clearAuth = () => {
  localStorage.removeItem('token');
  localStorage.removeItem('user');
};

export const isAuthenticated = (): boolean => {
  const { token } = getAuth();
  return !!token;
};

export const isAdmin = (): boolean => {
  const { user } = getAuth();
  return user?.role === 'admin';
};

export const isVoter = (): boolean => {
  const { user } = getAuth();
  return user?.role === 'voter';
};
