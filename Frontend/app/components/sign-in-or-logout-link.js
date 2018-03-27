import Component from '@ember/component';
import { inject } from '@ember/service';

export default Component.extend({
    session: inject('session'),
    tagName:'',
    actions: {
        invalidateSession() {
            this.get('session').invalidate();
        }
    }
});
