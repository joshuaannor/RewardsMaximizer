import Controller from '@ember/controller';
import { action } from '@ember/object';

export default class WalletController extends Controller {
  constructor() {
    super(...arguments);
    this.username = sessionStorage.getItem('username') || '';
    this.updateCards(); // Fetch cards on page load
  }

  async updateCards() {
    try {
      const response = await fetch('http://localhost:8080/cards');
      if (response.ok) {
        const updatedCards = await response.json();
        this.set('model', updatedCards); // Update the model with fetched cards
      } else {
        console.error('Failed to fetch cards.');
      }
    } catch (error) {
      console.error('Network error:', error);
    }
  }

  @action
  async submitCard(event) {
    event.preventDefault();

    const formData = new FormData(event.target);
    const cardData = {
      name: formData.get('cardName') || 'Default Name',
      type: formData.get('cardType') || 'Credit',
      icon: formData.get('iconChoice') || 'fa-credit-card',
      color: formData.get('cardColor') || '#000000',
      benefits: formData.get('cardBenefits') || 'Default Benefits',
      category: formData.get('cardCategory') || 'Other',
      rating: 0,
      created: new Date().toISOString(),
      updated: new Date().toISOString(),
      company_id: 1,
    };

    try {
      const response = await fetch('http://localhost:8080/add_card', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(cardData),
      });

      if (response.ok) {
        alert('Card added successfully!');
        await this.updateCards(); // Refresh the card list
      } else {
        const error = await response.text();
        alert(`Error adding card: ${error}`);
      }
    } catch (error) {
      console.error('Network error while adding card:', error);
      alert('An error occurred while adding the card.');
    }
  }

  @action
  confirmDelete(cardId, event) {
    // Stop the default behavior (e.g., navigation)
    if (event) {
      event.preventDefault(); // Prevent default behavior
      event.stopPropagation(); // Stop further event propagation
    }

    // Show confirmation dialog
    if (confirm("Are you sure you want to delete this card?")) {
      this.deleteCard(cardId);
    }
  }

  @action
  async deleteCard(cardId) {
    try {
      const response = await fetch(`http://localhost:8080/delete_card/${cardId}`, {
        method: "DELETE",
      });

      if (response.ok) {
        alert("Card deleted successfully!");
        this.updateCards(); // Refresh card list after deletion
      } else {
        alert("Failed to delete the card. Please try again.");
      }
    } catch (error) {
      console.error("Error deleting card:", error);
      alert("An error occurred while deleting the card.");
    }
  }
}