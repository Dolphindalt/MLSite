import Component from '@ember/component';
import SHA256 from 'cryptojs/sha256';
import { inject } from '@ember/service';

export default Component.extend({
    session: inject('session'),
    errorMessage: "",
    actions: {
        success() {
            
        },
        sign_in() {
            let { username, passwd } = this.getProperties('username', 'passwd');

            var hashword = SHA256(passwd).toString();
            var comp = this;

            var auth = this.get('session').authenticate('authenticator:auth', username, hashword);
            
            auth.then(function(value) {
                // cookies and remember me maybe?
                comp.send('success');
            }, function(reason) {
                comp.set('errorMessage', reason);
            });
        }
    }
});
