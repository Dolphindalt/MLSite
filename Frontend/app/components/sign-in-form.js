import Component from '@ember/component';
import SHA256 from 'cryptojs/sha256';
import { inject as service } from '@ember/service';
import $ from 'jquery';

export default Component.extend({
    classNames: ['sign-in-form'],
    tagName: '',
    auth_service: service('auth-service'),
    errorMessage: "",
    actions: {
        sign_in() {
            let { username, passwd } = this.getProperties('username', 'passwd');

            var hashword = SHA256(passwd).toString();
            this.set('errorMessage', '');

            (function(component) {
                component.get('auth_service').authenticate({
                    "username":username,
                    "hashword":hashword
                }).then(() => {
                    component.send('hideLogin');
                }).catch(() => {
                    // thankyou
                    component.set('errorMessage', "Incorrect username or password");
                });
                // route to the home page
            }) (this);
        },
        hideLogin() {
            this.set('errorMessage', "");
            $('#sign-in-paper').hide();
        }
    }
});
