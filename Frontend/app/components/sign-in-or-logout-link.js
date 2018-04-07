import Component from '@ember/component';
import { inject as service } from '@ember/service';
import $ from 'jquery';

export default Component.extend({
    session: service('auth-service'),
    tagName:'', // no tag, no ember view :)
    actions: {
        invalidateSession() {
            this.get('session').invalidate();
        },
        renderLogin() {
            $('#sign-in-paper').show("slow");
        },
        hideLogin() {
            $('#sign-in-paper').slideUp();
        }
    }
});
