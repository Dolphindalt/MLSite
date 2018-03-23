import Route from '@ember/routing/route';
import { inject } from '@ember/service';

export default Route.extend({
    session: inject('session'),
    identification: '',
    passwd: '',
    actions: {
        authenticate() {
            let { identification, password } = this.getProperties('identification', 'passwd');
            this.get('session').authenticate('authenticator:oauth2', identification, password).catch((reason) => {
                this.set('errorMessage', reason.error || reason);
            });
        }
    }
});
