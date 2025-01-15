import Component from '@glimmer/component';
import { action } from '@ember/object';
import { service } from '@ember/service';

export default class DropdownMenuComponent extends Component{
    @service router;

  @action
  initializeDropdown(element){
    const dropdown = new bootstrap.Dropdown(element.querySelector('.dropdown-toggle'));

    element.querySelector('#dropdownMenuButton').addEventListener('click', function(){
      this.classList.toggle('active');
    });

    if (element.querySelector('#dropdownMenuButton').classList.contains('show')){
      element.querySelector('#dropdownMenuButton').classList.add('active');
    } 
    else{
      element.querySelector('#dropdownMenuButton').classList.remove('active');
    }
  }

  @action
  navigateTo(route, event) {
    event.preventDefault();
    this.router.transitionTo(route);
  }

  // Navigate to home page
  @action
  home(event) {
    this.navigateTo('home', event);
  } 

  // Navigate to wallet
  @action
  wallet(event) {
    this.navigateTo('wallet', event);
  }

  // Navigate to search
  @action
  search(event) {
    this.navigateTo('search', event);
  }

  // Navigate to Rewards
  @action
  rewards(event) {
      this.navigateTo('rewards', event);
  }

  // Navigate to About us
  @action
    about(event) {
        this.navigateTo('about', event);
    }

  // Navigate to profile
  @action
  profile(event) {
    this.navigateTo('profile', event);
  }

  // Navigate to crowdsourcing
  @action
  crowdsourcing(event) {
    this.navigateTo('crowdsourcing', event);
  }

  // Logout and navigate to login page
  // Possibly need to modify API call to logout
    @action
    async signOut(event) {
    //await fetch('http://localhost:8080/logout', {});
    // End session storage
    sessionStorage.removeItem('username');
    this.navigateTo('login', event);
  }
}