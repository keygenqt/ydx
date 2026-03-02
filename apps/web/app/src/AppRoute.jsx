import { createBrowserRouter, RouterProvider } from "react-router";
import { BaseLayout } from "./layouts/BaseLayout";
import { ProtectedRoute } from "./components/ui";
import { AuthPage, ErrorPage, HomePage, ManagerPage } from "./pages";

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
