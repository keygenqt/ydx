import { createBrowserRouter, RouterProvider, redirect } from "react-router";
import { BaseLayout } from "./layouts/BaseLayout";
import { ProtectedRoute } from "./components/ui";
import { AuthPage, ErrorPage, HomePage, ManagerPage } from "./pages";
import { CacheUtils } from "./utils/CacheUtils";
import { AppCache } from "./constants/AppCache";
import { AppPath } from "./constants/AppPath";

const protectedLoader = async () => {
  const isLogin = CacheUtils.get(AppCache.isLogin) === 'true';
  return isLogin === false ? redirect(AppPath.auth) : null;
};

const router = createBrowserRouter([
  {
    path: "/",
    children: [
      {
        index: true,
        element: (
          <BaseLayout>
            <HomePage />
          </BaseLayout>
        ),
      },
      {
        path: "auth",
        element: (
          <BaseLayout isCenter={true}>
            <AuthPage />
          </BaseLayout>
        ),
      },
      {
        path: "manager",
        loader: protectedLoader,
        element: (
          <ProtectedRoute>
            <BaseLayout isCenter={true}>
              <ManagerPage />
            </BaseLayout>
          </ProtectedRoute>
        ),
      },
      {
        path: "*",
        element: (
          <BaseLayout isCenter={true}>
            <ErrorPage />
          </BaseLayout>
        ),
      },
    ],
  },
]);

export function AppRoute() {
  return <RouterProvider router={router} />;
}
