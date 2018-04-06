import DS from 'ember-data';
import { inject as service } from '@ember/service';
import { computed } from '@ember/object';

export default DS.JSONAPIAdapter.extend({
    session: service('auth-service'),
    host: 'http://localhost:8000',
    headers: computed('session.token', function() {
        return {
            'Authorization': this.get('session.token')
        };
    })
});
