import Route from '@ember/routing/route';

export default class WalletRoute extends Route {
  async model() {
    try {
      const response = await fetch('http://localhost:8080/cards');
      if (response.ok) {
        const cards = await response.json();
        return cards; // Return the list of cards as the model
      } else {
        console.error('Failed to fetch cards.');
        return [];
      }
    } catch (error) {
      console.error('Network error while fetching cards:', error);
      return [];
    }
  }
}
