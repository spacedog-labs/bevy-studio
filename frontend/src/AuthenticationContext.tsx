import React, { useContext } from "react";
import { writeStorage } from "@rehooks/local-storage";

const ACCESS_TOKEN = "access_token";

export type AuthenticationContext = {
  client_id: string;
  auth_uri: string;
  redirect_uri: string;
  logged_in: boolean;
  access_token: string;
  login: (context: AuthenticationContext) => void;
  logout: () => void;
  setAccessToken: (access_token: string) => void;
};

let local_token = localStorage.getItem(ACCESS_TOKEN);

export const GithubContext = React.createContext<AuthenticationContext>({
  client_id: "705625596ca39ae3136d",
  auth_uri: "https://github.com/login/oauth/authorize",
  redirect_uri: window.location.origin + "/auth",

  logged_in: local_token != null,
  access_token: local_token!,

  login: (context: AuthenticationContext) => {
    const auth_url = build_auth_url(context);
    window.open(auth_url, "_self");
  },

  logout: () => {
    localStorage.removeItem(ACCESS_TOKEN);
    window.open(window.origin, "_self");
  },

  setAccessToken: (access_token) => {
    writeStorage(ACCESS_TOKEN, access_token);
  },
});

export const build_auth_url = (context: AuthenticationContext): string => {
  return `${context.auth_uri}?client_id=${
    context.client_id
  }&redirect_uri=${build_redirect_url(context)}&scope=openid profile`;
};

export const build_redirect_url = (context: AuthenticationContext) => {
  return `${context.redirect_uri}?route=${window.location.pathname}`;
};

export const useAuth = (): AuthenticationContext => {
  return useContext<AuthenticationContext>(GithubContext);
};
