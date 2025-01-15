import { helper } from '@ember/component/helper';

export default helper(function categoryIcon([category]) {
  switch (category?.toLowerCase()) {
    case 'travel':
      return 'fa-plane';
    case 'shopping':
      return 'fa-shopping-cart';
    case 'dining':
      return 'fa-utensils';
    case 'cash-back':
      return 'fa-money-bill-wave';
    case 'groceries':
      return 'fa-apple-alt';
    case 'fuel':
      return 'fa-gas-pump';
    case 'other':
      return 'fa-star'; // A fun icon for 'Other'
    default:
      return 'fa-question-circle'; // Default fallback
  }
});
