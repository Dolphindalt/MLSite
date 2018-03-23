import Component from '@ember/component';
import { inject } from '@ember/service';

export default Component.extend({
    session: inject('session'),
    actions: {
        invalidateSession() {
            this.get('session').invalidate();
        }
    }
});
