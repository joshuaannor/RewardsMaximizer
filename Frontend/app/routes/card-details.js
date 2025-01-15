import Route from '@ember/routing/route';

export default class CardDetailsRoute extends Route {
  async model(params) {
    try {
      const response = await fetch(`http://localhost:8080/card/${params.cardId}`);
      if (response.ok) {
        const cardDetails = await response.json();
        return cardDetails; // Return the card details as the model for the route
      } else {
        console.error(`Failed to fetch card details for ID ${params.cardId}`);
        return null; // Ensure null is returned if fetching fails
      }
    } catch (error) {
      console.error('Error fetching card details:', error);
      return null;
    }
  }

  setupController(controller, model) {
    super.setupController(controller, model);
    if (model) {
      controller.loadCardDetails(); // Ensure card details are fetched when the model is ready
    }
  }
}
