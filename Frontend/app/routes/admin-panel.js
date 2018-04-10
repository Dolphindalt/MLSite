import Route from '@ember/routing/route';
import {inject as service } from '@ember/service';

export default Route.extend({
    session: service('auth-service'),
    beforeModel() {
        if(this.get('session').get('admin') !== true) {
            this.transitionTo('index');
        }
    }
});
