import DS from 'ember-data';
import { inject as service} from '@ember/service';

export default DS.RESTAdapter.extend({
    session: service('auth-service'),
    host: "http://127.0.0.1:8000",
    ajax: function(url, type, hash) {
        console.debug("Why");
        hash = hash || {};
        hash.headers = hash.headers || {};
        hash.headers['Authorization'] = 'Bearer ' + this.get('session').get('token');
        hash.crossDomain = true;
        return this._super(url, type, hash);
    }
});
