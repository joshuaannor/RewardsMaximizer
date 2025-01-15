import Controller from '@ember/controller';
export default class About extends Controller {
    // Get session storage username
    constructor() {
        super(...arguments);
        this.username = sessionStorage.getItem('username');
        console.log(this.username);
        sessionStorage.setItem('username', this.username);
    }
}