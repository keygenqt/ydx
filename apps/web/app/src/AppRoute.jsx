import { createBrowserRouter, RouterProvider } from "react-router";

import { BaseLayout } from "./layouts/BaseLayout";

import {
  ErrorPage,
  HomePage,
} from "./pages";

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
