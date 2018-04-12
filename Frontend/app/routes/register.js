import Route from '@ember/routing/route';
import $ from 'jquery';

export default Route.extend({
    model(params) {
        let stuff = $.getJSON('http://localhost:8000/register/' + params.linkUuid);
        stuff.fail(() => {
            this.transitionTo('index');
        });
        return stuff;
    }
});
