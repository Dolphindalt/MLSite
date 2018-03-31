import Component from '@ember/component';
import SHA256 from 'cryptojs/sha256';
import { inject as service } from '@ember/service';

export default Component.extend({
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
                }).catch(() => {
                    // thankyou
                    component.set('errorMessage', "Incorrect username or password");
                });
                // route to the home page
            }) (this);
        }
    }
});
