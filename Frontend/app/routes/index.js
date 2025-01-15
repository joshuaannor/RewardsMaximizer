import Route from '@ember/routing/route';
import { service } from '@ember/service';

// Direct to login page
export default class IndexRoute extends Route {
  @service router;

  beforeModel(/* transition */) {
    this.router.transitionTo('login'); // Implicitly aborts the on-going transition.
  }
}
