import React from "react";

/**
 * Save cache
 */
export function useEffectCache(key) {
  const parse = (data) => {
    try {
      return JSON.parse(data);
    } catch (e) {
      return null;
    }
  };
  const [value, setValue] = React.useState(parse(localStorage.getItem(key)));
  React.useLayoutEffect(() => {
    const element = document.querySelector("#root");
    const observer = new MutationObserver(function (mutations) {
      mutations.forEach(function (mutation) {
        if (mutation.type === "attributes") {
          try {
            setValue(JSON.parse(localStorage.getItem(key)));
          } catch (e) {
            setValue(null);
          }
        }
      });
    });
    observer.observe(element, {
      attributes: true,
    });
    return () => {
      observer.disconnect();
    };
  }, [key]);
  return value;
}
