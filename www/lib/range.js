export const range = (start, end) =>
  [...Array(end - start).keys()].map((_, i) => i + start);
