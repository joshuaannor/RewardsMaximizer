import Controller from '@ember/controller';
import { tracked } from '@glimmer/tracking';
import { action } from '@ember/object';

export default class Crowdsourcing extends Controller {
  @tracked rating = 0; // User rating
  @tracked comment = '';  // User comment
  @tracked created = new Date().toISOString(); 
  @tracked updated = new Date().toISOString(); 
  @tracked companyID = 1; // Hard coded company ID
  @tracked user_id = 0; // User ID
  // create a random number with four digits for feedback_id
  @tracked feedback_id = Math.floor(1000 + Math.random() * 9000);

  // Get session storage username
  constructor() {
    super(...arguments);
    this.username = sessionStorage.getItem('username');
    console.log(this.username);
    sessionStorage.setItem('username', this.username);
  }

  @action
  setComment(event) {
    this.comment = event.target.value;
  }

  @action
  setRating(event) {
    this.rating = parseInt(event.target.value);
    console.log(this.rating);
  }

  // Submit reward card recommendation
  @action
  async submitRecommendation(event) {
    // Get all user data
    try {
      const response = await fetch(`http://localhost:8080/users/${this.username}`);
      const data = await response.json();
      this.user_data = data; 
      console.log('Fetched user data:', this.user_data);
      // Search user data for matching username and assign user_id
      if (Array.isArray(this.user_data)) {
        const user = this.user_data.find(user => user.username === this.username);
        if (user) {
          this.user_id = user.user_id;
          console.log('User ID:', this.user_id);
        } 
      } 
      else if (this.user_data.username === this.username) {
        this.user_id = this.user_data.user_id;
        console.log('User ID:', this.user_id);
      } 
    } 
    catch (error) {
      console.error('Error:', error);
    }

    console.log('Feedback ID:', this.feedback_id);
    console.log('User ID:', this.user_id);
    console.log('Company ID:', this.companyID);
    console.log('Rating:', this.rating);
    console.log('Comment:', this.comment);
    console.log('Created:', this.created);
    console.log('Updated:', this.updated);

    // Post recommendation
    try {
      const response = await fetch('http://localhost:8080/crowdsourcing', {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify({
          feedback_id: this.feedback_id,
          user_id: this.user_id,
          company_id: this.companyID,
          rating: this.rating,
          comments: this.comment,
          created: this.created,
          updated: this.updated
        }),
      });

      if (response.ok) {
        this.errorMessage = 'Recommendation submitted!';
      }
      else{
        const error = await response.text();
        this.errorMessage = `Error submitting recommendation: ${error}`;
        throw new Error("Recommendation submission failed");
      }
    } 
    catch (error) {
      console.error('Error:', error);
      this.errorMessage = 'Error submitting recommendation';
    }
  }
}
