import { helper } from '@ember/component/helper';

/**
 * Returns true if the first value is less than the second value.
 * @param {Array} params - The parameters passed to the helper. Expected [a, b].
 * @returns {boolean} True if a < b, otherwise false.
 */
export default helper(function lt([a, b]) {
  return a < b;
});