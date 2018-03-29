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

            let data = JSON.parse(JSON.stringify({"identification":username, "password":hashword}));
            
            this.get('session').authenticate('authenticator:jwt', data).catch((reason) => {
                comp.set('errorMessage', reason);
            }).then(() => {
                comp.get('router').transitionTo('index');
            });
        }
    }
});
