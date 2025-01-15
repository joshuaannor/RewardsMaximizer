import Controller from '@ember/controller';
import { action } from '@ember/object';
import { tracked } from '@glimmer/tracking';

export default class CardDetailsController extends Controller {
  @tracked rating = 0; // Store the rating state
  @tracked card = {}; // Store the current card details

  get cardId() {
    return this.model?.card_id; // Safely get the card ID from the model
  }

  async loadCardDetails() {
    try {
      if (!this.cardId) {
        throw new Error('Model or card ID is not set.');
      }

      const response = await fetch(`http://localhost:8080/card/${this.cardId}`);
      if (response.ok) {
        const cardData = await response.json();
        this.card = cardData;
        this.rating = cardData.rating || 0; // Set the initial rating
      } else {
        console.error('Failed to fetch card details.');
      }
    } catch (error) {
      console.error('Error setting card details:', error);
    }
  }

  @action
  async setRating(newRating) {
    try {
      if (!this.cardId) {
        console.error('Card ID is not set.');
        return;
      }

      this.rating = newRating; // Optimistically update the rating locally
      const response = await fetch(`http://localhost:8080/card/${this.cardId}/rating`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ rating: newRating }),
      });

      if (!response.ok) {
        console.error('Failed to update card rating.');
      } else {
        console.log('Rating updated successfully.');
      }
    } catch (error) {
      console.error('Network error while updating rating:', error);
    }
  }
}
