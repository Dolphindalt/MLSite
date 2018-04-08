import Component from '@ember/component';
import { inject as service } from '@ember/service';
import $ from 'jquery';

export default Component.extend({
    router: service(),
    session: service('auth-service'),
    tagName:'', // no tag, no ember view :)
    actions: {
        invalidateSession() {
            this.get('session').invalidate();
            this.get('router').transitionTo('index');
        },
        renderLogin() {
            $('#sign-in-paper').show();
        }
    }
});
