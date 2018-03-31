import DS from 'ember-data';
import DataAdapterMixin from 'ember-simple-auth/mixins/data-adapter-mixin';
import { inject } from '@ember/service'; 
import { computed } from '@ember/object';

export default DS.JSONAPIAdapter.extend(DataAdapterMixin, {
    authorize(xhr) {
      let { access_token } = this.get('session.data.authenticated');
      console.debug(access_token);
      xhr.setRequestHeader('Authorization', `Bearer ${access_token}`);
    }
  });
  