import Controller from '@ember/controller';
import { action } from '@ember/object';
import { service } from '@ember/service';

export default class Search extends Controller {
  @service router;

  // Get session storage username
  constructor() {
    super(...arguments);
    this.username = sessionStorage.getItem('username');
    console.log(this.username);
    sessionStorage.setItem('username', this.username);
  }

  // Future functionality for product search
  @action
  results(event) {
    event.preventDefault();
    this.router.transitionTo('search_results');
  }
}
