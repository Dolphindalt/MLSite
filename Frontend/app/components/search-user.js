import Component from '@ember/component';
import $ from 'jquery';
import { inject as service } from '@ember/service';

export default Component.extend({
    tagName: 'div',
    classNames: ['search-bar-wrapper'],
    router: service(),
    actions: {
        search() {
            let value = $("input")[0].value;
            if(value == null || value === "")
                return;
            this.get('router').transitionTo('home.search', value);
        }
    }
});
