import React from "react";
import { useNavigate } from "react-router";
import { NavigationUtils } from "../../utils/NavigationUtils";
import { AppPath } from "../../constants/AppPath";
import { HttpClient } from "../../utils/HttpClient";
import { useEffectCache } from "../hooks";
import { AppCache } from "../../constants/AppCache";
import { CacheUtils } from "../../utils/CacheUtils";

/**
 * Protected Route Component
 *
 * Restricts access to authenticated users only.
 * Shows loading state while checking authentication.
 * Redirects to auth page if user is not authenticated.
 */
export function ProtectedRoute({ children }) {
  // navigate
  const navigate = useNavigate();
  // states
  const isLogin = useEffectCache(AppCache.isLogin);
  const checkAuth = React.useRef(false);
  // init callbacks
  React.useEffect(() => {
    if (isLogin === false) {
      NavigationUtils.replacePage(navigate, AppPath.auth);
    }
  }, [isLogin, navigate]);
  // check auth
  React.useEffect(() => {
    if (checkAuth.current) return;
    checkAuth.current = true;
    HttpClient.signcheck()
      .then((isAuthenticated) => {
        CacheUtils.set(AppCache.isLogin, isAuthenticated);
      });
  }, []);
  // result
  return children;
}
