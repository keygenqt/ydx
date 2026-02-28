import React from "react";
import { useLocation } from "react-router";
import { useSelector, useDispatch } from "react-redux";

import { setData } from "../../store/impl/pageScroll";

/**
 * Get windows size
 *
 * @returns {{x: number, y: number}}
 */
export function useEffectPageScroll() {
  const { pathname } = useLocation();
  const pageScroll = useSelector((state) => state.pageScroll.data);
  const dispatch = useDispatch();
  React.useLayoutEffect(() => {
    const el = document.getElementById("root");
    const handleWindowScroll = () => {
      dispatch(
        setData({
          pathname: pathname,
          y: el.scrollTop,
        }),
      );
    };
    el.addEventListener("scroll", handleWindowScroll);
    return () => {
      el.removeEventListener("scroll", handleWindowScroll);
    };
  });
  return pageScroll[pathname] ?? 0;
}
