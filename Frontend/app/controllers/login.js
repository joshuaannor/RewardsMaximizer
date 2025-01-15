import Controller from '@ember/controller';
import { tracked } from '@glimmer/tracking';
import { action } from '@ember/object';
import { service } from '@ember/service';

export default class Login extends Controller {
  @tracked username = '';
  @tracked password = '';
  @service router;

  @action
  setUsername(event) {
    this.username = event.target.value;
  }

  @action
  setPassword(event) {
    this.password = event.target.value;
  }

  @action
  async handleLogin(event) {
    event.preventDefault();

    // const formBody = JSON.stringify({
    //   username: this.username,
    //   password: this.password,
    // });

    try {
      // COMMENTED OUT SO IT DOESNT BREAK BACKEND(current login API call causes stack overflow)
      // let response = await fetch('http://localhost:8080/login', {
      //   method: 'POST',
      //   headers: {
      //     'Content-Type': 'application/json',
      //   },
      //   body: formBody,
      // });
      // console.log(response);

      // if (!response.ok) {
      //   throw new Error('Login failed');
      // }
      // Save username in session storage
      sessionStorage.setItem('username', this.username);
      this.router.transitionTo('home');
    } catch (error) {
      this.errorMessage = error.message || 'Error logging in';
    }
  }

  @action
  handleRegistration(event) {
    event.preventDefault();
    this.router.transitionTo('registration');
  }
}
