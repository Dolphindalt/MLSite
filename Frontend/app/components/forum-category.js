import Component from '@ember/component';
import $ from 'jquery';
import { run } from '@ember/runloop';

export default Component.extend({
    tagName: '',
    didReceiveAttrs() {
        let comp = this;
        $.getJSON("http://127.0.0.1:8000/forums/stats/category/" + this.get('category_name')).done((data) => {
            run(() => {
                comp.set('data', data);
            });
        });
    }
});
