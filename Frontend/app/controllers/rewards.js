import Controller from '@ember/controller';

export default class RewardsController extends Controller {
  constructor() {
    super(...arguments);
    this.username = sessionStorage.getItem('username') || '';
    this.getRewards()
  }

  async getRewards() {
    try {
      const response = await fetch('http://localhost:8080/viewRewards');
      if (response.ok) {
        const rewardsData = await response.json();
        this.set('model', rewardsData);
      } else {
        console.error('Failed to fetch rewards.');
      }
    } catch (error) {
      console.error('Network error:', error);
    }
  }
}
