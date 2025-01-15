import { helper } from '@ember/component/helper';

export function range([length]) {
  return Array.from({ length }, (_, index) => index + 1); // Create an array [1, 2, 3, ...]
}

export default helper(range);
